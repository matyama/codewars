use std::collections::HashMap;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

#[derive(Debug)]
pub enum ParseError {
    InvalidAtomName,
    UnsupportedToken,
    MismatchedBrackets,
}

enum Element {
    Atom(String),
    Count(usize),
    OpenBracket(char),
    CloseBracket(char),
}

enum Count {
    Atom(usize),
    Multiple(usize),
}

impl From<&Count> for usize {
    fn from(cnt: &Count) -> Self {
        match cnt {
            Count::Atom(c) => *c,
            Count::Multiple(c) => *c,
        }
    }
}

fn count_atoms(elems: &mut Vec<Element>) -> Result<HashMap<String, usize>, ParseError> {
    let mut counts = HashMap::new();
    let mut multipliers: Vec<(usize, char)> = Vec::new();
    let mut cnt: Option<Count> = None;

    while let Some(last) = elems.pop() {
        match last {
            Element::Count(n) => {
                let count = multipliers.last().map_or(n, |(m, _)| m * n);
                let count = if let Some(Element::Atom(_)) = elems.last() {
                    Count::Atom(count)
                } else {
                    Count::Multiple(count)
                };
                cnt = Some(count);
            }
            Element::Atom(name) => {
                let last_cnt = multipliers.last().map(|(c, _)| c).cloned();
                let count = match cnt {
                    Some(Count::Atom(count)) => {
                        // Retrack to the last multiplier when accounted for
                        // single atom (i.e. not a sub-molecule)
                        cnt = last_cnt.map(Count::Multiple);
                        count
                    }
                    Some(Count::Multiple(count)) => count,
                    None => last_cnt.unwrap_or(1),
                };
                *counts.entry(name).or_default() += count;
            }
            Element::CloseBracket(bracket) => {
                if let Some(ref count) = cnt {
                    // Record expected reverse bracket
                    let open_bracket = match bracket {
                        ')' => '(',
                        ']' => '[',
                        '}' => '{',
                        _ => unreachable!("No other brackets are allowed"),
                    };
                    multipliers.push((count.into(), open_bracket));
                }
            }
            Element::OpenBracket(bracket) => match multipliers.pop() {
                Some((_, expected)) if bracket == expected => {
                    cnt = None;
                }
                _ => return Err(ParseError::MismatchedBrackets),
            },
        }
    }

    Ok(counts)
}

fn tokenize(molecule: &str) -> Result<Vec<Element>, ParseError> {
    let mut elems: Vec<Element> = Vec::new();

    for symbol in molecule.chars() {
        match symbol {
            // Handle atoms
            _ if symbol.is_alphabetic() => {
                if symbol.is_uppercase() {
                    // Create new atom
                    elems.push(Element::Atom(symbol.to_string()));
                } else {
                    // Handle two letter atoms
                    if let Some(Element::Atom(last)) = elems.last_mut() {
                        last.push(symbol);
                    } else {
                        return Err(ParseError::InvalidAtomName);
                    }
                }
            }

            // Handle numbers
            _ if symbol.is_ascii_digit() => {
                let digit = symbol.to_digit(10).unwrap() as usize;
                if let Some(Element::Count(last)) = elems.last_mut() {
                    *last *= 10;
                    *last += digit;
                } else {
                    elems.push(Element::Count(digit))
                }
            }

            // Handle brackets
            '(' | '[' | '{' => elems.push(Element::OpenBracket(symbol)),
            ')' | ']' | '}' => elems.push(Element::CloseBracket(symbol)),

            // No other tokens are valid
            _ => return Err(ParseError::UnsupportedToken),
        }
    }

    Ok(elems)
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let mut elems = tokenize(s)?;
    let mut counts = count_atoms(&mut elems)?;
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
