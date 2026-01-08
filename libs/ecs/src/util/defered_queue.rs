use std::{marker::PhantomData, ptr};

use parking_lot::Mutex;

/// A rotating locked defered queue.
///
/// This means when popping, it will swap the current "public" queue with the
/// "private" and then pop all elements off of the new private queue.
///
/// This is to avoid deadlocks where the defered operations are dependendent on defered queue.
#[derive(Default)]
pub struct RotatingLockedDeferedQueue<Dependency> {
    public: Mutex<DeferedQueue<Dependency>>,
    private: Mutex<DeferedQueue<Dependency>>,
}

impl<Dependency> RotatingLockedDeferedQueue<Dependency> {
    /// Creates a new rotating defered queue
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            public: Mutex::new(DeferedQueue::new()),
            private: Mutex::new(DeferedQueue::new()),
        }
    }

    /// Pushes some data and a callback to handle that data onto the defered queue
    pub fn push<T>(&self, callback: fn(T, &Dependency), data: T) { self.public.lock().push(callback, data); }

    /// Pops all elements off of this rotating defered queue that are currently in the public queue
    ///
    /// Internally, this swaps the public and private queues, then consumes from the new private queue
    pub fn pop_all(&self, dependency: &Dependency) {
        let mut public = self.public.lock();
        let mut private = self.private.lock();
        std::mem::swap(&mut *public, &mut *private);
        drop(public);

        private.pop_all(dependency);
    }
}

/// A heterogenous queue a defered elements that have a dependency.
///
/// The data is stored as
/// meta padding | meta | data padding | data |
///
/// Note for simplicity, the queue can only have all elements popped to avoid
/// having to deal with alignment isues.
#[derive(Default)]
pub struct DeferedQueue<Dependency> {
    buffer: Vec<u8>,
    _phantom: PhantomData<fn() -> Dependency>,
}

impl<Dependency> DeferedQueue<Dependency> {
    /// Creates a new defered stack
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            _phantom: PhantomData::<fn() -> Dependency>,
        }
    }

    /// Pushes some data and a callback to handle that data with the dependency onto the defered queue
    pub fn push<T>(&mut self, callback: fn(T, &Dependency), data: T) {
        let size = std::mem::size_of::<T>();
        let meta = DefferedQueueEntryMeta {
            callback_dispatcher: callback_dispatcher::<T, Dependency>,
            callback: callback as *const (),
            align: std::mem::align_of::<T>(),
            total: 0,
        };

        let meta_start = self.buffer.len();
        let meta_pad = self.push_data_as_bytes(meta);
        let data_total = self.push_data_as_bytes(data) + size;
        let total = data_total + meta_pad + std::mem::size_of::<DefferedQueueEntryMeta>();

        let meta_offset = meta_start + meta_pad;
        unsafe {
            let meta_ptr = self.buffer.as_mut_ptr().add(meta_offset) as *mut DefferedQueueEntryMeta;
            (*meta_ptr).total = total;
        }
    }

    /// Utility helper to push data onto the buffer, and then return
    /// the padding used for the data
    fn push_data_as_bytes<T>(&mut self, data: T) -> usize {
        // Pad
        let align = std::mem::align_of::<T>();
        let pad = pad(align, self.buffer.len());
        self.buffer.extend(std::iter::repeat_n(0, pad));

        // Reserve
        let size = std::mem::size_of_val(&data);
        self.buffer.reserve(size);

        // Write
        let offset = self.buffer.len();
        unsafe {
            ptr::write(self.buffer.as_mut_ptr().add(offset) as *mut T, data);
            self.buffer.set_len(offset + std::mem::size_of::<T>());
        }

        pad
    }

    /// Pops all elements
    pub fn pop_all(&mut self, dependency: &Dependency) {
        let mut cursor = 0;

        while let Some(total) = self.pop_and_return_total_from(cursor, dependency) {
            cursor += total;
        }

        self.buffer.clear();
    }

    /// Pops an element from the buffer by invoking its callback
    ///
    /// Returns if an element was popped this way
    fn pop_and_return_total_from(&mut self, start: usize, dependency: &Dependency) -> Option<usize> {
        if start >= self.buffer.len() {
            return None;
        }

        let meta_offset = start + pad(std::mem::align_of::<DefferedQueueEntryMeta>(), start);
        let meta = unsafe { std::ptr::read(self.buffer.as_ptr().add(meta_offset) as *const DefferedQueueEntryMeta<Dependency>) };

        let data_start = meta_offset + std::mem::size_of::<DefferedQueueEntryMeta>();
        let data_pad = pad(meta.align, data_start);
        let data_offset = data_start + data_pad;
        let data_ptr = unsafe { self.buffer.as_mut_ptr().add(data_offset) };

        unsafe { (meta.callback_dispatcher)(data_ptr, dependency, meta.callback) };

        Some(meta.total)
    }
}

struct DefferedQueueEntryMeta<Dependency = ()> {
    callback_dispatcher: unsafe fn(*mut u8, &Dependency, *const ()),
    callback: *const (),
    /// The align of the data only
    align: usize,
    /// The total size of the meta and the entry
    total: usize,
}

unsafe fn callback_dispatcher<T, Dependency>(ptr: *mut u8, dependency: &Dependency, callback: *const ()) {
    let callback: fn(T, &Dependency) = unsafe { std::mem::transmute(callback) };
    let data = unsafe { std::ptr::read(ptr as *mut T) };
    callback(data, dependency);
}

fn pad(align: usize, start: usize) -> usize { (align - (start % align)) % align }

#[cfg(test)]
mod tests {
    use std::{any::type_name, cell::RefCell, collections::HashMap, fmt::Debug, path::PathBuf};

    use super::DeferedQueue;
    use crate::util::defered_queue::RotatingLockedDeferedQueue;

    type Map = RefCell<HashMap<&'static str, Box<dyn Debug>>>;

    fn callback<T: Debug + 'static>(data: T, map: &Map) { map.borrow_mut().insert(type_name::<T>(), Box::new(data)); }

    #[test]
    fn test_add_to_map() {
        let map = Map::default();

        let mut queue = DeferedQueue::<Map>::new();
        queue.push(callback, 0u32);
        queue.push(callback, PathBuf::from("fart"));
        queue.push(callback, "foobar");
        queue.push(callback, "bleep".to_string());
        queue.pop_all(&map);

        println!("{:?}", &map);
    }

    #[test]
    fn test_add_to_map_rotating_defered() {
        let map = Map::default();

        let queue = RotatingLockedDeferedQueue::<Map>::new();
        queue.push(callback, 0u32);
        queue.push(callback, PathBuf::from("fart"));
        queue.push(callback, "foobar");
        queue.push(callback, "bleep".to_string());
        queue.pop_all(&map);

        println!("{:?}", &map);
    }
}
