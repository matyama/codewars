use std::collections::HashMap;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

#[derive(Debug)]
pub enum ParseError {
    InvalidAtomName,
    UnsupportedToken,
}

#[derive(Debug)]
enum Element {
    Atom(String),
    Count(usize),
    OpenBracket(char),
    CloseBracket(char),
}

fn count_atoms(elems: &mut Vec<Element>) -> HashMap<String, usize> {
    let mut atom_counts = HashMap::new();
    let mut counts: Vec<usize> = Vec::new();

    println!("{:?}", elems);

    while let Some(last) = elems.pop() {
        let mut cnt: Option<usize> = None;
        println!("elem: {:?}\tcnt: {:?}\tcounts: {:?}", last, cnt, counts);
        match last {
            Element::Count(n) => {
                cnt = Some(counts.last().map(|c| c * n).unwrap_or(n));
                println!("new count: {:?}", cnt);
            }
            Element::Atom(name) => {
                let count = cnt.or(counts.last().cloned()).unwrap_or(1);
                // FIXME: cnt update does not carry through
                println!("updating {} with {}, cnt: {:?}", name, count, cnt);
                *atom_counts.entry(name).or_default() += count;
            }
            Element::OpenBracket(bracket) => {
                if let Some(c) = cnt {
                    // TODO: push (c, closing_bracket)
                    counts.push(c);
                }
            }
            Element::CloseBracket(bracket) => {
                // TODO: checks that
                //  1. there is still some element to pop
                //  2. that the bracket type matches -> (mul, expected)
                counts.pop();
                cnt = None;
            }
        }
    }

    atom_counts
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let mut stack: Vec<Element> = Vec::new();

    for symbol in s.chars() {
        match symbol {
            // Handle atoms
            _ if symbol.is_alphabetic() => {
                if symbol.is_uppercase() {
                    // Create new atom
                    stack.push(Element::Atom(symbol.to_string()));
                } else {
                    // Handle two letter atoms
                    if let Some(Element::Atom(last)) = stack.last_mut() {
                        last.push(symbol);
                    } else {
                        return Err(ParseError::InvalidAtomName);
                    }
                }
            }

            // Handle numbers
            _ if symbol.is_ascii_digit() => {
                let digit = symbol.to_digit(10).unwrap() as usize;
                if let Some(Element::Count(last)) = stack.last_mut() {
                    *last *= 10;
                    *last += digit;
                } else {
                    stack.push(Element::Count(digit))
                }
            }

            // Handle brackets
            '(' | '[' | '{' => stack.push(Element::OpenBracket(symbol)),
            ')' | ']' | '}' => stack.push(Element::CloseBracket(symbol)),

            // No other tokens are valid
            _ => return Err(ParseError::UnsupportedToken),
        }
    }

    let mut counts = count_atoms(&mut stack);
    Ok(counts.drain().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_parse {
        ($formula:expr, $expected:expr, $name:ident) => {
            #[test]
            fn $name() {
                super::assert_parse($formula, &$expected, "");
            }
        };
    }

    mod molecules {
        assert_parse!("H", [("H", 1)], hydrogen);
        assert_parse!("O2", [("O", 2)], oxygen);
        assert_parse!("H2O", [("H", 2), ("O", 1)], water);
        assert_parse!(
            "Mg(OH)2",
            [("Mg", 1), ("O", 2), ("H", 2)],
            magnesium_hydroxide
        );
        assert_parse!(
            "K4[ON(SO3)2]2",
            [("K", 4), ("O", 14), ("N", 2), ("S", 4)],
            fremys_salt
        );
    }

    #[test]
    fn errors() {
        assert_fail("pie", "Not a valid molecule");
        assert_fail("Mg(OH", "Mismatched parenthesis");
        assert_fail("Mg(OH}2", "Mismatched parenthesis");
    }

    #[allow(non_fmt_panic)]
    fn assert_fail(formula: &str, msg: &str) {
        let result = parse_molecule(formula);
        assert!(
            result.is_err(),
            format!(
                "expected {} {:?} to fail, got {:?}",
                msg,
                formula,
                result.unwrap()
            )
        );
    }

    #[allow(non_fmt_panic)]
    fn assert_parse(formula: &str, expected: &[(&str, usize)], _mst: &str) {
        let mut expected = expected
            .into_iter()
            .map(|&(name, usize)| (name.to_owned(), usize))
            .collect::<Molecule>();
        let result = parse_molecule(formula);
        assert!(
            result.is_ok(),
            format!("expected {:?} to pass, got {:?}", formula, result)
        );
        let mut actual = result.unwrap();
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
