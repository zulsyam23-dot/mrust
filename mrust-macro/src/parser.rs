use proc_macro2::{Delimiter, Literal, TokenStream, TokenTree};

use crate::ast::{Element, Node};
use crate::error::{span_of, Error};

pub(crate) struct Parser {
    toks: Vec<TokenTree>,
    pos: usize,
}

impl Parser {
    pub(crate) fn new(input: TokenStream) -> Self {
        Self { toks: input.into_iter().collect(), pos: 0 }
    }

    pub(crate) fn parse(&mut self) -> Result<Vec<Node>, Error> {
        let mut out = Vec::new();
        while self.peek().is_some() {
            out.push(self.parse_node()?);
        }
        Ok(out)
    }

    fn peek(&self) -> Option<&TokenTree> {
        self.toks.get(self.pos)
    }
    fn peek2(&self) -> Option<&TokenTree> {
        self.toks.get(self.pos + 1)
    }
    fn next(&mut self) -> Option<TokenTree> {
        let t = self.toks.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }
    fn at_punct(&self, c: char) -> bool {
        matches!(self.peek(), Some(TokenTree::Punct(p)) if p.as_char() == c)
    }
    fn accept(&mut self, c: char) -> bool {
        if self.at_punct(c) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn expect(&mut self, c: char) -> Result<(), Error> {
        if self.accept(c) {
            Ok(())
        } else {
            Err(Error::new(span_of(self.peek()), format!("diharapkan `{c}`")))
        }
    }
    fn ident(&mut self, what: &str) -> Result<String, Error> {
        match self.next() {
            Some(TokenTree::Ident(i)) => Ok(i.to_string()),
            t => Err(Error::new(span_of(t.as_ref()), format!("diharapkan {what}"))),
        }
    }

    fn at_brace(&self) -> bool {
        matches!(self.peek(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace)
    }

    fn parse_node(&mut self) -> Result<Node, Error> {
        match self.peek() {
            Some(TokenTree::Punct(p)) if p.as_char() == '<' => {
                self.parse_element().map(Node::Element)
            }
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
                let g = match self.next() {
                    Some(TokenTree::Group(g)) => g,
                    _ => unreachable!(),
                };
                Ok(Node::Expr(g.stream()))
            }
            Some(TokenTree::Literal(_)) => {
                let l = match self.next() {
                    Some(TokenTree::Literal(l)) => l,
                    _ => unreachable!(),
                };
                Ok(Node::Text(l))
            }
            t => Err(Error::new(
                span_of(t),
                "node tidak dikenal: harapkan <tag>, literal, atau { ekspresi }",
            )),
        }
    }

    /// Node dalam konteks konten tag: element, `{ expr }`, atau teks polos.
    fn content_node(&mut self) -> Result<Node, Error> {
        if self.at_punct('<') {
            return self.parse_element().map(Node::Element);
        }
        if self.at_brace() {
            let g = match self.next() {
                Some(TokenTree::Group(g)) => g,
                _ => unreachable!(),
            };
            return Ok(Node::Expr(g.stream()));
        }
        self.text_run()
    }

    /// Rangkaian token teks (ident, literal, punct) sampai tag/`{`/akhir.
    /// Satu string literal tunggal dipertahankan apa adanya; selain itu token
    /// dirangkai menjadi satu string literal baru.
    fn text_run(&mut self) -> Result<Node, Error> {
        let mut toks = Vec::new();
        while let Some(t) = self.peek().cloned() {
            match &t {
                TokenTree::Punct(p) if p.as_char() == '<' => break,
                TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => break,
                _ => {
                    self.next();
                    toks.push(t);
                }
            }
        }
        if toks.is_empty() {
            return Err(Error::new(
                span_of(self.peek()),
                "diharapkan tag, literal, atau { ekspresi }",
            ));
        }
        match toks.as_slice() {
            // pertahankan literal string apa adanya; literal lain (angka, dsb)
            // menjadi teks biasa
            [TokenTree::Literal(l)] if l.to_string().starts_with('"') => Ok(Node::Text(l.clone())),
            _ => {
                let text = toks.iter().map(|t| t.to_string()).collect::<String>();
                Ok(Node::Text(Literal::string(&text)))
            }
        }
    }

    fn parse_element(&mut self) -> Result<Element, Error> {
        let open = self.next().expect("<");
        let name = self.ident("nama tag")?;
        let mut attrs = Vec::new();

        loop {
            if self.at_punct('>') {
                self.next();
                break;
            }
            if self.at_punct('/') {
                self.next();
                self.expect('>')?;
                return Ok(Element {
                    name,
                    attrs,
                    children: Vec::new(),
                    span: open.span(),
                    cond: None,
                    disabled: None,
                });
            }
            if self.at_punct(',') {
                self.next();
                continue;
            }
            let attr = self.ident("nama attribute")?;
            let value = if self.accept('=') { Some(self.attr_value(&name)?) } else { None };
            attrs.push((attr, value));
        }

        let mut children = Vec::new();
        loop {
            if self.at_punct('<')
                && matches!(self.peek2(), Some(TokenTree::Punct(p)) if p.as_char() == '/')
            {
                self.next();
                self.next();
                let close = self.ident("nama tag penutup")?;
                if close != name {
                    return Err(Error::new(
                        open.span(),
                        format!("tag penutup `</{close}>` tak cocok dengan pembuka `<{name}>`"),
                    ));
                }
                self.expect('>')?;
                break;
            }
            if self.peek().is_none() {
                return Err(Error::new(open.span(), format!("tag `{name}` tidak ditutup")));
            }
            children.push(self.content_node()?);
        }

        Ok(Element { name, attrs, children, span: open.span(), cond: None, disabled: None })
    }

    fn attr_value(&mut self, tag: &str) -> Result<TokenStream, Error> {
        let mut out = TokenStream::new();
        loop {
            if self.at_punct('>') || self.at_punct('/') || self.at_punct(',') {
                break;
            }
            // berhenti saat bertemu awal attribute berikutnya: ident + `=`
            if let (Some(TokenTree::Ident(_)), Some(TokenTree::Punct(p))) = (self.peek(), self.peek2())
            {
                if p.as_char() == '=' {
                    break;
                }
            }
            match self.next() {
                Some(t) => out.extend([t]),
                None => break,
            }
        }
        if out.is_empty() {
            return Err(Error::new(
                span_of(self.peek()),
                format!("nilai attribute di tag `{tag}` tidak boleh kosong"),
            ));
        }
        // nilai berupa satu grup `{ ... }` -> buka kurungnya
        let mut it = out.into_iter();
        let first = it.next();
        let rest: Vec<TokenTree> = it.collect();
        match (first.as_ref(), rest.is_empty()) {
            (Some(TokenTree::Group(g)), true) if g.delimiter() == Delimiter::Brace => Ok(g.stream()),
            _ => {
                let mut ts = TokenStream::new();
                if let Some(f) = first {
                    ts.extend([f]);
                }
                ts.extend(rest);
                Ok(ts)
            }
        }
    }
}