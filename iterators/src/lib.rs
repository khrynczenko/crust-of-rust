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
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
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
        if self.front_iter.is_none() {
            if let Some(iter_next) = self.outer.next() {
                self.front_iter = Some(iter_next.into_iter());
            } else {
                return self.back_iter.as_mut()?.next()
            }
        }

        let x = self.front_iter.as_mut().unwrap().next();
        if let Some(v) = x {
            Some(v)
        } else {
            self.front_iter = None;
            self.next()
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: Iterator + DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back_iter.is_none() {
            if let Some(iter_next) = self.outer.next_back() {
                self.back_iter = Some(iter_next.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back()
            }
        }

        let x = self.back_iter.as_mut().unwrap().next_back();
        if let Some(v) = x {
            Some(v)
        } else {
            self.back_iter = None;
            self.next_back()
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
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
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
        assert_eq!(flatten(vec![vec!["a", "b"], vec!["b", "c"]]).count(), 4);
    }

    #[test]
    fn reverse() {
        let x = flatten(vec![vec!["a", "b"], vec!["b", "c"]])
            .rev()
            .collect::<Vec<&str>>();
        assert_eq!(x, vec!["c", "b", "b", "a"]);
    }
    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2"], vec!["b1", "b2"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b1"));
    }
}
