use std::iter::Peekable;

pub struct FlagLast<I>
where
    I: Iterator,
{
    iterator: Peekable<I>,
}

impl<I> FlagLast<I>
where
    I: Iterator,
{
    pub fn from_iter(i: I) -> FlagLast<I> {
        FlagLast { iterator: i.peekable() }
    }
}

impl<I> Iterator for FlagLast<I>
where
    I: Iterator,
{
    type Item = (<I as Iterator>::Item, bool);

    fn next(&mut self) -> Option<(<I as Iterator>::Item, bool)> {
        match self.iterator.next() {
            None => None,
            Some(v) => {
                match self.iterator.peek() {
                    None => Some((v, true)),
                    Some(_) => Some((v, false)),
                }
            }
        }
    }
}

#[test]
fn test_flag_last_iterator_by_step_through() {
    let v = vec![1, 2, 3];
    let mut iterator = FlagLast::from_iter(v.iter());

    if let Some((n, b)) = iterator.next() {
        assert_eq!(1, *n);
        assert_eq!(false, b);
    } else {
        assert!(false);
    }

    if let Some((n, b)) = iterator.next() {
        assert_eq!(2, *n);
        assert_eq!(false, b);
    } else {
        assert!(false);
    }

    if let Some((n, b)) = iterator.next() {
        assert_eq!(3, *n);
        assert_eq!(true, b);
    } else {
        assert!(false);
    }

    if let None = iterator.next() {
        // All is well
    } else {
        assert!(false);
    }

}
