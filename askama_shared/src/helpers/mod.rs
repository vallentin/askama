use std::iter::Enumerate;
use std::iter::Peekable;

pub struct TemplateLoop<I>
where
    I: IntoIterator,
{
    iter: Peekable<Enumerate<I::IntoIter>>,
}

impl<I> TemplateLoop<I>
where
    I: IntoIterator,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        TemplateLoop {
            iter: iter.into_iter().enumerate().peekable(),
        }
    }
}

impl<I> Iterator for TemplateLoop<I>
where
    I: IntoIterator,
{
    type Item = (<I as IntoIterator>::Item, LoopItem);

    #[inline]
    fn next(&mut self) -> Option<(<I as IntoIterator>::Item, LoopItem)> {
        self.iter.next().map(|(index, item)| {
            (
                item,
                LoopItem {
                    index,
                    first: index == 0,
                    last: self.iter.peek().is_none(),
                },
            )
        })
    }
}

#[derive(Copy, Clone)]
pub struct LoopItem {
    pub index: usize,
    pub first: bool,
    pub last: bool,
}
