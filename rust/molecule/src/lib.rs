use std::collections::HashMap;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

#[derive(Debug)]
pub enum ParseError {
    InvalidAtomName,
}

fn is_lower_alpha(c: &char) -> bool {
    c.is_alphabetic() && c.is_lowercase()
}

fn is_upper_alpha(s: &str) -> bool {
    s.chars().all(|c| c.is_alphabetic() && c.is_uppercase())
}

fn is_number(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let mut molecule = HashMap::new();
    let mut stack: Vec<String> = Vec::new();

    for symbol in s.chars() {
        match symbol {
            // Handle two letter atoms
            _ if is_lower_alpha(&symbol) => match stack.last_mut() {
                Some(last) if is_upper_alpha(&last) => {
                    last.push(symbol);
                }
                _ => return Err(ParseError::InvalidAtomName),
            },

            // Handle numbers
            _ if symbol.is_ascii_digit() => match stack.last_mut() {
                Some(last) if is_number(&last) => {
                    last.push(symbol);
                }
                _ => stack.push(symbol.to_string()),
            },

            _ => {
                stack.push(symbol.to_string());
            }
        }

        println!("{:?}", stack);
    }

    Ok(molecule.drain().collect())
}

#[cfg(test)]
mod tests {
    use super::{parse_molecule, Molecule};

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
