use std::{collections::HashMap, hash::Hash};

pub trait Nameable {
    type Id: Eq + Hash;

    fn id(&self) -> Self::Id;
    fn requested_name(&self) -> String;
}

#[derive(Debug)]
pub struct Namer<Subject>
where
    Subject: Nameable,
{
    named_subjects: HashMap<Subject::Id, String>,
    used_names: HashMap<String, u16>,
}

impl<Subject> Namer<Subject>
where
    Subject: Nameable,
{
    pub fn new() -> Namer<Subject> {
        Namer {
            named_subjects: HashMap::new(),
            used_names: HashMap::new(),
        }
    }

    pub fn name_subject(&mut self, subject: &Subject) -> String {
        if let Some(name) = self.named_subjects.get(&subject.id()) {
            return name.clone();
        }

        self.impl_naming(subject, subject.requested_name())
    }

    fn impl_naming(&mut self, subject: &Subject, requested_name: String) -> String {
        let used_count = self.used_names.entry(requested_name.clone()).or_insert(0);
        *used_count += 1;
        let name = if *used_count == 1 {
            requested_name
        } else {
            format!("{}{}", requested_name, used_count)
        };

        self.named_subjects.insert(subject.id(), name.clone());

        name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NamedThing {
        id: i32,
        my_name: String,
    }

    impl Nameable for NamedThing {
        type Id = i32;

        fn requested_name(&self) -> String {
            self.my_name.to_owned()
        }

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    #[test]
    fn test_naming() {
        let thing_one = NamedThing {
            id: 0,
            my_name: "Thing".into(),
        };
        let thing_two = NamedThing {
            id: 1,
            my_name: "Thing".into(),
        };
        let other_thing = NamedThing {
            id: 2,
            my_name: "OtherThing".into(),
        };

        let mut namer = Namer::new();

        // First give things names
        assert_eq!(namer.name_subject(&thing_one), "Thing");
        assert_eq!(namer.name_subject(&thing_two), "Thing2");
        assert_eq!(namer.name_subject(&other_thing), "OtherThing");

        // Now make sure the names are still the same when called again
        assert_eq!(namer.name_subject(&other_thing), "OtherThing");
        assert_eq!(namer.name_subject(&thing_two), "Thing2");
        assert_eq!(namer.name_subject(&thing_one), "Thing");
    }
}
