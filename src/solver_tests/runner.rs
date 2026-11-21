use crate::{Constraint, Problem, Solution, StateSolution, TypeInequality, TypeVariable};

pub(super) enum Expected {
    Optimal(f64),
    Infeasible,
    Unbounded,
}

pub(super) fn constraint(inequality: TypeInequality, coefficients: &[f64], b: f64) -> Constraint {
    Constraint {
        inequality,
        coefficients: coefficients.to_vec(),
        b,
    }
}

fn tolerance(scale: f64) -> f64 {
    1e-6 + 1e-7 * scale
}

pub(super) fn run_case(make_problem: impl Fn() -> Problem, expected: Expected) {
    let solution = make_problem().solve();
    check_solution(&make_problem(), &solution, expected);
}

pub(super) fn check_solution(problem: &Problem, solution: &Solution, expected: Expected) {
    let expected_state = match expected {
        Expected::Optimal(_) => StateSolution::Feasible,
        Expected::Infeasible => StateSolution::Unfeasible,
        Expected::Unbounded => StateSolution::Unbounded,
    };
    assert_eq!(solution.state, expected_state, "unexpected solver status");

    let Expected::Optimal(expected_objective) = expected else {
        return;
    };

    let count = problem.objective_coefficients.len();
    assert_eq!(solution.variables.len(), count, "incorrect variable count");
    assert_eq!(
        solution.variables_values.len(),
        count,
        "incorrect value count"
    );

    let mut values = vec![None; count];
    for (variable, &value) in solution.variables.iter().zip(&solution.variables_values) {
        assert_eq!(variable.type_var, TypeVariable::Objective);
        assert!(
            variable.id < count,
            "unexpected variable id {}",
            variable.id
        );
        assert!(value.is_finite(), "non-finite variable value");
        assert!(
            value >= -tolerance(value.abs()),
            "negative variable value: {value}"
        );
        assert!(
            values[variable.id].replace(value).is_none(),
            "duplicate variable id"
        );
    }

    let values: Vec<f64> = values
        .into_iter()
        .map(|value| value.expect("missing variable"))
        .collect();

    for (index, constraint) in problem.constraints.iter().enumerate() {
        assert_eq!(
            constraint.coefficients.len(),
            count,
            "invalid test constraint"
        );
        let terms: Vec<f64> = constraint
            .coefficients
            .iter()
            .zip(&values)
            .map(|(coefficient, value)| coefficient * value)
            .collect();
        let lhs: f64 = terms.iter().sum();
        let scale = constraint.b.abs() + terms.iter().map(|value| value.abs()).sum::<f64>();
        assert!(
            lhs.is_finite() && scale.is_finite(),
            "non-finite constraint calculation"
        );
        let allowed = tolerance(scale);
        let satisfied = match constraint.inequality {
            TypeInequality::Inf => lhs <= constraint.b + allowed,
            TypeInequality::Sup => lhs >= constraint.b - allowed,
            TypeInequality::Eq => (lhs - constraint.b).abs() <= allowed,
        };
        assert!(
            satisfied,
            "constraint {index} violated: lhs={lhs}, rhs={}",
            constraint.b
        );
    }

    let actual_objective: f64 = problem
        .objective_coefficients
        .iter()
        .zip(&values)
        .map(|(coefficient, value)| coefficient * value)
        .sum();
    assert!(
        actual_objective.is_finite() && solution.objective.is_finite(),
        "non-finite objective"
    );
    assert!(
        (actual_objective - expected_objective).abs()
            <= tolerance(actual_objective.abs().max(expected_objective.abs())),
        "objective from variables {actual_objective} differs from expected {expected_objective}"
    );
    assert!(
        (solution.objective - actual_objective).abs()
            <= tolerance(solution.objective.abs().max(actual_objective.abs())),
        "reported objective {} differs from objective from variables {actual_objective}",
        solution.objective
    );
}
