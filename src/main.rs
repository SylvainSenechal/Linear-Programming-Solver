use std::fmt;
const BIG_M: f64 = 1_000_000_000.0; // We are using simplex with BigM method (used when canonical base is not a feasible base)
const MAX_ITERATION: i32 = 15; // Avoiding Hell 

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
enum TypeVariable { // todo definir l'id directement ici : Objective(id)
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
    // todo incorporer valeur dedans direction, initialser à None puis en résultat à Some(f64) ?
}

#[derive(Debug, PartialEq)]
struct Solution {
    objective: f64,
    variables: Vec<Variable>,
    variables_values: Vec<f64>
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Solution :\n")?;
        for (index, var) in self.variables.iter().enumerate() {
            if var.type_var == TypeVariable::Objective {
                write!(f, "Var x{} = {}\n", var.id, self.variables_values[index])?;
            }
        }
        write!(f, "Objective value : {}", self.objective)?;
        Ok(())
    }
}

impl Problem {
    fn solve(&mut self) -> Option<Solution> { // Split le debut de cette fonction dans une compile function ?
        // Compiling problem steps :
        // #1 : If Problem is Min Z = f(x), transform to Max - Z
        // #2 : For each constraint, if b member is < 0, multiply the whole constraint by - 1
        // #3 : Add slack vars for inf constraint, excess + artificial for sup constraint, and artificial for equal
        // #4 : For each artifical variable, add - M*a to objective function, where M >> coefficients variable
        // #5 : Build simplex tableau accordingly, solve it 
 
        let nb_vars = self.objective_coefficients.len();

        let mut z_objective: Vec<f64> = vec![]; // Objective as represented in the tableau (includes slack, excess, artifical var)
        let mut all_vars: Vec<Variable> = vec![]; // List of all the vars uses in simplex's tableau
        let mut x_base: Vec<Variable> = vec![]; // Feasible base
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

            // Building a feasible base and creating the differents variables used in the tableau cf #3
            match constraint.inequality {
                TypeInequality::Inf => {
                    x_base.push(Variable{type_var: TypeVariable::Slack, id: current_id_var});
                    all_vars.push(Variable{type_var: TypeVariable::Slack, id: current_id_var});
                    current_id_var += 1;
                },
                TypeInequality::Sup => {
                    all_vars.push(Variable{type_var: TypeVariable::Excess, id: current_id_var});
                    current_id_var += 1;
                    x_base.push(Variable{type_var: TypeVariable::Artificial, id: current_id_var});
                    all_vars.push(Variable{type_var: TypeVariable::Artificial, id: current_id_var});
                    current_id_var += 1;
                },
                TypeInequality::Eq => {
                    x_base.push(Variable{type_var: TypeVariable::Artificial, id: current_id_var});
                    all_vars.push(Variable{type_var: TypeVariable::Artificial, id: current_id_var});
                    current_id_var += 1;
                }
            }
        }

        for _ in 0..(all_vars.len() - nb_vars) { // Filling the end of z_objective withs zeros representing the non Objective variable
            z_objective.push(0.0);
        }
 
        // Building the simplex's tableau cf #5
        for (id, constraint) in self.constraints.iter().enumerate() {
            let mut row: Vec<f64> = Vec::with_capacity(z_objective.len());
            for var in &all_vars {
                match constraint.inequality {
                    TypeInequality::Inf => {
                        match var.type_var {
                            TypeVariable::Objective => row.push(constraint.coefficients[var.id]),
                            TypeVariable::Slack if var.id == id => row.push(1.0),
                            _ => row.push(0.0),
                        }
                    },
                    TypeInequality::Sup => match var.type_var {
                        TypeVariable::Objective => row.push(constraint.coefficients[var.id]),
                        TypeVariable::Excess if var.id == id => row.push(- 1.0),
                        TypeVariable::Artificial if var.id == id => row.push(1.0),
                        _ => row.push(0.0),
                    },
                    TypeInequality::Eq => match var.type_var {
                        TypeVariable::Objective => row.push(constraint.coefficients[var.id]),
                        TypeVariable::Artificial if var.id == id => row.push(1.0),
                        _ => row.push(0.0),
                    }
                }
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
                for var in all_vars.iter() {
                    if var.type_var == TypeVariable::Excess {
                        z_objective[var.id] -= BIG_M;
                    }
                }
            }
        }

        println!("all_vars    : {:?}", all_vars);
        println!("z_objective : {:?}", z_objective);
        println!("x_base      : {:?}", x_base);
        println!("b_vector    : {:?}", b_vector);
        println!("constraints : {:?}", constraints);

        let solution = self.simplex(&mut objective, &all_vars, &mut z_objective, &mut x_base, &mut b_vector, &mut constraints); 
        solution
    }

    fn simplex(&self, objective: &mut f64, all_vars: &Vec<Variable>, z_objective: &mut Vec<f64>, x_base: &mut Vec<Variable>, b_vector: &mut Vec<f64>, constraints: &mut Vec<Vec<f64>>) -> Option<Solution> {
        println!("\nSTARTING SIMPLEX...\n");

        let mut current_iteration = 0;

        let solution = loop {
            println!("\nCURRENT ITERATION {:?}", current_iteration);
            if current_iteration == MAX_ITERATION {
                break None
            }
            current_iteration += 1;

            if let Some(entering_base) = arg_max(&z_objective, &all_vars) {
                let x_pivot = entering_base.id;
                let b_divided: Vec<f64> = b_vector.iter().enumerate()
                    .map(|(index, x)| {
                        match constraints[index][x_pivot] { // todo par forcement besoin de faire cette verif a ce moment pour perfs
                            0.0 => - 1.0,
                            val => x / val
                        }
                    }).collect();
                let y_pivot = arg_min(&b_divided);
                let pivot = constraints[y_pivot][x_pivot];
                println!("pivot {:?}", pivot);
                
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
                println!("NEW z_objective {:?}", z_objective);
                println!("NEW b_vector    {:?}", b_vector);
                println!("NEW x_base      {:?}", x_base);
                println!("NEW constraints {:?}", constraints);
            } else {
                println!("Simplex finished with success");
                let objective = match self.optimization {
                    Optimization::Maximization => - *objective,
                    Optimization::Minimization => *objective
                };
                println!("Variable value :");
                let mut variables_solution = vec![];
                let mut variables_values = vec![];
                for (index, base) in x_base.iter().enumerate() {
                    let name_var = match base.type_var {
                        TypeVariable::Objective => {
                            variables_solution.push(*base);
                            variables_values.push(b_vector[index]);
                            'x'
                        },
                        TypeVariable::Slack => 's',
                        TypeVariable::Excess => 'e',
                        TypeVariable::Artificial => 'a' 
                    };
                }
                let solution = Solution{objective: objective, variables: variables_solution, variables_values: variables_values};
                println!("{}", solution);

                break Some(solution)
            }
        };
        solution 
    }
}

fn arg_max(vector: &Vec<f64>, all_vars: &Vec<Variable>) -> Option<Variable> {
    let mut max = 0.0;
    let mut index_max = 0;
    for i in 0..vector.len() {
        if vector[i] > max {
            index_max = i;
            max = vector[i];
        }
    }

    if max == 0.0 {
        return None
    }
    Some(all_vars[index_max])
}

fn arg_min(vector: &Vec<f64>) -> usize { // todo : no optimal solutions if no positive entry ?
    let mut min = 1_000_000_000_000.0; // todo use const
    let mut index_min = 0;
    for i in 0..vector.len() {
        if vector[i] < min && vector[i] > 0.0 {
            index_min = i;
            min = vector[i];
        }
    }
    index_min
}

// TODO add timer 
fn main() -> Result<(), std::io::Error> { // todo add proper Result

    let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![10.0, 11.0], b: 10700.0};
    let c2 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![1.0, 1.0], b: 1000.0};
    let c3 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 0.0], b: 700.0};

    let mut problem = Problem {
        optimization: Optimization::Minimization,
        objective_coefficients: vec![56.0, 42.0],
        constraints: vec![c1, c2, c3]
    };

    let solution = problem.solve().expect("Didn't find any solution"); // todo gerer correctement probleme sans solution

    Ok(())
}

// todo : retravailler l'ordre des variables pour la comparaison
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maxi_full_inf_positive_b_solution_exists() {
        let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![2.0, 1.0], b: 8.0};
        let c2 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 2.0], b: 7.0};
        let c3 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![0.0, 1.0], b: 3.0};
    
        let mut problem = Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![4.0, 5.0],
            constraints: vec![c1, c2, c3]
        };

        let solution = problem.solve().expect("Didn't find any solution"); // todo gerer correctement probleme sans solution
        let target = Solution {
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
    fn mini_mixed_infsup_positive_b_solution_exists() {
        let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![10.0, 11.0], b: 10700.0};
        let c2 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![1.0, 1.0], b: 1000.0};
        let c3 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 0.0], b: 700.0};
    
        let mut problem = Problem {
            optimization: Optimization::Minimization,
            objective_coefficients: vec![56.0, 42.0],
            constraints: vec![c1, c2, c3]
        };

        let solution = problem.solve().expect("Didn't find any solution"); // todo gerer correctement probleme sans solution
        let target = Solution {
            objective: 46200.0,
            variables: vec![
                Variable{type_var: TypeVariable::Objective, id: 0},
                Variable{type_var: TypeVariable::Objective, id: 1},
            ],
            variables_values: vec![300.0, 700.0]
        };
        assert_eq!(target, solution);
    }
}