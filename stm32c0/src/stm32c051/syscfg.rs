#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    syscfg_cfgr1: SYSCFG_CFGR1,
    _reserved1: [u8; 0x14],
    syscfg_cfgr2: SYSCFG_CFGR2,
    _reserved2: [u8; 0x20],
    syscfg_cfgr3: SYSCFG_CFGR3,
    _reserved3: [u8; 0x40],
    syscfg_itline0: SYSCFG_ITLINE0,
    syscfg_itline1: SYSCFG_ITLINE1,
    syscfg_itline2: SYSCFG_ITLINE2,
    syscfg_itline3: SYSCFG_ITLINE3,
    syscfg_itline4: SYSCFG_ITLINE4,
    syscfg_itline5: SYSCFG_ITLINE5,
    syscfg_itline6: SYSCFG_ITLINE6,
    syscfg_itline7: SYSCFG_ITLINE7,
    syscfg_itline8: SYSCFG_ITLINE8,
    syscfg_itline9: SYSCFG_ITLINE9,
    syscfg_itline10: SYSCFG_ITLINE10,
    syscfg_itline11: SYSCFG_ITLINE11,
    syscfg_itline12: SYSCFG_ITLINE12,
    syscfg_itline13: SYSCFG_ITLINE13,
    syscfg_itline14: SYSCFG_ITLINE14,
    syscfg_itline15: SYSCFG_ITLINE15,
    syscfg_itline16: SYSCFG_ITLINE16,
    _reserved20: [u8; 0x08],
    syscfg_itline19: SYSCFG_ITLINE19,
    _reserved21: [u8; 0x04],
    syscfg_itline21: SYSCFG_ITLINE21,
    syscfg_itline22: SYSCFG_ITLINE22,
    syscfg_itline23: SYSCFG_ITLINE23,
    syscfg_itline24: SYSCFG_ITLINE24,
    syscfg_itline25: SYSCFG_ITLINE25,
    syscfg_itline26: SYSCFG_ITLINE26,
    syscfg_itline27: SYSCFG_ITLINE27,
    syscfg_itline28: SYSCFG_ITLINE28,
}
impl RegisterBlock {
    ///0x00 - SYSCFG configuration register 1
    #[inline(always)]
    pub const fn syscfg_cfgr1(&self) -> &SYSCFG_CFGR1 {
        &self.syscfg_cfgr1
    }
    ///0x18 - SYSCFG configuration register 2
    #[inline(always)]
    pub const fn syscfg_cfgr2(&self) -> &SYSCFG_CFGR2 {
        &self.syscfg_cfgr2
    }
    ///0x3c - SYSCFG configuration register 3
    #[inline(always)]
    pub const fn syscfg_cfgr3(&self) -> &SYSCFG_CFGR3 {
        &self.syscfg_cfgr3
    }
    ///0x80 - SYSCFG interrupt line 0 status register
    #[inline(always)]
    pub const fn syscfg_itline0(&self) -> &SYSCFG_ITLINE0 {
        &self.syscfg_itline0
    }
    ///0x84 - SYSCFG interrupt line 1 status register
    #[inline(always)]
    pub const fn syscfg_itline1(&self) -> &SYSCFG_ITLINE1 {
        &self.syscfg_itline1
    }
    ///0x88 - SYSCFG interrupt line 2 status register
    #[inline(always)]
    pub const fn syscfg_itline2(&self) -> &SYSCFG_ITLINE2 {
        &self.syscfg_itline2
    }
    ///0x8c - SYSCFG interrupt line 3 status register
    #[inline(always)]
    pub const fn syscfg_itline3(&self) -> &SYSCFG_ITLINE3 {
        &self.syscfg_itline3
    }
    ///0x90 - SYSCFG interrupt line 4 status register
    #[inline(always)]
    pub const fn syscfg_itline4(&self) -> &SYSCFG_ITLINE4 {
        &self.syscfg_itline4
    }
    ///0x94 - SYSCFG interrupt line 5 status register
    #[inline(always)]
    pub const fn syscfg_itline5(&self) -> &SYSCFG_ITLINE5 {
        &self.syscfg_itline5
    }
    ///0x98 - SYSCFG interrupt line 6 status register
    #[inline(always)]
    pub const fn syscfg_itline6(&self) -> &SYSCFG_ITLINE6 {
        &self.syscfg_itline6
    }
    ///0x9c - SYSCFG interrupt line 7 status register
    #[inline(always)]
    pub const fn syscfg_itline7(&self) -> &SYSCFG_ITLINE7 {
        &self.syscfg_itline7
    }
    ///0xa0 - SYSCFG interrupt line 8 status register
    #[inline(always)]
    pub const fn syscfg_itline8(&self) -> &SYSCFG_ITLINE8 {
        &self.syscfg_itline8
    }
    ///0xa4 - SYSCFG interrupt line 9 status register
    #[inline(always)]
    pub const fn syscfg_itline9(&self) -> &SYSCFG_ITLINE9 {
        &self.syscfg_itline9
    }
    ///0xa8 - SYSCFG interrupt line 10 status register
    #[inline(always)]
    pub const fn syscfg_itline10(&self) -> &SYSCFG_ITLINE10 {
        &self.syscfg_itline10
    }
    ///0xac - SYSCFG interrupt line 11 status register
    #[inline(always)]
    pub const fn syscfg_itline11(&self) -> &SYSCFG_ITLINE11 {
        &self.syscfg_itline11
    }
    ///0xb0 - SYSCFG interrupt line 12 status register
    #[inline(always)]
    pub const fn syscfg_itline12(&self) -> &SYSCFG_ITLINE12 {
        &self.syscfg_itline12
    }
    ///0xb4 - SYSCFG interrupt line 13 status register
    #[inline(always)]
    pub const fn syscfg_itline13(&self) -> &SYSCFG_ITLINE13 {
        &self.syscfg_itline13
    }
    ///0xb8 - SYSCFG interrupt line 14 status register
    #[inline(always)]
    pub const fn syscfg_itline14(&self) -> &SYSCFG_ITLINE14 {
        &self.syscfg_itline14
    }
    ///0xbc - SYSCFG interrupt line 15 status register
    #[inline(always)]
    pub const fn syscfg_itline15(&self) -> &SYSCFG_ITLINE15 {
        &self.syscfg_itline15
    }
    ///0xc0 - SYSCFG interrupt line 16 status register
    #[inline(always)]
    pub const fn syscfg_itline16(&self) -> &SYSCFG_ITLINE16 {
        &self.syscfg_itline16
    }
    ///0xcc - SYSCFG interrupt line 19 status register
    #[inline(always)]
    pub const fn syscfg_itline19(&self) -> &SYSCFG_ITLINE19 {
        &self.syscfg_itline19
    }
    ///0xd4 - SYSCFG interrupt line 21 status register
    #[inline(always)]
    pub const fn syscfg_itline21(&self) -> &SYSCFG_ITLINE21 {
        &self.syscfg_itline21
    }
    ///0xd8 - SYSCFG interrupt line 22 status register
    #[inline(always)]
    pub const fn syscfg_itline22(&self) -> &SYSCFG_ITLINE22 {
        &self.syscfg_itline22
    }
    ///0xdc - SYSCFG interrupt line 23 status register
    #[inline(always)]
    pub const fn syscfg_itline23(&self) -> &SYSCFG_ITLINE23 {
        &self.syscfg_itline23
    }
    ///0xe0 - SYSCFG interrupt line 24 status register
    #[inline(always)]
    pub const fn syscfg_itline24(&self) -> &SYSCFG_ITLINE24 {
        &self.syscfg_itline24
    }
    ///0xe4 - SYSCFG interrupt line 25 status register
    #[inline(always)]
    pub const fn syscfg_itline25(&self) -> &SYSCFG_ITLINE25 {
        &self.syscfg_itline25
    }
    ///0xe8 - SYSCFG interrupt line 26 status register
    #[inline(always)]
    pub const fn syscfg_itline26(&self) -> &SYSCFG_ITLINE26 {
        &self.syscfg_itline26
    }
    ///0xec - SYSCFG interrupt line 27 status register
    #[inline(always)]
    pub const fn syscfg_itline27(&self) -> &SYSCFG_ITLINE27 {
        &self.syscfg_itline27
    }
    ///0xf0 - SYSCFG interrupt line 28 status register
    #[inline(always)]
    pub const fn syscfg_itline28(&self) -> &SYSCFG_ITLINE28 {
        &self.syscfg_itline28
    }
}
/**SYSCFG_CFGR1 (rw) register accessor: SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_CFGR1)

For information about available fields see [`mod@syscfg_cfgr1`] module*/
pub type SYSCFG_CFGR1 = crate::Reg<syscfg_cfgr1::SYSCFG_CFGR1rs>;
///SYSCFG configuration register 1
pub mod syscfg_cfgr1;
/**SYSCFG_CFGR2 (rw) register accessor: SYSCFG configuration register 2

You can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_CFGR2)

For information about available fields see [`mod@syscfg_cfgr2`] module*/
pub type SYSCFG_CFGR2 = crate::Reg<syscfg_cfgr2::SYSCFG_CFGR2rs>;
///SYSCFG configuration register 2
pub mod syscfg_cfgr2;
/**SYSCFG_CFGR3 (rw) register accessor: SYSCFG configuration register 3

You can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_CFGR3)

For information about available fields see [`mod@syscfg_cfgr3`] module*/
pub type SYSCFG_CFGR3 = crate::Reg<syscfg_cfgr3::SYSCFG_CFGR3rs>;
///SYSCFG configuration register 3
pub mod syscfg_cfgr3;
/**SYSCFG_ITLINE0 (r) register accessor: SYSCFG interrupt line 0 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE0)

For information about available fields see [`mod@syscfg_itline0`] module*/
pub type SYSCFG_ITLINE0 = crate::Reg<syscfg_itline0::SYSCFG_ITLINE0rs>;
///SYSCFG interrupt line 0 status register
pub mod syscfg_itline0;
/**SYSCFG_ITLINE1 (r) register accessor: SYSCFG interrupt line 1 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE1)

For information about available fields see [`mod@syscfg_itline1`] module*/
pub type SYSCFG_ITLINE1 = crate::Reg<syscfg_itline1::SYSCFG_ITLINE1rs>;
///SYSCFG interrupt line 1 status register
pub mod syscfg_itline1;
/**SYSCFG_ITLINE2 (r) register accessor: SYSCFG interrupt line 2 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE2)

For information about available fields see [`mod@syscfg_itline2`] module*/
pub type SYSCFG_ITLINE2 = crate::Reg<syscfg_itline2::SYSCFG_ITLINE2rs>;
///SYSCFG interrupt line 2 status register
pub mod syscfg_itline2;
/**SYSCFG_ITLINE3 (r) register accessor: SYSCFG interrupt line 3 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE3)

For information about available fields see [`mod@syscfg_itline3`] module*/
pub type SYSCFG_ITLINE3 = crate::Reg<syscfg_itline3::SYSCFG_ITLINE3rs>;
///SYSCFG interrupt line 3 status register
pub mod syscfg_itline3;
/**SYSCFG_ITLINE4 (r) register accessor: SYSCFG interrupt line 4 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE4)

For information about available fields see [`mod@syscfg_itline4`] module*/
pub type SYSCFG_ITLINE4 = crate::Reg<syscfg_itline4::SYSCFG_ITLINE4rs>;
///SYSCFG interrupt line 4 status register
pub mod syscfg_itline4;
/**SYSCFG_ITLINE5 (r) register accessor: SYSCFG interrupt line 5 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE5)

For information about available fields see [`mod@syscfg_itline5`] module*/
pub type SYSCFG_ITLINE5 = crate::Reg<syscfg_itline5::SYSCFG_ITLINE5rs>;
///SYSCFG interrupt line 5 status register
pub mod syscfg_itline5;
/**SYSCFG_ITLINE6 (r) register accessor: SYSCFG interrupt line 6 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE6)

For information about available fields see [`mod@syscfg_itline6`] module*/
pub type SYSCFG_ITLINE6 = crate::Reg<syscfg_itline6::SYSCFG_ITLINE6rs>;
///SYSCFG interrupt line 6 status register
pub mod syscfg_itline6;
/**SYSCFG_ITLINE7 (r) register accessor: SYSCFG interrupt line 7 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE7)

For information about available fields see [`mod@syscfg_itline7`] module*/
pub type SYSCFG_ITLINE7 = crate::Reg<syscfg_itline7::SYSCFG_ITLINE7rs>;
///SYSCFG interrupt line 7 status register
pub mod syscfg_itline7;
/**SYSCFG_ITLINE8 (r) register accessor: SYSCFG interrupt line 8 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE8)

For information about available fields see [`mod@syscfg_itline8`] module*/
pub type SYSCFG_ITLINE8 = crate::Reg<syscfg_itline8::SYSCFG_ITLINE8rs>;
///SYSCFG interrupt line 8 status register
pub mod syscfg_itline8;
/**SYSCFG_ITLINE9 (r) register accessor: SYSCFG interrupt line 9 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE9)

For information about available fields see [`mod@syscfg_itline9`] module*/
pub type SYSCFG_ITLINE9 = crate::Reg<syscfg_itline9::SYSCFG_ITLINE9rs>;
///SYSCFG interrupt line 9 status register
pub mod syscfg_itline9;
/**SYSCFG_ITLINE10 (r) register accessor: SYSCFG interrupt line 10 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE10)

For information about available fields see [`mod@syscfg_itline10`] module*/
pub type SYSCFG_ITLINE10 = crate::Reg<syscfg_itline10::SYSCFG_ITLINE10rs>;
///SYSCFG interrupt line 10 status register
pub mod syscfg_itline10;
/**SYSCFG_ITLINE11 (r) register accessor: SYSCFG interrupt line 11 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE11)

For information about available fields see [`mod@syscfg_itline11`] module*/
pub type SYSCFG_ITLINE11 = crate::Reg<syscfg_itline11::SYSCFG_ITLINE11rs>;
///SYSCFG interrupt line 11 status register
pub mod syscfg_itline11;
/**SYSCFG_ITLINE12 (r) register accessor: SYSCFG interrupt line 12 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE12)

For information about available fields see [`mod@syscfg_itline12`] module*/
pub type SYSCFG_ITLINE12 = crate::Reg<syscfg_itline12::SYSCFG_ITLINE12rs>;
///SYSCFG interrupt line 12 status register
pub mod syscfg_itline12;
/**SYSCFG_ITLINE13 (r) register accessor: SYSCFG interrupt line 13 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE13)

For information about available fields see [`mod@syscfg_itline13`] module*/
pub type SYSCFG_ITLINE13 = crate::Reg<syscfg_itline13::SYSCFG_ITLINE13rs>;
///SYSCFG interrupt line 13 status register
pub mod syscfg_itline13;
/**SYSCFG_ITLINE14 (r) register accessor: SYSCFG interrupt line 14 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE14)

For information about available fields see [`mod@syscfg_itline14`] module*/
pub type SYSCFG_ITLINE14 = crate::Reg<syscfg_itline14::SYSCFG_ITLINE14rs>;
///SYSCFG interrupt line 14 status register
pub mod syscfg_itline14;
/**SYSCFG_ITLINE15 (r) register accessor: SYSCFG interrupt line 15 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE15)

For information about available fields see [`mod@syscfg_itline15`] module*/
pub type SYSCFG_ITLINE15 = crate::Reg<syscfg_itline15::SYSCFG_ITLINE15rs>;
///SYSCFG interrupt line 15 status register
pub mod syscfg_itline15;
/**SYSCFG_ITLINE16 (r) register accessor: SYSCFG interrupt line 16 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE16)

For information about available fields see [`mod@syscfg_itline16`] module*/
pub type SYSCFG_ITLINE16 = crate::Reg<syscfg_itline16::SYSCFG_ITLINE16rs>;
///SYSCFG interrupt line 16 status register
pub mod syscfg_itline16;
/**SYSCFG_ITLINE19 (r) register accessor: SYSCFG interrupt line 19 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE19)

For information about available fields see [`mod@syscfg_itline19`] module*/
pub type SYSCFG_ITLINE19 = crate::Reg<syscfg_itline19::SYSCFG_ITLINE19rs>;
///SYSCFG interrupt line 19 status register
pub mod syscfg_itline19;
/**SYSCFG_ITLINE21 (r) register accessor: SYSCFG interrupt line 21 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE21)

For information about available fields see [`mod@syscfg_itline21`] module*/
pub type SYSCFG_ITLINE21 = crate::Reg<syscfg_itline21::SYSCFG_ITLINE21rs>;
///SYSCFG interrupt line 21 status register
pub mod syscfg_itline21;
/**SYSCFG_ITLINE22 (r) register accessor: SYSCFG interrupt line 22 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE22)

For information about available fields see [`mod@syscfg_itline22`] module*/
pub type SYSCFG_ITLINE22 = crate::Reg<syscfg_itline22::SYSCFG_ITLINE22rs>;
///SYSCFG interrupt line 22 status register
pub mod syscfg_itline22;
/**SYSCFG_ITLINE23 (r) register accessor: SYSCFG interrupt line 23 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE23)

For information about available fields see [`mod@syscfg_itline23`] module*/
pub type SYSCFG_ITLINE23 = crate::Reg<syscfg_itline23::SYSCFG_ITLINE23rs>;
///SYSCFG interrupt line 23 status register
pub mod syscfg_itline23;
/**SYSCFG_ITLINE24 (r) register accessor: SYSCFG interrupt line 24 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE24)

For information about available fields see [`mod@syscfg_itline24`] module*/
pub type SYSCFG_ITLINE24 = crate::Reg<syscfg_itline24::SYSCFG_ITLINE24rs>;
///SYSCFG interrupt line 24 status register
pub mod syscfg_itline24;
/**SYSCFG_ITLINE25 (r) register accessor: SYSCFG interrupt line 25 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE25)

For information about available fields see [`mod@syscfg_itline25`] module*/
pub type SYSCFG_ITLINE25 = crate::Reg<syscfg_itline25::SYSCFG_ITLINE25rs>;
///SYSCFG interrupt line 25 status register
pub mod syscfg_itline25;
/**SYSCFG_ITLINE26 (r) register accessor: SYSCFG interrupt line 26 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE26)

For information about available fields see [`mod@syscfg_itline26`] module*/
pub type SYSCFG_ITLINE26 = crate::Reg<syscfg_itline26::SYSCFG_ITLINE26rs>;
///SYSCFG interrupt line 26 status register
pub mod syscfg_itline26;
/**SYSCFG_ITLINE27 (r) register accessor: SYSCFG interrupt line 27 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE27)

For information about available fields see [`mod@syscfg_itline27`] module*/
pub type SYSCFG_ITLINE27 = crate::Reg<syscfg_itline27::SYSCFG_ITLINE27rs>;
///SYSCFG interrupt line 27 status register
pub mod syscfg_itline27;
/**SYSCFG_ITLINE28 (r) register accessor: SYSCFG interrupt line 28 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_ITLINE28)

For information about available fields see [`mod@syscfg_itline28`] module*/
pub type SYSCFG_ITLINE28 = crate::Reg<syscfg_itline28::SYSCFG_ITLINE28rs>;
///SYSCFG interrupt line 28 status register
pub mod syscfg_itline28;
