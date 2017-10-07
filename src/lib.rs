// rule_set = [Rule.Rule("X", " F-[[X]+X]+F[+FX]-X"), Rule.Rule("F", "FF")]
// Barely modified implementation of https://github.com/atheriel/lsystem/blob/master/src/lib.rs

use std::mem;

pub struct LSystem<T, F: FnMut(T) -> Vec<T>> {
    axiom: Vec<T>,
    rules: F,
    zeroth: bool
}

impl<T, F> LSystem<T, F> where F: FnMut(T) -> Vec<T> {
    pub fn new(axiom: Vec<T>, rules: F) -> LSystem<T, F> {
        LSystem {axiom: axiom, rules: rules, zeroth: true}
    }
}

impl<T, F> Iterator for LSystem<T, F> where T: Clone, F: FnMut(T) -> Vec<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.zeroth {
            self.zeroth = false;
            return Some(self.axiom.clone())
        }

        let old_axiom = mem::replace(&mut self.axiom, Vec::new());

        for element in old_axiom.into_iter() {
            self.axiom.extend((self.rules)(element).into_iter());
        }
        Some(self.axiom.clone())
    }
}
