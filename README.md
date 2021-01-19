# Linear-Programming-Solver
Linear Programming Solver written with Rust

# Example

```math
Max Z = 750x0 + 900x1 - 450x2
Subject To 
1x0 + 2x1       <= 70
2x0 + 3x1 - 1x2 <= 100
1x0             >= 20
      1x1       >=25
```

```rust
fn main() -> Result<(), std::io::Error> {

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
```


```rust
println!("{}", solution);
```

Solution :

Var x0 = 20

Var x1 = 25

Var x2 = 15

Objective value : 30750

# Functionalities
This solver implements the Simplex algorithm with the tableau method.
The bigM method is used.

- [x] Continuous variables
- [x] Inf, Sup, Eq inequalities supported
- [x] Maximization & Minimization supported
- [x] Detection of unbounded problems
- [x] Detection of degeneracy
- [x] Detection of unfeasible problems
- [ ] TODO : Add Integer & Binary variables solved with Gomory's Cuts
- [ ] TODO : Add Branch & Cut
- [ ] TODO : Use Rayon crate on tableau iterations to multithread the solver
