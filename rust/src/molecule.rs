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
    OpenBracket,
    CloseBracket,
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

fn count_atoms(elems: &mut Vec<Element>) -> Molecule {
    let mut counts = HashMap::new();
    let mut multipliers: Vec<usize> = Vec::new();
    let mut cnt: Option<Count> = None;

    while let Some(last) = elems.pop() {
        match last {
            Element::Count(n) => {
                let count = multipliers.last().map_or(n, |m| m * n);
                let count = if let Some(Element::Atom(_)) = elems.last() {
                    Count::Atom(count)
                } else {
                    Count::Multiple(count)
                };
                cnt = Some(count);
            }
            Element::Atom(name) => {
                let last_cnt = multipliers.last().cloned();
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
            Element::CloseBracket => {
                if let Some(ref count) = cnt {
                    multipliers.push(count.into());
                }
            }
            Element::OpenBracket => {
                if multipliers.pop().is_some() {
                    cnt = None;
                }
            }
        }
    }

    counts.drain().collect()
}

fn tokenize(molecule: &str) -> Result<Vec<Element>, ParseError> {
    let mut elems: Vec<Element> = Vec::new();
    let mut brackets: Vec<char> = Vec::new();

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
            '(' | '[' | '{' => {
                elems.push(Element::OpenBracket);
                brackets.push(match symbol {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    _ => unreachable!("Matched types exhausted"),
                });
            }

            ')' | ']' | '}' => match brackets.pop() {
                Some(expected) if symbol == expected => {
                    elems.push(Element::CloseBracket);
                }
                _ => return Err(ParseError::MismatchedBrackets),
            },

            // No other tokens are valid
            _ => return Err(ParseError::UnsupportedToken),
        }
    }

    if brackets.is_empty() {
        Ok(elems)
    } else {
        Err(ParseError::MismatchedBrackets)
    }
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let mut elems = tokenize(s)?;
    let counts = count_atoms(&mut elems);
    Ok(counts)
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
        assert_parse!(
            "(C5H5)Fe(CO)2CH3",
            [("C", 8), ("H", 8), ("Fe", 1), ("O", 2)],
            cyclopentadienyliron_dicarbonyl_dimer
        );
        assert_parse!(
            "{[Co(NH3)4(OH)2]3Co}(SO4)3",
            [("Co", 4), ("N", 12), ("H", 42), ("O", 18), ("S", 3)],
            hexol_sulphate
        );
        assert_parse!("[Es2]", [("Es", 2)], random);
        assert_parse!(
            "Rg25{{{[[Pb]20]2}18}}PrBe21Cf",
            [("Rg", 25), ("Pb", 720), ("Pr", 1), ("Be", 21), ("Cf", 1)],
            random_nested
        );
    }

    #[test]
    fn errors() {
        assert_fail("pie", "Not a valid molecule");
        assert_fail("Mg(OH", "Mismatched parenthesis");
        assert_fail("Mg(OH}2", "Mismatched parenthesis");
    }

    #[allow(non_fmt_panics)]
    fn assert_fail(formula: &str, msg: &str) {
        let result = parse_molecule(formula);
        assert!(
            result.is_err(),
            "{}",
            format!(
                "expected {} {:?} to fail, got {:?}",
                msg,
                formula,
                result.unwrap()
            )
        );
    }

    #[allow(non_fmt_panics)]
    fn assert_parse(formula: &str, expected: &[(&str, usize)], _mst: &str) {
        let mut expected = expected
            .into_iter()
            .map(|&(name, usize)| (name.to_owned(), usize))
            .collect::<Molecule>();
        let result = parse_molecule(formula);
        assert!(
            result.is_ok(),
            "{}",
            format!("expected {:?} to pass, got {:?}", formula, result)
        );
        let mut actual = result.unwrap();
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
