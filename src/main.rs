use std::fmt;
// const 'static i32 M = 100_000;

#[derive(Debug)]
enum Optimization {
    Maximization,
    Minimization
}

#[derive(Debug)]
enum TypeInequality {
    Inf,
    Sup,
    Eq
}

#[derive(Debug)]
struct Constraint {
    inequality: TypeInequality,
    coefficients: Vec<f32>,
    b: f32
}

#[derive(Debug)]
struct Problem {
    optimization: Optimization,
    objective_coefficients: Vec<f32>,
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
    name: TypeVariable,
    id_all: usize, // reference var in all type of var
    id: usize // reference var in its type
    // second name pour donner un vrai nom ?
}

#[derive(Debug, PartialEq)]
struct Solution {
    objective: f32,
    variables: Vec<Variable>,
    variables_values: Vec<f32>
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Solution :\n")?;
        for (index, var) in self.variables.iter().enumerate() {
            if var.name == TypeVariable::Objective {
                write!(f, "Var x{} = {}\n", var.id, self.variables_values[index])?;
            }
        }
        write!(f, "Objective value : {}", self.objective)?;
        Ok(())
    }
}

impl Problem {
    fn solve(&self) -> Option<Solution> { // Split le debut de cette fonction dans une compile function ?
        // if pb::opti == min : * - 1
        // for each constraint, if rhx < 0 : * - 1
        // add s1 si <=, = ou >= -s1 + a1 !! pas de - si si contrainte = de base a reverifier
        /// ligne du pivot divisee par par le pivot

        // base Xb : non nulles, à regarder à la fin pour la solution : partie colonne gauche tableau
        // hors base X_n : nulles, 0 à la fin
        let nb_constraints = self.constraints.len();
        let nb_slacks = self.constraints.len(); // !! TODO PAS VRAI SI DES CONTRAINTES EGALES DEJA PRESENTES AVANT MISE EN FORME STANDARD
        let nb_vars = self.objective_coefficients.len();
        println!("nbSlacks : {:?}", nb_slacks);
        println!("nbVars : {:?}", nb_vars);
        println!("nbConstrains : {:?}", nb_constraints);

        let mut z_objective = Vec::with_capacity(nb_vars + nb_slacks);
        for i in 0..nb_vars {
            z_objective.push(self.objective_coefficients[i])
        }
        for _ in 0..nb_slacks {
            z_objective.push(0.0);
        }
        println!("zObjective : {:?}", z_objective);


        let mut all_vars: Vec<Variable> = vec![];
        for i in 0..nb_vars {
            all_vars.push(Variable{name: TypeVariable::Objective, id_all: i, id: i})
        } 
        for i in 0..nb_slacks {
            all_vars.push(Variable{name: TypeVariable::Slack, id_all: i + nb_vars, id: i})
        } 
        println!("all_vars : {:?}", all_vars);

        let mut x_base: Vec<Variable> = vec![];
        for i in 0..nb_constraints {
            x_base.push(Variable{name: TypeVariable::Slack, id_all: i + nb_vars, id: i}) // attention apres ce sera pas tout le temps une slack
        } 
        println!("xBase : {:?}", x_base);

        let mut b_vector = vec![];
        for i in 0..nb_constraints {
            b_vector.push(self.constraints[i].b);
        }
        println!("bVector : {:?}", b_vector);
        // z_objective[4] = 10.;
        let solution = self.simplex(&all_vars, &mut z_objective, &mut x_base, &mut b_vector, nb_constraints);

        solution
    }

    fn simplex(&self, all_vars: &Vec<Variable>, z_objective: &mut Vec<f32>, x_base: &mut Vec<Variable>, b_vector: &mut Vec<f32>, nb_constraints: usize) -> Option<Solution> {
        println!("STARTING SIMPLEX");

        println!("all_vars : {:?}", all_vars);
        println!("z_objective : {:?}", z_objective);
        println!("x_base      : {:?}", x_base);
        println!("b_vector    : {:?}", b_vector);
        println!("{:?}", self.constraints);
        let length_rows = z_objective.len();
        let mut constraints: Vec<Vec<f32>> = vec![]; // todo 1D ?
        println!("{:?}", length_rows);
        println!("{:?}", constraints);

        for i in 0..nb_constraints {
            println!("{}", i);
            let mut row: Vec<f32> = Vec::with_capacity(length_rows);
            for var in all_vars {
                match var.name {
                    TypeVariable::Objective => row.push(self.constraints[i].coefficients[var.id]),
                    TypeVariable::Slack if var.id == i => row.push(1.0),
                    TypeVariable::Slack => row.push(0.0),
                    TypeVariable::Excess if var.id == i => row.push(- 1.0),
                    TypeVariable::Excess => row.push(0.0),
                    TypeVariable::Artificial => println!("{:?}", "4"), // TODO

                }
            }
            constraints.push(row);
        }
        println!("constraints {:?}", constraints);

        // ligne du pivot divisee par le pivot
        let max_iterations = 15;
        let mut current_iteration = 0;
        let mut objective = 0.0;
        // while current_iteration < max_iterations {
        let solution = loop {
            if current_iteration == max_iterations {
                break None
            }
            current_iteration += 1;
            println!("CURRENT ITERATION {:?}", current_iteration);
            println!("z_objective {:?}", z_objective);

            if let Some(entering_base) = arg_max(&z_objective, &all_vars) {
                println!("entering_base {:?}", entering_base);
                let x_pivot = entering_base.id_all;
                println!("x_pivot {:?}", x_pivot);
                let b_divided: Vec<f32> = b_vector.iter().enumerate()
                    .map(|(index, x)| {
                        // todo par forcement besoin de faire cette verif a ce moment pour perfs
                        match constraints[index][x_pivot] {
                            0.0 => - 1.0,
                            val => x / val
                        }
                    }).collect();
                println!("b_divided {:?}", b_divided);
                let y_pivot = arg_min(&b_divided);
                println!("y_pivot {:?}", y_pivot);
                let pivot = constraints[y_pivot][x_pivot];
                println!("pivot {:?}", pivot);
                
                // Starting update tableau
                // todo : pas besoin de mut ici vu que overwrite a literation suivante
                let mut z_objective_clone = z_objective.clone();
                let mut b_vector_clone = b_vector.clone();
                let mut constraints_clone = constraints.clone();
                objective = objective - (z_objective[x_pivot] * b_vector[y_pivot]) / pivot;
                println!("NEW objective {:?}", objective);
                *z_objective = z_objective_clone.iter().enumerate()
                    .map(|(index, x)| x - (z_objective_clone[x_pivot] * constraints_clone[y_pivot][index]) / pivot).collect::<Vec<f32>>();
                println!("NEW z_objective {:?}", z_objective);

                *b_vector = b_vector_clone.iter().enumerate()
                    .map(|(index, x)| {
                        match index {
                            line_pivot if line_pivot == y_pivot => x / pivot,
                            _ => x - (constraints_clone[index][x_pivot] * b_vector_clone[y_pivot]) / pivot
                        }
                    }).collect::<Vec<f32>>();
                println!("NEW b_vector {:?}", b_vector);

                constraints = constraints_clone.iter().enumerate()
                    .map(|(index_row, row)| row.iter().enumerate()
                    .map(|(index_val, val)| {
                        match index_row {
                            line_pivot if line_pivot == y_pivot => val / pivot,
                            _ => val - (constraints_clone[index_row][x_pivot] * constraints_clone[y_pivot][index_val])
                        }
                    })
                    .collect::<Vec<f32>>())
                    .collect::<Vec<Vec<f32>>>();
                
                // updating base :
                x_base[y_pivot] = entering_base;
                println!("NEW x_base {:?}", x_base);

                println!("NEW constraints {:?}", constraints);


            } else {
                println!("Algo done");
                println!("Objective value : {}", - objective);
                println!("Variable value :");
                let mut variables_solution = vec![];
                let mut variables_values = vec![];
                for (index, base) in x_base.iter().enumerate() {
                    let name_var = match base.name {
                        TypeVariable::Objective => {
                            variables_solution.push(*base);
                            variables_values.push(b_vector[index]);
                            'x'
                        },
                        TypeVariable::Slack => 's',
                        TypeVariable::Excess => 'e',
                        TypeVariable::Artificial => 'a' 
                    };
                    println!("Variable {}{} = {}", name_var, base.id, b_vector[index]);
                }                
                break Some(Solution{objective: - objective, variables: variables_solution, variables_values: variables_values})
            }
        };
        solution
    }
}

fn arg_max(vector: &Vec<f32>, all_vars: &Vec<Variable>) -> Option<Variable> {
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

fn arg_min(vector: &Vec<f32>) -> usize { // no optimal solutions if no positive entry ?
    let mut min = 1_000_000_000.0;
    let mut index_min = 0;
    for i in 0..vector.len() {
        if vector[i] < min && vector[i] > 0.0 {
            index_min = i;
            min = vector[i];
        }
    }
    index_min
}
// TODO : METTRE CODE DANS UN CONFIG TEST
// TODO add timer 
// attention quand * - 1 les variables coefficients, voir s'il faut aussi le faire sur - M
fn main() -> Result<(), std::io::Error> { // todo add proper Result

    let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![10.0, 11.0], b: 10700.0};
    let c2 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![1.0, 2.0], b: 1000.0};
    let c3 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 0.0], b: 700.0};

    let problem = Problem {
        optimization: Optimization::Maximization,
        objective_coefficients: vec![56.0, 42.0],
        constraints: vec![c1, c2, c3]
    };

    let solution = problem.solve().expect("Didn't find any solution"); // todo gerer correctement probleme sans solution
    println!("{}", solution);

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
    
        let problem = Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![4.0, 5.0],
            constraints: vec![c1, c2, c3]
        };

        let solution = problem.solve().expect("Didn't find any solution"); // todo gerer correctement probleme sans solution
        let target = Solution {
            objective: 22.0,
            variables: vec![
                Variable{name: TypeVariable::Objective, id_all: 0, id: 0},
                Variable{name: TypeVariable::Objective, id_all: 1, id: 1},
            ],
            variables_values: vec![3.0, 2.0]
        };
        assert_eq!(target, solution);
    }

    #[test]
    fn maxi_mixed_infsup_positive_b_solution_exists() {
        let c1 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![10.0, 11.0], b: 10700.0};
        let c2 = Constraint{inequality: TypeInequality::Sup, coefficients: vec![1.0, 2.0], b: 1000.0};
        let c3 = Constraint{inequality: TypeInequality::Inf, coefficients: vec![1.0, 0.0], b: 700.0};
    
        let problem = Problem {
            optimization: Optimization::Maximization,
            objective_coefficients: vec![56.0, 42.0],
            constraints: vec![c1, c2, c3]
        };

        let solution = problem.solve().expect("Didn't find any solution"); // todo gerer correctement probleme sans solution
        let target = Solution {
            objective: 46200.0,
            variables: vec![
                Variable{name: TypeVariable::Objective, id_all: 0, id: 0},
                Variable{name: TypeVariable::Objective, id_all: 1, id: 1},
            ],
            variables_values: vec![300.0, 700.0]
        };
        assert_eq!(target, solution);
    }
}