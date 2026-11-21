use linear_programming_solver::{Constraint, Optimization, Problem, TypeInequality};

// TODO add timer
fn main() -> Result<(), std::io::Error> { // todo add proper Result

    let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 2.0, 0.0], b: 70.0};
    let c2 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![2.0, 3.0, - 1.0], b: 100.0};
    let c3 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![1.0, 0.0, 0.0], b: 20.0};
    let c4 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![0.0, 1.0, 0.0], b: 25.0};

    let mut problem = Problem {
        optimization: Optimization::Maximization,
        objective_coefficients: vec![750.0, 900.0, - 450.0],
        constraints: vec![c1, c2, c3, c4]
    };

    let solution = problem.solve();
    println!("{}", solution);

    Ok(())
}
