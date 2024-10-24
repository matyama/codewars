use std::collections::HashMap;

enum Loc<'prg> {
    Val(i64),
    Reg(&'prg str),
}

impl<'prg> From<&'prg str> for Loc<'prg> {
    #[inline]
    fn from(input: &'prg str) -> Self {
        input.parse().map_or_else(|_| Self::Reg(input), Self::Val)
    }
}

enum Instr<'prg> {
    Mov { x: &'prg str, y: Loc<'prg> },
    Inc(&'prg str),
    Dec(&'prg str),
    Jnz { x: Loc<'prg>, y: Loc<'prg> },
}

impl<'prg> TryFrom<&'prg str> for Instr<'prg> {
    type Error = String;

    fn try_from(instr: &'prg str) -> Result<Self, Self::Error> {
        let mut instr = instr.splitn(3, ' ');

        let instr = match instr.next() {
            Some(op @ "mov") => Self::Mov {
                x: instr
                    .next()
                    .ok_or_else(|| format!("{op} is missing an argument"))?,
                y: instr
                    .next()
                    .map(Loc::from)
                    .ok_or_else(|| format!("{op} is missing an argument"))?,
            },

            Some(op @ "inc") => instr
                .next()
                .map(Self::Inc)
                .ok_or_else(|| format!("{op} is missing an argument"))?,

            Some(op @ "dec") => instr
                .next()
                .map(Self::Dec)
                .ok_or_else(|| format!("{op} is missing an argument"))?,

            Some(op @ "jnz") => Self::Jnz {
                x: instr
                    .next()
                    .map(Loc::from)
                    .ok_or_else(|| format!("{op} is missing an argument"))?,
                y: instr
                    .next()
                    .map(Loc::from)
                    .ok_or_else(|| format!("{op} is missing an argument"))?,
            },

            instr => return Err(format!("unknown instruction: {instr:?}")),
        };

        Ok(instr)
    }
}

#[repr(transparent)]
struct Program<'prg>(Vec<Instr<'prg>>);

impl<'prg> TryFrom<Vec<&'prg str>> for Program<'prg> {
    type Error = String;

    fn try_from(prog: Vec<&'prg str>) -> Result<Self, Self::Error> {
        prog.into_iter()
            .map(Instr::try_from)
            .collect::<Result<_, _>>()
            .map(Self)
    }
}

impl<'prg> std::ops::Deref for Program<'prg> {
    type Target = [Instr<'prg>];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
#[repr(transparent)]
struct Registry<'prg>(HashMap<&'prg str, i64>);

impl<'prg> Registry<'prg> {
    #[inline]
    fn reg(&mut self, reg: &'prg str) -> &mut i64 {
        self.0.entry(reg).or_default()
    }

    #[inline]
    fn val(&mut self, at: &Loc<'prg>) -> i64 {
        match at {
            Loc::Val(val) => *val,
            Loc::Reg(reg) => *self.reg(reg),
        }
    }
}

impl From<Registry<'_>> for HashMap<String, i64> {
    fn from(Registry(regs): Registry<'_>) -> Self {
        regs.into_iter()
            .map(|(reg, val)| (reg.to_string(), val))
            .collect()
    }
}

pub fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let program = Program::try_from(program).expect("valid program");
    let mut registers = Registry::default();
    let mut pc = 0;

    while 0 <= pc && (pc as usize) < program.len() {
        match &program[pc as usize] {
            Instr::Mov { x, y } => {
                *registers.reg(x) = registers.val(y);
                pc += 1;
            }
            Instr::Inc(x) => {
                *registers.reg(x) += 1;
                pc += 1;
            }
            Instr::Dec(x) => {
                *registers.reg(x) -= 1;
                pc += 1;
            }
            Instr::Jnz { x, y } if registers.val(x) != 0 => {
                pc += registers.val(y);
            }
            Instr::Jnz { .. } => pc += 1,
        }
    }

    registers.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    macro_rules! map {
        ($($key:expr => $value:expr),*) => {{
             let mut map = HashMap::new();
             $(
                 map.insert($key.to_string(), $value);
             )*
             map
        }};
    }

    #[rstest]
    #[case(vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"], map! { "a" => 1 })]
    #[case(
        vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ],
        map! { "a" => 409600, "c" => 409600, "b" => 409600}
    )]
    fn short_tests(#[case] program: Vec<&str>, #[case] expected: HashMap<String, i64>) {
        let actual = simple_assembler(program);
        assert_eq!(expected, actual);
    }
}
