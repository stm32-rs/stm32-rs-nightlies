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
    huffmin: [HUFFMIN; 4],
    huffbase: [HUFFBASE; 32],
    huffsymb: [HUFFSYMB; 84],
    dhtmem: [DHTMEM; 103],
    _reserved21: [u8; 0x04],
    _reserved_21_huffenc_ac: [u8; 0x023c],
    _reserved22: [u8; 0x84],
    huffenc_dc0: [HUFFENC_DC0; 8],
    _reserved23: [u8; 0xbc],
    huffenc_dc1: [HUFFENC_DC1; 8],
}
impl RegisterBlock {
    ///0x00 - JPEG codec control register
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
    ///0x50..0x90 - JPEG quantization memory 0
    #[inline(always)]
    pub const fn qmem0(&self, n: usize) -> &QMEM0 {
        &self.qmem0[n]
    }
    ///Iterator for array of:
    ///0x50..0x90 - JPEG quantization memory 0
    #[inline(always)]
    pub fn qmem0_iter(&self) -> impl Iterator<Item = &QMEM0> {
        self.qmem0.iter()
    }
    ///0x90..0xd0 - JPEG quantization memory 1
    #[inline(always)]
    pub const fn qmem1(&self, n: usize) -> &QMEM1 {
        &self.qmem1[n]
    }
    ///Iterator for array of:
    ///0x90..0xd0 - JPEG quantization memory 1
    #[inline(always)]
    pub fn qmem1_iter(&self) -> impl Iterator<Item = &QMEM1> {
        self.qmem1.iter()
    }
    ///0xd0..0x110 - JPEG quantization memory 2
    #[inline(always)]
    pub const fn qmem2(&self, n: usize) -> &QMEM2 {
        &self.qmem2[n]
    }
    ///Iterator for array of:
    ///0xd0..0x110 - JPEG quantization memory 2
    #[inline(always)]
    pub fn qmem2_iter(&self) -> impl Iterator<Item = &QMEM2> {
        self.qmem2.iter()
    }
    ///0x110..0x150 - JPEG quantization memory 3
    #[inline(always)]
    pub const fn qmem3(&self, n: usize) -> &QMEM3 {
        &self.qmem3[n]
    }
    ///Iterator for array of:
    ///0x110..0x150 - JPEG quantization memory 3
    #[inline(always)]
    pub fn qmem3_iter(&self) -> impl Iterator<Item = &QMEM3> {
        self.qmem3.iter()
    }
    ///0x150..0x190 - HUFFMIN cluster: 100-bit minimum Huffman value
    #[inline(always)]
    pub const fn huffmin(&self, n: usize) -> &HUFFMIN {
        &self.huffmin[n]
    }
    ///Iterator for array of:
    ///0x150..0x190 - HUFFMIN cluster: 100-bit minimum Huffman value
    #[inline(always)]
    pub fn huffmin_iter(&self) -> impl Iterator<Item = &HUFFMIN> {
        self.huffmin.iter()
    }
    ///0x190..0x210 - JPEG Huffman base
    #[inline(always)]
    pub const fn huffbase(&self, n: usize) -> &HUFFBASE {
        &self.huffbase[n]
    }
    ///Iterator for array of:
    ///0x190..0x210 - JPEG Huffman base
    #[inline(always)]
    pub fn huffbase_iter(&self) -> impl Iterator<Item = &HUFFBASE> {
        self.huffbase.iter()
    }
    ///0x210..0x360 - JPEG Huffman symbol
    #[inline(always)]
    pub const fn huffsymb(&self, n: usize) -> &HUFFSYMB {
        &self.huffsymb[n]
    }
    ///Iterator for array of:
    ///0x210..0x360 - JPEG Huffman symbol
    #[inline(always)]
    pub fn huffsymb_iter(&self) -> impl Iterator<Item = &HUFFSYMB> {
        self.huffsymb.iter()
    }
    ///0x360..0x4fc - JPEG DHT memory
    #[inline(always)]
    pub const fn dhtmem(&self, n: usize) -> &DHTMEM {
        &self.dhtmem[n]
    }
    ///Iterator for array of:
    ///0x360..0x4fc - JPEG DHT memory
    #[inline(always)]
    pub fn dhtmem_iter(&self) -> impl Iterator<Item = &DHTMEM> {
        self.dhtmem.iter()
    }
    ///0x500..0x660 - JPEG encoder, AC Huffman table 0
    #[inline(always)]
    pub const fn huffenc_ac0(&self, n: usize) -> &HUFFENC_AC0 {
        #[allow(clippy::no_effect)]
        [(); 88][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1280)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x500..0x660 - JPEG encoder, AC Huffman table 0
    #[inline(always)]
    pub fn huffenc_ac0_iter(&self) -> impl Iterator<Item = &HUFFENC_AC0> {
        (0..88).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1280)
                .add(4 * n)
                .cast()
        })
    }
    ///0x5dc..0x73c - JPEG encoder, AC Huffman table 1
    #[inline(always)]
    pub const fn huffenc_ac1(&self, n: usize) -> &HUFFENC_AC1 {
        #[allow(clippy::no_effect)]
        [(); 88][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1500)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x5dc..0x73c - JPEG encoder, AC Huffman table 1
    #[inline(always)]
    pub fn huffenc_ac1_iter(&self) -> impl Iterator<Item = &HUFFENC_AC1> {
        (0..88).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1500)
                .add(4 * n)
                .cast()
        })
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
    ///0x89c..0x8bc - JPEG encoder, DC Huffman table 1
    #[inline(always)]
    pub const fn huffenc_dc1(&self, n: usize) -> &HUFFENC_DC1 {
        &self.huffenc_dc1[n]
    }
    ///Iterator for array of:
    ///0x89c..0x8bc - JPEG encoder, DC Huffman table 1
    #[inline(always)]
    pub fn huffenc_dc1_iter(&self) -> impl Iterator<Item = &HUFFENC_DC1> {
        self.huffenc_dc1.iter()
    }
}
/**CONFR0 (w) register accessor: JPEG codec control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:CONFR0)

For information about available fields see [`mod@confr0`] module*/
pub type CONFR0 = crate::Reg<confr0::CONFR0rs>;
///JPEG codec control register
pub mod confr0;
/**CONFR1 (rw) register accessor: JPEG codec configuration register 1

You can [`read`](crate::Reg::read) this register and get [`confr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:CONFR1)

For information about available fields see [`mod@confr1`] module*/
pub type CONFR1 = crate::Reg<confr1::CONFR1rs>;
///JPEG codec configuration register 1
pub mod confr1;
/**CONFR2 (rw) register accessor: JPEG codec configuration register 2

You can [`read`](crate::Reg::read) this register and get [`confr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:CONFR2)

For information about available fields see [`mod@confr2`] module*/
pub type CONFR2 = crate::Reg<confr2::CONFR2rs>;
///JPEG codec configuration register 2
pub mod confr2;
/**CONFR3 (rw) register accessor: JPEG codec configuration register 3

You can [`read`](crate::Reg::read) this register and get [`confr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:CONFR3)

For information about available fields see [`mod@confr3`] module*/
pub type CONFR3 = crate::Reg<confr3::CONFR3rs>;
///JPEG codec configuration register 3
pub mod confr3;
/**CONFR4 (rw) register accessor: JPEG codec configuration register 4

You can [`read`](crate::Reg::read) this register and get [`confr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:CONFR4)

For information about available fields see [`mod@confr4`] module*/
pub type CONFR4 = crate::Reg<confr4::CONFR4rs>;
///JPEG codec configuration register 4
pub mod confr4;
/**CONFR5 (rw) register accessor: JPEG codec configuration register 5

You can [`read`](crate::Reg::read) this register and get [`confr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:CONFR5)

For information about available fields see [`mod@confr5`] module*/
pub type CONFR5 = crate::Reg<confr5::CONFR5rs>;
///JPEG codec configuration register 5
pub mod confr5;
/**CONFR6 (rw) register accessor: JPEG codec configuration register 6

You can [`read`](crate::Reg::read) this register and get [`confr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:CONFR6)

For information about available fields see [`mod@confr6`] module*/
pub type CONFR6 = crate::Reg<confr6::CONFR6rs>;
///JPEG codec configuration register 6
pub mod confr6;
/**CONFR7 (rw) register accessor: JPEG codec configuration register 7

You can [`read`](crate::Reg::read) this register and get [`confr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:CONFR7)

For information about available fields see [`mod@confr7`] module*/
pub type CONFR7 = crate::Reg<confr7::CONFR7rs>;
///JPEG codec configuration register 7
pub mod confr7;
/**CR (rw) register accessor: JPEG control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///JPEG control register
pub mod cr;
/**SR (r) register accessor: JPEG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///JPEG status register
pub mod sr;
/**CFR (rw) register accessor: JPEG clear flag register

You can [`read`](crate::Reg::read) this register and get [`cfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:CFR)

For information about available fields see [`mod@cfr`] module*/
pub type CFR = crate::Reg<cfr::CFRrs>;
///JPEG clear flag register
pub mod cfr;
/**DIR (w) register accessor: JPEG data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:DIR)

For information about available fields see [`mod@dir`] module*/
pub type DIR = crate::Reg<dir::DIRrs>;
///JPEG data input register
pub mod dir;
/**DOR (r) register accessor: JPEG data output register

You can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:DOR)

For information about available fields see [`mod@dor`] module*/
pub type DOR = crate::Reg<dor::DORrs>;
///JPEG data output register
pub mod dor;
/**QMEM0 (rw) register accessor: JPEG quantization memory 0

You can [`read`](crate::Reg::read) this register and get [`qmem0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:QMEM0[0])

For information about available fields see [`mod@qmem0`] module*/
pub type QMEM0 = crate::Reg<qmem0::QMEM0rs>;
///JPEG quantization memory 0
pub mod qmem0;
/**QMEM1 (rw) register accessor: JPEG quantization memory 1

You can [`read`](crate::Reg::read) this register and get [`qmem1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:QMEM1[0])

For information about available fields see [`mod@qmem1`] module*/
pub type QMEM1 = crate::Reg<qmem1::QMEM1rs>;
///JPEG quantization memory 1
pub mod qmem1;
/**QMEM2 (rw) register accessor: JPEG quantization memory 2

You can [`read`](crate::Reg::read) this register and get [`qmem2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:QMEM2[0])

For information about available fields see [`mod@qmem2`] module*/
pub type QMEM2 = crate::Reg<qmem2::QMEM2rs>;
///JPEG quantization memory 2
pub mod qmem2;
/**QMEM3 (rw) register accessor: JPEG quantization memory 3

You can [`read`](crate::Reg::read) this register and get [`qmem3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qmem3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:QMEM3[0])

For information about available fields see [`mod@qmem3`] module*/
pub type QMEM3 = crate::Reg<qmem3::QMEM3rs>;
///JPEG quantization memory 3
pub mod qmem3;
///HUFFMIN cluster: 100-bit minimum Huffman value
pub use self::huffmin::HUFFMIN;
///Cluster
///HUFFMIN cluster: 100-bit minimum Huffman value
pub mod huffmin;
/**HUFFBASE (rw) register accessor: JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:HUFFBASE[0])

For information about available fields see [`mod@huffbase`] module*/
pub type HUFFBASE = crate::Reg<huffbase::HUFFBASErs>;
///JPEG Huffman base
pub mod huffbase;
/**HUFFSYMB (rw) register accessor: JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:HUFFSYMB[0])

For information about available fields see [`mod@huffsymb`] module*/
pub type HUFFSYMB = crate::Reg<huffsymb::HUFFSYMBrs>;
///JPEG Huffman symbol
pub mod huffsymb;
/**DHTMEM (rw) register accessor: JPEG DHT memory

You can [`read`](crate::Reg::read) this register and get [`dhtmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhtmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:DHTMEM[0])

For information about available fields see [`mod@dhtmem`] module*/
pub type DHTMEM = crate::Reg<dhtmem::DHTMEMrs>;
///JPEG DHT memory
pub mod dhtmem;
/**HUFFENC_AC0 (rw) register accessor: JPEG encoder, AC Huffman table 0

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:HUFFENC_AC0[0])

For information about available fields see [`mod@huffenc_ac0`] module*/
pub type HUFFENC_AC0 = crate::Reg<huffenc_ac0::HUFFENC_AC0rs>;
///JPEG encoder, AC Huffman table 0
pub mod huffenc_ac0;
/**HUFFENC_AC1 (rw) register accessor: JPEG encoder, AC Huffman table 1

You can [`read`](crate::Reg::read) this register and get [`huffenc_ac1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_ac1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:HUFFENC_AC1[0])

For information about available fields see [`mod@huffenc_ac1`] module*/
pub type HUFFENC_AC1 = crate::Reg<huffenc_ac1::HUFFENC_AC1rs>;
///JPEG encoder, AC Huffman table 1
pub mod huffenc_ac1;
/**HUFFENC_DC0 (rw) register accessor: JPEG encoder, DC Huffman table 0

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:HUFFENC_DC0[0])

For information about available fields see [`mod@huffenc_dc0`] module*/
pub type HUFFENC_DC0 = crate::Reg<huffenc_dc0::HUFFENC_DC0rs>;
///JPEG encoder, DC Huffman table 0
pub mod huffenc_dc0;
/**HUFFENC_DC1 (rw) register accessor: JPEG encoder, DC Huffman table 1

You can [`read`](crate::Reg::read) this register and get [`huffenc_dc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffenc_dc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#JPEG:HUFFENC_DC1[0])

For information about available fields see [`mod@huffenc_dc1`] module*/
pub type HUFFENC_DC1 = crate::Reg<huffenc_dc1::HUFFENC_DC1rs>;
///JPEG encoder, DC Huffman table 1
pub mod huffenc_dc1;
