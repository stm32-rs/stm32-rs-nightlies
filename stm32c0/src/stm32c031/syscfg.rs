#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cfgr1: CFGR1,
    _reserved1: [u8; 0x14],
    cfgr2: CFGR2,
    _reserved2: [u8; 0x20],
    cfgr3: CFGR3,
    _reserved3: [u8; 0x40],
    itline0: ITLINE0,
    _reserved4: [u8; 0x04],
    itline2: ITLINE2,
    itline3: ITLINE3,
    itline4: ITLINE4,
    itline5: ITLINE5,
    itline6: ITLINE6,
    itline7: ITLINE7,
    _reserved10: [u8; 0x04],
    itline9: ITLINE9,
    itline10: ITLINE10,
    itline11: ITLINE11,
    itline12: ITLINE12,
    itline13: ITLINE13,
    itline14: ITLINE14,
    _reserved16: [u8; 0x04],
    itline16: ITLINE16,
    _reserved17: [u8; 0x08],
    itline19: ITLINE19,
    _reserved18: [u8; 0x04],
    itline21: ITLINE21,
    itline22: ITLINE22,
    itline23: ITLINE23,
    _reserved21: [u8; 0x04],
    itline25: ITLINE25,
    _reserved22: [u8; 0x04],
    itline27: ITLINE27,
    itline28: ITLINE28,
}
impl RegisterBlock {
    ///0x00 - SYSCFG configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x18 - SYSCFG configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x3c - SYSCFG configuration register 3
    #[inline(always)]
    pub const fn cfgr3(&self) -> &CFGR3 {
        &self.cfgr3
    }
    ///0x80 - SYSCFG interrupt line 0 status register
    #[inline(always)]
    pub const fn itline0(&self) -> &ITLINE0 {
        &self.itline0
    }
    ///0x88 - SYSCFG interrupt line 2 status register
    #[inline(always)]
    pub const fn itline2(&self) -> &ITLINE2 {
        &self.itline2
    }
    ///0x8c - SYSCFG interrupt line 3 status register
    #[inline(always)]
    pub const fn itline3(&self) -> &ITLINE3 {
        &self.itline3
    }
    ///0x90 - SYSCFG interrupt line 4 status register
    #[inline(always)]
    pub const fn itline4(&self) -> &ITLINE4 {
        &self.itline4
    }
    ///0x94 - SYSCFG interrupt line 5 status register
    #[inline(always)]
    pub const fn itline5(&self) -> &ITLINE5 {
        &self.itline5
    }
    ///0x98 - SYSCFG interrupt line 6 status register
    #[inline(always)]
    pub const fn itline6(&self) -> &ITLINE6 {
        &self.itline6
    }
    ///0x9c - SYSCFG interrupt line 7 status register
    #[inline(always)]
    pub const fn itline7(&self) -> &ITLINE7 {
        &self.itline7
    }
    ///0xa4 - SYSCFG interrupt line 9 status register
    #[inline(always)]
    pub const fn itline9(&self) -> &ITLINE9 {
        &self.itline9
    }
    ///0xa8 - SYSCFG interrupt line 10 status register
    #[inline(always)]
    pub const fn itline10(&self) -> &ITLINE10 {
        &self.itline10
    }
    ///0xac - SYSCFG interrupt line 11 status register
    #[inline(always)]
    pub const fn itline11(&self) -> &ITLINE11 {
        &self.itline11
    }
    ///0xb0 - SYSCFG interrupt line 12 status register
    #[inline(always)]
    pub const fn itline12(&self) -> &ITLINE12 {
        &self.itline12
    }
    ///0xb4 - SYSCFG interrupt line 13 status register
    #[inline(always)]
    pub const fn itline13(&self) -> &ITLINE13 {
        &self.itline13
    }
    ///0xb8 - SYSCFG interrupt line 14 status register
    #[inline(always)]
    pub const fn itline14(&self) -> &ITLINE14 {
        &self.itline14
    }
    ///0xc0 - SYSCFG interrupt line 16 status register
    #[inline(always)]
    pub const fn itline16(&self) -> &ITLINE16 {
        &self.itline16
    }
    ///0xcc - SYSCFG interrupt line 19 status register
    #[inline(always)]
    pub const fn itline19(&self) -> &ITLINE19 {
        &self.itline19
    }
    ///0xd4 - SYSCFG interrupt line 21 status register
    #[inline(always)]
    pub const fn itline21(&self) -> &ITLINE21 {
        &self.itline21
    }
    ///0xd8 - SYSCFG interrupt line 22 status register
    #[inline(always)]
    pub const fn itline22(&self) -> &ITLINE22 {
        &self.itline22
    }
    ///0xdc - SYSCFG interrupt line 23 status register
    #[inline(always)]
    pub const fn itline23(&self) -> &ITLINE23 {
        &self.itline23
    }
    ///0xe4 - SYSCFG interrupt line 25 status register
    #[inline(always)]
    pub const fn itline25(&self) -> &ITLINE25 {
        &self.itline25
    }
    ///0xec - SYSCFG interrupt line 27 status register
    #[inline(always)]
    pub const fn itline27(&self) -> &ITLINE27 {
        &self.itline27
    }
    ///0xf0 - SYSCFG interrupt line 28 status register
    #[inline(always)]
    pub const fn itline28(&self) -> &ITLINE28 {
        &self.itline28
    }
}
/**CFGR1 (rw) register accessor: SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:CFGR1)

For information about available fields see [`mod@cfgr1`]
module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///SYSCFG configuration register 1
pub mod cfgr1;
/**CFGR2 (rw) register accessor: SYSCFG configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:CFGR2)

For information about available fields see [`mod@cfgr2`]
module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///SYSCFG configuration register 2
pub mod cfgr2;
/**CFGR3 (rw) register accessor: SYSCFG configuration register 3

You can [`read`](crate::Reg::read) this register and get [`cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:CFGR3)

For information about available fields see [`mod@cfgr3`]
module*/
pub type CFGR3 = crate::Reg<cfgr3::CFGR3rs>;
///SYSCFG configuration register 3
pub mod cfgr3;
/**ITLINE0 (r) register accessor: SYSCFG interrupt line 0 status register

You can [`read`](crate::Reg::read) this register and get [`itline0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE0)

For information about available fields see [`mod@itline0`]
module*/
pub type ITLINE0 = crate::Reg<itline0::ITLINE0rs>;
///SYSCFG interrupt line 0 status register
pub mod itline0;
/**ITLINE2 (r) register accessor: SYSCFG interrupt line 2 status register

You can [`read`](crate::Reg::read) this register and get [`itline2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE2)

For information about available fields see [`mod@itline2`]
module*/
pub type ITLINE2 = crate::Reg<itline2::ITLINE2rs>;
///SYSCFG interrupt line 2 status register
pub mod itline2;
/**ITLINE3 (r) register accessor: SYSCFG interrupt line 3 status register

You can [`read`](crate::Reg::read) this register and get [`itline3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE3)

For information about available fields see [`mod@itline3`]
module*/
pub type ITLINE3 = crate::Reg<itline3::ITLINE3rs>;
///SYSCFG interrupt line 3 status register
pub mod itline3;
/**ITLINE4 (r) register accessor: SYSCFG interrupt line 4 status register

You can [`read`](crate::Reg::read) this register and get [`itline4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE4)

For information about available fields see [`mod@itline4`]
module*/
pub type ITLINE4 = crate::Reg<itline4::ITLINE4rs>;
///SYSCFG interrupt line 4 status register
pub mod itline4;
/**ITLINE5 (r) register accessor: SYSCFG interrupt line 5 status register

You can [`read`](crate::Reg::read) this register and get [`itline5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE5)

For information about available fields see [`mod@itline5`]
module*/
pub type ITLINE5 = crate::Reg<itline5::ITLINE5rs>;
///SYSCFG interrupt line 5 status register
pub mod itline5;
/**ITLINE6 (r) register accessor: SYSCFG interrupt line 6 status register

You can [`read`](crate::Reg::read) this register and get [`itline6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE6)

For information about available fields see [`mod@itline6`]
module*/
pub type ITLINE6 = crate::Reg<itline6::ITLINE6rs>;
///SYSCFG interrupt line 6 status register
pub mod itline6;
/**ITLINE7 (r) register accessor: SYSCFG interrupt line 7 status register

You can [`read`](crate::Reg::read) this register and get [`itline7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE7)

For information about available fields see [`mod@itline7`]
module*/
pub type ITLINE7 = crate::Reg<itline7::ITLINE7rs>;
///SYSCFG interrupt line 7 status register
pub mod itline7;
/**ITLINE9 (r) register accessor: SYSCFG interrupt line 9 status register

You can [`read`](crate::Reg::read) this register and get [`itline9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE9)

For information about available fields see [`mod@itline9`]
module*/
pub type ITLINE9 = crate::Reg<itline9::ITLINE9rs>;
///SYSCFG interrupt line 9 status register
pub mod itline9;
/**ITLINE10 (r) register accessor: SYSCFG interrupt line 10 status register

You can [`read`](crate::Reg::read) this register and get [`itline10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE10)

For information about available fields see [`mod@itline10`]
module*/
pub type ITLINE10 = crate::Reg<itline10::ITLINE10rs>;
///SYSCFG interrupt line 10 status register
pub mod itline10;
/**ITLINE11 (r) register accessor: SYSCFG interrupt line 11 status register

You can [`read`](crate::Reg::read) this register and get [`itline11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE11)

For information about available fields see [`mod@itline11`]
module*/
pub type ITLINE11 = crate::Reg<itline11::ITLINE11rs>;
///SYSCFG interrupt line 11 status register
pub mod itline11;
/**ITLINE12 (r) register accessor: SYSCFG interrupt line 12 status register

You can [`read`](crate::Reg::read) this register and get [`itline12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE12)

For information about available fields see [`mod@itline12`]
module*/
pub type ITLINE12 = crate::Reg<itline12::ITLINE12rs>;
///SYSCFG interrupt line 12 status register
pub mod itline12;
/**ITLINE13 (r) register accessor: SYSCFG interrupt line 13 status register

You can [`read`](crate::Reg::read) this register and get [`itline13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE13)

For information about available fields see [`mod@itline13`]
module*/
pub type ITLINE13 = crate::Reg<itline13::ITLINE13rs>;
///SYSCFG interrupt line 13 status register
pub mod itline13;
/**ITLINE14 (r) register accessor: SYSCFG interrupt line 14 status register

You can [`read`](crate::Reg::read) this register and get [`itline14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE14)

For information about available fields see [`mod@itline14`]
module*/
pub type ITLINE14 = crate::Reg<itline14::ITLINE14rs>;
///SYSCFG interrupt line 14 status register
pub mod itline14;
/**ITLINE16 (r) register accessor: SYSCFG interrupt line 16 status register

You can [`read`](crate::Reg::read) this register and get [`itline16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE16)

For information about available fields see [`mod@itline16`]
module*/
pub type ITLINE16 = crate::Reg<itline16::ITLINE16rs>;
///SYSCFG interrupt line 16 status register
pub mod itline16;
/**ITLINE19 (r) register accessor: SYSCFG interrupt line 19 status register

You can [`read`](crate::Reg::read) this register and get [`itline19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE19)

For information about available fields see [`mod@itline19`]
module*/
pub type ITLINE19 = crate::Reg<itline19::ITLINE19rs>;
///SYSCFG interrupt line 19 status register
pub mod itline19;
/**ITLINE21 (r) register accessor: SYSCFG interrupt line 21 status register

You can [`read`](crate::Reg::read) this register and get [`itline21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE21)

For information about available fields see [`mod@itline21`]
module*/
pub type ITLINE21 = crate::Reg<itline21::ITLINE21rs>;
///SYSCFG interrupt line 21 status register
pub mod itline21;
/**ITLINE22 (r) register accessor: SYSCFG interrupt line 22 status register

You can [`read`](crate::Reg::read) this register and get [`itline22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE22)

For information about available fields see [`mod@itline22`]
module*/
pub type ITLINE22 = crate::Reg<itline22::ITLINE22rs>;
///SYSCFG interrupt line 22 status register
pub mod itline22;
/**ITLINE23 (r) register accessor: SYSCFG interrupt line 23 status register

You can [`read`](crate::Reg::read) this register and get [`itline23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE23)

For information about available fields see [`mod@itline23`]
module*/
pub type ITLINE23 = crate::Reg<itline23::ITLINE23rs>;
///SYSCFG interrupt line 23 status register
pub mod itline23;
/**ITLINE25 (r) register accessor: SYSCFG interrupt line 25 status register

You can [`read`](crate::Reg::read) this register and get [`itline25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE25)

For information about available fields see [`mod@itline25`]
module*/
pub type ITLINE25 = crate::Reg<itline25::ITLINE25rs>;
///SYSCFG interrupt line 25 status register
pub mod itline25;
/**ITLINE27 (r) register accessor: SYSCFG interrupt line 27 status register

You can [`read`](crate::Reg::read) this register and get [`itline27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE27)

For information about available fields see [`mod@itline27`]
module*/
pub type ITLINE27 = crate::Reg<itline27::ITLINE27rs>;
///SYSCFG interrupt line 27 status register
pub mod itline27;
/**ITLINE28 (r) register accessor: SYSCFG interrupt line 28 status register

You can [`read`](crate::Reg::read) this register and get [`itline28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:ITLINE28)

For information about available fields see [`mod@itline28`]
module*/
pub type ITLINE28 = crate::Reg<itline28::ITLINE28rs>;
///SYSCFG interrupt line 28 status register
pub mod itline28;
