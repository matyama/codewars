use std::collections::hash_map::{Entry, HashMap};
use std::fmt::{Display, Write as _};
use std::iter::Peekable;
use std::str::Chars;

pub struct AssemblerInterpreter {}

impl AssemblerInterpreter {
    pub fn interpret(input: &str) -> Option<String> {
        let prg = match Program::parse(input) {
            Ok(prg) => prg,
            Err(error) => {
                eprintln!("{error}");
                return None;
            }
        };

        for (i, instr) in prg.asm.iter().enumerate() {
            println!("[{i:4}] {instr:?}");
        }
        println!("{:?}", prg.labels);

        match Self::eval(prg) {
            Ok(output) => Some(output),
            Err(error) => {
                eprintln!("{error:?}");
                None
            }
        }
    }

    fn eval(Program { src, asm, labels }: Program<'_>) -> AsmResult<String> {
        let mut output = String::new();
        let mut regs = Registers::default();
        let mut stack = Vec::new();
        let mut cmp = None;
        let mut pc = 0;
        let mut last = None;

        while let Some(AsmLine { instr, span }) = asm.get(pc) {
            println!("[{pc:4}] {instr:?}");
            println!("| {regs:?} {cmp:?} {stack:?}");

            match instr {
                Instr::End => return Ok(output),

                Instr::Ret => {
                    let Some(ip) = stack.pop() else {
                        return Err(Error {
                            code: snippet(src, span),
                            span: span.clone(),
                            source: "no stack pointer to return to".into(),
                        });
                    };

                    // return to the instruction that called this subroutine
                    pc = ip;
                }

                Instr::Call(label) => {
                    let Some(&ip) = labels.get(label) else {
                        return Err(Error {
                            code: snippet(src, span),
                            span: span.clone(),
                            source: format!("unknown {label:?}").into(),
                        });
                    };

                    // stash current PC and go to the first instruction of the subroutine
                    stack.push(pc + 1);

                    pc = ip;
                }

                Instr::Cmp(c) => {
                    let _ = cmp.insert(*c);
                    pc += 1;
                }

                Instr::Jmp { lbl, cond } => {
                    let Some(&ip) = labels.get(lbl) else {
                        return Err(Error {
                            code: snippet(src, span),
                            span: span.clone(),
                            source: format!("unknown {lbl:?}").into(),
                        });
                    };

                    let jmp = if let Some(cond) = cond {
                        let Some(Cmp(x, y)) = cmp else {
                            return Err(Error {
                                code: snippet(src, span),
                                span: span.clone(),
                                source: "no previous cmp instruction".into(),
                            });
                        };

                        let x = regs.val(&x);
                        let y = regs.val(&y);

                        match cond {
                            Cond::Eq => x == y,
                            Cond::Ne => x != y,
                            Cond::Ge => x >= y,
                            Cond::Gt => x > y,
                            Cond::Le => x <= y,
                            Cond::Lt => x < y,
                        }
                    } else {
                        true
                    };

                    if jmp {
                        pc = ip;
                    } else {
                        pc += 1
                    }
                }

                Instr::Unary { reg, op } => {
                    *regs.reg(reg) += match op {
                        RegOp::Inc => 1,
                        RegOp::Dec => -1,
                    };
                    pc += 1;
                }

                Instr::Binary { reg, val, op } => {
                    match op {
                        BinOp::Mov => *regs.reg(reg) = regs.val(val),
                        BinOp::Add => *regs.reg(reg) += regs.val(val),
                        BinOp::Sub => *regs.reg(reg) -= regs.val(val),
                        BinOp::Mul => *regs.reg(reg) *= regs.val(val),
                        BinOp::Div => match regs.val(val) {
                            0 => {
                                return Err(Error {
                                    code: snippet(src, span),
                                    span: span.clone(),
                                    source: "division by zero".into(),
                                });
                            }
                            val => *regs.reg(reg) /= val,
                        },
                    }
                    pc += 1;
                }

                Instr::Msg(args) => {
                    args.iter()
                        .try_for_each(|arg| match arg {
                            Literal::Ident(ident) => {
                                let reg = Reg(ident);
                                write!(output, "{}", regs.reg(&reg))
                            }
                            Literal::Text(text) => write!(output, "{text}"),
                            Literal::Const(val) => write!(output, "{val}"),
                        })
                        .expect("write program output");
                    pc += 1;
                }
            }

            println!("| {regs:?} {cmp:?} {stack:?}");
            let _ = last.insert(span);
        }

        let (code, span) = match last.take() {
            Some(span) => (snippet(src, span), span.clone()),
            None => {
                let span = Span {
                    offset: 0,
                    length: src.len(),
                    lineno: 0,
                    lineof: 0,
                };

                (snippet(src, &span), span.clone())
            }
        };

        Err(Error {
            code,
            span,
            source: "program ended prematurely".into(),
        })
    }
}

#[derive(Debug, Default)]
#[repr(transparent)]
struct Registers<'prg>(HashMap<Reg<'prg>, i64>);

impl<'prg> Registers<'prg> {
    #[inline]
    fn reg(&mut self, reg: &Reg<'prg>) -> &mut i64 {
        self.0.entry(*reg).or_default()
    }

    #[inline]
    fn val(&mut self, val: &Val<'prg>) -> i64 {
        match val {
            Val::Const(val) => *val,
            Val::Reg(reg) => *self.reg(reg),
        }
    }
}

#[derive(Debug)]
pub struct Program<'prg> {
    /// Raw input assembler program
    src: &'prg str,
    /// Parsed assembly code
    asm: Vec<AsmLine<'prg>>,
    /// Maps each label to its first instruction index in `asm`
    labels: HashMap<Label<'prg>, usize>,
}

impl<'prg> Program<'prg> {
    /// Parse the input assembly and perform basic static analysis.
    ///
    /// Currently only collects and caches positions of _label definitions_, so these don't have to
    /// be scanned for during interpretation.
    pub fn parse(src: &'prg str) -> AsmResult<Self> {
        let mut asm = Vec::new();
        let mut labels = HashMap::new();
        let mut i = 0;

        for line in Parser::new(src) {
            let AsmStmt { stmt, span } = line?;

            // Filter out labels and return just a list of _instructions_. Each label
            // definition in the `labels` map, which points to the index of the first
            // instruction after the original label definition.
            match stmt {
                Stmt::Label(label) => match labels.entry(label) {
                    Entry::Occupied(e) => {
                        return Err(Error {
                            code: snippet(src, &span),
                            span,
                            source: format!("{label:?} defined earlier on line {}", e.get()).into(),
                        });
                    }
                    Entry::Vacant(e) => {
                        let _ = e.insert(i);
                    }
                },

                Stmt::Instr(instr) => {
                    asm.push(AsmLine { instr, span });
                    i += 1;
                }
            }
        }

        Ok(Program { src, asm, labels })
    }
}

#[derive(Debug)]
struct AsmLine<'a> {
    instr: Instr<'a>,
    span: Span,
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid input at [{}]: '{code}'\n{source:?}", self.span.loc())]
pub struct Error {
    code: String,
    span: Span,
    source: Box<dyn std::error::Error + 'static>,
}

pub type AsmResult<T> = Result<T, Error>;

const SNIPPET_LIMIT: usize = 60;

fn snippet(src: &str, span: &Span) -> String {
    let s = &src[span.offset..span.end()];
    s[..s.len().min(SNIPPET_LIMIT)].to_string()
}

#[derive(Clone, Copy, Debug)]
struct Lexer<'prg>(&'prg str);

impl<'prg> Lexer<'prg> {
    #[inline]
    pub fn new(input: &'prg str) -> Self {
        Self(input)
    }
}

impl<'prg> IntoIterator for Lexer<'prg> {
    type Item = <TokenStream<'prg> as Iterator>::Item;
    type IntoIter = TokenStream<'prg>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        TokenStream {
            prg: self.0,
            pos: 0,
            chars: self.0.chars().peekable(),
            line: 0,
            line_pos: 0,
        }
    }
}

pub struct TokenStream<'prg> {
    /// Raw input program
    prg: &'prg str,
    /// Current position in the input `prg`
    pos: usize,
    /// Character stream from the input `prg`
    chars: Peekable<Chars<'prg>>,
    /// Current line number in the input `prg`
    line: usize,
    /// Offset of the start of current line
    line_pos: usize,
}

impl<'prg> TokenStream<'prg> {
    #[inline]
    fn valid(c: char) -> bool {
        matches!(c, '0'..='9' | '-' | '\'' | '_' | ',' | ':') || c.is_alphabetic()
    }

    #[inline]
    fn span(&self, offset: usize, length: usize) -> Span {
        Span {
            offset,
            length,
            lineno: self.line,
            lineof: self.line_pos,
        }
    }

    fn error(
        &self,
        start: usize,
        reason: impl Into<Box<dyn std::error::Error + 'static>>,
    ) -> Error {
        Error {
            code: self.prg[start..self.pos.min(start + SNIPPET_LIMIT)].to_string(),
            span: self.span(start, self.pos - start),
            source: reason.into(),
        }
    }

    // try to skip over to the next separator (whitespace or newline)
    fn extend_err(&mut self, start: usize, before: impl Display, after: char) -> Error {
        while self
            .chars
            .next_if(|&c| !c.is_whitespace() || c == '\n')
            .is_some()
        {
            self.pos += 1;
        }
        self.error(start, format!("unexpected '{after}' following '{before}'"))
    }

    fn number(&mut self, start: usize) -> AsmResult<LexToken<'prg>> {
        while self.chars.next_if(|c| matches!(c, '0'..='9')).is_some() {
            self.pos += 1;
        }

        let lexeme = &self.prg[start..self.pos];

        // numbers must be followed by a whitespace or newline
        match self.chars.peek().cloned() {
            None | Some('\n') => {}
            Some(c) if c.is_whitespace() => {}
            Some(c) => return Err(self.extend_err(start, lexeme, c)),
        }

        let lexeme = &self.prg[start..self.pos];

        let token = lexeme
            .parse()
            .map(Literal::Const)
            .map(Token::Literal)
            .map_err(|err| self.error(start, err))?;

        Ok(LexToken {
            lexeme,
            token,
            span: self.span(start, lexeme.len()),
        })
    }

    fn ident_or_keyword(&mut self, start: usize) -> AsmResult<LexToken<'prg>> {
        while self
            .chars
            .next_if(|&c| c.is_alphanumeric() || c == '_')
            .is_some()
        {
            self.pos += 1;
        }

        let lexeme = &self.prg[start..self.pos];

        let token = lexeme
            .trim()
            .try_into()
            .map_err(Literal::Ident)
            .map_or_else(Token::Literal, Token::Keyword);

        Ok(LexToken {
            lexeme,
            token,
            span: self.span(start, lexeme.len()),
        })
    }

    fn text(&mut self, start: usize) -> AsmResult<LexToken<'prg>> {
        loop {
            match self.chars.next() {
                Some(c) => {
                    self.pos += 1;
                    if c == '\'' {
                        break;
                    }
                }

                None => return Err(self.error(start, "non-terminated string".to_string())),
            }
        }

        let lexeme = &self.prg[start..self.pos];
        let text = lexeme.trim().trim_matches('\'');

        Ok(LexToken {
            lexeme,
            token: Token::Literal(Literal::Text(text)),
            span: self.span(start, lexeme.len()),
        })
    }
}

impl<'prg> Iterator for TokenStream<'prg> {
    type Item = AsmResult<LexToken<'prg>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(c) = self.chars.next() else {
                return None;
            };

            let start = self.pos;
            self.pos += 1;

            match c {
                // skip over line comments
                ';' => {
                    for c in self.chars.by_ref() {
                        self.pos += 1;
                        if c == '\n' {
                            break;
                        }
                    }
                    self.line += 1;
                    self.line_pos = self.pos;
                }

                // negative numbers
                '-' => match self.chars.peek().copied() {
                    Some('0'..='9') => return Some(self.number(start)),
                    Some(c) => return Some(Err(self.extend_err(start, '-', c))),
                    None => continue,
                },

                // positive numbers
                '0'..='9' => return Some(self.number(start)),

                // single-character tokens
                ',' | ':' => {
                    let token = match c {
                        ',' => Token::Comma,
                        ':' => Token::Colon,
                        _ => unreachable!("char matched above"),
                    };

                    let lexeme = &self.prg[start..self.pos];

                    return Some(Ok(LexToken {
                        lexeme,
                        token,
                        span: self.span(start, lexeme.len()),
                    }));
                }

                // consume a string literal
                '\'' => return Some(self.text(start)),

                // consume an identifier (or a keyword)
                c if c.is_alphabetic() => return Some(self.ident_or_keyword(start)),

                // skip over whitespaces
                c if c.is_whitespace() => {
                    if c == '\n' {
                        self.line += 1;
                        self.line_pos = self.pos;
                    }
                }

                // unsupported characters
                _ => {
                    // try to skip over to the next recognized character
                    while self.chars.next_if(|&c| !Self::valid(c)).is_some() {
                        self.pos += 1;
                    }

                    return Some(Err(
                        self.error(start, "unexpected charter sequence".to_string())
                    ));
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    /// Offset where the span starts
    pub offset: usize,
    /// Span length
    pub length: usize,
    /// Line number (including comments)
    pub lineno: usize,
    /// Offset where the line starts
    pub lineof: usize,
}

impl Span {
    /// ```
    /// # use codewars::assembler_interpreter::Span;
    /// let s = Span { offset: 2, length: 6, lineno: 0, lineof: 10 };
    /// assert_eq!(s.end(), 8);
    /// ```
    #[inline]
    pub fn end(&self) -> usize {
        self.offset + self.length
    }

    /// Position within the line of where the span starts
    #[inline]
    pub fn column(&self) -> usize {
        self.offset - self.lineof
    }

    #[inline]
    pub fn loc(&self) -> SpanLoc<'_> {
        SpanLoc(self)
    }
}

impl std::ops::Add<Span> for Span {
    type Output = Span;

    /// ```
    /// # use codewars::assembler_interpreter::Span;
    /// let s1 = Span { offset: 0, length: 6, lineno: 0, lineof: 0 };
    /// let s2 = Span { offset: 7, length: 3, lineno: 0, lineof: 0 };
    /// let s3 = s1 + s2;
    /// assert_eq!(&s3, &Span { offset: 0, length: 10, lineno: 0, lineof: 0 });
    /// assert_eq!(s3.end(), 10);
    ///
    /// let s1 = Span { offset: 2, length: 6, lineno: 0, lineof: 1 };
    /// let s2 = Span { offset: 14, length: 3, lineno: 1, lineof: 0 };
    /// let s3 = s1 + s2;
    /// assert_eq!(&s3, &Span { offset: 2, length: 15, lineno: 0, lineof: 1 });
    /// assert_eq!(s3.end(), 17);
    /// ```
    fn add(self, other: Span) -> Self::Output {
        self + &other
    }
}

impl<S: std::ops::Deref<Target = Span>> std::ops::Add<S> for Span {
    type Output = Span;

    fn add(mut self, other: S) -> Self::Output {
        self += other;
        self
    }
}

impl std::ops::Add for &Span {
    type Output = Span;

    fn add(self, other: Self) -> Self::Output {
        debug_assert!(self.end() <= other.offset);
        debug_assert!(self.lineno <= other.lineno);
        Span {
            length: self.length + (other.offset - self.end()) + other.length,
            ..*self
        }
    }
}

impl std::ops::AddAssign<Span> for Span {
    #[inline]
    fn add_assign(&mut self, other: Span) {
        self.add_assign(&other)
    }
}

impl<S: std::ops::Deref<Target = Span>> std::ops::AddAssign<S> for Span {
    fn add_assign(&mut self, other: S) {
        debug_assert!(self.end() <= other.offset);
        debug_assert!(self.lineno <= other.lineno);
        self.length += (other.offset - self.end()) + other.length;
    }
}

pub struct SpanLoc<'s>(&'s Span);

impl Display for SpanLoc<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.0.lineno, self.0.column(), self.0.length)
    }
}

#[derive(Debug)]
pub struct LexToken<'prg> {
    /// Raw input slice
    lexeme: &'prg str,

    /// Token corresponding to the lexeme
    token: Token<'prg>,

    /// Span of this token in the input
    span: Span,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    /// Argument separator
    Comma,
    /// Label separator
    Colon,
    /// Literal values (identifiers, strings, numbers)
    Literal(Literal<'a>),
    /// Assembler keyword
    Keyword(Keyword),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literal<'a> {
    /// Name (alphabetical) of either a register or label
    Ident(&'a str),
    /// String values with lexeme of the form `'some text'`
    Text(&'a str),
    /// Integral numeric values
    Const(i64),
}

impl Display for Literal<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(s) => f.write_str(s),
            Self::Text(s) => write!(f, "'{s}'"),
            Self::Const(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    Mov,
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
    Div,
    Jmp,
    Cmp,
    Jne,
    Je,
    Jge,
    Jg,
    Jle,
    Jl,
    Call,
    Ret,
    Msg,
    End,
}

impl<'a> TryFrom<&'a str> for Keyword {
    type Error = &'a str;

    fn try_from(ident: &'a str) -> Result<Self, Self::Error> {
        Ok(match ident {
            "mov" => Self::Mov,
            "inc" => Self::Inc,
            "dec" => Self::Dec,
            "add" => Self::Add,
            "sub" => Self::Sub,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "jmp" => Self::Jmp,
            "cmp" => Self::Cmp,
            "jne" => Self::Jne,
            "je" => Self::Je,
            "jge" => Self::Jge,
            "jg" => Self::Jg,
            "jle" => Self::Jle,
            "jl" => Self::Jl,
            "call" => Self::Call,
            "ret" => Self::Ret,
            "msg" => Self::Msg,
            "end" => Self::End,
            ident => return Err(ident),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Parser<'prg>(Lexer<'prg>);

impl<'prg> Parser<'prg> {
    #[inline]
    pub fn new(input: &'prg str) -> Self {
        Self(Lexer::new(input))
    }
}

impl<'prg> IntoIterator for Parser<'prg> {
    type Item = <AsmLines<'prg> as Iterator>::Item;
    type IntoIter = AsmLines<'prg>;

    fn into_iter(self) -> Self::IntoIter {
        let lexer = self.0;
        AsmLines {
            prg: lexer.0,
            tokens: lexer.into_iter().peekable(),
        }
    }
}

/// Iterator of assembler instructions ([`AsmStmt`]s)
pub struct AsmLines<'prg> {
    prg: &'prg str,
    tokens: Peekable<TokenStream<'prg>>,
}

impl<'prg> AsmLines<'prg> {
    #[inline]
    fn snippet(&self, span: &Span) -> String {
        snippet(self.prg, span)
    }

    fn comma(&mut self, mut span: Span) -> AsmResult<Span> {
        match self.tokens.next().transpose()? {
            None => Err(Error {
                code: self.snippet(&span),
                span: span.clone(),
                source: "expected the ',', but got none".into(),
            }),

            Some(
                token @ LexToken {
                    token: Token::Comma,
                    ..
                },
            ) => Ok(span + token.span),

            Some(other) => {
                span += other.span;
                Err(Error {
                    code: self.snippet(&span),
                    span,
                    source: format!("expected ',', got {:?}", other.token).into(),
                })
            }
        }
    }

    fn colon(&mut self, mut span: Span) -> AsmResult<Span> {
        match self.tokens.next().transpose()? {
            None => Err(Error {
                code: self.snippet(&span),
                span: span.clone(),
                source: "expected the ':', but got none".into(),
            }),

            Some(
                token @ LexToken {
                    token: Token::Colon,
                    ..
                },
            ) => Ok(span + token.span),

            Some(other) => {
                span += other.span;
                Err(Error {
                    code: self.snippet(&span),
                    span,
                    source: format!("expected ':', got {:?}", other.token).into(),
                })
            }
        }
    }

    fn literal(&mut self, mut span: Span) -> AsmResult<(Literal<'prg>, Span)> {
        match self.tokens.next().transpose()? {
            None => Err(Error {
                code: self.snippet(&span),
                span: span.clone(),
                source: "expected a literal token, but got none".into(),
            }),

            Some(LexToken {
                token: Token::Literal(lit),
                span: lit_span,
                ..
            }) => Ok((lit, span + lit_span)),

            Some(other) => {
                span += other.span;
                Err(Error {
                    code: self.snippet(&span),
                    span,
                    source: format!("expected a literal token, got {:?}", other.token).into(),
                })
            }
        }
    }

    fn ident(&mut self, span: Span) -> AsmResult<(&'prg str, Span)> {
        match self.literal(span)? {
            (Literal::Ident(ident), span) => Ok((ident, span)),
            (literal, span) => Err(Error {
                code: self.snippet(&span),
                span: span.clone(),
                source: format!("expected an identifier, got {literal:?}").into(),
            }),
        }
    }

    fn value(&mut self, mut span: Span) -> AsmResult<(Val<'prg>, Span)> {
        match self.tokens.next().transpose()? {
            None => Err(Error {
                code: self.snippet(&span),
                span: span.clone(),
                source: "expected an ident or const token, but got none".into(),
            }),

            Some(LexToken {
                token: Token::Literal(lit),
                span: lit_span,
                ..
            }) => {
                span += lit_span;

                let val = match lit {
                    Literal::Ident(reg) => Val::Reg(Reg(reg)),
                    Literal::Const(val) => Val::Const(val),
                    other => {
                        return Err(Error {
                            code: self.snippet(&span),
                            span,
                            source: format!("expected an ident or constant, got {other:?}").into(),
                        });
                    }
                };

                Ok((val, span))
            }

            Some(other) => {
                span += other.span;
                let source = format!("expected a literal token, got {:?}", other.token);
                Err(Error {
                    code: self.snippet(&span),
                    span,
                    source: source.into(),
                })
            }
        }
    }

    fn instruction(&mut self, keyword: Keyword, span: Span) -> AsmResult<AsmStmt<'prg>> {
        use Keyword::*;
        match keyword {
            End => Ok(AsmStmt {
                stmt: Stmt::Instr(Instr::End),
                span,
            }),
            Ret => Ok(AsmStmt {
                stmt: Stmt::Instr(Instr::Ret),
                span,
            }),
            Inc | Dec => self.unary(keyword, span),
            Mov | Add | Sub | Mul | Div => self.binary(keyword, span),
            Cmp => self.cmp(span),
            Jmp | Jne | Je | Jge | Jg | Jle | Jl => self.jmp(keyword, span),
            Call => {
                let (label, span) = self.ident(span)?;
                let stmt = Stmt::Instr(Instr::Call(Label(label)));
                Ok(AsmStmt { stmt, span })
            }
            Msg => self.msg(span),
        }
    }

    fn unary(&mut self, keyword: Keyword, span: Span) -> AsmResult<AsmStmt<'prg>> {
        let op = RegOp::try_from(keyword).map_err(|kw| Error {
            code: self.snippet(&span),
            span: span.clone(),
            source: format!("{kw:?} does not represent a unary instruction").into(),
        })?;

        let (reg, span) = self.ident(span)?;

        Ok(AsmStmt {
            stmt: Stmt::Instr(Instr::Unary { reg: Reg(reg), op }),
            span,
        })
    }

    fn binary(&mut self, keyword: Keyword, span: Span) -> AsmResult<AsmStmt<'prg>> {
        let op = BinOp::try_from(keyword).map_err(|kw| Error {
            code: self.snippet(&span),
            span: span.clone(),
            source: format!("{kw:?} does not represent a binary instruction").into(),
        })?;

        let (reg, span) = self.ident(span)?;
        let span = self.comma(span)?;
        let (val, span) = self.value(span)?;

        let instr = Instr::Binary {
            reg: Reg(reg),
            val,
            op,
        };

        Ok(AsmStmt {
            stmt: Stmt::Instr(instr),
            span,
        })
    }

    fn cmp(&mut self, span: Span) -> AsmResult<AsmStmt<'prg>> {
        let (x, span) = self.value(span)?;
        let span = self.comma(span)?;
        let (y, span) = self.value(span)?;

        Ok(AsmStmt {
            stmt: Stmt::Instr(Instr::Cmp(Cmp(x, y))),
            span,
        })
    }

    fn label(&mut self, label: Label<'prg>, span: Span) -> AsmResult<AsmStmt<'prg>> {
        Ok(AsmStmt {
            stmt: Stmt::Label(label),
            span: self.colon(span)?,
        })
    }

    fn jmp(&mut self, keyword: Keyword, span: Span) -> AsmResult<AsmStmt<'prg>> {
        let cond = <Option<Cond>>::try_from(keyword).map_err(|kw| Error {
            code: self.snippet(&span),
            span: span.clone(),
            source: format!("{kw:?} does not represent a jump instruction").into(),
        })?;

        // parse a label identifier (i.e., don't attempt to parse a colon)
        let (label, span) = self.ident(span)?;

        let jmp = Instr::Jmp {
            lbl: Label(label),
            cond,
        };

        Ok(AsmStmt {
            stmt: Stmt::Instr(jmp),
            span,
        })
    }

    fn try_next(
        &mut self,
        pred: impl FnOnce(&AsmResult<LexToken<'prg>>) -> bool,
        span: &Span,
    ) -> AsmResult<Option<(Token<'prg>, Span)>> {
        self.tokens
            .next_if(pred)
            .map(move |r| r.map(move |t| (t.token, span + &t.span)))
            .transpose()
    }

    fn msg(&mut self, span: Span) -> AsmResult<AsmStmt<'prg>> {
        let literal = |t: &AsmResult<LexToken<'prg>>| {
            matches!(
                t,
                Ok(LexToken {
                    token: Token::Literal(_),
                    ..
                })
            )
        };

        let comma = |t: &AsmResult<LexToken<'prg>>| {
            matches!(
                t,
                Ok(LexToken {
                    token: Token::Comma,
                    ..
                })
            )
        };

        let mut args = Vec::new();

        // parse at least one literal, otherwise return an empty message
        let Some((Token::Literal(arg), mut span)) = self.try_next(literal, &span)? else {
            return Ok(AsmStmt {
                stmt: Stmt::Instr(Instr::Msg(args.into_boxed_slice())),
                span,
            });
        };

        args.push(arg);

        loop {
            // contrary to labels, msg argument literals are separated with commas
            let Some((Token::Comma, new_span)) = self.try_next(comma, &span)? else {
                break;
            };

            // because we've parsed a trailing comma, expect a literal next
            let (arg, new_span) = self.literal(new_span)?;
            args.push(arg);
            span = new_span;
        }

        Ok(AsmStmt {
            stmt: Stmt::Instr(Instr::Msg(args.into_boxed_slice())),
            span,
        })
    }
}

impl<'prg> Iterator for AsmLines<'prg> {
    type Item = AsmResult<AsmStmt<'prg>>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = match self.tokens.next()? {
            Ok(token) => token,
            Err(e) => return Some(Err(e)),
        };

        let stmt = match token {
            // token is a keyword, parse an instruction statement
            LexToken {
                token: Token::Keyword(keyword),
                span,
                ..
            } => self.instruction(keyword, span),

            // token is an identifier, check if it's a label definition
            LexToken {
                token: Token::Literal(Literal::Ident(ident)),
                span,
                ..
            } => self.label(Label(ident), span),

            // unexpected token at the start of a statement
            t => Err(Error {
                code: t.lexeme.to_string(),
                span: t.span,
                source: format!("unexpected start of a statement: {:?}", t.token).into(),
            }),
        };

        Some(stmt)
    }
}

#[derive(Debug)]
pub struct AsmStmt<'prg> {
    stmt: Stmt<'prg>,
    span: Span,
}

impl Display for AsmStmt<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stmt)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt<'prg> {
    Instr(Instr<'prg>),
    Label(Label<'prg>),
}

impl Display for Stmt<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Instr(instr) => write!(f, "{instr}"),
            Self::Label(label) => write!(f, "{label}:"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instr<'a> {
    Unary {
        reg: Reg<'a>,
        op: RegOp,
    },
    Binary {
        reg: Reg<'a>,
        val: Val<'a>,
        op: BinOp,
    },
    Jmp {
        lbl: Label<'a>,
        cond: Option<Cond>,
    },
    Cmp(Cmp<'a>),
    Call(Label<'a>),
    Ret,
    Msg(Box<[Literal<'a>]>),
    End,
}

impl Display for Instr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unary { reg, op } => write!(f, "{op} {reg}"),
            Self::Binary { reg, val, op } => write!(f, "{op} {reg}, {val}"),
            Self::Jmp { lbl, cond } => write!(
                f,
                "{} {lbl}",
                match cond {
                    None => "jmp",
                    Some(Cond::Eq) => "je",
                    Some(Cond::Ne) => "jne",
                    Some(Cond::Ge) => "jge",
                    Some(Cond::Gt) => "jg",
                    Some(Cond::Le) => "jle",
                    Some(Cond::Lt) => "jl",
                }
            ),
            Self::Cmp(cmp) => cmp.fmt(f),
            Self::Call(lbl) => write!(f, "call {lbl}"),
            Self::Ret => write!(f, "ret"),
            Self::Msg(args) => {
                f.write_str("msg")?;

                let mut args = args.iter();

                if let Some(arg) = args.next() {
                    write!(f, " {arg}")?;
                }

                for arg in args {
                    write!(f, ", {arg}")?;
                }

                Ok(())
            }
            Self::End => write!(f, "end"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cmp<'a>(Val<'a>, Val<'a>);

impl Display for Cmp<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cmp {}, {}", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegOp {
    Inc,
    Dec,
}

impl Display for RegOp {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Inc => "inc",
            Self::Dec => "dec",
        })
    }
}

impl TryFrom<Keyword> for RegOp {
    type Error = Keyword;

    fn try_from(keyword: Keyword) -> Result<Self, Self::Error> {
        match keyword {
            Keyword::Inc => Ok(Self::Inc),
            Keyword::Dec => Ok(Self::Dec),
            other => Err(other),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Mov,
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for BinOp {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Mov => "mov",
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Mul => "mul",
            Self::Div => "div",
        })
    }
}

impl TryFrom<Keyword> for BinOp {
    type Error = Keyword;

    fn try_from(keyword: Keyword) -> Result<Self, Self::Error> {
        match keyword {
            Keyword::Mov => Ok(Self::Mov),
            Keyword::Add => Ok(Self::Add),
            Keyword::Sub => Ok(Self::Sub),
            Keyword::Mul => Ok(Self::Mul),
            Keyword::Div => Ok(Self::Div),
            other => Err(other),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cond {
    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt,
}

impl TryFrom<Keyword> for Option<Cond> {
    type Error = Keyword;

    fn try_from(keyword: Keyword) -> Result<Self, Self::Error> {
        match keyword {
            Keyword::Jmp => Ok(None),
            Keyword::Je => Ok(Some(Cond::Eq)),
            Keyword::Jne => Ok(Some(Cond::Ne)),
            Keyword::Jge => Ok(Some(Cond::Ge)),
            Keyword::Jg => Ok(Some(Cond::Gt)),
            Keyword::Jle => Ok(Some(Cond::Le)),
            Keyword::Jl => Ok(Some(Cond::Lt)),
            other => Err(other),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Label<'a>(&'a str);

impl Display for Label<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Reg<'a>(&'a str);

impl Display for Reg<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Val<'a> {
    Reg(Reg<'a>),
    Const(i64),
}

impl Display for Val<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(reg) => write!(f, "{reg}"),
            Self::Const(val) => write!(f, "{val}"),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rstest::*;

    const PRG1: &str = include_str!("../fixtures/asm_interpreter/program_1.asm");
    const PRG2: &str = include_str!("../fixtures/asm_interpreter/program_2.asm");
    const PRG3: &str = include_str!("../fixtures/asm_interpreter/program_3.asm");
    const PRG4: &str = include_str!("../fixtures/asm_interpreter/program_4.asm");
    const PRG5: &str = include_str!("../fixtures/asm_interpreter/program_5.asm");
    const PRG6: &str = include_str!("../fixtures/asm_interpreter/program_6.asm");
    const PRG7: &str = include_str!("../fixtures/asm_interpreter/program_7.asm");

    #[rstest]
    #[case(PRG1, Some("(5+1)/2 = 3"))]
    #[case(PRG2, Some("5! = 120"))]
    #[case(PRG3, Some("Term 8 of Fibonacci series is: 21"))]
    #[case(PRG4, Some("mod(11, 3) = 2"))]
    #[case(PRG5, Some("gcd(81, 153) = 9"))]
    #[case(PRG6, None)]
    #[case(PRG7, Some("2^10 = 1024"))]
    #[trace]
    fn interpret_asm(#[case] prg: &str, #[case] expected: Option<&str>) {
        let expected = expected.map(String::from);
        let actual = AssemblerInterpreter::interpret(prg);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("", vec![], vec![])]
    #[case("end", vec!["end"], vec![Span { offset: 0, length: 3, lineno: 0, lineof: 0 }])]
    #[case(
        "inc  a",
        vec!["inc a"],
        vec![Span { offset: 0, length: 6, lineno: 0, lineof: 0 }]
    )]
    #[case(
        "mov  a, 5",
        vec!["mov a, 5"],
        vec![Span { offset: 0, length: 9, lineno: 0, lineof: 0 }]
    )]
    #[case(
        "cmp  a, b",
        vec!["cmp a, b"],
        vec![Span { offset: 0, length: 9, lineno: 0, lineof: 0 }]
    )]
    #[case(
        ";Some comment\nmov   b, -112  \n",
        vec!["mov b, -112"],
        vec![Span { offset: 14, length: 13, lineno: 1, lineof: 14 }]
    )]
    #[case(
        "jmp   proc_fib\njne  lbl\n",
        vec!["jmp proc_fib", "jne lbl"],
        vec![
            Span { offset: 0, length: 14, lineno: 0, lineof: 0 },
            Span { offset: 15, length: 8, lineno: 1, lineof: 15 },
        ]
    )]
    #[case(
        "msg  '(5+1)/2 = ', a    ; output message",
        vec!["msg '(5+1)/2 = ', a"],
        vec![Span { offset: 0, length: 20, lineno: 0, lineof: 0 }]
    )]
    #[case(
        "msg  'hello ', a\nlbl:",
        vec!["msg 'hello ', a", "lbl:"],
        vec![
            Span { offset: 0, length: 16, lineno: 0, lineof: 0 },
            Span { offset: 17, length: 4, lineno: 1, lineof: 17 },
        ]
    )]
    #[case(
        "\nfunc_0:\n    mov   b, c\n    inc   c\n    jmp   proc_fib\n",
        vec!["func_0:", "mov b, c", "inc c", "jmp proc_fib"],
        vec![
            Span { offset: 1, length: 7, lineno: 1, lineof: 1 },
            Span { offset: 13, length: 10, lineno: 2, lineof: 9 },
            Span { offset: 28, length: 7, lineno: 3, lineof: 24 },
            Span { offset: 40, length: 14, lineno: 4, lineof: 36 },
        ]
    )]
    #[trace]
    fn simple_stmts(#[case] prg: &str, #[case] expected: Vec<&str>, #[case] spans: Vec<Span>) {
        let (actual, actual_spans): (Vec<_>, Vec<_>) = Parser::new(prg)
            .into_iter()
            .map(|asm| {
                let asm = asm.expect("inputs should be valid");
                (asm.to_string(), asm.span)
            })
            .unzip();

        assert_eq!(expected, actual);
        assert_eq!(spans, actual_spans);
    }

    #[rstest]
    #[case::missing_reg("inc \ninc a", 0, 0, 8)]
    #[case::wrong_reg_type("mov 123,  x", 0, 4, 4)]
    #[case::missing_comma("msg x  'xyz'", 0, 7, 5)]
    #[case::missing_arg("msg x,  \ninc x", 0, 0, 12)]
    #[case::missing_colon("lbl\ninc x", 0, 0, 7)]
    #[trace]
    fn invalid_stmts(
        #[case] prg: &str,
        #[case] lineno: usize,
        #[case] column: usize,
        #[case] length: usize,
    ) {
        // XXX: std try_collect is unstable in 1.66.1
        use itertools::Itertools as _;

        let err = Parser::new(prg)
            .into_iter()
            .try_collect::<_, Vec<_>, _>()
            .expect_err("input should not be valid");

        assert_eq!(lineno, err.span.lineno, "line no.");
        assert_eq!(column, err.span.column(), "within line offset");
        assert_eq!(length, err.span.length, "sequence length");
    }

    #[rstest]
    #[case("", vec![], vec![])]
    #[case(
        "mov  a, 5", 
        vec!["mov", "a", ",", "5"],
        vec![(0, 0), (0, 5), (0, 6), (0, 8)]
    )]
    #[case(
        "mov   b, -112  \n",
        vec!["mov", "b", ",", "-112"],
        vec![(0, 0), (0, 6), (0, 7), (0, 9)]
    )]
    #[case(
        "msg  '(5+1)/2 = ', a    ; output message",
        vec!["msg", "'(5+1)/2 = '", ",", "a"],
        vec![(0, 0), (0, 5), (0, 17), (0, 19)]
    )]
    #[case(
        "\nfunc_0:\n    mov   b, c\n    inc   c\n    jmp   proc_fib\n",
        vec!["func_0", ":", "mov", "b", ",", "c", "inc", "c", "jmp", "proc_fib"],
        vec![(1, 0), (1, 6), (2, 4), (2, 10), (2, 11), (2, 13), (3, 4), (3, 10), (4, 4), (4, 10)]
    )]
    #[trace]
    fn simple_lexemes(
        #[case] prg: &str,
        #[case] expected: Vec<&str>,
        #[case] locs: Vec<(usize, usize)>,
    ) {
        let (actual, actual_locs): (Vec<_>, Vec<_>) = Lexer::new(prg)
            .into_iter()
            .map(|t| {
                let t = t.expect("inputs should be valid");
                (t.lexeme, (t.span.lineno, t.span.column()))
            })
            .unzip();

        assert_eq!(expected, actual);
        assert_eq!(locs, actual_locs);
    }

    #[rstest]
    #[case("mov x  123.456", 0, 7, 7)]
    #[case("mov x  123foo%!$456", 0, 7, 12)]
    #[case("mov x  123some-text456", 0, 7, 15)]
    #[case("\nmov x  123\ninc -text456", 2, 4, 8)]
    #[trace]
    fn invalid_inputs(
        #[case] prg: &str,
        #[case] lineno: usize,
        #[case] column: usize,
        #[case] length: usize,
    ) {
        // XXX: std try_collect is unstable in 1.66.1
        use itertools::Itertools as _;

        let err = Lexer::new(prg)
            .into_iter()
            .try_collect::<_, Vec<_>, _>()
            .expect_err("input should not be valid");

        assert_eq!(lineno, err.span.lineno, "line no.");
        assert_eq!(column, err.span.column(), "within line offset");
        assert_eq!(length, err.span.length, "sequence length");
    }
}
