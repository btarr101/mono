use std::num::NonZeroUsize;

pub fn parse_pagination(page: NonZeroUsize, page_size: NonZeroUsize) -> (i64, i64) {
    let limit = page_size.get() as i64;
    let offset = ((page.get() - 1) * page_size.get()) as i64;

    (limit, offset)
}
