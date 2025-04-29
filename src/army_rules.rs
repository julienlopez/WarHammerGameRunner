#[derive(Clone)]
pub struct Army {
    pub name: String,
    pub detachments: Vec<usize>,
}

#[derive(Clone, PartialEq)]
pub struct Enhancement {
    pub name: String,
    pub cost: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Key {
    EitherPlayersTurn,
    YourTurn,
    OpponentsTurn,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Category {
    BattleTactic,
    EpicDeed,
    StrategicPloy,
    Wargear,
}

#[derive(Clone, PartialEq)]
pub struct Stratagem {
    pub name: String,
    pub cp_cost: usize,
    pub key: Key,
    pub category: Category,
    pub when: String,
    pub target: String,
    pub effect: String,
}

#[derive(Clone)]
pub struct DetachmentRule {
    pub name: String,
}

#[derive(Clone)]
pub struct Detachment {
    pub name: String,
    pub enhancements: Vec<Enhancement>,
    pub stratagems: Vec<Stratagem>,
    pub detachement_rules: Vec<DetachmentRule>,
}

impl Detachment {
    pub fn new(name: &str) -> Self {
        Detachment {
            name: name.to_string(),
            enhancements: vec![],
            stratagems: vec![],
            detachement_rules: vec![],
        }
    }
}

#[derive(Clone)]
pub struct ArmyRules {
    pub armies: Vec<Army>,
    pub detachments: Vec<Detachment>,
}

pub fn generate_army_rules() -> ArmyRules {
    let armour_of_contempt = Stratagem {
        name: "Armour of Contempt".to_string(),
        cp_cost: 1,
        key: Key::EitherPlayersTurn,
        category: Category::BattleTactic,
        when: "".to_string(),
        target: "".to_string(),
        effect: "".to_string(),
    };
    ArmyRules {
        armies: vec![
            Army {
                name: "Space Marines".to_string(),
                detachments: vec![0, 1, 2, 3, 4, 5, 6],
            },
            Army {
                name: "Dark Angels".to_string(),
                detachments: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            },
        ],
        detachments: vec![
            Detachment::new("1st Company Task Force"),
            Detachment::new("Anvil Siege Force"),
            Detachment::new("Firestorm Assualt Force"),
            Detachment {
                name: "Gladius Task Force".to_string(),
                enhancements: vec![],
                stratagems: vec![
                    armour_of_contempt.clone(),
                    Stratagem {
                        name: "Only in the Death Does Duty End".to_string(),
                        cp_cost: 2,
                        key: Key::EitherPlayersTurn,
                        category: Category::EpicDeed,
                        when: "".to_string(),
                        target: "".to_string(),
                        effect: "".to_string(),
                    },
                    Stratagem {
                        name: "Honour the Chapter".to_string(),
                        cp_cost: 1,
                        key: Key::EitherPlayersTurn,
                        category: Category::BattleTactic,
                        when: "".to_string(),
                        target: "".to_string(),
                        effect: "".to_string(),
                    },
                    Stratagem {
                        name: "Adaptive Strategy".to_string(),
                        cp_cost: 1,
                        key: Key::YourTurn,
                        category: Category::StrategicPloy,
                        when: "".to_string(),
                        target: "".to_string(),
                        effect: "".to_string(),
                    },
                    Stratagem {
                        name: "Storm of Fire".to_string(),
                        cp_cost: 1,
                        key: Key::YourTurn,
                        category: Category::BattleTactic,
                        when: "".to_string(),
                        target: "".to_string(),
                        effect: "".to_string(),
                    },
                    Stratagem {
                        name: "Squad Tactics".to_string(),
                        cp_cost: 1,
                        key: Key::OpponentsTurn,
                        category: Category::StrategicPloy,
                        when: "".to_string(),
                        target: "".to_string(),
                        effect: "".to_string(),
                    },
                ],
                detachement_rules: vec![],
            },
            Detachment::new("Ironstorm Spearhead"),
            Detachment::new("Stormlance Task Force"),
            Detachment::new("Vanguard Spearhead"),
            Detachment::new("Company of Hunters"),
            Detachment {
                name: "Inner Circle Task Force".to_string(),
                enhancements: vec![
                    Enhancement {
                        name: "Champion of the Deathwing".to_string(),
                        cost: 15,
                    },
                    Enhancement {
                        name: "Eye of the Unseen".to_string(),
                        cost: 10,
                    },
                    Enhancement {
                        name: "Singular Will".to_string(),
                        cost: 20,
                    },
                    Enhancement {
                        name: "Deathwing Assault".to_string(),
                        cost: 30,
                    },
                ],
                stratagems: vec![
                    armour_of_contempt.clone(),
                    Stratagem {
                        name: "Martial Mastery".to_string(),
                        cp_cost: 1,
                        key: Key::EitherPlayersTurn,
                        category: Category::EpicDeed,
                        when: "".to_string(),
                        target: "".to_string(),
                        effect: "".to_string(),
                    },
                    Stratagem {
                        name: "Duty Unto Death".to_string(),
                        cp_cost: 1,
                        key: Key::EitherPlayersTurn,
                        category: Category::StrategicPloy,
                        when: "".to_string(),
                        target: "".to_string(),
                        effect: "".to_string(),
                    },
                    Stratagem {
                        name: "Relic Temportarium".to_string(),
                        cp_cost: 1,
                        key: Key::YourTurn,
                        category: Category::StrategicPloy,
                        when: "".to_string(),
                        target: "".to_string(),
                        effect: "".to_string(),
                    },
                    Stratagem {
                        name: "Wrath of the Lion".to_string(),
                        cp_cost: 1,
                        key: Key::YourTurn,
                        category: Category::EpicDeed,
                        when: "".to_string(),
                        target: "".to_string(),
                        effect: "".to_string(),
                    },
                    Stratagem {
                        name: "Unmatched Fortitude".to_string(),
                        cp_cost: 1,
                        key: Key::OpponentsTurn,
                        category: Category::BattleTactic,
                        when: "".to_string(),
                        target: "".to_string(),
                        effect: "".to_string(),
                    },
                ],
                detachement_rules: vec![DetachmentRule {
                    name: "Vowed Target".to_string(),
                }],
            },
            Detachment::new("Unforgiven Task Force"),
        ],
    }
}

impl ArmyRules {
    pub fn list_detachment_names_for_army(&self, army_index: usize) -> Vec<String> {
        assert!(army_index < self.armies.len());
        let army = self.armies[army_index].clone();
        let detachments = army.detachments.iter().map(|detachment_index| {
            assert!(*detachment_index < self.detachments.len());
            self.detachments[*detachment_index].clone()
        });
        detachments.map(|d| d.name.clone()).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_marine_detachments() {
        let rules = generate_army_rules();
        let space_marines_index = 0;
        assert_eq!(rules.armies[space_marines_index].name, "Space Marines");
        assert_eq!(
            rules.list_detachment_names_for_army(space_marines_index),
            vec![
                "1st Company Task Force",
                "Anvil Siege Force",
                "Firestorm Assualt Force",
                "Gladius Task Force",
                "Ironstorm Spearhead",
                "Stormlance Task Force",
                "Vanguard Spearhead"
            ]
        );
    }

    #[test]
    fn test_dark_angels_detachments() {
        let rules = generate_army_rules();
        let dark_angels_index = 1;
        assert_eq!(rules.armies[dark_angels_index].name, "Dark Angels");
        assert_eq!(
            rules.list_detachment_names_for_army(dark_angels_index),
            vec![
                "1st Company Task Force",
                "Anvil Siege Force",
                "Firestorm Assualt Force",
                "Gladius Task Force",
                "Ironstorm Spearhead",
                "Stormlance Task Force",
                "Vanguard Spearhead",
                "Company of Hunters",
                "Inner Circle Task Force",
                "Unforgiven Task Force"
            ]
        );
    }
}
