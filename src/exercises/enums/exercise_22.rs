//! Exercise 22: Expression Evaluator - AST with Enums
//! Difficulty: Hard
//!
//! # Learning Objectives
//! - Model abstract syntax trees with enums
//! - Implement recursive evaluation
//! - Handle complex nested structures

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// Creates a number expression
    pub fn num(n: f64) -> Self {
        Expr::Num(n)
    }

    /// Creates an addition expression
    pub fn add(left: Expr, right: Expr) -> Self {
        Expr::Add(Box::new(left), Box::new(right))
    }

    /// Creates a subtraction expression
    pub fn sub(left: Expr, right: Expr) -> Self {
        Expr::Sub(Box::new(left), Box::new(right))
    }

    /// Creates a multiplication expression
    pub fn mul(left: Expr, right: Expr) -> Self {
        Expr::Mul(Box::new(left), Box::new(right))
    }

    /// Creates a division expression
    pub fn div(left: Expr, right: Expr) -> Self {
        Expr::Div(Box::new(left), Box::new(right))
    }

    /// Evaluates the expression
    pub fn eval(&self) -> Result<f64, String> {
        match self {
            Expr::Num(n) => Ok(*n),
            Expr::Add(left, right) => {
                let l = left.eval()?;
                let r = right.eval()?;
                Ok(l + r)
            }
            Expr::Sub(left, right) => {
                let l = left.eval()?;
                let r = right.eval()?;
                Ok(l - r)
            }
            Expr::Mul(left, right) => {
                let l = left.eval()?;
                let r = right.eval()?;
                Ok(l * r)
            }
            Expr::Div(left, right) => {
                let l = left.eval()?;
                let r = right.eval()?;
                if r == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(l / r)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_number() {
        let expr = Expr::num(42.0);
        assert_eq!(expr.eval(), Ok(42.0));
    }

    #[test]
    fn test_addition() {
        let expr = Expr::add(Expr::num(10.0), Expr::num(5.0));
        assert_eq!(expr.eval(), Ok(15.0));
    }

    #[test]
    fn test_complex_expression() {
        // (10 + 5) * (20 - 15)
        let expr = Expr::mul(
            Expr::add(Expr::num(10.0), Expr::num(5.0)),
            Expr::sub(Expr::num(20.0), Expr::num(15.0)),
        );
        assert_eq!(expr.eval(), Ok(75.0));
    }

    #[test]
    fn test_division_by_zero() {
        let expr = Expr::div(Expr::num(10.0), Expr::num(0.0));
        assert!(expr.eval().is_err());
    }

    #[test]
    fn test_nested_operations() {
        // ((2 + 3) * 4) - 5
        let expr = Expr::sub(
            Expr::mul(Expr::add(Expr::num(2.0), Expr::num(3.0)), Expr::num(4.0)),
            Expr::num(5.0),
        );
        assert_eq!(expr.eval(), Ok(15.0));
    }
}
