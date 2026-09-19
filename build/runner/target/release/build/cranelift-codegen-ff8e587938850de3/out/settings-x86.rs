#[derive(Clone, PartialEq, Hash)] // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:397
/// Flags group `x86`.
pub struct Flags {
    bytes: [u8; 3], // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:400
}
impl Flags {
    /// Create flags x86 settings group.
    #[allow(unused_variables, reason = "generated code")] // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:24
    pub fn new(shared: &settings::Flags, builder: &Builder) -> Self {
        let bvec = builder.state_for("x86"); // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:29
        let mut x86 = Self { bytes: [0; 3] }; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:30
        debug_assert_eq!(bvec.len(), 3); // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:36
        x86.bytes[0..3].copy_from_slice(&bvec); // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:41
        x86 // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:48
    }
}
impl Flags {
    /// Iterates the setting values.
    pub fn iter(&self) -> impl Iterator<Item = Value> + use<> {
        let mut bytes = [0; 3]; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:58
        bytes.copy_from_slice(&self.bytes[0..3]); // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:59
        DESCRIPTORS.iter().filter_map(move |d| {
            let values = match &d.detail {
                detail::Detail::Preset => return None, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:62
                detail::Detail::Enum { last, enumerators } => Some(TEMPLATE.enums(*last, *enumerators)), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:63
                _ => None // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:64
            }
            ; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:66
            Some(Value { name: d.name, detail: d.detail, values, value: bytes[d.offset as usize] }) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:67
        }
        ) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:69
    }
}
/// User-defined settings.
#[allow(dead_code, reason = "generated code")] // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:183
impl Flags {
    /// Dynamic numbered predicate getter.
    fn numbered_predicate(&self, p: usize) -> bool {
        self.bytes[0 + p / 8] & (1 << (p % 8)) != 0 // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:188
    }
    /// Has support for SSE3.
    /// SSE3: CPUID.01H:ECX.SSE3[bit 0]
    pub fn has_sse3(&self) -> bool {
        self.numbered_predicate(0) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for SSSE3.
    /// SSSE3: CPUID.01H:ECX.SSSE3[bit 9]
    pub fn has_ssse3(&self) -> bool {
        self.numbered_predicate(1) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for CMPXCHG16b.
    /// CMPXCHG16b: CPUID.01H:ECX.CMPXCHG16B[bit 13]
    pub fn has_cmpxchg16b(&self) -> bool {
        self.numbered_predicate(2) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for SSE4.1.
    /// SSE4.1: CPUID.01H:ECX.SSE4_1[bit 19]
    pub fn has_sse41(&self) -> bool {
        self.numbered_predicate(3) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for SSE4.2.
    /// SSE4.2: CPUID.01H:ECX.SSE4_2[bit 20]
    pub fn has_sse42(&self) -> bool {
        self.numbered_predicate(4) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for AVX.
    /// AVX: CPUID.01H:ECX.AVX[bit 28]
    pub fn has_avx(&self) -> bool {
        self.numbered_predicate(5) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for AVX2.
    /// AVX2: CPUID.07H:EBX.AVX2[bit 5]
    pub fn has_avx2(&self) -> bool {
        self.numbered_predicate(6) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for FMA.
    /// FMA: CPUID.01H:ECX.FMA[bit 12]
    pub fn has_fma(&self) -> bool {
        self.numbered_predicate(7) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for AVX-VNNI.
    /// AVX-VNNI: CPUID.07H.01H:EAX.AVX_VNNI[bit 4]
    pub fn has_avx_vnni(&self) -> bool {
        self.numbered_predicate(8) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for AVX512BITALG.
    /// AVX512BITALG: CPUID.07H:ECX.AVX512BITALG[bit 12]
    pub fn has_avx512bitalg(&self) -> bool {
        self.numbered_predicate(9) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for AVX512DQ.
    /// AVX512DQ: CPUID.07H:EBX.AVX512DQ[bit 17]
    pub fn has_avx512dq(&self) -> bool {
        self.numbered_predicate(10) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for AVX512VL.
    /// AVX512VL: CPUID.07H:EBX.AVX512VL[bit 31]
    pub fn has_avx512vl(&self) -> bool {
        self.numbered_predicate(11) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for AVX512VMBI.
    /// AVX512VBMI: CPUID.07H:ECX.AVX512VBMI[bit 1]
    pub fn has_avx512vbmi(&self) -> bool {
        self.numbered_predicate(12) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for AVX512F.
    /// AVX512F: CPUID.07H:EBX.AVX512F[bit 16]
    pub fn has_avx512f(&self) -> bool {
        self.numbered_predicate(13) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for AVX512VNNI.
    /// AVX512VNNI: CPUID.07H:ECX.AVX512_VNNI[bit 11]
    pub fn has_avx512vnni(&self) -> bool {
        self.numbered_predicate(14) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for POPCNT.
    /// POPCNT: CPUID.01H:ECX.POPCNT[bit 23]
    pub fn has_popcnt(&self) -> bool {
        self.numbered_predicate(15) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for BMI1.
    /// BMI1: CPUID.(EAX=07H, ECX=0H):EBX.BMI1[bit 3]
    pub fn has_bmi1(&self) -> bool {
        self.numbered_predicate(16) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for BMI2.
    /// BMI2: CPUID.(EAX=07H, ECX=0H):EBX.BMI2[bit 8]
    pub fn has_bmi2(&self) -> bool {
        self.numbered_predicate(17) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
    /// Has support for LZCNT.
    /// LZCNT: CPUID.EAX=80000001H:ECX.LZCNT[bit 5]
    pub fn has_lzcnt(&self) -> bool {
        self.numbered_predicate(18) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:155
    }
}
static DESCRIPTORS: [detail::Descriptor; 86] = [ // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:224
    detail::Descriptor {
        name: "has_sse3", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for SSE3.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 0, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 0 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_ssse3", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for SSSE3.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 0, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 1 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_cmpxchg16b", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for CMPXCHG16b.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 0, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 2 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_sse41", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for SSE4.1.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 0, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 3 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_sse42", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for SSE4.2.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 0, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 4 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_avx", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for AVX.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 0, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 5 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_avx2", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for AVX2.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 0, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 6 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_fma", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for FMA.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 0, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 7 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_avx_vnni", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for AVX-VNNI.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 1, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 0 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_avx512bitalg", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for AVX512BITALG.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 1, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 1 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_avx512dq", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for AVX512DQ.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 1, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 2 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_avx512vl", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for AVX512VL.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 1, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 3 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_avx512vbmi", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for AVX512VMBI.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 1, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 4 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_avx512f", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for AVX512F.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 1, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 5 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_avx512vnni", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for AVX512VNNI.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 1, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 6 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_popcnt", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for POPCNT.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 1, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 7 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_bmi1", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for BMI1.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 2, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 0 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_bmi2", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for BMI2.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 2, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 1 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "has_lzcnt", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:232
        description: "Has support for LZCNT.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:233
        offset: 2, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:234
        detail: detail::Detail::Bool { bit: 2 }, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:237
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:259
    detail::Descriptor {
        name: "sse3", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "SSE3 and earlier.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 0, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "ssse3", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "SSSE3 and earlier.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 3, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "sse41", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "SSE4.1 and earlier.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 6, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "sse42", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "SSE4.2 and earlier.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 9, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "baseline", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "A baseline preset with no extensions enabled.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 12, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "nocona", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Nocona microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 15, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "core2", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Core 2 microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 18, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "penryn", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Penryn microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 21, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "atom", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Atom microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 24, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "bonnell", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Bonnell microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 27, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "silvermont", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Silvermont microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 30, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "slm", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Silvermont microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 33, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "goldmont", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Goldmont microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 36, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "goldmont-plus", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Goldmont Plus microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 39, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "tremont", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Tremont microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 42, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "alderlake", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Alderlake microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 45, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "sierraforest", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Sierra Forest microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 48, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "grandridge", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Grandridge microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 51, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "nehalem", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Nehalem microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 54, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "corei7", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Core i7 microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 57, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "westmere", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Westmere microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 60, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "sandybridge", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Sandy Bridge microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 63, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "corei7-avx", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Core i7 AVX microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 66, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "ivybridge", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Ivy Bridge microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 69, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "core-avx-i", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Intel Core CPU with 64-bit extensions.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 72, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "haswell", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Haswell microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 75, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "core-avx2", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Intel Core CPU with AVX2 extensions.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 78, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "broadwell", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Broadwell microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 81, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "skylake", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Skylake microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 84, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "knl", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Knights Landing microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 87, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "knm", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Knights Mill microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 90, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "skylake-avx512", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Skylake AVX512 microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 93, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "skx", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Skylake AVX512 microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 96, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "cascadelake", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Cascade Lake microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 99, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "cooperlake", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Cooper Lake microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 102, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "cannonlake", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Canon Lake microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 105, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "icelake-client", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Ice Lake microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 108, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "icelake", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Ice Lake microarchitecture", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 111, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "icelake-server", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Ice Lake (server) microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 114, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "tigerlake", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Tiger Lake microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 117, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "sapphirerapids", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Sapphire Rapids microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 120, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "raptorlake", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Raptor Lake microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 123, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "meteorlake", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Meteor Lake microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 126, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "graniterapids", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Granite Rapids microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 129, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "opteron", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Opteron microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 132, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "k8", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "K8 Hammer microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 135, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "athlon64", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Athlon64 microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 138, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "athlon-fx", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Athlon FX microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 141, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "opteron-sse3", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Opteron microarchitecture with support for SSE3 instructions.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 144, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "k8-sse3", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "K8 Hammer microarchitecture with support for SSE3 instructions.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 147, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "athlon64-sse3", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Athlon 64 microarchitecture with support for SSE3 instructions.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 150, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "barcelona", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Barcelona microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 153, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "amdfam10", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "AMD Family 10h microarchitecture", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 156, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "btver1", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Bobcat microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 159, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "btver2", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Jaguar microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 162, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "bdver1", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Bulldozer microarchitecture", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 165, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "bdver2", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Piledriver microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 168, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "bdver3", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Steamroller microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 171, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "bdver4", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Excavator microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 174, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "znver1", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Zen (first generation) microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 177, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "znver2", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Zen (second generation) microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 180, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "znver3", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Zen (third generation) microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 183, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "znver4", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Zen (fourth generation) microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 186, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "x86-64", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Generic x86-64 microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 189, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "x86-64-v2", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Generic x86-64 (V2) microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 192, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "x86-64-v3", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Generic x86-64 (V3) microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 195, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
    detail::Descriptor {
        name: "x86-64-v4", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:264
        description: "Generic x86-64 (V4) microarchitecture.", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:265
        offset: 198, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:266
        detail: detail::Detail::Preset, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:267
    }
    , // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:269
]; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:275
static ENUMERATORS: [&str; 0] = [ // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:278
]; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:284
static HASH_TABLE: [u16; 128] = [ // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:294
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    80, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    79, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    78, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    26, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    81, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    69, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    83, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    25, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    53, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    62, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    17, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    16, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    32, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    1, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    44, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    73, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    70, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    5, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    38, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    68, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    6, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    47, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    24, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    67, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    18, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    7, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    50, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    52, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    27, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    65, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    85, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    13, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    46, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    41, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    55, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    72, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    4, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    34, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    84, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    3, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    61, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    12, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    14, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    15, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    33, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    82, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    76, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    42, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    31, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    49, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    48, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    10, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    57, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    74, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    11, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    77, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    75, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    2, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    64, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    36, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    9, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    21, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    22, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    51, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    19, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    56, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    63, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    8, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    23, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    66, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    71, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    59, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    29, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    30, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    37, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    39, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    43, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    45, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    35, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    60, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    54, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    20, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    58, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    0xffff, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:306
    28, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
    40, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:298
]; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:310
static PRESETS: [(u8, u8); 201] = [ // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:313
    // sse3: has_sse3
    (0b00000001, 0b00000001), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // ssse3: has_sse3, has_ssse3
    (0b00000011, 0b00000011), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // sse41: has_sse3, has_ssse3, has_sse41
    (0b00001011, 0b00001011), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // sse42: has_sse3, has_ssse3, has_sse41, has_sse42
    (0b00011011, 0b00011011), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // baseline: 
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // nocona: has_sse3, has_cmpxchg16b
    (0b00000101, 0b00000101), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // core2: has_sse3, has_cmpxchg16b
    (0b00000101, 0b00000101), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // penryn: has_sse3, has_ssse3, has_sse41, has_cmpxchg16b
    (0b00001111, 0b00001111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // atom: has_sse3, has_ssse3, has_cmpxchg16b
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // bonnell: has_sse3, has_ssse3, has_cmpxchg16b
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // silvermont: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt
    (0b00011111, 0b00011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // slm: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt
    (0b00011111, 0b00011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // goldmont: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt
    (0b00011111, 0b00011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // goldmont-plus: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt
    (0b00011111, 0b00011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // tremont: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt
    (0b00011111, 0b00011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // alderlake: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_avx_vnni
    (0b10011111, 0b10011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000001, 0b10000001), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // sierraforest: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_avx_vnni
    (0b10011111, 0b10011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000001, 0b10000001), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // grandridge: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_avx_vnni
    (0b10011111, 0b10011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000001, 0b10000001), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // nehalem: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b
    (0b00011111, 0b00011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // corei7: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b
    (0b00011111, 0b00011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // westmere: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b
    (0b00011111, 0b00011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // sandybridge: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx
    (0b00111111, 0b00111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // corei7-avx: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx
    (0b00111111, 0b00111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // ivybridge: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx
    (0b00111111, 0b00111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // core-avx-i: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx
    (0b00111111, 0b00111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // haswell: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // core-avx2: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // broadwell: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // skylake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // knl: has_popcnt, has_avx512f, has_fma, has_bmi1, has_bmi2, has_lzcnt, has_cmpxchg16b
    (0b10000100, 0b10000100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10100000, 0b10100000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // knm: has_popcnt, has_avx512f, has_fma, has_bmi1, has_bmi2, has_lzcnt, has_cmpxchg16b
    (0b10000100, 0b10000100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10100000, 0b10100000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // skylake-avx512: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10101100, 0b10101100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // skx: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10101100, 0b10101100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // cascadelake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vnni
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b11101100, 0b11101100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // cooperlake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vnni
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b11101100, 0b11101100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // cannonlake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10111100, 0b10111100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // icelake-client: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg, has_avx512vnni
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b11111110, 0b11111110), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // icelake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg, has_avx512vnni
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b11111110, 0b11111110), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // icelake-server: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg, has_avx512vnni
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b11111110, 0b11111110), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // tigerlake: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg, has_avx512vnni
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b11111110, 0b11111110), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // sapphirerapids: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg, has_avx512vnni
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b11111110, 0b11111110), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // raptorlake: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_avx_vnni
    (0b10011111, 0b10011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000001, 0b10000001), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // meteorlake: has_sse3, has_ssse3, has_cmpxchg16b, has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_avx_vnni
    (0b10011111, 0b10011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000001, 0b10000001), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // graniterapids: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_avx, has_avx2, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx512f, has_avx512dq, has_avx512vl, has_avx512vbmi, has_avx512bitalg, has_avx512vnni
    (0b11111111, 0b11111111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b11111110, 0b11111110), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // opteron: 
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // k8: 
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // athlon64: 
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // athlon-fx: 
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // opteron-sse3: has_sse3, has_cmpxchg16b
    (0b00000101, 0b00000101), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // k8-sse3: has_sse3, has_cmpxchg16b
    (0b00000101, 0b00000101), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // athlon64-sse3: has_sse3, has_cmpxchg16b
    (0b00000101, 0b00000101), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // barcelona: has_popcnt, has_lzcnt, has_cmpxchg16b
    (0b00000100, 0b00000100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000100, 0b00000100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // amdfam10: has_popcnt, has_lzcnt, has_cmpxchg16b
    (0b00000100, 0b00000100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000100, 0b00000100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // btver1: has_sse3, has_ssse3, has_lzcnt, has_popcnt, has_cmpxchg16b
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000100, 0b00000100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // btver2: has_sse3, has_ssse3, has_lzcnt, has_popcnt, has_cmpxchg16b, has_avx, has_bmi1
    (0b00100111, 0b00100111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000101, 0b00000101), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // bdver1: has_lzcnt, has_popcnt, has_sse3, has_ssse3, has_cmpxchg16b
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000100, 0b00000100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // bdver2: has_lzcnt, has_popcnt, has_sse3, has_ssse3, has_cmpxchg16b, has_bmi1
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000101, 0b00000101), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // bdver3: has_lzcnt, has_popcnt, has_sse3, has_ssse3, has_cmpxchg16b, has_bmi1
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000101, 0b00000101), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // bdver4: has_lzcnt, has_popcnt, has_sse3, has_ssse3, has_cmpxchg16b, has_bmi1, has_avx2, has_bmi2
    (0b01000111, 0b01000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // znver1: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_cmpxchg16b
    (0b10011111, 0b10011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // znver2: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_cmpxchg16b
    (0b10011111, 0b10011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // znver3: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_cmpxchg16b
    (0b10011111, 0b10011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // znver4: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_bmi1, has_bmi2, has_lzcnt, has_fma, has_cmpxchg16b, has_avx512bitalg, has_avx512dq, has_avx512f, has_avx512vbmi, has_avx512vl
    (0b10011111, 0b10011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10111110, 0b10111110), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // x86-64: 
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // x86-64-v2: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b
    (0b00011111, 0b00011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000000, 0b00000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // x86-64-v3: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx2
    (0b11011111, 0b11011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10000000, 0b10000000), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    // x86-64-v4: has_sse3, has_ssse3, has_sse41, has_sse42, has_popcnt, has_cmpxchg16b, has_bmi1, has_bmi2, has_fma, has_lzcnt, has_avx2, has_avx512dq, has_avx512vl
    (0b11011111, 0b11011111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b10001100, 0b10001100), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
    (0b00000111, 0b00000111), // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:326
]; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:330
static TEMPLATE: detail::Template = detail::Template {
    name: "x86", // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:345
    descriptors: &DESCRIPTORS, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:346
    enumerators: &ENUMERATORS, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:347
    hash_table: &HASH_TABLE, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:348
    defaults: &[0x00, 0x00, 0x00], // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:349
    presets: &PRESETS, // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:350
}
; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:353
/// Create a `settings::Builder` for the x86 settings group.
pub fn builder() -> Builder {
    Builder::new(&TEMPLATE) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:360
}
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[x86]")?; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:369
        for d in &DESCRIPTORS {
            if !d.detail.is_preset() {
                write!(f, "{} = ", d.name)?; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:372
                TEMPLATE.format_toml_value(d.detail, self.bytes[d.offset as usize], f)?; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:373
                writeln!(f)?; // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:377
            }
        }
        Ok(()) // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:380
    }
}
impl Flags {
    /// Get the flag values as raw bytes for hashing.
    pub fn hash_key(&self) -> &[u8] {
        &self.bytes // /home/vi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cranelift-codegen-meta-0.135.2/src/gen_settings.rs:390
    }
}
