use core::hash;
use core::hash::Hasher as _;
use core::mem;
use core::ptr;

static OFFSET_BASIS: u64 = 0xcbf29ce484222325;
static PRIME: u64 = 0x100000001b3;

struct Hasher(u64);

impl hash::Hasher for Hasher {
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= *byte as u64;
            self.0 = self.0.wrapping_mul(PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

fn f<T>(v: &T) -> u64 
where
    T: hash::Hash {
    let mut hasher = Hasher(OFFSET_BASIS);
    v.hash(&mut hasher);
    hasher.finish()
}


// ===

struct Slot<A, B> {
    k: A,
    v: B,
    active: bool
}

pub struct Map<const A: usize, B, C> {
    slots: [Option<Slot<B, C>>; A],
    len: usize
}

impl<const A: usize, B, C> Map<A, B, C>
where
    B: Eq,
    B: hash::Hash {
    pub const fn new() -> Self {
        const U: Slot<(), ()> = Slot {
            k: mem::MaybeUninit::uninit(),
            v: mem::MaybeUninit::uninit(),
            active: false
        };
        Self {
            slots: unsafe {
                core::mem::transmute([U; A])
            },
            len: 0
        }
    }
}