//! So, when are two type, `a` and `b`, considered equal? A definition might be, it is possible to
//! go from `a` to `b`, and from `b` to `a`. Going a roundway trip should leave you the same value.
//! Unfortunately it is virtually impossible to test this in Rust. This is called Isomorphism.

use std::convert::identity as id;

/// Returns new composite function `g . f`
pub fn compose<A, B, C, F, G>(g: G, f: F) -> impl Fn(A) -> C + 'static
where
    F: Fn(A) -> B + 'static,
    G: Fn(B) -> C + 'static,
{
    move |x| g(f(x))
}

pub enum Void {}

impl PartialEq for Void {
    fn eq(&self, _: &Void) -> bool {
        true
    }
}

pub fn absurd(_: Void) -> ! {
    panic!("You must be kidding! Where did you find that void instance?");
}

// NOTE: this is not quite right, because a proper fmap does not return arbitrary Functor
pub trait Functor {
    type Obj;
    type Fun<T>: Functor<Obj = T>;

    fn fmap<F, B>(self, f: F) -> Self::Fun<B>
    where
        F: AsRef<dyn Fn(Self::Obj) -> B + 'static>;
}

impl<A> Functor for Vec<A> {
    type Obj = A;
    type Fun<T> = Vec<T>;

    fn fmap<F, B>(self, f: F) -> Self::Fun<B>
    where
        F: AsRef<dyn Fn(Self::Obj) -> B + 'static>,
    {
        self.into_iter().map(f.as_ref()).collect()
    }
}

impl<A> Functor for Option<A> {
    type Obj = A;
    type Fun<T> = Option<T>;

    fn fmap<F, B>(self, f: F) -> Self::Fun<B>
    where
        F: AsRef<dyn Fn(Self::Obj) -> B + 'static>,
    {
        self.map(f.as_ref())
    }
}

impl<A, E> Functor for Result<A, E> {
    type Obj = A;
    type Fun<T> = Result<T, E>;

    fn fmap<F, B>(self, f: F) -> Self::Fun<B>
    where
        F: AsRef<dyn Fn(Self::Obj) -> B + 'static>,
    {
        self.map(f.as_ref())
    }
}

// XXX: sadly this would be a conflicting impl
//impl<T, A> Functor for Result<T, A> {
//    type Obj = A;
//    type Func<U> = Result<T, U>;
//
//    fn fmap<F, B>(self, f: F) -> Self::Func<B>
//    where
//        F: AsRef<dyn Fn(Self::Obj) -> B + 'static>,
//    {
//        self.map_err(f.as_ref())
//    }
//}

pub trait Bifunctor {
    type Fst; // a
    type Snd; // c
    type P<B, D>: Bifunctor<Fst = B, Snd = D>;

    // bimap :: (a -> b) -> (c -> d) -> p a c -> p b d
    // self :: p a c
    fn bimap<B, D, F, G>(self, f: F, g: G) -> Self::P<B, D>
    where
        F: AsRef<dyn Fn(Self::Fst) -> B + 'static>,
        G: AsRef<dyn Fn(Self::Snd) -> D + 'static>;
}

impl<A, C> Bifunctor for (A, C) {
    type Fst = A;
    type Snd = C;
    type P<B, D> = (B, D);

    fn bimap<B, D, F, G>(self, f: F, g: G) -> Self::P<B, D>
    where
        F: AsRef<dyn Fn(Self::Fst) -> B + 'static>,
        G: AsRef<dyn Fn(Self::Snd) -> D + 'static>,
    {
        (f.as_ref()(self.0), g.as_ref()(self.1))
    }
}

impl<A, C> Bifunctor for Result<A, C> {
    type Fst = A;
    type Snd = C;
    type P<B, D> = Result<B, D>;

    fn bimap<B, D, F, G>(self, f: F, g: G) -> Self::P<B, D>
    where
        F: AsRef<dyn Fn(Self::Fst) -> B + 'static>,
        G: AsRef<dyn Fn(Self::Snd) -> D + 'static>,
    {
        self.map(f.as_ref()).map_err(g.as_ref())
    }
}

pub trait Profunctor {
    type Fst; // b
    type Snd; // c
    type P<A: 'static, D: 'static>: Profunctor<Fst = A, Snd = D>;

    // dimap :: (a -> b) -> (c -> d) -> p b c -> p a d
    // self :: p b c
    fn dimap<A: 'static, D: 'static, F, G>(self, f: F, g: G) -> Self::P<A, D>
    where
        F: Fn(A) -> Self::Fst + 'static,
        G: Fn(Self::Snd) -> D + 'static;
}

impl<B: 'static, C: 'static> Profunctor for Box<dyn Fn(B) -> C> {
    type Fst = B;
    type Snd = C;
    // XXX: TAIT is not available/stable (i.e., `impl Fn(A) -> D`)
    type P<A: 'static, D: 'static> = Box<dyn Fn(A) -> D>;

    fn dimap<A: 'static, D: 'static, F, G>(self, f: F, g: G) -> Self::P<A, D>
    where
        F: Fn(A) -> Self::Fst + 'static,
        G: Fn(Self::Snd) -> D + 'static,
    {
        Box::new(compose(g, compose(self, f)))
    }
}

pub type ISO<A, B> = (Box<dyn Fn(A) -> B>, Box<dyn Fn(B) -> A>);

pub fn iso<A: 'static, B: 'static, F1, F2>(a: F1, b: F2) -> ISO<A, B>
where
    F1: 'static + Fn(A) -> B,
    F2: 'static + Fn(B) -> A,
{
    (Box::new(a), Box::new(b))
}

/// given ISO a b, we can go from a to b
pub fn sub_st_l<A, B>(iso: ISO<A, B>) -> Box<dyn Fn(A) -> B> {
    iso.0
}

/// and vise versa
pub fn sub_st_r<A, B>(iso: ISO<A, B>) -> Box<dyn Fn(B) -> A> {
    iso.1
}

/// There can be more than one ISO a b
pub fn iso_bool() -> ISO<bool, bool> {
    refl()
}

pub fn iso_bool_not() -> ISO<bool, bool> {
    iso(|b: bool| !b, |b| !b)
}

/// isomorphism is reflexive
pub fn refl<A: 'static>() -> ISO<A, A> {
    iso(id, id)
}

/// isomorphism is symmetric
pub fn symm<A: 'static, B: 'static>((f, g): ISO<A, B>) -> ISO<B, A> {
    iso(g, f)
}

/// isomorphism is transitive
pub fn trans<A: 'static, B: 'static, C: 'static>(
    (ab, ba): ISO<A, B>,
    (bc, cb): ISO<B, C>,
) -> ISO<A, C> {
    iso(compose::<A, B, C, _, _>(bc, ab), compose(ba, cb))
}

/// Isomorphism lifted to an arbitrary [`Functor`]
pub fn iso_functor<A: 'static, B: 'static, F, G>((ab, ba): ISO<A, B>) -> ISO<F, G>
where
    F: Functor<Obj = A, Fun<B> = G> + 'static,
    G: Functor<Obj = B, Fun<A> = F> + 'static,
{
    iso(move |f: F| f.fmap(&ab), move |g: G| g.fmap(&ba))
}

pub fn iso_vec<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Vec<A>, Vec<B>> {
    iso_functor(i)
}

pub fn iso_option<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Option<A>, Option<B>> {
    iso_functor(i)
}

/// Two isomorphisms lifted to an [`ISO`] over an arbitrary [`Bifunctor`]
pub fn iso_bifunctor<A, B, C, D, F, G>((ab, ba): ISO<A, B>, (cd, dc): ISO<C, D>) -> ISO<F, G>
where
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    F: Bifunctor<Fst = A, Snd = C, P<B, D> = G> + 'static,
    G: Bifunctor<Fst = B, Snd = D, P<A, C> = F> + 'static,
{
    iso(move |f: F| f.bimap(&ab, &cd), move |g: G| g.bimap(&ba, &dc))
}

/// Product of two isomorphisms
pub fn iso_tuple<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<(A, C), (B, D)> {
    iso_bifunctor(ab, cd)
}

/// Two isomorphisms lifted to an [`ISO`] of [`Result`]
pub fn iso_result<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<Result<A, C>, Result<B, D>> {
    iso_bifunctor(ab, cd)
}

/// Going another way is hard (and is generally impossible). Remember, for all valid ISO,
/// converting and converting back is the same as the original value. You need this to prove some
/// case are impossible.
pub fn iso_un_option<A: 'static, B: 'static>((fab, fba): ISO<Option<A>, Option<B>>) -> ISO<A, B> {
    iso(
        move |a| match fab(Some(a)) {
            Some(b) => b,
            None => fab(None).expect("some"),
        },
        move |b| match fba(Some(b)) {
            Some(a) => a,
            None => fba(None).expect("some"),
        },
    )
}

/// inf + 0 = inf + 1
pub fn iso_eu() -> ISO<Result<Vec<()>, ()>, Result<Vec<()>, Void>> {
    iso(
        |r: Result<Vec<()>, ()>| match r {
            Ok(mut xs) => {
                xs.push(());
                Ok(xs)
            }
            Err(_) => Ok(Vec::new()),
        },
        |r| match r {
            Ok(xs) if xs.is_empty() => Err(()),
            Ok(mut xs) => {
                let _ = xs.pop();
                Ok(xs)
            }
            Err(x) => absurd(x),
        },
    )
}

pub type IsoFL<A, B, C, D> = Box<dyn FnOnce(Box<dyn Fn(A) -> C>) -> Box<dyn FnOnce(B) -> D>>;
pub type IsoFR<A, B, C, D> = Box<dyn FnOnce(Box<dyn Fn(B) -> D>) -> Box<dyn FnOnce(A) -> C>>;
pub type IsoF<A, B, C, D> = (IsoFL<A, B, C, D>, IsoFR<A, B, C, D>);

/// Function isomorphism
pub fn iso_func<A: 'static, B: 'static, C: 'static, D: 'static>(
    (ab, ba): ISO<A, B>,
    (cd, dc): ISO<C, D>,
) -> IsoF<A, B, C, D> {
    // NOTE: trait upcasting coercion (Fn to FnOnce) is experimental, hence the aux closure
    (
        Box::new(|ac| Box::new(|b| ac.dimap(ba, cd)(b))),
        Box::new(|bd| Box::new(|a| bd.dimap(ab, dc)(a))),
    )
}

/// Symmetry of two isomorphisms lifted to an [`ISO`]
pub fn iso_symm<A: 'static, B: 'static>() -> ISO<ISO<A, B>, ISO<B, A>> {
    iso(symm, symm)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
