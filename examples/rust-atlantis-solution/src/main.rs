use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

#[derive(Debug, Deserialize)]
struct State {
    // Add fields here, see README
}

#[derive(Debug, Serialize)]
struct Action {
    // Add fields here, see README
}

#[derive(Debug, Default, Serialize)]
struct Actions(HashMap<usize, Action>);

fn plan(state: State) -> Actions {
    Actions::default()
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = BufReader::new(io::stdin());
    let mut stdout = io::stdout();

    loop {
        buffer.clear();
        stdin.read_line(&mut buffer)?;

        let state: State = serde_json::from_str(&buffer)?;
        let actions = plan(state);


        let actions_json = serde_json::to_string(&actions).context("serialize commands")?;

        stdout.write_all(format!("{}\n", actions_json).as_bytes())?;
        stdout.flush()?;
    }
}
