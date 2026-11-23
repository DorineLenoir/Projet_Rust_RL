pub mod decod;
pub mod interpret;

const MAX_STEPS:usize=200; //nombre maximum de step par épisode

use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};
use decod::{Instruction, decode_instruction};
use interpret::Interpreter;


fn run_episode(program: &[Instruction], rng: &mut PCGRng) -> f32 {

    let mut env = CartPole::default();
    // Reset the environment to its initial state
    let mut total_reward = 0.0;


    // démarrer l'épisode 
    let mut observation=env.reset(rng);

    // à chaque pas on crée un interpeteur 

    for _ in 0..MAX_STEPS {
        // Nouvel interpréteur à chaque step entrées = observation et les registres sont init à 0
        let mut interp = Interpreter::new(observation);
        let output = interp.run(program);

// convertir la sortie en action et faire avancer l'environement 

        let (new_obs, reward, done) = env.step(output,&mut rng);
        observation=new_obs;

        total_reward += reward;

        if done {
            break;
        }
    }
   
    total_reward
}

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
        let (, reward, done) = env.step(action, &mut rng); // Take a step in the environment with the chosen action

        // Accumulate the reward received from the environment
        total_reward += reward;
        // Check if the episode is done (i.e., the pole has fallen or the cart has moved out of bounds)
        if done {
            break;
        }
    }

    println!("Total reward: {}", total_reward);


}
