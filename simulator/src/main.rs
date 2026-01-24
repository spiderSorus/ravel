mod ravel_ecs;

use ravel_ecs::Universe;

fn main() {
    println!("Initializing Ravel Universe (Ecology Mode)...");
    
    let mut universe = Universe::new();
    
    println!("Spawning Harmonic Organisms...");
    
    // Initial seeding
    for i in 0..10 {
        universe.spawn_harmonic_organism(50, 0.5); // High mutation initially to create variance
        if i % 2 == 0 {
            universe.update_census();
        }
    }
    
    // Simulation Loop
    for i in 0..5 {
        println!("--- Generation {} ---", i);
        
        // 1. Calculate Demand
        universe.update_census();
        let demand = universe.get_ecological_demand();
        
        // 2. Respond to Demand (Spawn new life)
        let _entity = universe.spawn_harmonic_organism(50, 0.1); // Low mutation, high harmony
        
        let state = universe.get_global_state();
        let energy: i64 = state.iter().map(|&x| x.abs() as i64).sum();
        
        println!("Total Energy: {}", energy);
        println!("Demand [0..5]: {:?}", &demand[0..5]);
    }
}
