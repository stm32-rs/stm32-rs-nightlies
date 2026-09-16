#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cfgr1: CFGR1,
    _reserved1: [u8; 0x14],
    cfgr2: CFGR2,
    _reserved2: [u8; 0x64],
    itline0: ITLINE0,
    _reserved3: [u8; 0x04],
    itline2: ITLINE2,
    itline3: ITLINE3,
    itline4: ITLINE4,
    itline5: ITLINE5,
    itline6: ITLINE6,
    itline7: ITLINE7,
    itline8: ITLINE8,
    itline9: ITLINE9,
    itline10: ITLINE10,
    itline11: ITLINE11,
    itline12: ITLINE12,
    itline13: ITLINE13,
    itline14: ITLINE14,
    itline15: ITLINE15,
    itline16: ITLINE16,
    itline17: ITLINE17,
    itline18: ITLINE18,
    itline19: ITLINE19,
    itline20: ITLINE20,
    itline21: ITLINE21,
    itline22: ITLINE22,
    itline23: ITLINE23,
    itline24: ITLINE24,
    itline25: ITLINE25,
    itline26: ITLINE26,
    itline27: ITLINE27,
    itline28: ITLINE28,
    itline29: ITLINE29,
    itline30: ITLINE30,
    itline31: ITLINE31,
}
impl RegisterBlock {
    ///0x00 - SYSCFG configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x18 - SYSCFG configuration register 1
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x80 - interrupt line 0 status register
    #[inline(always)]
    pub const fn itline0(&self) -> &ITLINE0 {
        &self.itline0
    }
    ///0x88 - interrupt line 2 status register
    #[inline(always)]
    pub const fn itline2(&self) -> &ITLINE2 {
        &self.itline2
    }
    ///0x8c - interrupt line 3 status register
    #[inline(always)]
    pub const fn itline3(&self) -> &ITLINE3 {
        &self.itline3
    }
    ///0x90 - interrupt line 4 status register
    #[inline(always)]
    pub const fn itline4(&self) -> &ITLINE4 {
        &self.itline4
    }
    ///0x94 - interrupt line 5 status register
    #[inline(always)]
    pub const fn itline5(&self) -> &ITLINE5 {
        &self.itline5
    }
    ///0x98 - interrupt line 6 status register
    #[inline(always)]
    pub const fn itline6(&self) -> &ITLINE6 {
        &self.itline6
    }
    ///0x9c - interrupt line 7 status register
    #[inline(always)]
    pub const fn itline7(&self) -> &ITLINE7 {
        &self.itline7
    }
    ///0xa0 - interrupt line 8 status register
    #[inline(always)]
    pub const fn itline8(&self) -> &ITLINE8 {
        &self.itline8
    }
    ///0xa4 - interrupt line 9 status register
    #[inline(always)]
    pub const fn itline9(&self) -> &ITLINE9 {
        &self.itline9
    }
    ///0xa8 - interrupt line 10 status register
    #[inline(always)]
    pub const fn itline10(&self) -> &ITLINE10 {
        &self.itline10
    }
    ///0xac - interrupt line 11 status register
    #[inline(always)]
    pub const fn itline11(&self) -> &ITLINE11 {
        &self.itline11
    }
    ///0xb0 - interrupt line 12 status register
    #[inline(always)]
    pub const fn itline12(&self) -> &ITLINE12 {
        &self.itline12
    }
    ///0xb4 - interrupt line 13 status register
    #[inline(always)]
    pub const fn itline13(&self) -> &ITLINE13 {
        &self.itline13
    }
    ///0xb8 - interrupt line 14 status register
    #[inline(always)]
    pub const fn itline14(&self) -> &ITLINE14 {
        &self.itline14
    }
    ///0xbc - interrupt line 15 status register
    #[inline(always)]
    pub const fn itline15(&self) -> &ITLINE15 {
        &self.itline15
    }
    ///0xc0 - interrupt line 16 status register
    #[inline(always)]
    pub const fn itline16(&self) -> &ITLINE16 {
        &self.itline16
    }
    ///0xc4 - interrupt line 17 status register
    #[inline(always)]
    pub const fn itline17(&self) -> &ITLINE17 {
        &self.itline17
    }
    ///0xc8 - interrupt line 18 status register
    #[inline(always)]
    pub const fn itline18(&self) -> &ITLINE18 {
        &self.itline18
    }
    ///0xcc - interrupt line 19 status register
    #[inline(always)]
    pub const fn itline19(&self) -> &ITLINE19 {
        &self.itline19
    }
    ///0xd0 - interrupt line 20 status register
    #[inline(always)]
    pub const fn itline20(&self) -> &ITLINE20 {
        &self.itline20
    }
    ///0xd4 - interrupt line 21 status register
    #[inline(always)]
    pub const fn itline21(&self) -> &ITLINE21 {
        &self.itline21
    }
    ///0xd8 - interrupt line 22 status register
    #[inline(always)]
    pub const fn itline22(&self) -> &ITLINE22 {
        &self.itline22
    }
    ///0xdc - interrupt line 23 status register
    #[inline(always)]
    pub const fn itline23(&self) -> &ITLINE23 {
        &self.itline23
    }
    ///0xe0 - interrupt line 24 status register
    #[inline(always)]
    pub const fn itline24(&self) -> &ITLINE24 {
        &self.itline24
    }
    ///0xe4 - interrupt line 25 status register
    #[inline(always)]
    pub const fn itline25(&self) -> &ITLINE25 {
        &self.itline25
    }
    ///0xe8 - interrupt line 26 status register
    #[inline(always)]
    pub const fn itline26(&self) -> &ITLINE26 {
        &self.itline26
    }
    ///0xec - interrupt line 27 status register
    #[inline(always)]
    pub const fn itline27(&self) -> &ITLINE27 {
        &self.itline27
    }
    ///0xf0 - interrupt line 28 status register
    #[inline(always)]
    pub const fn itline28(&self) -> &ITLINE28 {
        &self.itline28
    }
    ///0xf4 - interrupt line 29 status register
    #[inline(always)]
    pub const fn itline29(&self) -> &ITLINE29 {
        &self.itline29
    }
    ///0xf8 - interrupt line 25 status register
    #[inline(always)]
    pub const fn itline30(&self) -> &ITLINE30 {
        &self.itline30
    }
    ///0xfc - interrupt line 25 status register
    #[inline(always)]
    pub const fn itline31(&self) -> &ITLINE31 {
        &self.itline31
    }
}
/**CFGR1 (rw) register accessor: SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///SYSCFG configuration register 1
pub mod cfgr1;
/**CFGR2 (rw) register accessor: SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///SYSCFG configuration register 1
pub mod cfgr2;
/**ITLINE0 (r) register accessor: interrupt line 0 status register

You can [`read`](crate::Reg::read) this register and get [`itline0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE0)

For information about available fields see [`mod@itline0`] module*/
pub type ITLINE0 = crate::Reg<itline0::ITLINE0rs>;
///interrupt line 0 status register
pub mod itline0;
/**ITLINE2 (r) register accessor: interrupt line 2 status register

You can [`read`](crate::Reg::read) this register and get [`itline2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE2)

For information about available fields see [`mod@itline2`] module*/
pub type ITLINE2 = crate::Reg<itline2::ITLINE2rs>;
///interrupt line 2 status register
pub mod itline2;
/**ITLINE3 (r) register accessor: interrupt line 3 status register

You can [`read`](crate::Reg::read) this register and get [`itline3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE3)

For information about available fields see [`mod@itline3`] module*/
pub type ITLINE3 = crate::Reg<itline3::ITLINE3rs>;
///interrupt line 3 status register
pub mod itline3;
/**ITLINE4 (r) register accessor: interrupt line 4 status register

You can [`read`](crate::Reg::read) this register and get [`itline4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE4)

For information about available fields see [`mod@itline4`] module*/
pub type ITLINE4 = crate::Reg<itline4::ITLINE4rs>;
///interrupt line 4 status register
pub mod itline4;
/**ITLINE5 (r) register accessor: interrupt line 5 status register

You can [`read`](crate::Reg::read) this register and get [`itline5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE5)

For information about available fields see [`mod@itline5`] module*/
pub type ITLINE5 = crate::Reg<itline5::ITLINE5rs>;
///interrupt line 5 status register
pub mod itline5;
/**ITLINE6 (r) register accessor: interrupt line 6 status register

You can [`read`](crate::Reg::read) this register and get [`itline6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE6)

For information about available fields see [`mod@itline6`] module*/
pub type ITLINE6 = crate::Reg<itline6::ITLINE6rs>;
///interrupt line 6 status register
pub mod itline6;
/**ITLINE7 (r) register accessor: interrupt line 7 status register

You can [`read`](crate::Reg::read) this register and get [`itline7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE7)

For information about available fields see [`mod@itline7`] module*/
pub type ITLINE7 = crate::Reg<itline7::ITLINE7rs>;
///interrupt line 7 status register
pub mod itline7;
/**ITLINE8 (r) register accessor: interrupt line 8 status register

You can [`read`](crate::Reg::read) this register and get [`itline8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE8)

For information about available fields see [`mod@itline8`] module*/
pub type ITLINE8 = crate::Reg<itline8::ITLINE8rs>;
///interrupt line 8 status register
pub mod itline8;
/**ITLINE9 (r) register accessor: interrupt line 9 status register

You can [`read`](crate::Reg::read) this register and get [`itline9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE9)

For information about available fields see [`mod@itline9`] module*/
pub type ITLINE9 = crate::Reg<itline9::ITLINE9rs>;
///interrupt line 9 status register
pub mod itline9;
/**ITLINE10 (r) register accessor: interrupt line 10 status register

You can [`read`](crate::Reg::read) this register and get [`itline10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE10)

For information about available fields see [`mod@itline10`] module*/
pub type ITLINE10 = crate::Reg<itline10::ITLINE10rs>;
///interrupt line 10 status register
pub mod itline10;
/**ITLINE11 (r) register accessor: interrupt line 11 status register

You can [`read`](crate::Reg::read) this register and get [`itline11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE11)

For information about available fields see [`mod@itline11`] module*/
pub type ITLINE11 = crate::Reg<itline11::ITLINE11rs>;
///interrupt line 11 status register
pub mod itline11;
/**ITLINE12 (r) register accessor: interrupt line 12 status register

You can [`read`](crate::Reg::read) this register and get [`itline12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE12)

For information about available fields see [`mod@itline12`] module*/
pub type ITLINE12 = crate::Reg<itline12::ITLINE12rs>;
///interrupt line 12 status register
pub mod itline12;
/**ITLINE13 (r) register accessor: interrupt line 13 status register

You can [`read`](crate::Reg::read) this register and get [`itline13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE13)

For information about available fields see [`mod@itline13`] module*/
pub type ITLINE13 = crate::Reg<itline13::ITLINE13rs>;
///interrupt line 13 status register
pub mod itline13;
/**ITLINE14 (r) register accessor: interrupt line 14 status register

You can [`read`](crate::Reg::read) this register and get [`itline14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE14)

For information about available fields see [`mod@itline14`] module*/
pub type ITLINE14 = crate::Reg<itline14::ITLINE14rs>;
///interrupt line 14 status register
pub mod itline14;
/**ITLINE15 (r) register accessor: interrupt line 15 status register

You can [`read`](crate::Reg::read) this register and get [`itline15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE15)

For information about available fields see [`mod@itline15`] module*/
pub type ITLINE15 = crate::Reg<itline15::ITLINE15rs>;
///interrupt line 15 status register
pub mod itline15;
/**ITLINE16 (r) register accessor: interrupt line 16 status register

You can [`read`](crate::Reg::read) this register and get [`itline16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE16)

For information about available fields see [`mod@itline16`] module*/
pub type ITLINE16 = crate::Reg<itline16::ITLINE16rs>;
///interrupt line 16 status register
pub mod itline16;
/**ITLINE17 (r) register accessor: interrupt line 17 status register

You can [`read`](crate::Reg::read) this register and get [`itline17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE17)

For information about available fields see [`mod@itline17`] module*/
pub type ITLINE17 = crate::Reg<itline17::ITLINE17rs>;
///interrupt line 17 status register
pub mod itline17;
/**ITLINE18 (r) register accessor: interrupt line 18 status register

You can [`read`](crate::Reg::read) this register and get [`itline18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE18)

For information about available fields see [`mod@itline18`] module*/
pub type ITLINE18 = crate::Reg<itline18::ITLINE18rs>;
///interrupt line 18 status register
pub mod itline18;
/**ITLINE19 (r) register accessor: interrupt line 19 status register

You can [`read`](crate::Reg::read) this register and get [`itline19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE19)

For information about available fields see [`mod@itline19`] module*/
pub type ITLINE19 = crate::Reg<itline19::ITLINE19rs>;
///interrupt line 19 status register
pub mod itline19;
/**ITLINE20 (r) register accessor: interrupt line 20 status register

You can [`read`](crate::Reg::read) this register and get [`itline20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE20)

For information about available fields see [`mod@itline20`] module*/
pub type ITLINE20 = crate::Reg<itline20::ITLINE20rs>;
///interrupt line 20 status register
pub mod itline20;
/**ITLINE21 (r) register accessor: interrupt line 21 status register

You can [`read`](crate::Reg::read) this register and get [`itline21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE21)

For information about available fields see [`mod@itline21`] module*/
pub type ITLINE21 = crate::Reg<itline21::ITLINE21rs>;
///interrupt line 21 status register
pub mod itline21;
/**ITLINE22 (r) register accessor: interrupt line 22 status register

You can [`read`](crate::Reg::read) this register and get [`itline22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE22)

For information about available fields see [`mod@itline22`] module*/
pub type ITLINE22 = crate::Reg<itline22::ITLINE22rs>;
///interrupt line 22 status register
pub mod itline22;
/**ITLINE23 (r) register accessor: interrupt line 23 status register

You can [`read`](crate::Reg::read) this register and get [`itline23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE23)

For information about available fields see [`mod@itline23`] module*/
pub type ITLINE23 = crate::Reg<itline23::ITLINE23rs>;
///interrupt line 23 status register
pub mod itline23;
/**ITLINE24 (r) register accessor: interrupt line 24 status register

You can [`read`](crate::Reg::read) this register and get [`itline24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE24)

For information about available fields see [`mod@itline24`] module*/
pub type ITLINE24 = crate::Reg<itline24::ITLINE24rs>;
///interrupt line 24 status register
pub mod itline24;
/**ITLINE25 (r) register accessor: interrupt line 25 status register

You can [`read`](crate::Reg::read) this register and get [`itline25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE25)

For information about available fields see [`mod@itline25`] module*/
pub type ITLINE25 = crate::Reg<itline25::ITLINE25rs>;
///interrupt line 25 status register
pub mod itline25;
/**ITLINE26 (r) register accessor: interrupt line 26 status register

You can [`read`](crate::Reg::read) this register and get [`itline26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE26)

For information about available fields see [`mod@itline26`] module*/
pub type ITLINE26 = crate::Reg<itline26::ITLINE26rs>;
///interrupt line 26 status register
pub mod itline26;
/**ITLINE27 (r) register accessor: interrupt line 27 status register

You can [`read`](crate::Reg::read) this register and get [`itline27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE27)

For information about available fields see [`mod@itline27`] module*/
pub type ITLINE27 = crate::Reg<itline27::ITLINE27rs>;
///interrupt line 27 status register
pub mod itline27;
/**ITLINE28 (r) register accessor: interrupt line 28 status register

You can [`read`](crate::Reg::read) this register and get [`itline28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE28)

For information about available fields see [`mod@itline28`] module*/
pub type ITLINE28 = crate::Reg<itline28::ITLINE28rs>;
///interrupt line 28 status register
pub mod itline28;
/**ITLINE29 (r) register accessor: interrupt line 29 status register

You can [`read`](crate::Reg::read) this register and get [`itline29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE29)

For information about available fields see [`mod@itline29`] module*/
pub type ITLINE29 = crate::Reg<itline29::ITLINE29rs>;
///interrupt line 29 status register
pub mod itline29;
/**ITLINE30 (r) register accessor: interrupt line 25 status register

You can [`read`](crate::Reg::read) this register and get [`itline30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE30)

For information about available fields see [`mod@itline30`] module*/
pub type ITLINE30 = crate::Reg<itline30::ITLINE30rs>;
///interrupt line 25 status register
pub mod itline30;
/**ITLINE31 (r) register accessor: interrupt line 25 status register

You can [`read`](crate::Reg::read) this register and get [`itline31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:ITLINE31)

For information about available fields see [`mod@itline31`] module*/
pub type ITLINE31 = crate::Reg<itline31::ITLINE31rs>;
///interrupt line 25 status register
pub mod itline31;
