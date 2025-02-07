union DoubleLong {
    d: f64,
    l: u64,
}

unsafe fn _rand_int(mut seed: f64) -> u64 {
    let max_uint64 = 0x7fffffffffffffff;

    let mut state: u64;
    let mut randint: u64 = 0;

    let mut r: i32 = 0x11090601;

    let mut m: u64 = 1 << (r & 255);
    r >>= 8;
    seed = seed * 3.14159265358979323846;
    seed = seed + 2.7182818284590452354;

    let mut u = DoubleLong { d: seed };

    if u.l < m {
        u.l = u.l + m;
    }

    state = u.l;

    for _ in 0..5 {
        state = (((state << 31) ^ state) >> 45) ^ ((state & (max_uint64 << 1)) << 18);
        state = (((state << 31) ^ state) >> 45) ^ ((state & (max_uint64 << 1)) << 18);
    }

    state = (((state << 31) ^ state) >> 45) ^ ((state & (max_uint64 << 1)) << 18);

    randint ^= state;

    //State 1
    m = 1 << (r & 255);
    r >>= 8;
    seed = seed * 3.14159265358979323846;
    seed = seed + 2.7182818284590452354;

    u = DoubleLong { d: seed };

    if u.l < m {
        u.l = u.l + m;
    }

    state = u.l;

    for _ in 0..5 {
        state = (((state << 19) ^ state) >> 30) ^ ((state & (max_uint64 << 6)) << 28);
        state = (((state << 19) ^ state) >> 30) ^ ((state & (max_uint64 << 6)) << 28);
    }

    state = (((state << 19) ^ state) >> 30) ^ ((state & (max_uint64 << 6)) << 28);

    randint ^= state;

    //State 2
    m = 1 << (r & 255);
    _ = r >>= 8;
    seed = seed * 3.14159265358979323846;
    seed = seed + 2.7182818284590452354;

    u = DoubleLong { d: seed };

    if u.l < m {
        u.l = u.l + m;
    }

    state = u.l;

    for _ in 0..5 {
        state = (((state << 24) ^ state) >> 48) ^ ((state & (max_uint64 << 9)) << 7);
        state = (((state << 24) ^ state) >> 48) ^ ((state & (max_uint64 << 9)) << 7);
    }

    state = (((state << 24) ^ state) >> 48) ^ ((state & (max_uint64 << 9)) << 7);

    randint ^= state;

    //State 3
    m = 1 << (r & 255);
    //_ = r >>= 8;
    seed = seed * 3.14159265358979323846;
    seed = seed + 2.7182818284590452354;

    u = DoubleLong { d: seed };

    if u.l < m {
        u.l = u.l + m;
    }

    state = u.l;

    for _ in 0..5 {
        state = (((state << 21) ^ state) >> 39) ^ ((state & (max_uint64 << 17)) << 8);
        state = (((state << 21) ^ state) >> 39) ^ ((state & (max_uint64 << 17)) << 8);
    }

    state = (((state << 21) ^ state) >> 39) ^ ((state & (max_uint64 << 17)) << 8);

    randint ^= state;

    randint
}

pub unsafe fn randdblmem(seed: f64) -> u64 {
    (_rand_int(seed) & 4503599627370495) | 4607182418800017408
}

pub unsafe fn random(seed: f64) -> f64 {
    let u = DoubleLong {
        l: randdblmem(seed),
    };
    u.d - 1.0
}

pub unsafe fn randint(seed: f64, min: i64, max: i64) -> i64 {
    (random(seed) * (max - min + 1) as f64 + min as f64) as i64
}