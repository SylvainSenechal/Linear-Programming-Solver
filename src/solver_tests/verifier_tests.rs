use super::runner::{Expected, check_solution};
use crate::{
    Constraint, Optimization, Problem, Solution, StateSolution, TypeInequality, TypeVariable,
    Variable,
};

fn problem() -> Problem {
    Problem {
        optimization: Optimization::Minimization,
        objective_coefficients: vec![1.0],
        constraints: vec![Constraint {
            inequality: TypeInequality::Sup,
            coefficients: vec![1.0],
            b: 2.0,
        }],
    }
}

fn solution(value: f64, objective: f64) -> Solution {
    Solution {
        state: StateSolution::Feasible,
        objective,
        variables: vec![Variable {
            type_var: TypeVariable::Objective,
            id: 0,
        }],
        variables_values: vec![value],
    }
}

#[test]
#[should_panic(expected = "constraint 0 violated")]
fn rejects_infeasible_solution_with_correct_objective() {
    check_solution(&problem(), &solution(1.0, 1.0), Expected::Optimal(1.0));
}

#[test]
#[should_panic(expected = "reported objective")]
fn rejects_incorrect_reported_objective() {
    check_solution(&problem(), &solution(2.0, 3.0), Expected::Optimal(2.0));
}

#[test]
#[should_panic(expected = "non-finite variable value")]
fn rejects_non_finite_values() {
    check_solution(&problem(), &solution(f64::NAN, 2.0), Expected::Optimal(2.0));
}
