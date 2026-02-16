//! Exercise 29: Functor and Monad patterns
//! Difficulty: Expert
//!
//! # Learning Objectives
//! - Understand functor pattern (map)
//! - Understand monad pattern (and_then/flatMap)
//! - Apply functional programming concepts
//! - Implement custom monadic operations

/// Functor: Apply a function inside a context (map).
pub fn functor_map<T, U>(opt: Option<T>, f: impl FnOnce(T) -> U) -> Option<U> {
    opt.map(f)
}

/// Monad: Chain computations that return contexts (and_then).
pub fn monad_bind<T, U>(
    opt: Option<T>,
    f: impl FnOnce(T) -> Option<U>,
) -> Option<U> {
    opt.and_then(f)
}

/// Applicative: Apply a function in a context to a value in a context.
pub fn applicative_apply<T, U>(
    opt_f: Option<impl FnOnce(T) -> U>,
    opt_val: Option<T>,
) -> Option<U> {
    opt_f.and_then(|f| opt_val.map(f))
}

/// Implement liftA2 - lift a binary function to work with Options.
pub fn lift_a2<A, B, C>(
    f: impl FnOnce(A, B) -> C,
    opt_a: Option<A>,
    opt_b: Option<B>,
) -> Option<C> {
    opt_a.and_then(|a| opt_b.map(|b| f(a, b)))
}

/// Implement sequence - convert Vec<Option<T>> to Option<Vec<T>>.
pub fn sequence<T>(vec: Vec<Option<T>>) -> Option<Vec<T>> {
    vec.into_iter().collect()
}

/// Implement traverse - map and sequence in one operation.
pub fn traverse<T, U>(
    vec: Vec<T>,
    f: impl Fn(T) -> Option<U>,
) -> Option<Vec<U>> {
    vec.into_iter().map(f).collect()
}

/// Kleisli composition: compose two monadic functions.
pub fn kleisli_compose<A, B, C>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
) -> impl Fn(A) -> Option<C> {
    move |a| f(a).and_then(&g)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functor_map() {
        assert_eq!(functor_map(Some(5), |x| x * 2), Some(10));
        assert_eq!(functor_map(None, |x: i32| x * 2), None);
    }

    #[test]
    fn test_monad_bind() {
        let f = |x: i32| if x > 0 { Some(x * 2) } else { None };
        assert_eq!(monad_bind(Some(5), f), Some(10));
        assert_eq!(monad_bind(Some(0), f), None);
        assert_eq!(monad_bind(None, f), None);
    }

    #[test]
    fn test_applicative_apply() {
        let add_one = |x: i32| x + 1;
        assert_eq!(applicative_apply(Some(add_one), Some(5)), Some(6));
        let add_one2 = |x: i32| x + 1;
        assert_eq!(applicative_apply(None::<fn(i32) -> i32>, Some(5)), None);
        assert_eq!(
            applicative_apply(Some(add_one2), None::<i32>),
            None
        );
    }

    #[test]
    fn test_lift_a2() {
        let add = |a: i32, b: i32| a + b;
        assert_eq!(lift_a2(add, Some(3), Some(4)), Some(7));
        assert_eq!(lift_a2(add, None, Some(4)), None);
        assert_eq!(lift_a2(add, Some(3), None), None);
    }

    #[test]
    fn test_sequence() {
        assert_eq!(
            sequence(vec![Some(1), Some(2), Some(3)]),
            Some(vec![1, 2, 3])
        );
        assert_eq!(sequence(vec![Some(1), None, Some(3)]), None);
    }

    #[test]
    fn test_traverse() {
        let f = |x: i32| if x > 0 { Some(x * 2) } else { None };
        assert_eq!(traverse(vec![1, 2, 3], f), Some(vec![2, 4, 6]));
        assert_eq!(traverse(vec![1, 0, 3], f), None);
    }

    #[test]
    fn test_kleisli_compose() {
        let f = |x: i32| if x > 0 { Some(x * 2) } else { None };
        let g = |x: i32| if x < 100 { Some(x + 10) } else { None };
        let composed = kleisli_compose(f, g);

        assert_eq!(composed(5), Some(20)); // 5 * 2 + 10
        assert_eq!(composed(0), None); // f returns None
        assert_eq!(composed(50), None); // g returns None (100 >= 100)
    }
}
