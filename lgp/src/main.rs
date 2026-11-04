use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

fn main() {
    // Création de l'envrionnement 

    // Initialize the random number generator and the environment
    let seed = 1;
    // Create a new PCGRng with the given seed
    let mut rng = PCGRng::seed(seed);
    // Create a new CartPole environment
    let mut env = CartPole::default();
    // Reset the environment to its initial state
    let mut total_reward = 0.0;

// Commence le loop
    loop {
        let action = (rng.next_u32() % 4) as f32; // Generate a random action between 0 and 3
        let (_, reward, done) = env.step(action, &mut rng); // Take a step in the environment with the chosen action

        // Accumulate the reward received from the environment
        total_reward += reward;
        // Check if the episode is done (i.e., the pole has fallen or the cart has moved out of bounds)
        if done {
            break;
        }
    }

    println!("Total reward: {}", total_reward)
}
