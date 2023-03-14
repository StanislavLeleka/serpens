use rand::{distributions::uniform::SampleUniform, rngs::ThreadRng, Rng};

pub struct Generator {}

impl Generator {
    pub fn random_elements<T>(low: T, high: T, size: usize) -> Vec<T>
    where
        T: Copy + PartialOrd + SampleUniform,
    {
        let mut rng: ThreadRng = rand::thread_rng();
        (0..size).map(|_| rng.gen_range(low..high)).collect()
    }
}
