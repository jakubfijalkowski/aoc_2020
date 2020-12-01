use std::iter::FusedIterator;

pub struct Pairwise<I1, I2>
where
    I1: IntoIterator,
    I2: IntoIterator,
{
    iter2: I2,

    curr_iter1: I1::IntoIter,
    curr_iter2: I2::IntoIter,

    curr_item1: Option<I1::Item>,
}

impl<I1, I2> Iterator for Pairwise<I1, I2>
where
    I1: IntoIterator<Item: Copy>,
    I2: IntoIterator + Copy,
{
    type Item = (I1::Item, I2::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_item1.is_none() {
            if let Some(x) = self.curr_iter1.next() {
                self.curr_item1 = Some(x);
                self.next()
            } else {
                None
            }
        } else if let Some(y) = self.curr_iter2.next() {
            Some((self.curr_item1.unwrap(), y))
        } else {
            self.curr_item1 = self.curr_iter1.next();
            self.curr_iter2 = self.iter2.into_iter();
            self.next()
        }
    }
}

impl<I1, I2> FusedIterator for Pairwise<I1, I2>
where
    I1: IntoIterator<Item: Copy, IntoIter: FusedIterator>,
    I2: IntoIterator + Copy,
{
}

impl<I1, I2> Pairwise<I1, I2>
where
    I1: IntoIterator<Item: Copy>,
    I2: IntoIterator + Copy,
{
    pub fn from(a: I1, b: I2) -> Self {
        Pairwise {
            iter2: b,

            curr_iter1: a.into_iter(),
            curr_iter2: b.into_iter(),

            curr_item1: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let empty: [i32; 0] = [];
        let a = [1];

        let mut p = Pairwise::from(&a, &empty);
        assert_eq!(None, p.next());

        let mut p = Pairwise::from(&empty, &a);
        assert_eq!(None, p.next());

        let mut p = Pairwise::from(&empty, &empty);
        assert_eq!(None, p.next());
    }

    #[test]
    fn single_element() {
        let a = [1];
        let mut p = Pairwise::from(&a, &a);
        assert_eq!(Some((&1, &1)), p.next());
        assert_eq!(None, p.next());
    }

    #[test]
    fn restartable() {
        let a = [1];
        let mut p = Pairwise::from(&a, &a);
        assert_eq!(Some((&1, &1)), p.next());
        assert_eq!(None, p.next());
        assert_eq!(None, p.next());
        assert_eq!(None, p.next());
    }

    #[test]
    fn same_size() {
        let a = [1, 2];
        let mut p = Pairwise::from(&a, &a);
        assert_eq!(Some((&1, &1)), p.next());
        assert_eq!(Some((&1, &2)), p.next());
        assert_eq!(Some((&2, &1)), p.next());
        assert_eq!(Some((&2, &2)), p.next());
        assert_eq!(None, p.next());
    }

    #[test]
    fn different_collections() {
        let a = [1, 2];
        let b = ["A", "B"];

        let mut p = Pairwise::from(&a, &b);

        assert_eq!(Some((&1, &"A")), p.next());
        assert_eq!(Some((&1, &"B")), p.next());
        assert_eq!(Some((&2, &"A")), p.next());
        assert_eq!(Some((&2, &"B")), p.next());
        assert_eq!(None, p.next());
    }

    #[test]
    fn composability() {
        let a = [1];
        let b = ["A"];
        let c = [()];

        let pairs = Pairwise::from(&a, &b);
        let mut triples = Pairwise::from(pairs, &c);

        assert_eq!(Some(((&1, &"A"), &())), triples.next());
        assert_eq!(None, triples.next());
    }
}
