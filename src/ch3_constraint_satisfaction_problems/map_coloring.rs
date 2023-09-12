use crate::ch3_constraint_satisfaction_problems::csp::{Constraint, CSP};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct MapColoringConstraint<'a> {
    variables: Vec<&'a str>,
}

impl<'a> MapColoringConstraint<'a> {
    fn new(place1: &'a str, place2: &'a str) -> Self {
        Self {
            variables: vec![place1, place2],
        }
    }
}

impl<'a> Constraint<&'a str, &'a str> for MapColoringConstraint<'a> {
    fn get_variables(&self) -> &Vec<&'a str> {
        &self.variables
    }

    fn satisfied(&self, assignment: &HashMap<&str, &str>) -> bool {
        if let (Some(place1_color), Some(place2_color)) = (
            assignment.get(self.variables.first().unwrap()),
            assignment.get(self.variables.last().unwrap()),
        ) {
            place1_color != place2_color
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_australia_map_coloring() {
        // Define the variables (regions) and their domains (colors)
        let variables = vec![
            "Western Australia",
            "Northern Territory",
            "South Australia",
            "Queensland",
            "New South Wales",
            "Victoria",
            "Tasmania",
        ];

        let domains: HashMap<&str, Vec<&str>> = vec![
            ("Western Australia", vec!["red", "green", "blue"]),
            ("Northern Territory", vec!["red", "green", "blue"]),
            ("South Australia", vec!["red", "green", "blue"]),
            ("Queensland", vec!["red", "green", "blue"]),
            ("New South Wales", vec!["red", "green", "blue"]),
            ("Victoria", vec!["red", "green", "blue"]),
            ("Tasmania", vec!["red", "green", "blue"]),
        ]
        .iter()
        .cloned()
        .collect();

        // Define the constraints (no two adjacent regions should share a color)
        let constraints = vec![
            MapColoringConstraint::new("Western Australia", "Northern Territory"),
            MapColoringConstraint::new("Western Australia", "South Australia"),
            MapColoringConstraint::new("Northern Territory", "South Australia"),
            MapColoringConstraint::new("Northern Territory", "Queensland"),
            MapColoringConstraint::new("South Australia", "Queensland"),
            MapColoringConstraint::new("South Australia", "New South Wales"),
            MapColoringConstraint::new("South Australia", "Victoria"),
            MapColoringConstraint::new("Queensland", "New South Wales"),
            MapColoringConstraint::new("New South Wales", "Victoria"),
            MapColoringConstraint::new("Victoria", "Tasmania"),
        ];

        // Create the CSP and solve it using backtracking search
        let mut csp = CSP::new(variables, domains);

        constraints
            .iter()
            .for_each(|constraint| csp.add_constraint(Arc::new((*constraint).clone())));

        let solution = csp.backtracking_search(HashMap::new());

        // Check that the solution is valid (no two adjacent regions share a color)
        assert!(solution.is_some());

        let solution = solution.unwrap();

        assert_eq!(solution.len(), 7);

        assert!(solution
            .get("Western Australia")
            .ne(&solution.get("Northern Territory")));

        assert!(solution
            .get("Western Australia")
            .ne(&solution.get("South Australia")));

        assert!(solution
            .get("Northern Territory")
            .ne(&solution.get("South Australia")));

        assert!(solution
            .get("Northern Territory")
            .ne(&solution.get("Queensland")));

        assert!(solution
            .get("South Australia")
            .ne(&solution.get("Queensland")));

        assert!(solution
            .get("South Australia")
            .ne(&solution.get("New South Wales")));

        assert!(solution
            .get("South Australia")
            .ne(&solution.get("Victoria")));

        assert!(solution
            .get("Queensland")
            .ne(&solution.get("New South Wales")));

        assert!(solution
            .get("New South Wales")
            .ne(&solution.get("Victoria")));

        assert!(solution.get("Victoria").ne(&solution.get("Tasmania")));
    }
}
