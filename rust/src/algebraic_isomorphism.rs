//! Sometimes, we can treat a Type as a Number.
//!
//! If a Type `t` has `n` distinct value, it's Number is `n`. This is formally called
//! [cardinality][cardinality].
//!
//! Void has cardinality of 0 (we will abbreviate it Void is 0).
//!  - () is 1
//!  - Bool is 2
//!  - Maybe a is 1 + a
//!
//! We will be using [peano arithmetic][peano] so we will write it as S a.
//!  - Either a b is a + b
//!  - (a, b) is a * b
//!  - a => b is b ^ a (Try counting `(() => Bool)` and `(Bool => ())`)
//!
//! Algebraic data type got the name because it satisfies a lot of algebraic rules under
//! isomorphism.
//!
//! [cardinality]: https://en.wikipedia.org/wiki/Cardinality
//! [peano]: https://en.wikipedia.org/wiki/Peano_axioms

use crate::isomorphism::*;

/// a = b => c = d => a * c = b * d
pub fn iso_prod<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<(A, C), (B, D)> {
    iso_tuple(ab, cd)
}

/// a = b => c = d => a + c = b + d
pub fn iso_plus<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<Result<A, C>, Result<B, D>> {
    iso_result(ab, cd)
}

/// a = b => S a = S b
pub fn iso_s<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Option<A>, Option<B>> {
    iso_option(i)
}

/// a = b => c = d => c ^ a = d ^ b
pub fn iso_pow<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> IsoF<A, B, C, D> {
    iso_func(ab, cd)
}

/// a + b = b + a
pub fn plus_comm<A: 'static, B: 'static>() -> ISO<Result<A, B>, Result<B, A>> {
    iso(
        |r| match r {
            Ok(a) => Err(a),
            Err(b) => Ok(b),
        },
        |r| match r {
            Ok(b) => Err(b),
            Err(a) => Ok(a),
        },
    )
}

#[allow(clippy::type_complexity)]
/// a + b + c = a + (b + c)
pub fn plus_assoc<A: 'static, B: 'static, C: 'static>(
) -> ISO<Result<Result<A, B>, C>, Result<A, Result<B, C>>> {
    iso(
        |r| match r {
            Ok(Ok(a)) => Ok(a),
            Ok(Err(b)) => Err(Ok(b)),
            Err(c) => Err(Err(c)),
        },
        |r| match r {
            Ok(a) => Ok(Ok(a)),
            Err(Ok(b)) => Ok(Err(b)),
            Err(Err(c)) => Err(c),
        },
    )
}

#[inline]
fn swap<A, B>((a, b): (A, B)) -> (B, A) {
    (b, a)
}

/// a * b = b * a
pub fn mult_comm<A: 'static, B: 'static>() -> ISO<(A, B), (B, A)> {
    iso(swap, swap)
}

#[allow(clippy::type_complexity)]
/// a * b * c = a * (b * c)
pub fn mult_assoc<A: 'static, B: 'static, C: 'static>() -> ISO<((A, B), C), (A, (B, C))> {
    iso(|((a, b), c)| (a, (b, c)), |(a, (b, c))| ((a, b), c))
}

#[allow(clippy::type_complexity)]
/// a * (b + c) = a * b + a * c
pub fn dist<A: 'static, B: 'static, C: 'static>() -> ISO<(A, Result<B, C>), Result<(A, B), (A, C)>>
{
    iso(
        |(a, r)| match r {
            Ok(b) => Ok((a, b)),
            Err(c) => Err((a, c)),
        },
        |r| match r {
            Ok((a, b)) => (a, Ok(b)),
            Err((a, c)) => (a, Err(c)),
        },
    )
}

// Translator note:
//
// FnBox is not yet supported, we can only return an uncallable Box<FnOnce> (RetFunc). You should
// return the function with correct type, which will be checked by the tests. Later you'll have to
// implement three more functions that related to `RetFunc`.

pub type Func<A, B> = Box<dyn Fn(A) -> B>;
pub type RetFunc<A, B> = Box<dyn FnOnce(A) -> B>;
pub type IsoCL<A, B, C> = RetFunc<Func<A, Func<B, C>>, RetFunc<(A, B), C>>;
pub type IsoCR<A, B, C> = RetFunc<Func<(A, B), C>, RetFunc<A, RetFunc<B, C>>>;
pub type IsoC<A, B, C> = (IsoCL<A, B, C>, IsoCR<A, B, C>);

// curry :: ((a, b) -> c) -> a -> b -> c
fn curry<A: 'static, B: 'static, C: 'static>(f: Func<(A, B), C>) -> RetFunc<A, RetFunc<B, C>> {
    Box::new(move |a| Box::new(move |b| f((a, b))))
}

// uncurry :: (a -> b -> c) -> (a, b) -> c
fn uncurry<A: 'static, B: 'static, C: 'static>(f: Func<A, Func<B, C>>) -> RetFunc<(A, B), C> {
    Box::new(move |(a, b)| f(a)(b))
}

/// (c ^ b) ^ a = c ^ (a * b)
pub fn curry_iso<A: 'static, B: 'static, C: 'static>() -> IsoC<A, B, C> {
    (Box::new(uncurry), Box::new(curry))
}

/// 1 = S O
///
/// We are using [peano arithmetic](https://en.wikipedia.org/wiki/Peano_axioms).
pub fn one() -> ISO<(), Option<Void>> {
    iso(|_| None, |_| ())
}

/// 2 = S (S O)
pub fn two() -> ISO<bool, Option<Option<Void>>> {
    iso(
        |b| if b { Some(None) } else { None },
        |o| match o {
            Some(Some(z)) => absurd(z),
            o => o.is_some(),
        },
    )
}

/// 0 + b = b
pub fn plus_o<B: 'static>() -> ISO<Result<Void, B>, B> {
    iso(
        |r| match r {
            Ok(z) => absurd(z),
            Err(b) => b,
        },
        Err,
    )
}

#[allow(clippy::type_complexity)]
/// S a + b = S (a + b)
pub fn plus_s<A: 'static, B: 'static>() -> ISO<Result<Option<A>, B>, Option<Result<A, B>>> {
    iso(
        |r| match r {
            Ok(None) => None,
            Ok(Some(a)) => Some(Ok(a)),
            Err(b) => Some(Err(b)),
        },
        |o| match o {
            None => Ok(None),
            Some(Ok(a)) => Ok(Some(a)),
            Some(Err(b)) => Err(b),
        },
    )
}

/// 1 + b = S b
pub fn plus_so<B: 'static>() -> ISO<Result<(), B>, Option<B>> {
    trans(iso_plus(one(), refl()), trans(plus_s(), iso_s(plus_o())))
}

/// 0 * a = 0
pub fn mult_o<A: 'static>() -> ISO<(Void, A), Void> {
    iso(|(z, _)| z, |z| absurd(z))
}

#[allow(clippy::type_complexity)]
/// S a * b = b + a * b
pub fn mult_s<A: 'static, B: 'static>() -> ISO<(Option<A>, B), Result<B, (A, B)>> {
    iso(
        |(o, b)| match o {
            Some(a) => Err((a, b)),
            None => Ok(b),
        },
        |r| match r {
            Ok(b) => (None, b),
            Err((a, b)) => (Some(a), b),
        },
    )
}

/// S a * b = b + a * b
pub fn mult_so<B: 'static>() -> ISO<((), B), B> {
    trans(
        iso_prod(one(), refl()),
        trans(
            mult_s(),
            trans(iso_plus(refl(), mult_o()), trans(plus_comm(), plus_o())),
        ),
    )
}

// Here we go, the last three functions related to RetFunc. They're easy!

pub type IsoPL<A> = RetFunc<Func<Void, A>, ()>;
pub type IsoPR<A> = RetFunc<(), RetFunc<Void, A>>;
pub type IsoP<A> = (IsoPL<A>, IsoPR<A>);

/// a ^ 0 = 1
pub fn pow_o<A: 'static>() -> IsoP<A> {
    // (const (), const absurd)
    (Box::new(|_| ()), Box::new(|_| Box::new(|z| absurd(z))))
}

pub type IsoPsL<A, B> = RetFunc<Func<Option<B>, A>, (A, RetFunc<B, A>)>;
pub type IsoPsR<A, B> = RetFunc<(A, Func<B, A>), RetFunc<Option<B>, A>>;
pub type IsoPs<A, B> = (IsoPsL<A, B>, IsoPsR<A, B>);

// maybe :: b -> (a -> b) -> Maybe a -> b
fn maybe<A: 'static, B: 'static>(b: B, f: Func<A, B>) -> impl FnOnce(Option<A>) -> B {
    move |ma| match ma {
        Some(a) => f(a),
        None => b,
    }
}

/// a ^ (S b) = a * (a ^ b)
pub fn pow_s<A: 'static, B: 'static>() -> IsoPs<A, B> {
    (
        Box::new(|f| (f(None), Box::new(compose(f, Some)))),
        Box::new(|(a, f)| Box::new(maybe(a, f))),
    )
}

pub type IsoPsoL<A> = RetFunc<Func<(), A>, A>;
pub type IsoPsoR<A> = RetFunc<A, RetFunc<(), A>>;
pub type IsoPso<A> = (IsoPsoL<A>, IsoPsoR<A>);

/// a ^ 1 = a
///
/// In Haskell/Java/Dart, you can go the hard way (like mult_so, plus_so) to prove that you really
/// get what is going on. Unfortunately, in Rust, you can only go the trivial way. Because Rust
/// doesn't support FnBox ATM.
pub fn pow_so<A: 'static>() -> IsoPso<A> {
    (Box::new(|f| f(())), Box::new(|a| Box::new(move |()| a)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verbose() -> String {
        "It was me, DIO!".to_string()
    }

    fn lrl<A: 'static, B: 'static>(i: ISO<A, B>, a: A) -> A {
        let (fw, bw) = i;
        bw(fw(a))
    }

    fn rlr<A: 'static, B: 'static>(i: ISO<A, B>, b: B) -> B {
        let (fw, bw) = i;
        fw(bw(b))
    }

    #[test]
    fn sub_st_l_test() {
        assert!(sub_st_l(iso_bool())(true));
        assert!(!sub_st_l(iso_bool())(false));
        assert!(sub_st_l(iso_bool_not())(false));
    }

    #[test]
    fn sub_st_r_test() {
        assert!(sub_st_r(iso_bool())(true));
        assert!(!sub_st_r(iso_bool())(false));
    }

    #[test]
    fn assoc_test() {
        assert_eq!(
            Ok::<Result<i16, bool>, String>(Ok(233)),
            lrl(plus_assoc(), Ok::<Result<i16, bool>, String>(Ok(233)))
        );
        assert_eq!(
            Ok::<Result<i16, bool>, String>(Err(true)),
            lrl(plus_assoc(), Ok::<Result<i16, bool>, String>(Err(true)))
        );
        assert_eq!(
            Err::<Result<i16, bool>, String>(verbose()),
            lrl(plus_assoc(), Err::<Result<i16, bool>, String>(verbose()))
        );
        assert_eq!(
            Ok::<i16, Result<bool, String>>(233),
            rlr(plus_assoc(), Ok::<i16, Result<bool, String>>(233))
        );
        assert_eq!(
            Err::<i16, Result<bool, String>>(Ok(true)),
            rlr(plus_assoc(), Err::<i16, Result<bool, String>>(Ok(true)))
        );
        assert_eq!(
            Err::<i16, Result<bool, String>>(Err(verbose())),
            rlr(
                plus_assoc(),
                Err::<i16, Result<bool, String>>(Err(verbose()))
            )
        );
    }
}
