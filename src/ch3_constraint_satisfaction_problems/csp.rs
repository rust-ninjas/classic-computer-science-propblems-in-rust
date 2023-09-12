use std::{collections::HashMap, hash::Hash, sync::Arc};

pub trait Constraint<V: Eq + PartialEq + Hash, D: Clone + PartialEq> {
    fn get_variables(&self) -> &Vec<V>;
    fn satisfied(&self, assignment: &HashMap<V, D>) -> bool;
}

#[allow(dead_code)]
pub struct CSP<V: Eq + PartialEq + Hash + Clone, D: Clone + PartialEq> {
    variables: Vec<V>,
    domains: HashMap<V, Vec<D>>,
    constraints: HashMap<V, Vec<Arc<dyn Constraint<V, D>>>>,
}

#[allow(dead_code)]
impl<V: Eq + PartialEq + Hash + Clone, D: Clone + PartialEq> CSP<V, D> {
    pub fn new(variables: Vec<V>, domains: HashMap<V, Vec<D>>) -> CSP<V, D> {
        for variable in &variables {
            if !domains.contains_key(variable) {
                panic!("Every variable should have a domain assigned to it.")
            }
        }

        CSP {
            variables,
            domains,
            constraints: HashMap::new(),
        }
    }

    pub fn add_constraint(&mut self, constraint: Arc<dyn Constraint<V, D>>) {
        for variable in constraint.get_variables() {
            if !self.variables.contains(variable) {
                panic!("Variable in constraint not in CSP")
            } else {
                let constraints_for_variable = self
                    .constraints
                    .entry((*variable).clone())
                    .or_insert(vec![]);

                constraints_for_variable.push(constraint.clone());
            }
        }
    }

    fn consistent(&self, variable: V, assignment: &HashMap<V, D>) -> bool {
        let constraint_list = self.constraints.get(&variable);

        if let Some(constraints) = constraint_list {
            for constraint in constraints {
                if !constraint.satisfied(assignment) {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn backtracking_search<'a>(&self, assignment: HashMap<V, D>) -> Option<HashMap<V, D>> {
        if assignment.len() == self.variables.len() {
            return Some(assignment);
        }

        let mut unassigned: Vec<V> = vec![];

        for v in &self.variables {
            if !assignment.contains_key(&v) {
                unassigned.push(v.clone());
            }
        }

        let first_option = unassigned.first();

        if let Some(first) = first_option {
            for value in &self.domains[first] {
                let mut local_assignment = assignment.clone();
                local_assignment.insert(first.clone(), value.clone());

                if self.consistent(first.clone(), &local_assignment) {
                    let result = self.backtracking_search(local_assignment);

                    if result.is_some() {
                        return result;
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(dead_code)]
    #[derive(Clone)]
    struct NotEqualConstraint<V: Eq + PartialEq + Hash + Clone> {
        variables: Vec<V>,
    }
    #[allow(dead_code)]
    impl<V: Eq + PartialEq + Hash + Clone> NotEqualConstraint<V> {
        fn new(variables: Vec<V>) -> Self {
            Self { variables }
        }
    }

    impl<V: Eq + PartialEq + Hash + Clone, D: Clone + PartialEq> Constraint<V, D>
        for NotEqualConstraint<V>
    {
        fn get_variables(&self) -> &Vec<V> {
            &self.variables
        }

        fn satisfied(&self, assignment: &HashMap<V, D>) -> bool {
            let values: Vec<&D> = self
                .variables
                .iter()
                .filter(|variable| assignment.get(variable).is_some())
                .map(|variable| assignment.get(variable).unwrap())
                .collect();

            for (i, value1) in values.iter().enumerate() {
                for (j, value2) in values.iter().enumerate() {
                    if i != j && **value1 == **value2 {
                        return false;
                    }
                }
            }

            true
        }
    }

    #[test]
    fn test_csp() {
        let variables = vec!["A", "B", "C"];
        let domains: HashMap<&str, Vec<i32>> = [
            ("A", vec![1, 2, 3]),
            ("B", vec![1, 2, 3]),
            ("C", vec![1, 2, 3]),
        ]
        .iter()
        .cloned()
        .collect();

        let mut csp = CSP::new(variables, domains);

        let a_b_not_equal = Arc::new(NotEqualConstraint {
            variables: vec!["A", "B"],
        });

        let b_c_not_equal = Arc::new(NotEqualConstraint {
            variables: vec!["B", "C"],
        });

        let a_c_not_equal = Arc::new(NotEqualConstraint {
            variables: vec!["A", "C"],
        });

        csp.add_constraint(a_b_not_equal);
        csp.add_constraint(b_c_not_equal);
        csp.add_constraint(a_c_not_equal);

        let result = csp.backtracking_search(HashMap::<&str, i32>::new());

        assert!(result.is_some());

        let mut expected: HashMap<&str, i32> = HashMap::new();
        expected.insert("A", 1);
        expected.insert("B", 2);
        expected.insert("C", 3);

        assert_eq!(result.unwrap(), expected);
    }
}
