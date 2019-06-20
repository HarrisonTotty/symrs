// {% include 'doc.template' %}

use crate::core::expression::*;

// --------------- Types ---------------

pub type Elements    = Option<Vec<Expression>>;
pub type KeyElements = Option<std::collections::HashMap<Expression, Expression>>;

// -------------------------------------


// ------------- Structures ------------

/// Represents a mathematical structure.
///
/// # Remarks
/// Structures may or may not have a name and may or may not contain a set of
/// iterative/keyword subexpressions called `Elements`.
pub trait Structure {
    /// Returns the names of the top-level elements contained within this structure.
    fn element_names(&self) -> Option<Vec<Option<String>>> {
        match self.elements() {
            Some(elements) => Some(elements.into_iter().map(|e| e.name()).collect()),
            None => None
        }
    }
    
    /// Returns the collection of elements contained within this expression.
    fn elements(&self) -> Elements {
        None
    }

    /// Returns the top-level flattened collection of names of the elements
    /// contained within this structure.
    fn flat_element_names(&self) -> Option<Vec<Option<String>>> {
        match self.flat_elements() {
            Some(elements) => Some(elements.into_iter().map(|e| e.name()).collect()),
            None => None
        }
    }

    /// Returns the flattened collection of elements contained within this structure.
    fn flat_elements(&self) -> Elements {
        let my_name = self.name();
        match self.elements() {
            Some(elements) => {
                let mut flat_elements: Vec<Expression> = Vec::new();
                for element in elements {
                    if element.name() == my_name {
                        if let Some(sub_elements) = element.flat_elements() {
                            flat_elements.extend(sub_elements);
                        }
                    } else {
                        flat_elements.push(element);
                    }
                }
                Some(flat_elements)
            },
            None => None
        }
    }

    /// Returns the collection of key elements contained within this structure.
    fn key_elements(&self) -> KeyElements {
        None
    }
    
    /// Returns the name of this mathematical structure.
    fn name(&self) -> Option<String> {
        None
    }

    /// Returns the string representation of this structure.
    fn to_string(&self) -> String {
        let name_string = match self.name() {
            Some(name) => name,
            None => String::from("")
        };
        match (self.elements(), self.key_elements()) {
            (Some(elements), Some(_key_elements)) => {
                let elements_string: String = elements.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let key_elements_string: String = String::from("");
                // {% raw %}
                format!("{}{{{}, {}}}", name_string, elements_string, key_elements_string)
                // {% endraw %}
            },
            (Some(elements), None) => {
                let elements_string: String = elements.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                // {% raw %}
                format!("{}{{{}}}", name_string, elements_string)
                // {% endraw %}
            },
            (None, Some(_key_elements)) => {
                String::from("")
            },
            (None, None) => name_string
        }
    }
}

// -------------------------------------
