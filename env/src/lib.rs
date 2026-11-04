use rand::Rng;

pub mod cartpole;
pub mod pcgrng;

/// The [Score] is the value representing the performances of
/// a [Program] on an episode (or various episodes).
pub type Score = f32; 
// le score est une valeur flottante qui représente les performances d'un programme sur un épisode (ou plusieurs épisodes).
// A chaque étaoe rapporte 1 point, donc le score final est le nombre d'étapes avant que l'épisode ne se termine (le pendule tombe ou le chariot sort de la piste).
// En général, un score plus élevé signifie que le pendule a tenu plus longtemps.


/// The [State] are 4 values that represent the current state 
/// of an environment. 
pub type State = [f32; 4];

// Le state est un tableau de 4 valeurs flottantes représentant l'état actuel de l'environnement.
// on retrouve : - la position du chariot x (où il se trouve sur la piste), 
// - la vitesse du chariot x_dot (vitesse du chariot), 
// - l'angle du pendule theta (à quel point il est incliné), 
// - la vitesse angulaire theta_dot du pendule. (à quelle vitesse et dans quel sens l'angle)

// L'agent utilise ces valeurs pour choisir une action (force à appliquer au chariot) afin de maintenir le pendule en équilibre.
// APrès ces, l'environnement met à jour son état en fonction de l'action choisie et renvoie la nouvelle observation (nouvel état), la récompense obtenue et un indicateur de fin d'épisode (done).


pub trait Environment {
    fn step<R: Rng>(&mut self, action: f32, rng: &mut R) -> (State, Score, bool);
    fn reset<R: Rng>(&mut self, rng: &mut R) -> State;
}
