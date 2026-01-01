#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    confr0: CONFR0,
    confr1: CONFR1,
    confr2: CONFR2,
    confr3: CONFR3,
    confr4: CONFR4,
    confr5: CONFR5,
    confr6: CONFR6,
    confr7: CONFR7,
    _reserved8: [u8; 0x10],
    cr: CR,
    sr: SR,
    cfr: CFR,
    _reserved11: [u8; 0x04],
    dir: DIR,
    dor: DOR,
    _reserved13: [u8; 0x08],
    qmem0: [QMEM0; 16],
    qmem1: [QMEM1; 16],
    qmem2: [QMEM2; 16],
    qmem3: [QMEM3; 16],
    huffmin: [HUFFMIN; 16],
    huffbase: [HUFFBASE; 32],
    huffsymb: [HUFFSYMB; 84],
    dhtmem: [DHTMEM; 103],
    _reserved21: [u8; 0x04],
    huffenc_ac0: [HUFFENC_AC0; 88],
    huffenc_ac1: [HUFFENC_AC1; 88],
    huffenc_dc0: [HUFFENC_DC0; 8],
    huffenc_dc1: [HUFFENC_DC1; 8],
}
impl RegisterBlock {
    ///0x00 - JPEG codec configuration register 0
    #[inline(always)]
    pub const fn confr0(&self) -> &CONFR0 {
        &self.confr0
    }
    ///0x04 - JPEG codec configuration register 1
    #[inline(always)]
    pub const fn confr1(&self) -> &CONFR1 {
        &self.confr1
    }
    ///0x08 - JPEG codec configuration register 2
    #[inline(always)]
    pub const fn confr2(&self) -> &CONFR2 {
        &self.confr2
    }
    ///0x0c - JPEG codec configuration register 3
    #[inline(always)]
    pub const fn confr3(&self) -> &CONFR3 {
        &self.confr3
    }
    ///0x10 - JPEG codec configuration register 4
    #[inline(always)]
    pub const fn confr4(&self) -> &CONFR4 {
        &self.confr4
    }
    ///0x14 - JPEG codec configuration register 5
    #[inline(always)]
    pub const fn confr5(&self) -> &CONFR5 {
        &self.confr5
    }
    ///0x18 - JPEG codec configuration register 6
    #[inline(always)]
    pub const fn confr6(&self) -> &CONFR6 {
        &self.confr6
    }
    ///0x1c - JPEG codec configuration register 7
    #[inline(always)]
    pub const fn confr7(&self) -> &CONFR7 {
        &self.confr7
    }
    ///0x30 - JPEG control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x34 - JPEG status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x38 - JPEG clear flag register
    #[inline(always)]
    pub const fn cfr(&self) -> &CFR {
        &self.cfr
    }
    ///0x40 - JPEG data input register
    #[inline(always)]
    pub const fn dir(&self) -> &DIR {
        &self.dir
    }
    ///0x44 - JPEG data output register
    #[inline(always)]
    pub const fn dor(&self) -> &DOR {
        &self.dor
    }
    ///0x50..0x90 - JPEG quantization tables
    #[inline(always)]
    pub const fn qmem0(&self, n: usize) -> &QMEM0 {
        &self.qmem0[n]
    }
    ///Iterator for array of:
    ///0x50..0x90 - JPEG quantization tables
    #[inline(always)]
    pub fn qmem0_iter(&self) -> impl Iterator<Item = &QMEM0> {
        self.qmem0.iter()
    }
    ///0x90..0xd0 - JPEG quantization tables
    #[inline(always)]
    pub const fn qmem1(&self, n: usize) -> &QMEM1 {
        &self.qmem1[n]
    }
    ///Iterator for array of:
    ///0x90..0xd0 - JPEG quantization tables
    #[inline(always)]
    pub fn qmem1_iter(&self) -> impl Iterator<Item = &QMEM1> {
        self.qmem1.iter()
    }
    ///0xd0..0x110 - JPEG quantization tables
    #[inline(always)]
    pub const fn qmem2(&self, n: usize) -> &QMEM2 {
        &self.qmem2[n]
    }
    ///Iterator for array of:
    ///0xd0..0x110 - JPEG quantization tables
    #[inline(always)]
    pub fn qmem2_iter(&self) -> impl Iterator<Item = &QMEM2> {
        self.qmem2.iter()
    }
    ///0x110..0x150 - JPEG quantization tables
    #[inline(always)]
    pub const fn qmem3(&self, n: usize) -> &QMEM3 {
        &self.qmem3[n]
    }
    ///Iterator for array of:
    ///0x110..0x150 - JPEG quantization tables
    #[inline(always)]
    pub fn qmem3_iter(&self) -> impl Iterator<Item = &QMEM3> {
        self.qmem3.iter()
    }
    ///0x150..0x190 - JPEG HuffMin tables
    #[inline(always)]
    pub const fn huffmin(&self, n: usize) -> &HUFFMIN {
        &self.huffmin[n]
    }
    ///Iterator for array of:
    ///0x150..0x190 - JPEG HuffMin tables
    #[inline(always)]
    pub fn huffmin_iter(&self) -> impl Iterator<Item = &HUFFMIN> {
        self.huffmin.iter()
    }
    ///0x190..0x210 - JPEG HuffSymb tables
    #[inline(always)]
    pub const fn huffbase(&self, n: usize) -> &HUFFBASE {
        &self.huffbase[n]
    }
    ///Iterator for array of:
    ///0x190..0x210 - JPEG HuffSymb tables
    #[inline(always)]
    pub fn huffbase_iter(&self) -> impl Iterator<Item = &HUFFBASE> {
        self.huffbase.iter()
    }
    ///0x210..0x360 - JPEG HUFFSYMB tables
    #[inline(always)]
    pub const fn huffsymb(&self, n: usize) -> &HUFFSYMB {
        &self.huffsymb[n]
    }
    ///Iterator for array of:
    ///0x210..0x360 - JPEG HUFFSYMB tables
    #[inline(always)]
    pub fn huffsymb_iter(&self) -> impl Iterator<Item = &HUFFSYMB> {
        self.huffsymb.iter()
    }
    ///0x360..0x4fc - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem(&self, n: usize) -> &DHTMEM {
        &self.dhtmem[n]
    }
    ///Iterator for array of:
    ///0x360..0x4fc - JPEG DHTMem tables
    #[inline(always)]
    pub fn dhtmem_iter(&self) -> impl Iterator<Item = &DHTMEM> {
        self.dhtmem.iter()
    }
    ///0x360 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem0(&self) -> &DHTMEM {
        self.dhtmem(0)
    }
    ///0x364 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem2(&self) -> &DHTMEM {
        self.dhtmem(1)
    }
    ///0x368 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem3(&self) -> &DHTMEM {
        self.dhtmem(2)
    }
    ///0x36c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem4(&self) -> &DHTMEM {
        self.dhtmem(3)
    }
    ///0x370 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem5(&self) -> &DHTMEM {
        self.dhtmem(4)
    }
    ///0x374 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem6(&self) -> &DHTMEM {
        self.dhtmem(5)
    }
    ///0x378 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem7(&self) -> &DHTMEM {
        self.dhtmem(6)
    }
    ///0x37c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem8(&self) -> &DHTMEM {
        self.dhtmem(7)
    }
    ///0x380 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem9(&self) -> &DHTMEM {
        self.dhtmem(8)
    }
    ///0x384 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem10(&self) -> &DHTMEM {
        self.dhtmem(9)
    }
    ///0x388 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem11(&self) -> &DHTMEM {
        self.dhtmem(10)
    }
    ///0x38c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem12(&self) -> &DHTMEM {
        self.dhtmem(11)
    }
    ///0x390 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem13(&self) -> &DHTMEM {
        self.dhtmem(12)
    }
    ///0x394 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem14(&self) -> &DHTMEM {
        self.dhtmem(13)
    }
    ///0x398 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem15(&self) -> &DHTMEM {
        self.dhtmem(14)
    }
    ///0x39c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem16(&self) -> &DHTMEM {
        self.dhtmem(15)
    }
    ///0x3a0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem17(&self) -> &DHTMEM {
        self.dhtmem(16)
    }
    ///0x3a4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem18(&self) -> &DHTMEM {
        self.dhtmem(17)
    }
    ///0x3a8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem19(&self) -> &DHTMEM {
        self.dhtmem(18)
    }
    ///0x3ac - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem20(&self) -> &DHTMEM {
        self.dhtmem(19)
    }
    ///0x3b0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem21(&self) -> &DHTMEM {
        self.dhtmem(20)
    }
    ///0x3b4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem22(&self) -> &DHTMEM {
        self.dhtmem(21)
    }
    ///0x3b8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem23(&self) -> &DHTMEM {
        self.dhtmem(22)
    }
    ///0x3bc - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem24(&self) -> &DHTMEM {
        self.dhtmem(23)
    }
    ///0x3c0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem25(&self) -> &DHTMEM {
        self.dhtmem(24)
    }
    ///0x3c4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem26(&self) -> &DHTMEM {
        self.dhtmem(25)
    }
    ///0x3c8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem27(&self) -> &DHTMEM {
        self.dhtmem(26)
    }
    ///0x3cc - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem28(&self) -> &DHTMEM {
        self.dhtmem(27)
    }
    ///0x3d0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem29(&self) -> &DHTMEM {
        self.dhtmem(28)
    }
    ///0x3d4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem30(&self) -> &DHTMEM {
        self.dhtmem(29)
    }
    ///0x3d8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem31(&self) -> &DHTMEM {
        self.dhtmem(30)
    }
    ///0x3dc - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem32(&self) -> &DHTMEM {
        self.dhtmem(31)
    }
    ///0x3e0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem33(&self) -> &DHTMEM {
        self.dhtmem(32)
    }
    ///0x3e4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem34(&self) -> &DHTMEM {
        self.dhtmem(33)
    }
    ///0x3e8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem35(&self) -> &DHTMEM {
        self.dhtmem(34)
    }
    ///0x3ec - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem36(&self) -> &DHTMEM {
        self.dhtmem(35)
    }
    ///0x3f0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem37(&self) -> &DHTMEM {
        self.dhtmem(36)
    }
    ///0x3f4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem38(&self) -> &DHTMEM {
        self.dhtmem(37)
    }
    ///0x3f8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem39(&self) -> &DHTMEM {
        self.dhtmem(38)
    }
    ///0x3fc - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem40(&self) -> &DHTMEM {
        self.dhtmem(39)
    }
    ///0x400 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem41(&self) -> &DHTMEM {
        self.dhtmem(40)
    }
    ///0x404 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem42(&self) -> &DHTMEM {
        self.dhtmem(41)
    }
    ///0x408 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem43(&self) -> &DHTMEM {
        self.dhtmem(42)
    }
    ///0x40c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem44(&self) -> &DHTMEM {
        self.dhtmem(43)
    }
    ///0x410 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem45(&self) -> &DHTMEM {
        self.dhtmem(44)
    }
    ///0x414 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem46(&self) -> &DHTMEM {
        self.dhtmem(45)
    }
    ///0x418 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem47(&self) -> &DHTMEM {
        self.dhtmem(46)
    }
    ///0x41c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem48(&self) -> &DHTMEM {
        self.dhtmem(47)
    }
    ///0x420 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem49(&self) -> &DHTMEM {
        self.dhtmem(48)
    }
    ///0x424 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem50(&self) -> &DHTMEM {
        self.dhtmem(49)
    }
    ///0x428 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem51(&self) -> &DHTMEM {
        self.dhtmem(50)
    }
    ///0x42c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem52(&self) -> &DHTMEM {
        self.dhtmem(51)
    }
    ///0x430 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem53(&self) -> &DHTMEM {
        self.dhtmem(52)
    }
    ///0x434 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem54(&self) -> &DHTMEM {
        self.dhtmem(53)
    }
    ///0x438 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem55(&self) -> &DHTMEM {
        self.dhtmem(54)
    }
    ///0x43c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem56(&self) -> &DHTMEM {
        self.dhtmem(55)
    }
    ///0x440 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem57(&self) -> &DHTMEM {
        self.dhtmem(56)
    }
    ///0x444 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem58(&self) -> &DHTMEM {
        self.dhtmem(57)
    }
    ///0x448 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem59(&self) -> &DHTMEM {
        self.dhtmem(58)
    }
    ///0x44c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem60(&self) -> &DHTMEM {
        self.dhtmem(59)
    }
    ///0x450 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem61(&self) -> &DHTMEM {
        self.dhtmem(60)
    }
    ///0x454 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem62(&self) -> &DHTMEM {
        self.dhtmem(61)
    }
    ///0x458 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem63(&self) -> &DHTMEM {
        self.dhtmem(62)
    }
    ///0x45c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem64(&self) -> &DHTMEM {
        self.dhtmem(63)
    }
    ///0x460 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem65(&self) -> &DHTMEM {
        self.dhtmem(64)
    }
    ///0x464 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem66(&self) -> &DHTMEM {
        self.dhtmem(65)
    }
    ///0x468 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem67(&self) -> &DHTMEM {
        self.dhtmem(66)
    }
    ///0x46c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem68(&self) -> &DHTMEM {
        self.dhtmem(67)
    }
    ///0x470 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem69(&self) -> &DHTMEM {
        self.dhtmem(68)
    }
    ///0x474 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem70(&self) -> &DHTMEM {
        self.dhtmem(69)
    }
    ///0x478 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem71(&self) -> &DHTMEM {
        self.dhtmem(70)
    }
    ///0x47c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem72(&self) -> &DHTMEM {
        self.dhtmem(71)
    }
    ///0x480 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem73(&self) -> &DHTMEM {
        self.dhtmem(72)
    }
    ///0x484 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem74(&self) -> &DHTMEM {
        self.dhtmem(73)
    }
    ///0x488 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem75(&self) -> &DHTMEM {
        self.dhtmem(74)
    }
    ///0x48c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem76(&self) -> &DHTMEM {
        self.dhtmem(75)
    }
    ///0x490 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem77(&self) -> &DHTMEM {
        self.dhtmem(76)
    }
    ///0x494 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem78(&self) -> &DHTMEM {
        self.dhtmem(77)
    }
    ///0x498 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem79(&self) -> &DHTMEM {
        self.dhtmem(78)
    }
    ///0x49c - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem80(&self) -> &DHTMEM {
        self.dhtmem(79)
    }
    ///0x4a0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem81(&self) -> &DHTMEM {
        self.dhtmem(80)
    }
    ///0x4a4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem82(&self) -> &DHTMEM {
        self.dhtmem(81)
    }
    ///0x4a8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem83(&self) -> &DHTMEM {
        self.dhtmem(82)
    }
    ///0x4ac - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem84(&self) -> &DHTMEM {
        self.dhtmem(83)
    }
    ///0x4b0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem85(&self) -> &DHTMEM {
        self.dhtmem(84)
    }
    ///0x4b4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem86(&self) -> &DHTMEM {
        self.dhtmem(85)
    }
    ///0x4b8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem87(&self) -> &DHTMEM {
        self.dhtmem(86)
    }
    ///0x4bc - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem88(&self) -> &DHTMEM {
        self.dhtmem(87)
    }
    ///0x4c0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem89(&self) -> &DHTMEM {
        self.dhtmem(88)
    }
    ///0x4c4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem90(&self) -> &DHTMEM {
        self.dhtmem(89)
    }
    ///0x4c8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem91(&self) -> &DHTMEM {
        self.dhtmem(90)
    }
    ///0x4cc - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem92(&self) -> &DHTMEM {
        self.dhtmem(91)
    }
    ///0x4d0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem93(&self) -> &DHTMEM {
        self.dhtmem(92)
    }
    ///0x4d4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem94(&self) -> &DHTMEM {
        self.dhtmem(93)
    }
    ///0x4d8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem95(&self) -> &DHTMEM {
        self.dhtmem(94)
    }
    ///0x4dc - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem96(&self) -> &DHTMEM {
        self.dhtmem(95)
    }
    ///0x4e0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem97(&self) -> &DHTMEM {
        self.dhtmem(96)
    }
    ///0x4e4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem98(&self) -> &DHTMEM {
        self.dhtmem(97)
    }
    ///0x4e8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem99(&self) -> &DHTMEM {
        self.dhtmem(98)
    }
    ///0x4ec - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem100(&self) -> &DHTMEM {
        self.dhtmem(99)
    }
    ///0x4f0 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem101(&self) -> &DHTMEM {
        self.dhtmem(100)
    }
    ///0x4f4 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem102(&self) -> &DHTMEM {
        self.dhtmem(101)
    }
    ///0x4f8 - JPEG DHTMem tables
    #[inline(always)]
    pub const fn dhtmem103(&self) -> &DHTMEM {
        self.dhtmem(102)
    }
    ///0x500..0x660 - JPEG encoder, AC Huffman table 0
    #[inline(always)]
    pub const fn huffenc_ac0(&self, n: usize) -> &HUFFENC_AC0 {
        &self.huffenc_ac0[n]
    }
    ///Iterator for array of:
    ///0x500..0x660 - JPEG encoder, AC Huffman table 0
    #[inline(always)]
    pub fn huffenc_ac0_iter(&self) -> impl Iterator<Item = &HUFFENC_AC0> {
        self.huffenc_ac0.iter()
    }
    ///0x660..0x7c0 - JPEG encoder, AC Huffman table 1
    #[inline(always)]
    pub const fn huffenc_ac1(&self, n: usize) -> &HUFFENC_AC1 {
        &self.huffenc_ac1[n]
    }
    ///Iterator for array of:
    ///0x660..0x7c0 - JPEG encoder, AC Huffman table 1
    #[inline(always)]
    pub fn huffenc_ac1_iter(&self) -> impl Iterator<Item = &HUFFENC_AC1> {
        self.huffenc_ac1.iter()
    }
    ///0x7c0..0x7e0 - JPEG encoder, DC Huffman table 0
    #[inline(always)]
    pub const fn huffenc_dc0(&self, n: usize) -> &HUFFENC_DC0 {
        &self.huffenc_dc0[n]
    }
    ///Iterator for array of:
    ///0x7c0..0x7e0 - JPEG encoder, DC Huffman table 0
    #[inline(always)]
    pub fn huffenc_dc0_iter(&self) -> impl Iterator<Item = &HUFFENC_DC0> {
        self.huffenc_dc0.iter()
    }
    ///0x7e0..0x800 - JPEG encoder, DC Huffman table 1
    #[inline(always)]
    pub const fn huffenc_dc1(&self, n: usize) -> &HUFFENC_DC1 {
        &self.huffenc_dc1[n]
    }
    ///Iterator for array of:
    ///0x7e0..0x800 - JPEG encoder, DC Huffman table 1
    #[inline(always)]
    pub fn huffenc_dc1_iter(&self) -> impl Iterator<Item = &HUFFENC_DC1> {
        self.huffenc_dc1.iter()
    }
}
/**CONFR0 (w) register accessor: JPEG codec configuration register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:CONFR0)

For information about available fields see [`mod@confr0`] module*/
pub type CONFR0 = crate::Reg<confr0::CONFR0rs>;
///JPEG codec configuration register 0
pub mod confr0;
/**CONFR1 (rw) register accessor: JPEG codec configuration register 1

You can [`read`](crate::Reg::read) this register and get [`confr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:CONFR1)

For information about available fields see [`mod@confr1`] module*/
pub type CONFR1 = crate::Reg<confr1::CONFR1rs>;
///JPEG codec configuration register 1
pub mod confr1;
/**CONFR2 (rw) register accessor: JPEG codec configuration register 2

You can [`read`](crate::Reg::read) this register and get [`confr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:CONFR2)

For information about available fields see [`mod@confr2`] module*/
pub type CONFR2 = crate::Reg<confr2::CONFR2rs>;
///JPEG codec configuration register 2
pub mod confr2;
/**CONFR3 (rw) register accessor: JPEG codec configuration register 3

You can [`read`](crate::Reg::read) this register and get [`confr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:CONFR3)

For information about available fields see [`mod@confr3`] module*/
pub type CONFR3 = crate::Reg<confr3::CONFR3rs>;
///JPEG codec configuration register 3
pub mod confr3;
/**CONFR4 (rw) register accessor: JPEG codec configuration register 4

You can [`read`](crate::Reg::read) this register and get [`confr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:CONFR4)

For information about available fields see [`mod@confr4`] module*/
pub type CONFR4 = crate::Reg<confr4::CONFR4rs>;
///JPEG codec configuration register 4
pub mod confr4;
/**CONFR5 (rw) register accessor: JPEG codec configuration register 5

You can [`read`](crate::Reg::read) this register and get [`confr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:CONFR5)

For information about available fields see [`mod@confr5`] module*/
pub type CONFR5 = crate::Reg<confr5::CONFR5rs>;
///JPEG codec configuration register 5
pub mod confr5;
/**CONFR6 (rw) register accessor: JPEG codec configuration register 6

You can [`read`](crate::Reg::read) this register and get [`confr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:CONFR6)

For information about available fields see [`mod@confr6`] module*/
pub type CONFR6 = crate::Reg<confr6::CONFR6rs>;
///JPEG codec configuration register 6
pub mod confr6;
/**CONFR7 (rw) register accessor: JPEG codec configuration register 7

You can [`read`](crate::Reg::read) this register and get [`confr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:CONFR7)

For information about available fields see [`mod@confr7`] module*/
pub type CONFR7 = crate::Reg<confr7::CONFR7rs>;
///JPEG codec configuration register 7
pub mod confr7;
/**CR (rw) register accessor: JPEG control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///JPEG control register
pub mod cr;
/**SR (r) register accessor: JPEG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///JPEG status register
pub mod sr;
/**CFR (w) register accessor: JPEG clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:CFR)

For information about available fields see [`mod@cfr`] module*/
pub type CFR = crate::Reg<cfr::CFRrs>;
///JPEG clear flag register
pub mod cfr;
/**DIR (w) register accessor: JPEG data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:DIR)

For information about available fields see [`mod@dir`] module*/
pub type DIR = crate::Reg<dir::DIRrs>;
///JPEG data input register
pub mod dir;
/**DOR (r) register accessor: JPEG data output register

You can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:DOR)

For information about available fields see [`mod@dor`] module*/
pub type DOR = crate::Reg<dor::DORrs>;
///JPEG data output register
pub mod dor;
/**QMEM0 (rw) register accessor: JPEG quantization tables

You can [`read`](crate::Reg::read) this register and get [`qmem0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:QMEM0[0])

For information about available fields see [`mod@qmem0`] module*/
pub type QMEM0 = crate::Reg<qmem0::QMEM0rs>;
///JPEG quantization tables
pub mod qmem0;
/**QMEM1 (rw) register accessor: JPEG quantization tables

You can [`read`](crate::Reg::read) this register and get [`qmem1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:QMEM1[0])

For information about available fields see [`mod@qmem1`] module*/
pub type QMEM1 = crate::Reg<qmem1::QMEM1rs>;
///JPEG quantization tables
pub mod qmem1;
/**QMEM2 (rw) register accessor: JPEG quantization tables

You can [`read`](crate::Reg::read) this register and get [`qmem2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:QMEM2[0])

For information about available fields see [`mod@qmem2`] module*/
pub type QMEM2 = crate::Reg<qmem2::QMEM2rs>;
///JPEG quantization tables
pub mod qmem2;
/**QMEM3 (rw) register accessor: JPEG quantization tables

You can [`read`](crate::Reg::read) this register and get [`qmem3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:QMEM3[0])

For information about available fields see [`mod@qmem3`] module*/
pub type QMEM3 = crate::Reg<qmem3::QMEM3rs>;
///JPEG quantization tables
pub mod qmem3;
/**HUFFMIN (rw) register accessor: JPEG HuffMin tables

You can [`read`](crate::Reg::read) this register and get [`huffmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:HUFFMIN[0])

For information about available fields see [`mod@huffmin`] module*/
pub type HUFFMIN = crate::Reg<huffmin::HUFFMINrs>;
///JPEG HuffMin tables
pub mod huffmin;
/**HUFFBASE (rw) register accessor: JPEG HuffSymb tables

You can [`read`](crate::Reg::read) this register and get [`huffbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:HUFFBASE[0])

For information about available fields see [`mod@huffbase`] module*/
pub type HUFFBASE = crate::Reg<huffbase::HUFFBASErs>;
///JPEG HuffSymb tables
pub mod huffbase;
/**HUFFSYMB (rw) register accessor: JPEG HUFFSYMB tables

You can [`read`](crate::Reg::read) this register and get [`huffsymb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:HUFFSYMB[0])

For information about available fields see [`mod@huffsymb`] module*/
pub type HUFFSYMB = crate::Reg<huffsymb::HUFFSYMBrs>;
///JPEG HUFFSYMB tables
pub mod huffsymb;
/**DHTMEM (rw) register accessor: JPEG DHTMem tables

You can [`read`](crate::Reg::read) this register and get [`dhtmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:DHTMEM[0])

For information about available fields see [`mod@dhtmem`] module*/
pub type DHTMEM = crate::Reg<dhtmem::DHTMEMrs>;
///JPEG DHTMem tables
pub mod dhtmem;
/**HUFFENC_AC0 (rw) register accessor: JPEG encoder, AC Huffman table 0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:HUFFENC_AC0[0])

For information about available fields see [`mod@huffenc_ac0`] module*/
pub type HUFFENC_AC0 = crate::Reg<huffenc_ac0::HUFFENC_AC0rs>;
///JPEG encoder, AC Huffman table 0
pub mod huffenc_ac0;
/**HUFFENC_AC1 (rw) register accessor: JPEG encoder, AC Huffman table 1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:HUFFENC_AC1[0])

For information about available fields see [`mod@huffenc_ac1`] module*/
pub type HUFFENC_AC1 = crate::Reg<huffenc_ac1::HUFFENC_AC1rs>;
///JPEG encoder, AC Huffman table 1
pub mod huffenc_ac1;
/**HUFFENC_DC0 (rw) register accessor: JPEG encoder, DC Huffman table 0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:HUFFENC_DC0[0])

For information about available fields see [`mod@huffenc_dc0`] module*/
pub type HUFFENC_DC0 = crate::Reg<huffenc_dc0::HUFFENC_DC0rs>;
///JPEG encoder, DC Huffman table 0
pub mod huffenc_dc0;
/**HUFFENC_DC1 (rw) register accessor: JPEG encoder, DC Huffman table 1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#JPEG:HUFFENC_DC1[0])

For information about available fields see [`mod@huffenc_dc1`] module*/
pub type HUFFENC_DC1 = crate::Reg<huffenc_dc1::HUFFENC_DC1rs>;
///JPEG encoder, DC Huffman table 1
pub mod huffenc_dc1;
