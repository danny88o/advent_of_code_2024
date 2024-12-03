
pub trait Solution {
    const NAME: &'static str;

    fn part_one(&self);

    fn part_two(&self);

    fn run(&self, inp: String);

    #[inline]
    #[must_use]
    fn name(&self) -> &'static str {
        Self::NAME
    }
}