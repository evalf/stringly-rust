//! Utility functions for generating and parsing Stringly serializations.

/// Created with the function [`safesplit`].
pub struct SafesplitIter<'a> {
    // string to split
    s: &'a str,
    // character to split at
    sep: char,
    // start point for the next slice
    i: usize,
    // flag that indicates if the iterator is exhausted
    exhausted: bool,
}

impl<'a> Iterator for SafesplitIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        let i = self.i;
        let mut level: i32 = 0;
        for (n, c) in self.s[i..].char_indices() {
            if c == self.sep && level == 0 {
                self.i += n + 1;
                return Some(&self.s[i..i + n]);
            } else if c == '{' {
                level += 1;
            } else if c == '}' {
                level -= 1;
            }
        }
        self.exhausted = true;
        Some(&self.s[i..])
    }
}

/// An iterator over substrings of a string, separated by a separation
/// character, but only if not enclosed in curly braces.
///
/// # Examples
///
/// Split a string at comma's:
///
/// ```
/// let v: Vec<&str> = stringly::util::safesplit("a,b,{c,d}", ',').collect();
/// assert_eq!(v, ["a", "b", "{c,d}"]);
/// ```
///
/// Splitting an empty string yields zero items:
///
/// ```
/// assert_eq!(stringly::util::safesplit("", ',').next(), None);
/// ```
pub fn safesplit(s: &str, sep: char) -> SafesplitIter<'_> {
    //assert_eq!(sep.len_utf8(), 1);
    SafesplitIter {
        s,
        sep,
        i: 0,
        exhausted: s.is_empty(),
    }
}

/// The error type for [`safesplit_once`].
#[derive(Debug, PartialEq)]
pub enum SafesplitOnceError {
    SeparatorNotFound,
}

/// Splits string precisely once at the first occurence of the separation
/// characer that is not enclosed in curly braces.
///
/// # Errors
///
/// Returns `Err` if the separation character is not found.
///
/// # Examples
///
/// ```
/// assert_eq!(stringly::util::safesplit_once("a,b", ','), Ok(("a", "b")));
/// assert_eq!(stringly::util::safesplit_once("{a,b}", ','), Err(stringly::util::SafesplitOnceError::SeparatorNotFound));
/// ```
pub fn safesplit_once(s: &str, sep: char) -> Result<(&str, &str), SafesplitOnceError> {
    if s.is_empty() {
        return Err(SafesplitOnceError::SeparatorNotFound);
    }
    let mut level: i32 = 0;
    for (i, c) in s.char_indices() {
        if c == sep && level == 0 {
            return Ok((&s[..i], &s[i + sep.len_utf8()..]));
        } else if c == '{' {
            level += 1;
        } else if c == '}' {
            level -= 1;
        }
    }
    Err(SafesplitOnceError::SeparatorNotFound)
}

// Returns the index `i` for which `s[..i]` is the left balancer (`'<' '{'*
// '>'`) or returns `None` if there is no such balancer.
fn left_balancer_end(s: &str) -> Option<usize> {
    let mut chars = s.char_indices();
    match chars.next() {
        None => return None,
        Some((_i, ch)) => {
            if ch != '<' {
                return None;
            }
        }
    }
    for (i, ch) in chars {
        match ch {
            '{' => {}
            '>' => return Some(i + 1),
            _ => return None,
        }
    }
    None
}

// Returns the index `i` for which `s[i..]` is the right balancer (`'<' '}'*
// '>'`) or returns `None` if there is no such balancer.
fn right_balancer_start(s: &str) -> Option<usize> {
    let mut chars = s.char_indices().rev();
    match chars.next() {
        None => return None,
        Some((_i, ch)) => {
            if ch != '>' {
                return None;
            }
        }
    }
    for (i, ch) in chars {
        match ch {
            '}' => {}
            '<' => return Some(i),
            _ => return None,
        }
    }
    None
}

// Returns `True` if `s` starts with a left balancer (`'<' '{'* '>'`).
#[inline(always)]
fn starts_with_balancer(s: &str) -> bool {
    left_balancer_end(s).is_some()
}

// Returns `True` if `s` ends with a right balancer (`'<' '}'* '>'`).
#[inline(always)]
fn ends_with_balancer(s: &str) -> bool {
    right_balancer_start(s).is_some()
}

/// Character test for [`protect`].
pub trait ProtectTest {
    /// `true` when [`protect`] should protect the string unconditionally.
    const UNCONDITIONAL: bool;

    /// Tests if character needs protection.
    fn test(&self, ch: char) -> bool;
}

/// Conditionally encloses string in curly braces and makes balanced.
///
/// # Examples
///
/// The string `"a,b,c"` needs protection for `','`:
///
/// ```
/// assert_eq!(stringly::util::protect("a,b,c", ','), "{a,b,c}");
/// ```
///
/// The string `"a{b,c}"` does not need protection for `","` because the comma
/// is enclosed in curly braces:
///
/// ```
/// assert_eq!(stringly::util::protect("a{b,c}", ','), "a{b,c}");
/// ```
///
/// The strings `"a=b"` and `"a,b"` need proctection for `','` or `'='`:
///
/// ```
/// assert_eq!(stringly::util::protect("a=b", [',', '=']), "{a=b}");
/// assert_eq!(stringly::util::protect("a,b", [',', '=']), "{a,b}");
/// ```
///
/// Unbalanced strings or strings starting and ending with `'{'` and `'}'`,
/// respectively, are always protected:
///
/// ```
/// assert_eq!(stringly::util::protect("}", ','), "{<{>}}");
/// ```
pub fn protect<T: ProtectTest>(s: &str, test: T) -> String {
    // Determine the number of braces that need to be added to the left (`l`) and
    // right (`r`) to make `s` balanced. Furthermore, detect if any character at
    // brace `level` zero tests true using `test`, in which case we need
    // protection.
    let (l, r, needs_protection) = {
        let mut needs_protection = if T::UNCONDITIONAL {
            true
        } else {
            s.starts_with('{') && s.ends_with('}')
        };
        let mut level = 0;
        let mut l = 0;
        for ch in s.chars() {
            if ch == '{' {
                level += 1;
            } else if ch == '}' {
                level -= 1;
                if -level > l {
                    l = -level;
                }
            } else if !T::UNCONDITIONAL && !needs_protection && level == 0 && test.test(ch) {
                needs_protection = true;
            }
        }
        (l, level + l, needs_protection)
    };
    if needs_protection || l > 0 || r > 0 {
        // Prepend `'<{{...{>'` to `s` only if necessary to balance (`l > 0`) or if
        // `s` starts with something that can be parsed as a balancer
        // (`starts_with_balancer(s)`). Append `'<}...}}>'` to `s` following
        // similar rules.  Finally enclose in braces.
        let mut v = vec!["{"];
        if l > 0 || starts_with_balancer(s) {
            v.push("<");
            for _i in 0..l {
                v.push("{");
            }
            v.push(">");
        }
        v.push(s);
        if r > 0 || ends_with_balancer(s) {
            v.push("<");
            for _i in 0..r {
                v.push("}");
            }
            v.push(">");
        }
        v.push("}");
        v.concat()
    } else {
        s.to_owned()
    }
}

struct ProtectTestTrue;

impl ProtectTest for ProtectTestTrue {
    const UNCONDITIONAL: bool = true;
    fn test(&self, _ch: char) -> bool {
        true
    }
}

/// Unconditionally protect a string.
pub fn protect_unconditionally(s: &str) -> String {
    protect(s, ProtectTestTrue)
}

struct ProtectTestFalse;

impl ProtectTest for ProtectTestFalse {
    const UNCONDITIONAL: bool = false;
    fn test(&self, _ch: char) -> bool {
        false
    }
}

/// Protect unbalanced strings.
pub fn protect_unbalanced(s: &str) -> String {
    protect(s, ProtectTestFalse)
}

/// Tests `true` for a single characer.
impl ProtectTest for char {
    const UNCONDITIONAL: bool = false;
    fn test(&self, ch: char) -> bool {
        ch == *self
    }
}

/// Tests `true` if any of the characters of this array matches.
impl<const N: usize> ProtectTest for [char; N] {
    const UNCONDITIONAL: bool = false;
    fn test(&self, ch: char) -> bool {
        self.iter().any(|&x| ch == x)
        //ch == self[0] || ch == self[1]
    }
}

/// Inverse of [`protect`].
pub fn unprotect(s: &str) -> &str {
    if s.starts_with('{') && s.ends_with('}') {
        let r = match right_balancer_start(&s[1..s.len() - 1]) {
            Some(i) => i + 1,
            None => s.len() - 1,
        };
        let l = match left_balancer_end(&s[1..r]) {
            Some(i) => i + 1,
            None => 1,
        };
        &s[l..r]
    } else {
        s
    }
}

/// Returns `true` if the string is balanced.
pub fn is_balanced(s: &str) -> bool {
    let mut level = 0;
    for ch in s.chars() {
        if ch == '{' {
            level += 1;
        } else if ch == '}' {
            level -= 1;
            if level < 0 {
                return false;
            }
        }
    }
    level == 0
}

/// The error type for [`splitarg`].
#[derive(Debug, PartialEq)]
pub enum SplitArgError {
    NotAnEnum,
}

/// Splits key from optional, unconditionally protected value.
///
/// Inverse of `key + protect_unconditionally(value)` or `key`, where `key` is
/// a string without curly braces. Returns `key` and `value`.
///
/// # Errors
///
/// Returns `Err` only if `s` contains a `'{'` but does not end with `'}'`. The
/// behavior of other invalid inputs is undefined.
pub fn splitarg(s: &str) -> Result<(&str, &str), SplitArgError> {
    if !is_balanced(s) {
        return Err(SplitArgError::NotAnEnum);
    }
    match s.find('{') {
        Some(i) => {
            if i > 0 && s.ends_with('}') {
                Ok((&s[..i], unprotect(&s[i..])))
            } else {
                Err(SplitArgError::NotAnEnum)
            }
        }
        None => Ok((s, "")),
    }
}

#[cfg(test)]
mod tests {

    fn assert_protected(orig: &str, check_protected: Option<&str>, sep: Option<char>) {
        let orig_protected = match sep {
            Some(sep_) => super::protect(orig, sep_),
            None => super::protect_unconditionally(orig),
        };
        if let Some(protected_) = check_protected {
            assert_eq!(orig_protected, protected_);
        }
        if let Some(sep_) = sep {
            let parts: Vec<&str> = super::safesplit(&orig_protected, sep_).collect();
            match orig {
                "" => assert_eq!(parts.len(), 0),
                _ => assert_eq!(parts, [&orig_protected]),
            };
        }
        assert_eq!(super::unprotect(&orig_protected), orig);
    }

    fn assert_normal(s: &str) {
        let sep = ',';
        assert_eq!(s.contains(sep), false);
        assert_protected(s, Some(s), Some(sep));
        assert_protected(s, Some(&["{", s, "}"].concat()), None);
    }

    fn assert_normal_unless_unconditional_protection(s: &str, protected: &str) {
        let sep = ',';
        assert_eq!(s.contains(sep), false);
        assert_protected(s, Some(s), Some(sep));
        assert_protected(s, Some(protected), None);
    }

    #[test]
    fn test_safesplit() {
        assert_eq!(super::safesplit("", ',').collect::<Vec<&str>>().len(), 0);
        assert_eq!(super::safesplit(" ", ',').collect::<Vec<&str>>(), [" "]);
        assert_eq!(
            super::safesplit("a,b{c,d}", ',').collect::<Vec<&str>>(),
            ["a", "b{c,d}"]
        );
        assert_eq!(super::safesplit(",", ',').collect::<Vec<&str>>(), ["", ""]);
    }

    #[test]
    fn test_safesplit_once() {
        assert_eq!(
            super::safesplit_once("", ','),
            Err(super::SafesplitOnceError::SeparatorNotFound)
        );
        assert_eq!(
            super::safesplit_once("{,}", ','),
            Err(super::SafesplitOnceError::SeparatorNotFound)
        );
        assert_eq!(super::safesplit_once("a,b", ','), Ok(("a", "b")));
        assert_eq!(super::safesplit_once("a,b,c", ','), Ok(("a", "b,c")));
        assert_eq!(super::safesplit_once("{a,b},c", ','), Ok(("{a,b}", "c")));
    }

    #[test]
    fn test_protect_combinations() {
        let chs1 = ['{', '}', '<', '>'];
        let chs2 = ['{', '}', ',', 'x'];
        let makestr = |i, length, chs: [char; 4]| {
            let mut v = String::new();
            let mut k = i;
            for _j in 0..length {
                v.push(chs[k % 4]);
                k /= 4;
            }
            v
        };
        for length in 0..6 {
            for i in 0..(4 as usize).pow(length) {
                assert_protected(&makestr(i, length, chs1), None, Some(','));
                assert_protected(&makestr(i, length, chs1), None, None);
                assert_protected(&makestr(i, length, chs2), None, Some(','));
            }
        }
    }

    #[test]
    fn test_protect_normality() {
        assert_normal("");
        assert_normal("abc");
        assert_normal("ab{cd}ef");
        assert_normal("<abc>");
        assert_normal("<abc></abc>");
        assert_normal("<{}>");
        assert_normal("a\nb");
    }

    #[test]
    fn test_protect_protection() {
        assert_protected("abc,def", Some("{abc,def}"), Some(','));
        assert_protected("ab{c,d}ef", Some("ab{c,d}ef"), Some(','));
        assert_protected("a{b,c}d,ef", Some("{a{b,c}d,ef}"), Some(','));
    }

    #[test]
    fn test_protect_braces() {
        assert_protected("{abc}", Some("{{abc}}"), Some(','));
        assert_protected("{abc{", Some("{{abc{<}}>}"), Some(','));
        assert_protected("}abc}", Some("{<{{>}abc}}"), Some(','));
        assert_protected("}abc{", Some("{<{>}abc{<}>}"), Some(','));
        assert_protected("}abc", Some("{<{>}abc}"), Some(','));
        assert_protected("abc{", Some("{abc{<}>}"), Some(','));
        assert_protected("abc}def", Some("{<{>abc}def}"), Some(','));
        assert_protected("abc{def", Some("{abc{def<}>}"), Some(','));
        assert_protected("a{bc}de{f", Some("{a{bc}de{f<}>}"), Some(','));
        assert_protected("a}bc{de}f", Some("{<{>a}bc{de}f}"), Some(','));
    }

    #[test]
    fn test_protect_balancers() {
        assert_normal_unless_unconditional_protection("<>", "{<><><>}");
        assert_normal_unless_unconditional_protection("a<>", "{a<><>}");
        assert_normal_unless_unconditional_protection("<>a", "{<><>a}");
        assert_normal_unless_unconditional_protection("<{><}>", "{<><{><}><>}");
        assert_normal_unless_unconditional_protection("<{{><}}>", "{<><{{><}}><>}");
        assert_protected("<{>", Some("{<><{><}>}"), Some(','));
        assert_protected("<}>", Some("{<{><}><>}"), Some(','));
        assert_protected("<{{>", Some("{<><{{><}}>}"), Some(','));
        assert_protected("<}}>", Some("{<{{><}}><>}"), Some(','));
        assert_protected("<>,", Some("{<><>,}"), Some(','));
        assert_protected(",<>", Some("{,<><>}"), Some(','));
        assert_protected("<>,<>", Some("{<><>,<><>}"), Some(','));
    }

    #[test]
    fn test_protect_multiple_symbols() {
        assert_eq!(super::protect("a=b", ['=', ',']), "{a=b}");
        assert_eq!(super::protect("a=b", ['X', 'Y', '=']), "{a=b}");
    }

    #[test]
    fn test_splitarg() {
        assert_eq!(super::splitarg("key"), Ok(("key", "")));
        assert_eq!(super::splitarg("key{val}"), Ok(("key", "val")));
        assert_eq!(super::splitarg("key{<{>val}}"), Ok(("key", "val}")));
        assert_eq!(
            super::splitarg("key{val"),
            Err(super::SplitArgError::NotAnEnum)
        );
        assert_eq!(
            super::splitarg("k}ey{val"),
            Err(super::SplitArgError::NotAnEnum)
        );
        assert_eq!(
            super::splitarg("key{val}}"),
            Err(super::SplitArgError::NotAnEnum)
        );
        assert_eq!(
            super::splitarg("key{val}}{}"),
            Err(super::SplitArgError::NotAnEnum)
        );
    }
}
