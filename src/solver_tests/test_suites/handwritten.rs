use crate::{Optimization, Problem, TypeInequality};

use super::super::runner::{Expected, constraint, run_case};

#[test]
fn maxi_full_inf_positive_b_solution_exists_1() {
    run_case(
        || Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![4.0, 5.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[2.0, 1.0], 8.0),
                constraint(TypeInequality::Inf, &[1.0, 2.0], 7.0),
                constraint(TypeInequality::Inf, &[0.0, 1.0], 3.0),
            ],
        },
        Expected::Optimal(22.0),
    );
}

#[test]
fn maxi_full_inf_positive_b_solution_exists_2() {
    run_case(
        || Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![3.0, 5.0, 4.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[2.0, 3.0, 0.0], 8.0),
                constraint(TypeInequality::Inf, &[0.0, 2.0, 5.0], 10.0),
                constraint(TypeInequality::Inf, &[3.0, 2.0, 4.0], 15.0),
            ],
        },
        Expected::Optimal(765.0 / 41.0),
    );
}

#[test]
fn mini_mixed_infsup_positive_b_solution_exists() {
    run_case(
        || Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![56.0, 42.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[10.0, 11.0], 10700.0),
                constraint(TypeInequality::Sup, &[1.0, 1.0], 1000.0),
                constraint(TypeInequality::Inf, &[1.0, 0.0], 700.0),
            ],
        },
        Expected::Optimal(46200.0),
    );
}

#[test]
fn mini_full_sup_positive_b_solution_exists_degeneracy() {
    run_case(
        || Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![1.0, 1.0],
            constraints: vec![
                constraint(TypeInequality::Sup, &[2.0, 4.0], 4.0),
                constraint(TypeInequality::Sup, &[1.0, 7.0], 7.0),
            ],
        },
        Expected::Optimal(1.0),
    );
}

#[test]
fn maxi_full_inf_positive_b_solution_exists_degeneracy_tie_basis() {
    run_case(
        || Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![3.0, 9.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[1.0, 4.0], 8.0),
                constraint(TypeInequality::Inf, &[1.0, 2.0], 4.0),
            ],
        },
        Expected::Optimal(18.0),
    );
}

#[test]
fn maxi_mixed_infsup_positive_b_solution_exists_degeneracy_tie_artifical() {
    run_case(
        || Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![750.0, 900.0, -450.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[1.0, 2.0, 0.0], 70.0),
                constraint(TypeInequality::Inf, &[2.0, 3.0, -1.0], 100.0),
                constraint(TypeInequality::Sup, &[1.0, 0.0, 0.0], 20.0),
                constraint(TypeInequality::Sup, &[0.0, 1.0, 0.0], 25.0),
            ],
        },
        Expected::Optimal(30750.0),
    );
}

#[test]
fn maxi_mixed_infsup_positive_b_unbouded() {
    run_case(
        || Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![3.0, 5.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[1.0, -2.0], 6.0),
                constraint(TypeInequality::Inf, &[1.0, 0.0], 10.0),
                constraint(TypeInequality::Sup, &[0.0, 1.0], 1.0),
            ],
        },
        Expected::Unbounded,
    );
}

#[test]
fn maxi_mixed_infsup_positive_b_infeasible() {
    run_case(
        || Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![6.0, 4.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[1.0, 1.0], 5.0),
                constraint(TypeInequality::Sup, &[0.0, 1.0], 8.0),
            ],
        },
        Expected::Infeasible,
    );
}

#[test]
fn maxi_full_inf_positive_b_infeasible() {
    run_case(
        || Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![1.0, -1.0],
            constraints: vec![
                constraint(TypeInequality::Inf, &[1.0, 1.0], 3.0),
                constraint(TypeInequality::Inf, &[-1.0, 3.0], -4.0),
            ],
        },
        Expected::Infeasible,
    );
}
