//! Exercise 28: Monad-like bind operations
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Understand monadic operations
//! - Build complex compositions
//! - Master functional patterns

/// Bind operation for chaining optional computations.
pub fn bind_chain<T, U, V>(
    value: Option<T>,
    f: impl FnOnce(T) -> Option<U>,
    g: impl FnOnce(U) -> Option<V>,
) -> Option<V>  {
    todo!("Bind operation for chaining optional computations.")
}

/// Triple bind composition.
pub fn triple_bind<T, U, V, W>(
    value: Option<T>,
    f: impl FnOnce(T) -> Option<U>,
    g: impl FnOnce(U) -> Option<V>,
    h: impl FnOnce(V) -> Option<W>,
) -> Option<W>  {
    todo!("Triple bind composition.")
}

/// Kleisli composition - compose two functions that return Options.
pub fn kleisli_compose<T, U, V>(
    f: impl FnOnce(T) -> Option<U>,
    g: impl FnOnce(U) -> Option<V>,
) -> impl FnOnce(T) -> Option<V>  {
    todo!("Kleisli composition - compose two functions that return Options.");
    #[allow(unreachable_code)]
    move |_: T| { let _ = (f, g); None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_chain() {
        let result = bind_chain(Some(5), |x| Some(x * 2), |x| Some(x + 3));
        assert_eq!(result, Some(13));

        let result = bind_chain(Some(5), |_| None, |x: i32| Some(x + 3));
        assert_eq!(result, None);
    }

    #[test]
    fn test_triple_bind() {
        let result = triple_bind(
            Some(2),
            |x| Some(x * 2),
            |x| Some(x + 1),
            |x| Some(x * x),
        );
        assert_eq!(result, Some(25)); // ((2 * 2) + 1) * ((2 * 2) + 1) = 5 * 5 = 25
    }

    #[test]
    fn test_kleisli_compose() {
        let double = |x| Some(x * 2);
        let add_three = |x| Some(x + 3);
        let composed = kleisli_compose(double, add_three);
        
        assert_eq!(composed(5), Some(13));
    }

    #[test]
    fn test_kleisli_with_failure() {
        let safe_div = |x: i32| if x == 0 { None } else { Some(10 / x) };
        let double = |x| Some(x * 2);
        
        let composed1 = kleisli_compose(safe_div, double);
        assert_eq!(composed1(2), Some(10));
        
        let composed2 = kleisli_compose(safe_div, double);
        assert_eq!(composed2(0), None);
    }
}
