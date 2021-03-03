// We use traits with an associated types for traits that will have only
// one implementation for a given type. Otherwise we would use generics.
// This is a rule of thumb, and should be taken with a grain of salt.
//
//
// trait<T> Iterator<T> {
// fn next(&mut self) -> Option<T>
// }
//
// This would work but in general it would be used
// impl<T> Iterator<T> for Vector<T> {
// ...
// }
//
// There is no reason to have eny other implementation for a Vector. This is
// why we would use trait with an associated types instead.
// Since there can exist only one implementation of a trait for a given type
// the type checker has easier job.

//trait Iterator {
//type Item;
//fn next(&mut self) -> Option<Self::Item>;
//}

pub fn flatten<I: IntoIterator>(iter: I) -> Flatten<I::IntoIter>
where
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_none() {
            self.inner = Some(self.outer.next()?.into_iter());
        }

        let x = self.inner.as_mut().unwrap().next();
        if let Some(v) = x {
            Some(v)
        } else {
            self.inner = None;
            self.next()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<&[()]>()).count(), 0);
    }

    #[test]
    fn many_empty() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(),
            0
        );
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_inner() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }

    #[test]
    fn two_inner_four() {
        assert_eq!(
            flatten(vec![vec!["a", "b"], vec!["b", "c"]]).count(),
            4
        );
    }
}
