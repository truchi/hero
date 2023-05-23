use super::*;

#[derive(Clone, Debug)]
pub struct Chord {
    pub rythm: Rythm,
    pub mode: Mode,
}

#[derive(Clone, Debug)]
pub struct Score(pub Vec<Chord>);
