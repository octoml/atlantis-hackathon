use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SimState {
    pub workers: Vec<Worker>,
    pub neighbor_map: Vec<(usize, usize)>,
    pub score: i64,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Worker {
    pub id: usize,
    pub flavor: Flavor,
    pub desk: Vec<Pearl>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum Flavor {
    General,
    Vector,
    Matrix,
}

impl Flavor {
    pub fn nom_speed(&self, color: Color) -> usize {
        match self {
            General => match color {
                Red => 1,
                Green => 1,
                Blue => 1,
            },
            Vector => match color {
                Red => 1,
                Green => 5,
                Blue => 2,
            },
            Matrix => match color {
                Red => 0,
                Green => 0,
                Blue => 10,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Pearl {
    #[serde(rename = "id")]
    pub pearl_id: usize,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Layer {
    pub color: Color,
    pub thickness: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum Color {
    Red,
    Green,
    Blue,
}

//////////

#[derive(Debug, Clone, Default, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Instructions(pub HashMap<usize, Instruction>);

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum Instruction {
    Pass { pearl_id: i64, to_worker: i64 },
    Nom(usize),
}

/// Policy function
///
/// Implement your policy for selecting action for workers here
fn plan(state: SimState) -> Instructions {
    Instructions::default()
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = BufReader::new(io::stdin());
    let mut stdout = io::stdout();

    loop {
        buffer.clear();
        stdin.read_line(&mut buffer)?;

        let state: SimState = serde_json::from_str(&buffer)?;
        let actions = plan(state);


        let actions_json = serde_json::to_string(&actions).context("serialize commands")?;

        stdout.write_all(format!("{}\n", actions_json).as_bytes())?;
        stdout.flush()?;
    }
}
