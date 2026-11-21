# Linear-Programming-Solver
Linear Programming Solver written with Rust

# Example

```math
\begin{gathered}
\max Z = 750x_0 + 900x_1 - 450x_2 \\[6pt]
\text{subject to}\qquad
\begin{cases}
x_0 + 2x_1 &\leq 70 \\
2x_0 + 3x_1 - x_2 &\leq 100 \\
x_0 &\geq 20 \\
x_1 &\geq 25
\end{cases}
\end{gathered}
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
- [ ] TODO : Read from MPS file