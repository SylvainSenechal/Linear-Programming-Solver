use crate::{Optimization, Problem, TypeInequality};

use super::super::runner::{Expected, constraint, run_case};

#[test]
fn inequality_constraints() {
    run_case(
        || Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![-3.0, -2.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[2.0, 1.0], 10.0),
                constraint(TypeInequality::Inf, &[1.0, 1.0], 8.0),
                constraint(TypeInequality::Inf, &[1.0, 0.0], 4.0),
            ],
        },
        Expected::Optimal(-18.0),
    );
}

#[test]
fn inequality_constraints2() {
    run_case(
        || Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![6.0, 3.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[0.0, 3.0], 2.0),
                constraint(TypeInequality::Inf, &[-1.0, -1.0], -1.0),
                constraint(TypeInequality::Inf, &[-2.0, 1.0], -1.0),
            ],
        },
        Expected::Optimal(5.0),
    );
}

#[test]
fn zero_row_1() {
    run_case(
        || Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![1.0, 2.0, 3.0],
            constraints: vec![
                constraint(TypeInequality::Eq, &[0.0, 0.0, 0.0], 0.0),
                constraint(TypeInequality::Eq, &[1.0, 1.0, 1.0], 3.0),
                constraint(TypeInequality::Eq, &[0.0, 0.0, 0.0], 0.0),
            ],
        },
        Expected::Optimal(3.0),
    );
}

#[test]
fn singleton_row_eq_1() {
    run_case(
        || Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![1.0, 1.0, 1.0, 2.0],
            constraints: vec![
                constraint(TypeInequality::Eq, &[1.0, 0.0, 0.0, 0.0], 1.0),
                constraint(TypeInequality::Eq, &[0.0, 2.0, 0.0, 0.0], 2.0),
                constraint(TypeInequality::Eq, &[1.0, 0.0, 0.0, 0.0], 2.0),
                constraint(TypeInequality::Eq, &[1.0, 1.0, 1.0, 1.0], 4.0),
            ],
        },
        Expected::Infeasible,
    );
}

#[test]
fn infeasible() {
    run_case(
        || Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![-1.0, -1.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[1.0, 0.0], 2.0),
                constraint(TypeInequality::Inf, &[0.0, 1.0], 2.0),
                constraint(TypeInequality::Inf, &[-1.0, -1.0], -5.0),
            ],
        },
        Expected::Infeasible,
    );
}

#[test]
fn empty_constraint_1() {
    run_case(
        || Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![-1.0, -2.0],
            constraints: vec![],
        },
        Expected::Unbounded,
    );
}

#[test]
fn unbounded() {
    run_case(
        || Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![-1.0, -1.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[-1.0, 1.0], -1.0),
                constraint(TypeInequality::Inf, &[-1.0, -1.0], -2.0),
            ],
        },
        Expected::Unbounded,
    );
}

#[test]
fn simplex_algorithm_wikipedia_example() {
    run_case(
        || Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![-2.0, -3.0, -4.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[3.0, 2.0, 1.0], 10.0),
                constraint(TypeInequality::Inf, &[2.0, 5.0, 3.0], 15.0),
            ],
        },
        Expected::Optimal(-20.0),
    );
}

#[test]
fn nontrivial_problem() {
    run_case(
        || Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![-1.0, 8.0, 4.0, -6.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[-7.0, -7.0, 6.0, 9.0], -3.0),
                constraint(TypeInequality::Inf, &[1.0, -1.0, -3.0, 0.0], 6.0),
                constraint(TypeInequality::Inf, &[10.0, -10.0, -7.0, 7.0], -6.0),
                constraint(TypeInequality::Inf, &[6.0, -1.0, 3.0, 4.0], 6.0),
                constraint(TypeInequality::Eq, &[-10.0, 1.0, 1.0, -8.0], -4.0),
            ],
        },
        Expected::Optimal(7083.0 / 1391.0),
    );
}
