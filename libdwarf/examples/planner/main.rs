use libdwarf::planner::{Condition, Planner, State};
use ron::de::from_reader;
use std::fs::File;

fn main() {
    let input_path = format!(
        "{}/examples/resources/actions.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    println!("{}", input_path);

    let f = File::open(&input_path).expect("Failed opening file");

    let planner: Planner = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    println!("{:?}", planner);

    let mut initial_state = State::new();
    initial_state.insert(Condition::Visible("enemy".to_string()), false);
    initial_state.insert(Condition::Has("gun".to_string()), true);
    initial_state.insert(
        Condition::HasProperty("weapon".to_string(), "loaded".to_string()),
        false,
    );

    initial_state.insert(Condition::Alive("enemy".to_string()), true);
    initial_state.insert(Condition::Has("bomb".to_string()), true);
    initial_state.insert(Condition::Near("enemy".to_string()), false);
    initial_state.insert(Condition::Alive("me".to_string()), true);

    let mut end_state = State::new();
    end_state.insert(Condition::Alive("enemy".to_string()), false);
    end_state.insert(Condition::Alive("me".to_string()), true);

    let mut planned = planner.plan(&initial_state, &end_state);
    while let Some(action) = planned.pop() {
        println!("{:?}", action);
    }
}
