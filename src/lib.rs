pub mod count_down {
    use itertools::Itertools;
    use std::fmt;
    use std::sync::Arc;
    use rayon::prelude::*;

    type Int = i64;

    #[derive(Copy, Clone)]
    pub enum Op {
        Add,
        Sub,
        Mul,
        Div,
    }

    #[derive(Clone)]
    pub enum Expr {
        Val(Int),
        App(Op, Arc<Expr>, Arc<Expr>),
    }

    use Expr::*;
    use Op::*;

    fn valid(op: &Op, x: Int, y: Int) -> bool {
        match op {
            Add => x <= y,
            Sub => x > y,
            Mul => x != 1 && y != 1 && x <= y,
            Div => y > 1 && ((x % y) == 0),
        }
    }

    fn apply(op: &Op, a: Int, b: Int) -> Int {
        match op {
            Add => a + b,
            Sub => a - b,
            Mul => a * b,
            Div => a / b,
        }
    }

    fn split<T>(xs: &[T]) -> Vec<(&[T], &[T])> {
        (1..xs.len())
            .map(|i| xs.split_at(i))
            .collect()
    }

    fn sub_bags<T: Clone>(xs: Vec<T>) -> Vec<Vec<T>> {
        (0..xs.len() + 1)
            .flat_map(|i| xs.iter().cloned().permutations(i))
            .collect()
    }

    type Result = (Expr, Int);

    fn combine((l, x): &Result, (r, y): &Result) -> Vec<Result> {
        [Add, Sub, Mul, Div].iter()
            .filter(|op| valid(op, *x, *y))
            .map(|op|
                (App(*op, Arc::new(l.clone()), Arc::new(r.clone())),
                 apply(op, *x, *y)))
            .collect()
    }

    fn results(ns: &[Int]) -> Vec<Result> {
        match ns {
            [] => vec!(),
            [n] => vec!((Val(*n), *n)),
            _ => _results(ns),
        }
    }

    fn _results(ns: &[Int]) -> Vec<Result> {
        split(ns).iter()
            .flat_map(|(ls, rs)| results(ls).into_iter()
                .flat_map(move |lx| results(rs).into_iter()
                    .flat_map(move |ry| combine(&lx, &ry))))
            .collect()
    }

    pub fn solutions(ns: Vec<Int>, n: Int) -> Vec<Expr> {
        sub_bags(ns).par_iter()
            .flat_map(|bag|
                results(&bag).into_iter()
                    .filter(|(_, m)| *m == n)
                    .map(|(e, _)| e)
                    .collect::<Vec<Expr>>()
            )
            .collect()
    }

    // Utilities for displaying expressions
    fn get_str(expr: &Expr) -> String {
        match &*expr {
            Val(v) => v.to_string(),
            App(op, l, r) => format!("({} {:?} {})", get_str(&l), op, get_str(&r)),
        }
    }

    impl fmt::Debug for Op {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Add => write!(f, "+"),
                Sub => write!(f, "-"),
                Mul => write!(f, "*"),
                Div => write!(f, "/"),
            }
        }
    }

    impl fmt::Debug for Expr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", get_str(self))
        }
    }
}