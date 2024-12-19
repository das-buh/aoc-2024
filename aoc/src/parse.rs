#[macro_export]
macro_rules! seq {
    ($( $( _ = )? $parsers:expr ),* $(,)?) => {
        |s| seq!(@ parsers = { $( $parsers, )* }, in = s, out = {})
    };
    (@ parsers = { $pat:literal, $( $rest:tt )* }, in = $s:ident, out = { $( $out:tt )* }) => {{
        let (_, s) = prefix($pat)($s);
        seq!(@ parsers = { $( $rest )* }, in = s, out = { $( $out )* })
    }};
    (@ parsers = { _ = $parser:expr, $( $rest:tt )* }, in = $s:ident, out = { $( $out:tt )* }) => {{
        let (_, s) = $parser($s);
        seq!(@ parsers = { $( $rest )* }, in = s, out = { $( $out )* })
    }};
    (@ parsers = { $parser:expr, $( $rest:tt )* }, in = $s:ident, out = { $( $out:tt )* }) => {{
        let (o, s) = $parser($s);
        seq!(@ parsers = { $( $rest )* }, in = s, out = { $( $out )* o, })
    }};
    (@ parsers = {}, in = $s:ident, out = { $( $out:tt )* }) => {
        ($( $out )* $s)
    };
}
pub use seq;

pub fn prefix(prefix: &str) -> impl Fn(&str) -> (&str, &str) + use<'_> {
    move |s| {
        let rem = s.strip_prefix(prefix).unwrap();
        (&s[..prefix.len()], rem)
    }
}

pub fn prefix_if(
    mut predicate: impl FnMut(char) -> bool,
) -> impl FnMut(&str) -> (Option<char>, &str) {
    move |s| {
        if let Some((i, c)) = s.char_indices().next().filter(|(_, c)| predicate(*c)) {
            (Some(c), &s[i..])
        } else {
            (None, s)
        }
    }
}

pub fn prefix_while(mut predicate: impl FnMut(char) -> bool) -> impl FnMut(&str) -> (&str, &str) {
    move |s| {
        let split = s
            .char_indices()
            .find(|(_, c)| !predicate(*c))
            .map_or(s.len(), |(i, _)| i);
        s.split_at(split)
    }
}

pub fn whitespace(s: &str) -> (&str, &str) {
    let rem = s.trim_start();
    (&s[..s.len() - rem.len()], rem)
}

pub fn spaces(s: &str) -> (&str, &str) {
    let split = s
        .char_indices()
        .find(|(_, c)| *c != ' ')
        .map_or(s.len(), |(i, _)| i);
    s.split_at(split)
}

pub fn newline(s: &str) -> (char, &str) {
    let rem = s.strip_prefix('\n').unwrap();
    ('\n', rem)
}

pub fn uint(s: &str) -> (u64, &str) {
    let (num, s) = prefix_while(|c| c.is_ascii_digit())(s);
    (num.parse().unwrap(), s)
}

pub fn int(s: &str) -> (i64, &str) {
    let (sign, num, s) = seq!(prefix_if(|s| s == '+' || s == '-'), uint)(s);
    match sign {
        Some('+') | None => (num as i64, s),
        Some('-') => (-(num as i64), s),
        _ => panic!(),
    }
}

pub fn sep_by<'a, 'src, T, P: FnMut(&'src str) -> (T, &'src str)>(
    separator: &'a str,
    elem: P,
) -> impl FnOnce(&'src str) -> SeparatedBy<'a, 'src, P> {
    |s| SeparatedBy {
        separator,
        elem,
        src: s,
        finished: false,
    }
}

pub struct SeparatedBy<'a, 'src, P> {
    separator: &'a str,
    elem: P,
    src: &'src str,
    finished: bool,
}

impl<'a, 'src, P> SeparatedBy<'a, 'src, P> {
    pub fn src(&self) -> &'src str {
        self.src
    }
}

impl<'a, 'src, T, P: FnMut(&'src str) -> (T, &'src str)> Iterator for SeparatedBy<'a, 'src, P> {
    type Item = (T, &'src str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let (elem, s) = (self.elem)(self.src);

        match s.strip_prefix(self.separator) {
            Some(s) => self.src = s,
            None => self.finished = true,
        }

        Some((elem, s))
    }
}
