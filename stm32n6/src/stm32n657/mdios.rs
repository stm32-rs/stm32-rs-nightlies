#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    wrfr: WRFR,
    cwrfr: CWRFR,
    rdfr: RDFR,
    crdfr: CRDFR,
    sr: SR,
    clrfr: CLRFR,
    _reserved7: [u8; 0xe4],
    dinr0: DINR0,
    dinr1: DINR1,
    dinr2: DINR2,
    dinr3: DINR3,
    dinr4: DINR4,
    dinr5: DINR5,
    dinr6: DINR6,
    dinr7: DINR7,
    dinr8: DINR8,
    dinr9: DINR9,
    dinr10: DINR10,
    dinr11: DINR11,
    dinr12: DINR12,
    dinr13: DINR13,
    dinr14: DINR14,
    dinr15: DINR15,
    dinr16: DINR16,
    dinr17: DINR17,
    dinr18: DINR18,
    dinr19: DINR19,
    dinr20: DINR20,
    dinr21: DINR21,
    dinr22: DINR22,
    dinr23: DINR23,
    dinr24: DINR24,
    dinr25: DINR25,
    dinr26: DINR26,
    dinr27: DINR27,
    dinr28: DINR28,
    dinr29: DINR29,
    dinr30: DINR30,
    dinr31: DINR31,
    doutr0: DOUTR0,
    doutr1: DOUTR1,
    doutr2: DOUTR2,
    doutr3: DOUTR3,
    doutr4: DOUTR4,
    doutr5: DOUTR5,
    doutr6: DOUTR6,
    doutr7: DOUTR7,
    doutr8: DOUTR8,
    doutr9: DOUTR9,
    doutr10: DOUTR10,
    doutr11: DOUTR11,
    doutr12: DOUTR12,
    doutr13: DOUTR13,
    doutr14: DOUTR14,
    doutr15: DOUTR15,
    doutr16: DOUTR16,
    doutr17: DOUTR17,
    doutr18: DOUTR18,
    doutr19: DOUTR19,
    doutr20: DOUTR20,
    doutr21: DOUTR21,
    doutr22: DOUTR22,
    doutr23: DOUTR23,
    doutr24: DOUTR24,
    doutr25: DOUTR25,
    doutr26: DOUTR26,
    doutr27: DOUTR27,
    doutr28: DOUTR28,
    doutr29: DOUTR29,
    doutr30: DOUTR30,
    doutr31: DOUTR31,
}
impl RegisterBlock {
    ///0x00 - MDIOS configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - MDIOS write flag register
    #[inline(always)]
    pub const fn wrfr(&self) -> &WRFR {
        &self.wrfr
    }
    ///0x08 - MDIOS clear write flag register
    #[inline(always)]
    pub const fn cwrfr(&self) -> &CWRFR {
        &self.cwrfr
    }
    ///0x0c - MDIOS read flag register
    #[inline(always)]
    pub const fn rdfr(&self) -> &RDFR {
        &self.rdfr
    }
    ///0x10 - MDIOS clear read flag register
    #[inline(always)]
    pub const fn crdfr(&self) -> &CRDFR {
        &self.crdfr
    }
    ///0x14 - MDIOS status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x18 - MDIOS clear flag register
    #[inline(always)]
    pub const fn clrfr(&self) -> &CLRFR {
        &self.clrfr
    }
    ///0x100 - MDIOS input data register 0
    #[inline(always)]
    pub const fn dinr0(&self) -> &DINR0 {
        &self.dinr0
    }
    ///0x104 - MDIOS input data register 1
    #[inline(always)]
    pub const fn dinr1(&self) -> &DINR1 {
        &self.dinr1
    }
    ///0x108 - MDIOS input data register 2
    #[inline(always)]
    pub const fn dinr2(&self) -> &DINR2 {
        &self.dinr2
    }
    ///0x10c - MDIOS input data register 3
    #[inline(always)]
    pub const fn dinr3(&self) -> &DINR3 {
        &self.dinr3
    }
    ///0x110 - MDIOS input data register 4
    #[inline(always)]
    pub const fn dinr4(&self) -> &DINR4 {
        &self.dinr4
    }
    ///0x114 - MDIOS input data register 5
    #[inline(always)]
    pub const fn dinr5(&self) -> &DINR5 {
        &self.dinr5
    }
    ///0x118 - MDIOS input data register 6
    #[inline(always)]
    pub const fn dinr6(&self) -> &DINR6 {
        &self.dinr6
    }
    ///0x11c - MDIOS input data register 7
    #[inline(always)]
    pub const fn dinr7(&self) -> &DINR7 {
        &self.dinr7
    }
    ///0x120 - MDIOS input data register 8
    #[inline(always)]
    pub const fn dinr8(&self) -> &DINR8 {
        &self.dinr8
    }
    ///0x124 - MDIOS input data register 9
    #[inline(always)]
    pub const fn dinr9(&self) -> &DINR9 {
        &self.dinr9
    }
    ///0x128 - MDIOS input data register 10
    #[inline(always)]
    pub const fn dinr10(&self) -> &DINR10 {
        &self.dinr10
    }
    ///0x12c - MDIOS input data register 11
    #[inline(always)]
    pub const fn dinr11(&self) -> &DINR11 {
        &self.dinr11
    }
    ///0x130 - MDIOS input data register 12
    #[inline(always)]
    pub const fn dinr12(&self) -> &DINR12 {
        &self.dinr12
    }
    ///0x134 - MDIOS input data register 13
    #[inline(always)]
    pub const fn dinr13(&self) -> &DINR13 {
        &self.dinr13
    }
    ///0x138 - MDIOS input data register 14
    #[inline(always)]
    pub const fn dinr14(&self) -> &DINR14 {
        &self.dinr14
    }
    ///0x13c - MDIOS input data register 15
    #[inline(always)]
    pub const fn dinr15(&self) -> &DINR15 {
        &self.dinr15
    }
    ///0x140 - MDIOS input data register 16
    #[inline(always)]
    pub const fn dinr16(&self) -> &DINR16 {
        &self.dinr16
    }
    ///0x144 - MDIOS input data register 17
    #[inline(always)]
    pub const fn dinr17(&self) -> &DINR17 {
        &self.dinr17
    }
    ///0x148 - MDIOS input data register 18
    #[inline(always)]
    pub const fn dinr18(&self) -> &DINR18 {
        &self.dinr18
    }
    ///0x14c - MDIOS input data register 19
    #[inline(always)]
    pub const fn dinr19(&self) -> &DINR19 {
        &self.dinr19
    }
    ///0x150 - MDIOS input data register 20
    #[inline(always)]
    pub const fn dinr20(&self) -> &DINR20 {
        &self.dinr20
    }
    ///0x154 - MDIOS input data register 21
    #[inline(always)]
    pub const fn dinr21(&self) -> &DINR21 {
        &self.dinr21
    }
    ///0x158 - MDIOS input data register 22
    #[inline(always)]
    pub const fn dinr22(&self) -> &DINR22 {
        &self.dinr22
    }
    ///0x15c - MDIOS input data register 23
    #[inline(always)]
    pub const fn dinr23(&self) -> &DINR23 {
        &self.dinr23
    }
    ///0x160 - MDIOS input data register 24
    #[inline(always)]
    pub const fn dinr24(&self) -> &DINR24 {
        &self.dinr24
    }
    ///0x164 - MDIOS input data register 25
    #[inline(always)]
    pub const fn dinr25(&self) -> &DINR25 {
        &self.dinr25
    }
    ///0x168 - MDIOS input data register 26
    #[inline(always)]
    pub const fn dinr26(&self) -> &DINR26 {
        &self.dinr26
    }
    ///0x16c - MDIOS input data register 27
    #[inline(always)]
    pub const fn dinr27(&self) -> &DINR27 {
        &self.dinr27
    }
    ///0x170 - MDIOS input data register 28
    #[inline(always)]
    pub const fn dinr28(&self) -> &DINR28 {
        &self.dinr28
    }
    ///0x174 - MDIOS input data register 29
    #[inline(always)]
    pub const fn dinr29(&self) -> &DINR29 {
        &self.dinr29
    }
    ///0x178 - MDIOS input data register 30
    #[inline(always)]
    pub const fn dinr30(&self) -> &DINR30 {
        &self.dinr30
    }
    ///0x17c - MDIOS input data register 31
    #[inline(always)]
    pub const fn dinr31(&self) -> &DINR31 {
        &self.dinr31
    }
    ///0x180 - MDIOS output data register 0
    #[inline(always)]
    pub const fn doutr0(&self) -> &DOUTR0 {
        &self.doutr0
    }
    ///0x184 - MDIOS output data register 1
    #[inline(always)]
    pub const fn doutr1(&self) -> &DOUTR1 {
        &self.doutr1
    }
    ///0x188 - MDIOS output data register 2
    #[inline(always)]
    pub const fn doutr2(&self) -> &DOUTR2 {
        &self.doutr2
    }
    ///0x18c - MDIOS output data register 3
    #[inline(always)]
    pub const fn doutr3(&self) -> &DOUTR3 {
        &self.doutr3
    }
    ///0x190 - MDIOS output data register 4
    #[inline(always)]
    pub const fn doutr4(&self) -> &DOUTR4 {
        &self.doutr4
    }
    ///0x194 - MDIOS output data register 5
    #[inline(always)]
    pub const fn doutr5(&self) -> &DOUTR5 {
        &self.doutr5
    }
    ///0x198 - MDIOS output data register 6
    #[inline(always)]
    pub const fn doutr6(&self) -> &DOUTR6 {
        &self.doutr6
    }
    ///0x19c - MDIOS output data register 7
    #[inline(always)]
    pub const fn doutr7(&self) -> &DOUTR7 {
        &self.doutr7
    }
    ///0x1a0 - MDIOS output data register 8
    #[inline(always)]
    pub const fn doutr8(&self) -> &DOUTR8 {
        &self.doutr8
    }
    ///0x1a4 - MDIOS output data register 9
    #[inline(always)]
    pub const fn doutr9(&self) -> &DOUTR9 {
        &self.doutr9
    }
    ///0x1a8 - MDIOS output data register 10
    #[inline(always)]
    pub const fn doutr10(&self) -> &DOUTR10 {
        &self.doutr10
    }
    ///0x1ac - MDIOS output data register 11
    #[inline(always)]
    pub const fn doutr11(&self) -> &DOUTR11 {
        &self.doutr11
    }
    ///0x1b0 - MDIOS output data register 12
    #[inline(always)]
    pub const fn doutr12(&self) -> &DOUTR12 {
        &self.doutr12
    }
    ///0x1b4 - MDIOS output data register 13
    #[inline(always)]
    pub const fn doutr13(&self) -> &DOUTR13 {
        &self.doutr13
    }
    ///0x1b8 - MDIOS output data register 14
    #[inline(always)]
    pub const fn doutr14(&self) -> &DOUTR14 {
        &self.doutr14
    }
    ///0x1bc - MDIOS output data register 15
    #[inline(always)]
    pub const fn doutr15(&self) -> &DOUTR15 {
        &self.doutr15
    }
    ///0x1c0 - MDIOS output data register 16
    #[inline(always)]
    pub const fn doutr16(&self) -> &DOUTR16 {
        &self.doutr16
    }
    ///0x1c4 - MDIOS output data register 17
    #[inline(always)]
    pub const fn doutr17(&self) -> &DOUTR17 {
        &self.doutr17
    }
    ///0x1c8 - MDIOS output data register 18
    #[inline(always)]
    pub const fn doutr18(&self) -> &DOUTR18 {
        &self.doutr18
    }
    ///0x1cc - MDIOS output data register 19
    #[inline(always)]
    pub const fn doutr19(&self) -> &DOUTR19 {
        &self.doutr19
    }
    ///0x1d0 - MDIOS output data register 20
    #[inline(always)]
    pub const fn doutr20(&self) -> &DOUTR20 {
        &self.doutr20
    }
    ///0x1d4 - MDIOS output data register 21
    #[inline(always)]
    pub const fn doutr21(&self) -> &DOUTR21 {
        &self.doutr21
    }
    ///0x1d8 - MDIOS output data register 22
    #[inline(always)]
    pub const fn doutr22(&self) -> &DOUTR22 {
        &self.doutr22
    }
    ///0x1dc - MDIOS output data register 23
    #[inline(always)]
    pub const fn doutr23(&self) -> &DOUTR23 {
        &self.doutr23
    }
    ///0x1e0 - MDIOS output data register 24
    #[inline(always)]
    pub const fn doutr24(&self) -> &DOUTR24 {
        &self.doutr24
    }
    ///0x1e4 - MDIOS output data register 25
    #[inline(always)]
    pub const fn doutr25(&self) -> &DOUTR25 {
        &self.doutr25
    }
    ///0x1e8 - MDIOS output data register 26
    #[inline(always)]
    pub const fn doutr26(&self) -> &DOUTR26 {
        &self.doutr26
    }
    ///0x1ec - MDIOS output data register 27
    #[inline(always)]
    pub const fn doutr27(&self) -> &DOUTR27 {
        &self.doutr27
    }
    ///0x1f0 - MDIOS output data register 28
    #[inline(always)]
    pub const fn doutr28(&self) -> &DOUTR28 {
        &self.doutr28
    }
    ///0x1f4 - MDIOS output data register 29
    #[inline(always)]
    pub const fn doutr29(&self) -> &DOUTR29 {
        &self.doutr29
    }
    ///0x1f8 - MDIOS output data register 30
    #[inline(always)]
    pub const fn doutr30(&self) -> &DOUTR30 {
        &self.doutr30
    }
    ///0x1fc - MDIOS output data register 31
    #[inline(always)]
    pub const fn doutr31(&self) -> &DOUTR31 {
        &self.doutr31
    }
}
/**CR (rw) register accessor: MDIOS configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///MDIOS configuration register
pub mod cr;
/**WRFR (r) register accessor: MDIOS write flag register

You can [`read`](crate::Reg::read) this register and get [`wrfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:WRFR)

For information about available fields see [`mod@wrfr`] module*/
pub type WRFR = crate::Reg<wrfr::WRFRrs>;
///MDIOS write flag register
pub mod wrfr;
/**CWRFR (rw) register accessor: MDIOS clear write flag register

You can [`read`](crate::Reg::read) this register and get [`cwrfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwrfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:CWRFR)

For information about available fields see [`mod@cwrfr`] module*/
pub type CWRFR = crate::Reg<cwrfr::CWRFRrs>;
///MDIOS clear write flag register
pub mod cwrfr;
/**RDFR (r) register accessor: MDIOS read flag register

You can [`read`](crate::Reg::read) this register and get [`rdfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:RDFR)

For information about available fields see [`mod@rdfr`] module*/
pub type RDFR = crate::Reg<rdfr::RDFRrs>;
///MDIOS read flag register
pub mod rdfr;
/**CRDFR (rw) register accessor: MDIOS clear read flag register

You can [`read`](crate::Reg::read) this register and get [`crdfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crdfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:CRDFR)

For information about available fields see [`mod@crdfr`] module*/
pub type CRDFR = crate::Reg<crdfr::CRDFRrs>;
///MDIOS clear read flag register
pub mod crdfr;
/**SR (r) register accessor: MDIOS status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///MDIOS status register
pub mod sr;
/**CLRFR (rw) register accessor: MDIOS clear flag register

You can [`read`](crate::Reg::read) this register and get [`clrfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:CLRFR)

For information about available fields see [`mod@clrfr`] module*/
pub type CLRFR = crate::Reg<clrfr::CLRFRrs>;
///MDIOS clear flag register
pub mod clrfr;
/**DINR0 (r) register accessor: MDIOS input data register 0

You can [`read`](crate::Reg::read) this register and get [`dinr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR0)

For information about available fields see [`mod@dinr0`] module*/
pub type DINR0 = crate::Reg<dinr0::DINR0rs>;
///MDIOS input data register 0
pub mod dinr0;
/**DINR1 (r) register accessor: MDIOS input data register 1

You can [`read`](crate::Reg::read) this register and get [`dinr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR1)

For information about available fields see [`mod@dinr1`] module*/
pub type DINR1 = crate::Reg<dinr1::DINR1rs>;
///MDIOS input data register 1
pub mod dinr1;
/**DINR2 (r) register accessor: MDIOS input data register 2

You can [`read`](crate::Reg::read) this register and get [`dinr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR2)

For information about available fields see [`mod@dinr2`] module*/
pub type DINR2 = crate::Reg<dinr2::DINR2rs>;
///MDIOS input data register 2
pub mod dinr2;
/**DINR3 (r) register accessor: MDIOS input data register 3

You can [`read`](crate::Reg::read) this register and get [`dinr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR3)

For information about available fields see [`mod@dinr3`] module*/
pub type DINR3 = crate::Reg<dinr3::DINR3rs>;
///MDIOS input data register 3
pub mod dinr3;
/**DINR4 (r) register accessor: MDIOS input data register 4

You can [`read`](crate::Reg::read) this register and get [`dinr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR4)

For information about available fields see [`mod@dinr4`] module*/
pub type DINR4 = crate::Reg<dinr4::DINR4rs>;
///MDIOS input data register 4
pub mod dinr4;
/**DINR5 (r) register accessor: MDIOS input data register 5

You can [`read`](crate::Reg::read) this register and get [`dinr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR5)

For information about available fields see [`mod@dinr5`] module*/
pub type DINR5 = crate::Reg<dinr5::DINR5rs>;
///MDIOS input data register 5
pub mod dinr5;
/**DINR6 (r) register accessor: MDIOS input data register 6

You can [`read`](crate::Reg::read) this register and get [`dinr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR6)

For information about available fields see [`mod@dinr6`] module*/
pub type DINR6 = crate::Reg<dinr6::DINR6rs>;
///MDIOS input data register 6
pub mod dinr6;
/**DINR7 (r) register accessor: MDIOS input data register 7

You can [`read`](crate::Reg::read) this register and get [`dinr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR7)

For information about available fields see [`mod@dinr7`] module*/
pub type DINR7 = crate::Reg<dinr7::DINR7rs>;
///MDIOS input data register 7
pub mod dinr7;
/**DINR8 (r) register accessor: MDIOS input data register 8

You can [`read`](crate::Reg::read) this register and get [`dinr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR8)

For information about available fields see [`mod@dinr8`] module*/
pub type DINR8 = crate::Reg<dinr8::DINR8rs>;
///MDIOS input data register 8
pub mod dinr8;
/**DINR9 (r) register accessor: MDIOS input data register 9

You can [`read`](crate::Reg::read) this register and get [`dinr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR9)

For information about available fields see [`mod@dinr9`] module*/
pub type DINR9 = crate::Reg<dinr9::DINR9rs>;
///MDIOS input data register 9
pub mod dinr9;
/**DINR10 (r) register accessor: MDIOS input data register 10

You can [`read`](crate::Reg::read) this register and get [`dinr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR10)

For information about available fields see [`mod@dinr10`] module*/
pub type DINR10 = crate::Reg<dinr10::DINR10rs>;
///MDIOS input data register 10
pub mod dinr10;
/**DINR11 (r) register accessor: MDIOS input data register 11

You can [`read`](crate::Reg::read) this register and get [`dinr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR11)

For information about available fields see [`mod@dinr11`] module*/
pub type DINR11 = crate::Reg<dinr11::DINR11rs>;
///MDIOS input data register 11
pub mod dinr11;
/**DINR12 (r) register accessor: MDIOS input data register 12

You can [`read`](crate::Reg::read) this register and get [`dinr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR12)

For information about available fields see [`mod@dinr12`] module*/
pub type DINR12 = crate::Reg<dinr12::DINR12rs>;
///MDIOS input data register 12
pub mod dinr12;
/**DINR13 (r) register accessor: MDIOS input data register 13

You can [`read`](crate::Reg::read) this register and get [`dinr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR13)

For information about available fields see [`mod@dinr13`] module*/
pub type DINR13 = crate::Reg<dinr13::DINR13rs>;
///MDIOS input data register 13
pub mod dinr13;
/**DINR14 (r) register accessor: MDIOS input data register 14

You can [`read`](crate::Reg::read) this register and get [`dinr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR14)

For information about available fields see [`mod@dinr14`] module*/
pub type DINR14 = crate::Reg<dinr14::DINR14rs>;
///MDIOS input data register 14
pub mod dinr14;
/**DINR15 (r) register accessor: MDIOS input data register 15

You can [`read`](crate::Reg::read) this register and get [`dinr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR15)

For information about available fields see [`mod@dinr15`] module*/
pub type DINR15 = crate::Reg<dinr15::DINR15rs>;
///MDIOS input data register 15
pub mod dinr15;
/**DINR16 (r) register accessor: MDIOS input data register 16

You can [`read`](crate::Reg::read) this register and get [`dinr16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR16)

For information about available fields see [`mod@dinr16`] module*/
pub type DINR16 = crate::Reg<dinr16::DINR16rs>;
///MDIOS input data register 16
pub mod dinr16;
/**DINR17 (r) register accessor: MDIOS input data register 17

You can [`read`](crate::Reg::read) this register and get [`dinr17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR17)

For information about available fields see [`mod@dinr17`] module*/
pub type DINR17 = crate::Reg<dinr17::DINR17rs>;
///MDIOS input data register 17
pub mod dinr17;
/**DINR18 (r) register accessor: MDIOS input data register 18

You can [`read`](crate::Reg::read) this register and get [`dinr18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR18)

For information about available fields see [`mod@dinr18`] module*/
pub type DINR18 = crate::Reg<dinr18::DINR18rs>;
///MDIOS input data register 18
pub mod dinr18;
/**DINR19 (r) register accessor: MDIOS input data register 19

You can [`read`](crate::Reg::read) this register and get [`dinr19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR19)

For information about available fields see [`mod@dinr19`] module*/
pub type DINR19 = crate::Reg<dinr19::DINR19rs>;
///MDIOS input data register 19
pub mod dinr19;
/**DINR20 (r) register accessor: MDIOS input data register 20

You can [`read`](crate::Reg::read) this register and get [`dinr20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR20)

For information about available fields see [`mod@dinr20`] module*/
pub type DINR20 = crate::Reg<dinr20::DINR20rs>;
///MDIOS input data register 20
pub mod dinr20;
/**DINR21 (r) register accessor: MDIOS input data register 21

You can [`read`](crate::Reg::read) this register and get [`dinr21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR21)

For information about available fields see [`mod@dinr21`] module*/
pub type DINR21 = crate::Reg<dinr21::DINR21rs>;
///MDIOS input data register 21
pub mod dinr21;
/**DINR22 (r) register accessor: MDIOS input data register 22

You can [`read`](crate::Reg::read) this register and get [`dinr22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR22)

For information about available fields see [`mod@dinr22`] module*/
pub type DINR22 = crate::Reg<dinr22::DINR22rs>;
///MDIOS input data register 22
pub mod dinr22;
/**DINR23 (r) register accessor: MDIOS input data register 23

You can [`read`](crate::Reg::read) this register and get [`dinr23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR23)

For information about available fields see [`mod@dinr23`] module*/
pub type DINR23 = crate::Reg<dinr23::DINR23rs>;
///MDIOS input data register 23
pub mod dinr23;
/**DINR24 (r) register accessor: MDIOS input data register 24

You can [`read`](crate::Reg::read) this register and get [`dinr24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR24)

For information about available fields see [`mod@dinr24`] module*/
pub type DINR24 = crate::Reg<dinr24::DINR24rs>;
///MDIOS input data register 24
pub mod dinr24;
/**DINR25 (r) register accessor: MDIOS input data register 25

You can [`read`](crate::Reg::read) this register and get [`dinr25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR25)

For information about available fields see [`mod@dinr25`] module*/
pub type DINR25 = crate::Reg<dinr25::DINR25rs>;
///MDIOS input data register 25
pub mod dinr25;
/**DINR26 (r) register accessor: MDIOS input data register 26

You can [`read`](crate::Reg::read) this register and get [`dinr26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR26)

For information about available fields see [`mod@dinr26`] module*/
pub type DINR26 = crate::Reg<dinr26::DINR26rs>;
///MDIOS input data register 26
pub mod dinr26;
/**DINR27 (r) register accessor: MDIOS input data register 27

You can [`read`](crate::Reg::read) this register and get [`dinr27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR27)

For information about available fields see [`mod@dinr27`] module*/
pub type DINR27 = crate::Reg<dinr27::DINR27rs>;
///MDIOS input data register 27
pub mod dinr27;
/**DINR28 (r) register accessor: MDIOS input data register 28

You can [`read`](crate::Reg::read) this register and get [`dinr28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR28)

For information about available fields see [`mod@dinr28`] module*/
pub type DINR28 = crate::Reg<dinr28::DINR28rs>;
///MDIOS input data register 28
pub mod dinr28;
/**DINR29 (r) register accessor: MDIOS input data register 29

You can [`read`](crate::Reg::read) this register and get [`dinr29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR29)

For information about available fields see [`mod@dinr29`] module*/
pub type DINR29 = crate::Reg<dinr29::DINR29rs>;
///MDIOS input data register 29
pub mod dinr29;
/**DINR30 (r) register accessor: MDIOS input data register 30

You can [`read`](crate::Reg::read) this register and get [`dinr30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR30)

For information about available fields see [`mod@dinr30`] module*/
pub type DINR30 = crate::Reg<dinr30::DINR30rs>;
///MDIOS input data register 30
pub mod dinr30;
/**DINR31 (r) register accessor: MDIOS input data register 31

You can [`read`](crate::Reg::read) this register and get [`dinr31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DINR31)

For information about available fields see [`mod@dinr31`] module*/
pub type DINR31 = crate::Reg<dinr31::DINR31rs>;
///MDIOS input data register 31
pub mod dinr31;
/**DOUTR0 (rw) register accessor: MDIOS output data register 0

You can [`read`](crate::Reg::read) this register and get [`doutr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR0)

For information about available fields see [`mod@doutr0`] module*/
pub type DOUTR0 = crate::Reg<doutr0::DOUTR0rs>;
///MDIOS output data register 0
pub mod doutr0;
/**DOUTR1 (rw) register accessor: MDIOS output data register 1

You can [`read`](crate::Reg::read) this register and get [`doutr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR1)

For information about available fields see [`mod@doutr1`] module*/
pub type DOUTR1 = crate::Reg<doutr1::DOUTR1rs>;
///MDIOS output data register 1
pub mod doutr1;
/**DOUTR2 (rw) register accessor: MDIOS output data register 2

You can [`read`](crate::Reg::read) this register and get [`doutr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR2)

For information about available fields see [`mod@doutr2`] module*/
pub type DOUTR2 = crate::Reg<doutr2::DOUTR2rs>;
///MDIOS output data register 2
pub mod doutr2;
/**DOUTR3 (rw) register accessor: MDIOS output data register 3

You can [`read`](crate::Reg::read) this register and get [`doutr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR3)

For information about available fields see [`mod@doutr3`] module*/
pub type DOUTR3 = crate::Reg<doutr3::DOUTR3rs>;
///MDIOS output data register 3
pub mod doutr3;
/**DOUTR4 (rw) register accessor: MDIOS output data register 4

You can [`read`](crate::Reg::read) this register and get [`doutr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR4)

For information about available fields see [`mod@doutr4`] module*/
pub type DOUTR4 = crate::Reg<doutr4::DOUTR4rs>;
///MDIOS output data register 4
pub mod doutr4;
/**DOUTR5 (rw) register accessor: MDIOS output data register 5

You can [`read`](crate::Reg::read) this register and get [`doutr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR5)

For information about available fields see [`mod@doutr5`] module*/
pub type DOUTR5 = crate::Reg<doutr5::DOUTR5rs>;
///MDIOS output data register 5
pub mod doutr5;
/**DOUTR6 (rw) register accessor: MDIOS output data register 6

You can [`read`](crate::Reg::read) this register and get [`doutr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR6)

For information about available fields see [`mod@doutr6`] module*/
pub type DOUTR6 = crate::Reg<doutr6::DOUTR6rs>;
///MDIOS output data register 6
pub mod doutr6;
/**DOUTR7 (rw) register accessor: MDIOS output data register 7

You can [`read`](crate::Reg::read) this register and get [`doutr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR7)

For information about available fields see [`mod@doutr7`] module*/
pub type DOUTR7 = crate::Reg<doutr7::DOUTR7rs>;
///MDIOS output data register 7
pub mod doutr7;
/**DOUTR8 (rw) register accessor: MDIOS output data register 8

You can [`read`](crate::Reg::read) this register and get [`doutr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR8)

For information about available fields see [`mod@doutr8`] module*/
pub type DOUTR8 = crate::Reg<doutr8::DOUTR8rs>;
///MDIOS output data register 8
pub mod doutr8;
/**DOUTR9 (rw) register accessor: MDIOS output data register 9

You can [`read`](crate::Reg::read) this register and get [`doutr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR9)

For information about available fields see [`mod@doutr9`] module*/
pub type DOUTR9 = crate::Reg<doutr9::DOUTR9rs>;
///MDIOS output data register 9
pub mod doutr9;
/**DOUTR10 (rw) register accessor: MDIOS output data register 10

You can [`read`](crate::Reg::read) this register and get [`doutr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR10)

For information about available fields see [`mod@doutr10`] module*/
pub type DOUTR10 = crate::Reg<doutr10::DOUTR10rs>;
///MDIOS output data register 10
pub mod doutr10;
/**DOUTR11 (rw) register accessor: MDIOS output data register 11

You can [`read`](crate::Reg::read) this register and get [`doutr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR11)

For information about available fields see [`mod@doutr11`] module*/
pub type DOUTR11 = crate::Reg<doutr11::DOUTR11rs>;
///MDIOS output data register 11
pub mod doutr11;
/**DOUTR12 (rw) register accessor: MDIOS output data register 12

You can [`read`](crate::Reg::read) this register and get [`doutr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR12)

For information about available fields see [`mod@doutr12`] module*/
pub type DOUTR12 = crate::Reg<doutr12::DOUTR12rs>;
///MDIOS output data register 12
pub mod doutr12;
/**DOUTR13 (rw) register accessor: MDIOS output data register 13

You can [`read`](crate::Reg::read) this register and get [`doutr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR13)

For information about available fields see [`mod@doutr13`] module*/
pub type DOUTR13 = crate::Reg<doutr13::DOUTR13rs>;
///MDIOS output data register 13
pub mod doutr13;
/**DOUTR14 (rw) register accessor: MDIOS output data register 14

You can [`read`](crate::Reg::read) this register and get [`doutr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR14)

For information about available fields see [`mod@doutr14`] module*/
pub type DOUTR14 = crate::Reg<doutr14::DOUTR14rs>;
///MDIOS output data register 14
pub mod doutr14;
/**DOUTR15 (rw) register accessor: MDIOS output data register 15

You can [`read`](crate::Reg::read) this register and get [`doutr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR15)

For information about available fields see [`mod@doutr15`] module*/
pub type DOUTR15 = crate::Reg<doutr15::DOUTR15rs>;
///MDIOS output data register 15
pub mod doutr15;
/**DOUTR16 (rw) register accessor: MDIOS output data register 16

You can [`read`](crate::Reg::read) this register and get [`doutr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR16)

For information about available fields see [`mod@doutr16`] module*/
pub type DOUTR16 = crate::Reg<doutr16::DOUTR16rs>;
///MDIOS output data register 16
pub mod doutr16;
/**DOUTR17 (rw) register accessor: MDIOS output data register 17

You can [`read`](crate::Reg::read) this register and get [`doutr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR17)

For information about available fields see [`mod@doutr17`] module*/
pub type DOUTR17 = crate::Reg<doutr17::DOUTR17rs>;
///MDIOS output data register 17
pub mod doutr17;
/**DOUTR18 (rw) register accessor: MDIOS output data register 18

You can [`read`](crate::Reg::read) this register and get [`doutr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR18)

For information about available fields see [`mod@doutr18`] module*/
pub type DOUTR18 = crate::Reg<doutr18::DOUTR18rs>;
///MDIOS output data register 18
pub mod doutr18;
/**DOUTR19 (rw) register accessor: MDIOS output data register 19

You can [`read`](crate::Reg::read) this register and get [`doutr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR19)

For information about available fields see [`mod@doutr19`] module*/
pub type DOUTR19 = crate::Reg<doutr19::DOUTR19rs>;
///MDIOS output data register 19
pub mod doutr19;
/**DOUTR20 (rw) register accessor: MDIOS output data register 20

You can [`read`](crate::Reg::read) this register and get [`doutr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR20)

For information about available fields see [`mod@doutr20`] module*/
pub type DOUTR20 = crate::Reg<doutr20::DOUTR20rs>;
///MDIOS output data register 20
pub mod doutr20;
/**DOUTR21 (rw) register accessor: MDIOS output data register 21

You can [`read`](crate::Reg::read) this register and get [`doutr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR21)

For information about available fields see [`mod@doutr21`] module*/
pub type DOUTR21 = crate::Reg<doutr21::DOUTR21rs>;
///MDIOS output data register 21
pub mod doutr21;
/**DOUTR22 (rw) register accessor: MDIOS output data register 22

You can [`read`](crate::Reg::read) this register and get [`doutr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR22)

For information about available fields see [`mod@doutr22`] module*/
pub type DOUTR22 = crate::Reg<doutr22::DOUTR22rs>;
///MDIOS output data register 22
pub mod doutr22;
/**DOUTR23 (rw) register accessor: MDIOS output data register 23

You can [`read`](crate::Reg::read) this register and get [`doutr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR23)

For information about available fields see [`mod@doutr23`] module*/
pub type DOUTR23 = crate::Reg<doutr23::DOUTR23rs>;
///MDIOS output data register 23
pub mod doutr23;
/**DOUTR24 (rw) register accessor: MDIOS output data register 24

You can [`read`](crate::Reg::read) this register and get [`doutr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR24)

For information about available fields see [`mod@doutr24`] module*/
pub type DOUTR24 = crate::Reg<doutr24::DOUTR24rs>;
///MDIOS output data register 24
pub mod doutr24;
/**DOUTR25 (rw) register accessor: MDIOS output data register 25

You can [`read`](crate::Reg::read) this register and get [`doutr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR25)

For information about available fields see [`mod@doutr25`] module*/
pub type DOUTR25 = crate::Reg<doutr25::DOUTR25rs>;
///MDIOS output data register 25
pub mod doutr25;
/**DOUTR26 (rw) register accessor: MDIOS output data register 26

You can [`read`](crate::Reg::read) this register and get [`doutr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR26)

For information about available fields see [`mod@doutr26`] module*/
pub type DOUTR26 = crate::Reg<doutr26::DOUTR26rs>;
///MDIOS output data register 26
pub mod doutr26;
/**DOUTR27 (rw) register accessor: MDIOS output data register 27

You can [`read`](crate::Reg::read) this register and get [`doutr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR27)

For information about available fields see [`mod@doutr27`] module*/
pub type DOUTR27 = crate::Reg<doutr27::DOUTR27rs>;
///MDIOS output data register 27
pub mod doutr27;
/**DOUTR28 (rw) register accessor: MDIOS output data register 28

You can [`read`](crate::Reg::read) this register and get [`doutr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR28)

For information about available fields see [`mod@doutr28`] module*/
pub type DOUTR28 = crate::Reg<doutr28::DOUTR28rs>;
///MDIOS output data register 28
pub mod doutr28;
/**DOUTR29 (rw) register accessor: MDIOS output data register 29

You can [`read`](crate::Reg::read) this register and get [`doutr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR29)

For information about available fields see [`mod@doutr29`] module*/
pub type DOUTR29 = crate::Reg<doutr29::DOUTR29rs>;
///MDIOS output data register 29
pub mod doutr29;
/**DOUTR30 (rw) register accessor: MDIOS output data register 30

You can [`read`](crate::Reg::read) this register and get [`doutr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR30)

For information about available fields see [`mod@doutr30`] module*/
pub type DOUTR30 = crate::Reg<doutr30::DOUTR30rs>;
///MDIOS output data register 30
pub mod doutr30;
/**DOUTR31 (rw) register accessor: MDIOS output data register 31

You can [`read`](crate::Reg::read) this register and get [`doutr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDIOS:DOUTR31)

For information about available fields see [`mod@doutr31`] module*/
pub type DOUTR31 = crate::Reg<doutr31::DOUTR31rs>;
///MDIOS output data register 31
pub mod doutr31;
