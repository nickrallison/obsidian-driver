use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Yaml {
    yaml: serde_yaml::Value,
}