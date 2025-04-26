#[derive(Copy, Clone, Debug)]
pub enum PageSizeKb {
    One = 0x400,
    Four = 0x1000,
    Eight = 0x2000,
    Sixteen = 0x4000,
}

#[derive(Copy, Clone, Debug)]
pub enum Page {
    First(PageSizeKb),
    Last(PageSizeKb),
    Number(usize, PageSizeKb),
    FromEnd(usize, PageSizeKb),
}

pub struct Pager {
    pub data: Vec<u8>,
}

impl Pager {
    pub fn new(data: Vec<u8>) -> Self {
        Pager { data }
    }

    pub fn read(&self, page: Page, offset: u16) -> u8 {
        let i = self.index(page, offset);
        self.data[i]
    }

    pub fn write(&mut self, page: Page, offset: u16, value: u8) {
        let i = self.index(page, offset);
        self.data[i] = value;
    }

    fn page_count(&self, size: PageSizeKb) -> usize {
        if self.data.len() % (size as usize) != 0 {
            panic!("Page size must divide evenly into data length")
        }

        self.data.len() / (size as usize)
    }

    fn index(&self, page: Page, offset: u16) -> usize {
        match page {
            Page::First(size) => self.index(Page::Number(0, size), offset),
            Page::Last(size) => {
                let last_page = self.page_count(size) - 1;
                self.index(Page::Number(last_page, size), offset)
            }
            Page::Number(n, size) => {
                let last_page = self.page_count(size) - 1;
                if (offset as usize) > (size as usize) {
                    panic!("Offset cannot exceed page bounds")
                }
                if n > last_page {
                    panic!("Page out of bounds")
                }
                n * (size as usize) + (offset as usize)
            }
            Page::FromEnd(n, size) => {
                let last_page = self.page_count(size) - 1;
                self.index(Page::Number(last_page - n, size), offset)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build_pager() -> Pager {
        let mut data = Vec::new();
        for i in 0..(PageSizeKb::Sixteen as usize * 4) {
            data.push(i as u8);
        }
        Pager::new(data)
    }

    #[test]
    fn test_page_count() {
        let pager = build_pager();
        assert_eq!(4, pager.page_count(PageSizeKb::Sixteen));
        assert_eq!(8, pager.page_count(PageSizeKb::Eight));
        assert_eq!(16, pager.page_count(PageSizeKb::Four));
    }

    #[test]
    fn test_index_first() {
        let pager = build_pager();
        assert_eq!(4, pager.index(Page::First(PageSizeKb::Sixteen), 4));
        assert_eq!(8, pager.index(Page::First(PageSizeKb::Sixteen), 8));
    }

    #[test]
    fn test_index_last() {
        let pager = build_pager();
        assert_eq!(
            0x4000 * 3 + 42,
            pager.index(Page::Last(PageSizeKb::Sixteen), 42)
        );
    }

    #[test]
    fn test_index_number() {
        let pager = build_pager();
        assert_eq!(
            0x1000 * 3 + 36,
            pager.index(Page::Number(3, PageSizeKb::Four), 36)
        );
    }

    #[test]
    #[should_panic]
    fn test_index_overflow() {
        let pager = build_pager();
        pager.index(
            Page::First(PageSizeKb::Sixteen),
            PageSizeKb::Sixteen as u16 + 1,
        );
    }

    #[test]
    #[should_panic]
    fn test_index_nopage() {
        let pager = build_pager();
        pager.index(Page::Number(100, PageSizeKb::Sixteen), 0);
    }

    #[test]
    fn test_rw() {
        let mut pager = build_pager();
        pager.write(Page::Last(PageSizeKb::Four), 5, 0x66);
        assert_eq!(0x66, pager.read(Page::Last(PageSizeKb::Four), 5));
        assert_eq!(
            0x66,
            pager.read(Page::Last(PageSizeKb::Sixteen), 0x1000 * 3 + 5)
        );
    }
}
