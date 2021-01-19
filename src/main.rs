use std::fmt;
const BIG_M: f64 = 1_000_000_000.0; // We are using simplex with BigM method (used when canonical base is not a bounded base)
const MIN_RATIO: f64 = 1_000_000_000_000.0; // Leaving variable ratio should be inferior to this
const MAX_ITERATION: i32 = 15; // Avoiding Hell 
const EPSILON: f64 = 0.0001; // Comparing the value of 2 variables (we lose precision even with f64)

#[derive(Debug)]
enum Optimization {
    Maximization,
    Minimization
}

#[derive(Debug, PartialEq)]
enum TypeInequality {
    Inf,
    Sup,
    Eq
}

#[derive(Debug)]
struct Constraint {
    inequality: TypeInequality,
    coefficients: Vec<f64>,
    b: f64
}

#[derive(Debug)]
struct Problem {
    optimization: Optimization,
    objective_coefficients: Vec<f64>,
    constraints: Vec<Constraint>
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum TypeVariable {
    Objective,
    Slack,
    Excess,
    Artificial
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Variable {
    type_var: TypeVariable,
    id: usize, // reference variable among all type of var
    // todo ajouter un vrai nom ?
}

#[derive(Debug, PartialEq)]
enum StateSolution {
    Feasible,
    Unfeasible,
    Unbounded,
    MaxIterationReached
}

#[derive(Debug)]
struct Solution {
    state: StateSolution,
    objective: f64,
    variables: Vec<Variable>,
    variables_values: Vec<f64>
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        if self.state != other.state {
            return false
        } else {
            if (self.objective - other.objective).abs() > EPSILON { // Objectives should be assigned very close values
                return false
            }
            for (variable, value) in self.variables.iter().zip(self.variables_values.iter()) {           
                let index_var = other.variables.iter().position(|x| x == variable);
                if let Some(id) = index_var {
                    if (other.variables_values[id] - value).abs() > EPSILON { // 2 identicals variables should be assigned very close values
                        return false
                    }
                } else { // Variable in one solution needs to be found in the other
                    return false
                }
            }
        }
        true
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.state {
            StateSolution::Unfeasible => write!(f, "This problem was found to be unfeasible")?,
            StateSolution::Unbounded => write!(f, "This problem was found to be unbounded")?,
            StateSolution::MaxIterationReached => write!(f, "Couldn't find a solution within max iterations = {} allowed", MAX_ITERATION)?,
            StateSolution::Feasible => {
                write!(f, "Solution :\n")?;
                for (index, var) in self.variables.iter().enumerate() {
                    if var.type_var == TypeVariable::Objective {
                        write!(f, "Var x{} = {}\n", var.id, self.variables_values[index])?;
                    }
                }
                write!(f, "Objective value : {}", self.objective)?;
            }
        }

        Ok(())
    }
}

impl Problem {
    fn solve(&mut self) -> Solution { // Split le debut de cette fonction dans une compile function ?
        // Compiling problem steps :
        // #1 : If Problem is Min Z = f(x), transform to Max - Z
        // #2 : For each constraint, if b member is < 0, multiply the whole constraint by - 1
        // #3 : Add slack vars for inf constraint, excess + artificial for sup constraint, and artificial for equal
        // #4 : For each artifical variable, add - M*a to objective function, where M >> coefficients variable
        // #5 : Build simplex tableau accordingly, solve it 
 
        let nb_vars = self.objective_coefficients.len();

        let mut z_objective: Vec<f64> = vec![]; // Objective as represented in the tableau (includes slack, excess, artifical var)
        let mut all_vars: Vec<Variable> = vec![]; // List of all the vars uses in simplex's tableau
        let mut x_base: Vec<Variable> = vec![]; // bounded base
        let mut b_vector: Vec<f64> = vec![]; // Vector of all b member
        let mut constraints: Vec<Vec<f64>> = vec![]; // Simplex's tableau


        for i in 0..nb_vars {
            match self.optimization { // Dealing with Min vs Max cf #1
                Optimization::Maximization => z_objective.push(self.objective_coefficients[i]),
                Optimization::Minimization => z_objective.push( - self.objective_coefficients[i]),
            }
        }

        for i in 0..nb_vars {
            all_vars.push(Variable{type_var: TypeVariable::Objective, id: i})
        }

        let mut current_id_var = nb_vars; // Unique reference to any var
        let mut ref_var_id: Vec<usize> = vec![];
        for constraint in self.constraints.iter_mut() {
            // Dealing with negative b member cf #2
            let multiplier = if constraint.b >= 0.0 {1.0} else {- 1.0};
            b_vector.push(constraint.b * multiplier);
            if constraint.b < 0.0 {
                constraint.b = - constraint.b;
                if constraint.inequality == TypeInequality::Inf {
                    constraint.inequality = TypeInequality::Sup;
                } else {
                    constraint.inequality = TypeInequality::Inf;
                }
                for val in &mut constraint.coefficients {
                    *val +=  - 1.0 * *val;
                }
            }

            // Building a bounded base and creating the differents variables used in the tableau cf #3
            match constraint.inequality {
                TypeInequality::Inf => {
                    x_base.push(Variable{type_var: TypeVariable::Slack, id: current_id_var});
                    all_vars.push(Variable{type_var: TypeVariable::Slack, id: current_id_var});
                    ref_var_id.push(current_id_var);
                    current_id_var += 1;
                },
                TypeInequality::Sup => {
                    all_vars.push(Variable{type_var: TypeVariable::Excess, id: current_id_var});
                    ref_var_id.push(current_id_var);
                    ref_var_id.push(current_id_var);
                    current_id_var += 1;
                    x_base.push(Variable{type_var: TypeVariable::Artificial, id: current_id_var});
                    all_vars.push(Variable{type_var: TypeVariable::Artificial, id: current_id_var});
                    current_id_var += 1;
                },
                TypeInequality::Eq => {
                    x_base.push(Variable{type_var: TypeVariable::Artificial, id: current_id_var});
                    all_vars.push(Variable{type_var: TypeVariable::Artificial, id: current_id_var});
                    ref_var_id.push(current_id_var);
                    current_id_var += 1;
                }
            }
        }

        for _ in 0..(all_vars.len() - nb_vars) { // Filling the end of z_objective withs zeros representing the non Objective variable
            z_objective.push(0.0);
        }
 
        // Building the simplex's tableau cf #5
        let mut nb_inf = 0;
        let mut nb_sup = 0;
        let mut nb_eq = 0;
        for constraint in self.constraints.iter() {
            let mut row: Vec<f64> = Vec::with_capacity(z_objective.len());

            for var in all_vars.iter() {
                match constraint.inequality {
                    TypeInequality::Inf => match var.type_var {
                        TypeVariable::Objective => row.push(constraint.coefficients[var.id]),
                        TypeVariable::Slack if var.id == (nb_vars + nb_inf + nb_eq + nb_sup * 2) => row.push(1.0),
                        _ => row.push(0.0)
                    },
                    TypeInequality::Sup => match var.type_var {
                        TypeVariable::Objective => row.push(constraint.coefficients[var.id]),
                        TypeVariable::Excess if var.id == (nb_vars + nb_inf + nb_eq + nb_sup * 2) => row.push(- 1.0),
                        TypeVariable::Artificial if var.id == (nb_vars + nb_inf + nb_eq + nb_sup * 2 + 1) => row.push(1.0),
                        _ => row.push(0.0)
                    },
                    TypeInequality::Eq => match var.type_var {
                        TypeVariable::Objective => row.push(constraint.coefficients[var.id]),
                        TypeVariable::Artificial if var.id == (nb_vars + nb_inf + nb_eq + nb_sup * 2) => row.push(1.0),
                        _ => row.push(0.0)
                    }
                }
            }
            match constraint.inequality {
                TypeInequality::Inf => nb_inf += 1,
                TypeInequality::Sup => nb_sup += 1,
                TypeInequality::Eq => nb_eq += 1
            }
            constraints.push(row);
        }

        // BigM Method, modifying the simplex's tableau to use this method
        let mut objective = 0.0;
        for (id_constraint, constraint) in self.constraints.iter().enumerate() {
            if constraint.inequality == TypeInequality::Sup || constraint.inequality == TypeInequality::Eq {
                objective += b_vector[id_constraint] * BIG_M;
                for (index, coefficient) in constraint.coefficients.iter().enumerate() {
                    z_objective[index] += coefficient * BIG_M;
                }
            }
        }
        for var in all_vars.iter() { // Adding penalties from Excess variables
            if var.type_var == TypeVariable::Excess {
                z_objective[var.id] = - BIG_M;
            }
        }

        println!("all_vars    : {:?}", all_vars);
        println!("objective   : {:?}", objective);
        println!("z_objective : {:?}", z_objective);
        println!("x_base      : {:?}", x_base);
        println!("b_vector    : {:?}", b_vector);
        println!("constraints : {:?}", constraints);

        let solution = self.simplex(&mut objective, &all_vars, &mut z_objective, &mut x_base, &mut b_vector, &mut constraints); 
        solution
    }

    fn simplex(&self, objective: &mut f64, all_vars: &Vec<Variable>, z_objective: &mut Vec<f64>, x_base: &mut Vec<Variable>, b_vector: &mut Vec<f64>, constraints: &mut Vec<Vec<f64>>) -> Solution {
        println!("\nSTARTING SIMPLEX...\n");

        let mut current_iteration = 0;

        let solution = 'outer: loop {
            println!("\nCURRENT ITERATION {:?}", current_iteration);
            if current_iteration == MAX_ITERATION {
                break Solution{state: StateSolution::MaxIterationReached, objective: 0.0, variables: vec![], variables_values: vec![]}
            }
            current_iteration += 1;

            if let Some(entering_base) = arg_max(&z_objective, &all_vars) {
                let x_pivot = entering_base.id;
                let b_divided: Vec<f64> = b_vector.iter().enumerate()
                    .map(|(index, x)| {
                        match constraints[index][x_pivot] {
                            0.0 => - 1.0, // discard division by 0
                            val => x / val
                        }
                    }).collect();
                    
                let y_pivot: usize;
                match arg_min(&b_divided) {
                    Some(y) => y_pivot = y,
                    None => break Solution{state: StateSolution::Unbounded, objective: 0.0, variables: vec![], variables_values: vec![]}
                }
                let pivot = constraints[y_pivot][x_pivot];
                println!("pivot {:?}", pivot);
                println!("x_pivot {:?}", x_pivot);
                println!("y_pivot {:?}", y_pivot);
                println!("b_divided {:?}", b_divided);
                
                // Starting the tableau update 
                let z_objective_clone = z_objective.clone();
                let b_vector_clone = b_vector.clone();
                let constraints_clone = constraints.clone();

                *objective = *objective - (z_objective[x_pivot] * b_vector[y_pivot]) / pivot;
                *z_objective = z_objective_clone.iter().enumerate()
                    .map(|(index, x)| x - (z_objective_clone[x_pivot] * constraints_clone[y_pivot][index]) / pivot).collect::<Vec<f64>>();

                *b_vector = b_vector_clone.iter().enumerate()
                    .map(|(index, x)| {
                        match index {
                            line_pivot if line_pivot == y_pivot => x / pivot,
                            _ => x - (constraints_clone[index][x_pivot] * b_vector_clone[y_pivot]) / pivot
                        }
                    }).collect::<Vec<f64>>();

                *constraints = constraints_clone.iter().enumerate()
                    .map(|(index_row, row)| row.iter().enumerate()
                    .map(|(index_val, val)| {
                        match index_row {
                            line_pivot if line_pivot == y_pivot => val / pivot,
                            _ => val - (constraints_clone[index_row][x_pivot] * constraints_clone[y_pivot][index_val]) / pivot
                        }
                    })
                    .collect::<Vec<f64>>())
                    .collect::<Vec<Vec<f64>>>();
                
                // Updating the Base :
                x_base[y_pivot] = entering_base;
                println!("New objective   {:?}", objective);
                println!("NEW z_objective {:?}", z_objective);
                println!("NEW b_vector    {:?}", b_vector);
                println!("NEW x_base      {:?}", x_base);
                println!("NEW constraints {:?}", constraints);
            } else {
                for (index_base, base) in x_base.iter().enumerate() { // If we couldn't get rid of positive artificial variable in the base, problem is unfeasible
                    if base.type_var == TypeVariable::Artificial && b_vector[index_base] > 0.0 {
                        break 'outer Solution{state: StateSolution::Unfeasible, objective: 0.0, variables: vec![], variables_values: vec![]} 
                    }
                }
                println!("Simplex finished with success");
                let objective = match self.optimization {
                    Optimization::Maximization => - *objective,
                    Optimization::Minimization => *objective
                };
                let nb_vars = self.objective_coefficients.len();
                let mut variables_solution = vec![];
                let mut variables_values = vec![];
                for id in 0..nb_vars {
                    let index_base = x_base.iter().position(|x| x.id == id);
                    match index_base { // If we find objective variable in the current base, we get it's value from b_vector...
                        Some(found_id) => {
                            variables_solution.push(x_base[found_id]);
                            variables_values.push(b_vector[found_id]);
                        }
                        None => { // ... if objective variable isn't found in the base, it means it's equal to 0
                            variables_solution.push(all_vars[id]);
                            variables_values.push(0.0);
                        }
                    }
                }

                let solution = Solution{state: StateSolution::Feasible, objective: objective, variables: variables_solution, variables_values: variables_values};
                println!("{}", solution);

                break solution
            }
        };
        solution 
    }
}

// This arg_max version might degenerate..
// fn arg_max(vector: &Vec<f64>, all_vars: &Vec<Variable>) -> Option<Variable> {
//     let mut max = 0.0;
//     let mut index_max = 0;
//     for i in 0..vector.len() {
//         if vector[i] > max {
//             index_max = i;
//             max = vector[i];
//         }
//     }
//     match max {
//         0.0 => None,
//         _ => Some(all_vars[index_max])
//     }
// }

// These version won't degenerate : Bland's rule version
fn arg_max(vector: &Vec<f64>, all_vars: &Vec<Variable>) -> Option<Variable> {
    for i in 0..vector.len() {
        if vector[i] > 0.0 {
            return Some(all_vars[i])
        }
    }
    None
}

fn arg_min(vector: &Vec<f64>) -> Option<usize> {
    let mut min = MIN_RATIO;
    let mut index_min = 0;
    for i in 0..vector.len() {
        if vector[i] < min && vector[i] >= 0.0 {
            index_min = i;
            min = vector[i];
        }
    }
    match min {
        MIN_RATIO => None,
        _ => Some(index_min)
    }
}

// TODO add timer 
fn main() -> Result<(), std::io::Error> { // todo add proper Result

    let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, - 2.0], b: 6.0};
    let c2 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 0.0], b: 10.0};
    let c3 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![0.0, 1.0], b: 1.0};

    let mut problem = Problem {
        optimization: Optimization::Maximization,
        objective_coefficients: vec![3.0, 5.0],
        constraints: vec![c1, c2, c3]
    };

    let solution = problem.solve();
    println!("{}", solution);

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maxi_full_inf_positive_b_solution_exists_1() {
        let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![2.0, 1.0], b: 8.0};
        let c2 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 2.0], b: 7.0};
        let c3 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![0.0, 1.0], b: 3.0};
    
        let mut problem = Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![4.0, 5.0],
            constraints: vec![c1, c2, c3]
        };

        let solution = problem.solve();
        let target = Solution {
            state: StateSolution::Feasible,
            objective: 22.0,
            variables: vec![
                Variable{type_var: TypeVariable::Objective, id: 0},
                Variable{type_var: TypeVariable::Objective, id: 1},
            ],
            variables_values: vec![3.0, 2.0]
        };
        assert_eq!(target, solution);
    }

    #[test]
    fn maxi_full_inf_positive_b_solution_exists_2() {
        let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![2.0, 3.0, 0.0], b: 8.0};
        let c2 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![0.0, 2.0, 5.0], b: 10.0};
        let c3 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![3.0, 2.0, 4.0], b: 15.0};
    
        let mut problem = Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![3.0, 5.0, 4.0],
            constraints: vec![c1, c2, c3]
        };

        let solution = problem.solve();
        let target = Solution {
            state: StateSolution::Feasible,
            objective: 765.0 / 41.0,
            variables: vec![
                Variable{type_var: TypeVariable::Objective, id: 0},
                Variable{type_var: TypeVariable::Objective, id: 1},
                Variable{type_var: TypeVariable::Objective, id: 2},
            ],
            variables_values: vec![89.0 / 41.0, 50.0 / 41.0, 62.0 / 41.0]
        };
        assert_eq!(target, solution);
    }

    #[test]
    fn mini_mixed_infsup_positive_b_solution_exists() {
        let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![10.0, 11.0], b: 10700.0};
        let c2 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![1.0, 1.0], b: 1000.0};
        let c3 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 0.0], b: 700.0};
    
        let mut problem = Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![56.0, 42.0],
            constraints: vec![c1, c2, c3]
        };

        let solution = problem.solve();
        let target = Solution {
            state: StateSolution::Feasible,
            objective: 46200.0,
            variables: vec![
                Variable{type_var: TypeVariable::Objective, id: 0},
                Variable{type_var: TypeVariable::Objective, id: 1},
            ],
            variables_values: vec![300.0, 700.0]
        };
        assert_eq!(target, solution);
    }

    #[test]
    fn mini_full_sup_positive_b_solution_exists_degeneracy() {
        let c1 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![2.0, 4.0], b: 4.0};
        let c2 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![1.0, 7.0], b: 7.0};
    
        let mut problem = Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![1.0, 1.0],
            constraints: vec![c1, c2]
        };

        let solution = problem.solve();
        let target = Solution {
            state: StateSolution::Feasible,
            objective: 1.0,
            variables: vec![
                Variable{type_var: TypeVariable::Objective, id: 0},
                Variable{type_var: TypeVariable::Objective, id: 1},
            ],
            variables_values: vec![0.0, 1.0]
        };
        assert_eq!(target, solution);
    }

    #[test]
    fn maxi_full_inf_positive_b_solution_exists_degeneracy_tie_basis() {
        let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 4.0], b: 8.0};
        let c2 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 2.0], b: 4.0};
    
        let mut problem = Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![3.0, 9.0],
            constraints: vec![c1, c2]
        };

        let solution = problem.solve();
        let target = Solution {
            state: StateSolution::Feasible,
            objective: 18.0,
            variables: vec![
                Variable{type_var: TypeVariable::Objective, id: 0},
                Variable{type_var: TypeVariable::Objective, id: 1},
            ],
            variables_values: vec![0.0, 2.0]
        };
        assert_eq!(target, solution);
    }

    #[test]
    fn maxi_mixed_infsup_positive_b_solution_exists_degeneracy_tie_artifical() {
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
        let target = Solution {
            state: StateSolution::Feasible,
            objective: 30750.0,
            variables: vec![
                Variable{type_var: TypeVariable::Objective, id: 0},
                Variable{type_var: TypeVariable::Objective, id: 1},
                Variable{type_var: TypeVariable::Objective, id: 2},
            ],
            variables_values: vec![20.0, 25.0, 15.0]
        };
        assert_eq!(target, solution);
    }

    #[test]
    fn maxi_mixed_infsup_positive_b_unbouded() {
        let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, - 2.0], b: 6.0};
        let c2 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 0.0], b: 10.0};
        let c3 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![0.0, 1.0], b: 1.0};
    
        let mut problem = Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![3.0, 5.0],
            constraints: vec![c1, c2, c3]
        };
    
        let solution = problem.solve();
        let target = Solution {
            state: StateSolution::Unbounded,
            objective: 0.0,
            variables: vec![],
            variables_values: vec![]
        };
        assert_eq!(target, solution);
    }

    #[test]
    fn maxi_mixed_infsup_positive_b_infeasible() {
        let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 1.0], b: 5.0};
        let c2 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![0.0, 1.0], b: 8.0};
    
        let mut problem = Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![6.0, 4.0],
            constraints: vec![c1, c2]
        };
    
        let solution = problem.solve();
        let target = Solution {
            state: StateSolution::Unfeasible,
            objective: 0.0,
            variables: vec![],
            variables_values: vec![]
        };
        assert_eq!(target, solution);
    }

    #[test]
    fn maxi_full_inf_positive_b_infeasible() {
        let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 1.0], b: 3.0};
        let c2 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![- 1.0, 3.0], b: - 4.0};
    
        let mut problem = Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![1.0, - 1.0],
            constraints: vec![c1, c2]
        };
    
        let solution = problem.solve();
        let target = Solution {
            state: StateSolution::Unfeasible,
            objective: 0.0,
            variables: vec![],
            variables_values: vec![]
        };
        assert_eq!(target, solution);
    }
}