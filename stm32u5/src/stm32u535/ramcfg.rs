#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    m1cr: M1CR,
    _reserved1: [u8; 0x04],
    m1isr: M1ISR,
    _reserved2: [u8; 0x1c],
    ram1erkeyr: RAM1ERKEYR,
    _reserved3: [u8; 0x14],
    m2cr: M2CR,
    m2ier: M2IER,
    m2isr: M2ISR,
    m2sear: M2SEAR,
    m2dear: M2DEAR,
    m2icr: M2ICR,
    m2wpr1: M2WPR1,
    m2wpr2: M2WPR2,
    _reserved11: [u8; 0x04],
    m2ecckeyr: M2ECCKEYR,
    m2erkeyr: M2ERKEYR,
    _reserved13: [u8; 0x14],
    m3cr: M3CR,
    m3ier: M3IER,
    m3isr: M3ISR,
    m3sear: M3SEAR,
    m3dear: M3DEAR,
    m3icr: M3ICR,
    _reserved19: [u8; 0x0c],
    m3ecckeyr: M3ECCKEYR,
    m3erkeyr: M3ERKEYR,
    _reserved21: [u8; 0x14],
    m4cr: M4CR,
    _reserved22: [u8; 0x04],
    m4isr: M4ISR,
    _reserved23: [u8; 0x1c],
    m4erkeyr: M4ERKEYR,
    _reserved24: [u8; 0x14],
    m5cr: M5CR,
    m5ier: M5IER,
    m5isr: M5ISR,
    m5sear: M5SEAR,
    m5dear: M5DEAR,
    m5icr: M5ICR,
    _reserved30: [u8; 0x0c],
    m5ecckeyr: M5ECCKEYR,
    m5erkeyr: M5ERKEYR,
    _reserved32: [u8; 0x14],
    m6cr: M6CR,
    _reserved33: [u8; 0x04],
    m6isr: M6ISR,
    _reserved34: [u8; 0x1c],
    m6erkeyr: M6ERKEYR,
}
impl RegisterBlock {
    #[doc = "0x00 - RAMCFG SRAM x control register"]
    #[inline(always)]
    pub const fn m1cr(&self) -> &M1CR {
        &self.m1cr
    }
    #[doc = "0x08 - RAMCFG RAMx interrupt status register"]
    #[inline(always)]
    pub const fn m1isr(&self) -> &M1ISR {
        &self.m1isr
    }
    #[doc = "0x28 - RAMCFG SRAM x erase key register"]
    #[inline(always)]
    pub const fn ram1erkeyr(&self) -> &RAM1ERKEYR {
        &self.ram1erkeyr
    }
    #[doc = "0x40 - RAMCFG SRAM x control register"]
    #[inline(always)]
    pub const fn m2cr(&self) -> &M2CR {
        &self.m2cr
    }
    #[doc = "0x44 - RAMCFG SRAM x interrupt enable register"]
    #[inline(always)]
    pub const fn m2ier(&self) -> &M2IER {
        &self.m2ier
    }
    #[doc = "0x48 - RAMCFG RAMx interrupt status register"]
    #[inline(always)]
    pub const fn m2isr(&self) -> &M2ISR {
        &self.m2isr
    }
    #[doc = "0x4c - RAMCFG RAM x ECC single error address register"]
    #[inline(always)]
    pub const fn m2sear(&self) -> &M2SEAR {
        &self.m2sear
    }
    #[doc = "0x50 - RAMCFG RAM x ECC double error address register"]
    #[inline(always)]
    pub const fn m2dear(&self) -> &M2DEAR {
        &self.m2dear
    }
    #[doc = "0x54 - RAMCFG RAM x interrupt clear register x"]
    #[inline(always)]
    pub const fn m2icr(&self) -> &M2ICR {
        &self.m2icr
    }
    #[doc = "0x58 - RAMCFG SRAM2 write protection register 1"]
    #[inline(always)]
    pub const fn m2wpr1(&self) -> &M2WPR1 {
        &self.m2wpr1
    }
    #[doc = "0x5c - RAMCFG SRAM2 write protection register 2"]
    #[inline(always)]
    pub const fn m2wpr2(&self) -> &M2WPR2 {
        &self.m2wpr2
    }
    #[doc = "0x64 - RAMCFG SRAM x ECC key register"]
    #[inline(always)]
    pub const fn m2ecckeyr(&self) -> &M2ECCKEYR {
        &self.m2ecckeyr
    }
    #[doc = "0x68 - RAMCFG SRAM x erase key register"]
    #[inline(always)]
    pub const fn m2erkeyr(&self) -> &M2ERKEYR {
        &self.m2erkeyr
    }
    #[doc = "0x80 - RAMCFG SRAM x control register"]
    #[inline(always)]
    pub const fn m3cr(&self) -> &M3CR {
        &self.m3cr
    }
    #[doc = "0x84 - RAMCFG SRAM x interrupt enable register"]
    #[inline(always)]
    pub const fn m3ier(&self) -> &M3IER {
        &self.m3ier
    }
    #[doc = "0x88 - RAMCFG RAMx interrupt status register"]
    #[inline(always)]
    pub const fn m3isr(&self) -> &M3ISR {
        &self.m3isr
    }
    #[doc = "0x8c - RAMCFG RAM x ECC single error address register"]
    #[inline(always)]
    pub const fn m3sear(&self) -> &M3SEAR {
        &self.m3sear
    }
    #[doc = "0x90 - RAMCFG RAM x ECC double error address register"]
    #[inline(always)]
    pub const fn m3dear(&self) -> &M3DEAR {
        &self.m3dear
    }
    #[doc = "0x94 - RAMCFG RAM x interrupt clear register x"]
    #[inline(always)]
    pub const fn m3icr(&self) -> &M3ICR {
        &self.m3icr
    }
    #[doc = "0xa4 - RAMCFG SRAM x ECC key register"]
    #[inline(always)]
    pub const fn m3ecckeyr(&self) -> &M3ECCKEYR {
        &self.m3ecckeyr
    }
    #[doc = "0xa8 - RAMCFG SRAM x erase key register"]
    #[inline(always)]
    pub const fn m3erkeyr(&self) -> &M3ERKEYR {
        &self.m3erkeyr
    }
    #[doc = "0xc0 - RAMCFG SRAM x control register"]
    #[inline(always)]
    pub const fn m4cr(&self) -> &M4CR {
        &self.m4cr
    }
    #[doc = "0xc8 - RAMCFG RAMx interrupt status register"]
    #[inline(always)]
    pub const fn m4isr(&self) -> &M4ISR {
        &self.m4isr
    }
    #[doc = "0xe8 - RAMCFG SRAM x erase key register"]
    #[inline(always)]
    pub const fn m4erkeyr(&self) -> &M4ERKEYR {
        &self.m4erkeyr
    }
    #[doc = "0x100 - RAMCFG SRAM x control register"]
    #[inline(always)]
    pub const fn m5cr(&self) -> &M5CR {
        &self.m5cr
    }
    #[doc = "0x104 - RAMCFG SRAM x interrupt enable register"]
    #[inline(always)]
    pub const fn m5ier(&self) -> &M5IER {
        &self.m5ier
    }
    #[doc = "0x108 - RAMCFG RAMx interrupt status register"]
    #[inline(always)]
    pub const fn m5isr(&self) -> &M5ISR {
        &self.m5isr
    }
    #[doc = "0x10c - RAMCFG RAM x ECC single error address register"]
    #[inline(always)]
    pub const fn m5sear(&self) -> &M5SEAR {
        &self.m5sear
    }
    #[doc = "0x110 - RAMCFG RAM x ECC double error address register"]
    #[inline(always)]
    pub const fn m5dear(&self) -> &M5DEAR {
        &self.m5dear
    }
    #[doc = "0x114 - RAMCFG RAM x interrupt clear register x"]
    #[inline(always)]
    pub const fn m5icr(&self) -> &M5ICR {
        &self.m5icr
    }
    #[doc = "0x124 - RAMCFG RAM x interrupt clear register x"]
    #[inline(always)]
    pub const fn m5ecckeyr(&self) -> &M5ECCKEYR {
        &self.m5ecckeyr
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn m5erkeyr(&self) -> &M5ERKEYR {
        &self.m5erkeyr
    }
    #[doc = "0x140 - memory x control register"]
    #[inline(always)]
    pub const fn m6cr(&self) -> &M6CR {
        &self.m6cr
    }
    #[doc = "0x148 - "]
    #[inline(always)]
    pub const fn m6isr(&self) -> &M6ISR {
        &self.m6isr
    }
    #[doc = "0x168 - "]
    #[inline(always)]
    pub const fn m6erkeyr(&self) -> &M6ERKEYR {
        &self.m6erkeyr
    }
}
#[doc = "M1CR (rw) register accessor: RAMCFG SRAM x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1cr`]
module"]
pub type M1CR = crate::Reg<m1cr::M1CRrs>;
#[doc = "RAMCFG SRAM x control register"]
pub mod m1cr;
#[doc = "M1ISR (r) register accessor: RAMCFG RAMx interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1isr`]
module"]
pub type M1ISR = crate::Reg<m1isr::M1ISRrs>;
#[doc = "RAMCFG RAMx interrupt status register"]
pub mod m1isr;
#[doc = "RAM1ERKEYR (w) register accessor: RAMCFG SRAM x erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram1erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1erkeyr`]
module"]
pub type RAM1ERKEYR = crate::Reg<ram1erkeyr::RAM1ERKEYRrs>;
#[doc = "RAMCFG SRAM x erase key register"]
pub mod ram1erkeyr;
#[doc = "M2CR (rw) register accessor: RAMCFG SRAM x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2cr`]
module"]
pub type M2CR = crate::Reg<m2cr::M2CRrs>;
#[doc = "RAMCFG SRAM x control register"]
pub mod m2cr;
#[doc = "M2IER (rw) register accessor: RAMCFG SRAM x interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2ier`]
module"]
pub type M2IER = crate::Reg<m2ier::M2IERrs>;
#[doc = "RAMCFG SRAM x interrupt enable register"]
pub mod m2ier;
#[doc = "M2ISR (r) register accessor: RAMCFG RAMx interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2isr`]
module"]
pub type M2ISR = crate::Reg<m2isr::M2ISRrs>;
#[doc = "RAMCFG RAMx interrupt status register"]
pub mod m2isr;
#[doc = "M2SEAR (r) register accessor: RAMCFG RAM x ECC single error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2sear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2sear`]
module"]
pub type M2SEAR = crate::Reg<m2sear::M2SEARrs>;
#[doc = "RAMCFG RAM x ECC single error address register"]
pub mod m2sear;
#[doc = "M2DEAR (r) register accessor: RAMCFG RAM x ECC double error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2dear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2dear`]
module"]
pub type M2DEAR = crate::Reg<m2dear::M2DEARrs>;
#[doc = "RAMCFG RAM x ECC double error address register"]
pub mod m2dear;
#[doc = "M2ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2icr`]
module"]
pub type M2ICR = crate::Reg<m2icr::M2ICRrs>;
#[doc = "RAMCFG RAM x interrupt clear register x"]
pub mod m2icr;
#[doc = "M2WPR1 (rw) register accessor: RAMCFG SRAM2 write protection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2wpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2wpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2wpr1`]
module"]
pub type M2WPR1 = crate::Reg<m2wpr1::M2WPR1rs>;
#[doc = "RAMCFG SRAM2 write protection register 1"]
pub mod m2wpr1;
#[doc = "M2WPR2 (rw) register accessor: RAMCFG SRAM2 write protection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2wpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2wpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2wpr2`]
module"]
pub type M2WPR2 = crate::Reg<m2wpr2::M2WPR2rs>;
#[doc = "RAMCFG SRAM2 write protection register 2"]
pub mod m2wpr2;
#[doc = "M2ECCKEYR (w) register accessor: RAMCFG SRAM x ECC key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2ecckeyr`]
module"]
pub type M2ECCKEYR = crate::Reg<m2ecckeyr::M2ECCKEYRrs>;
#[doc = "RAMCFG SRAM x ECC key register"]
pub mod m2ecckeyr;
#[doc = "M2ERKEYR (w) register accessor: RAMCFG SRAM x erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2erkeyr`]
module"]
pub type M2ERKEYR = crate::Reg<m2erkeyr::M2ERKEYRrs>;
#[doc = "RAMCFG SRAM x erase key register"]
pub mod m2erkeyr;
#[doc = "M3CR (rw) register accessor: RAMCFG SRAM x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3cr`]
module"]
pub type M3CR = crate::Reg<m3cr::M3CRrs>;
#[doc = "RAMCFG SRAM x control register"]
pub mod m3cr;
#[doc = "M3IER (rw) register accessor: RAMCFG SRAM x interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3ier`]
module"]
pub type M3IER = crate::Reg<m3ier::M3IERrs>;
#[doc = "RAMCFG SRAM x interrupt enable register"]
pub mod m3ier;
#[doc = "M3ISR (r) register accessor: RAMCFG RAMx interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3isr`]
module"]
pub type M3ISR = crate::Reg<m3isr::M3ISRrs>;
#[doc = "RAMCFG RAMx interrupt status register"]
pub mod m3isr;
#[doc = "M3SEAR (r) register accessor: RAMCFG RAM x ECC single error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3sear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3sear`]
module"]
pub type M3SEAR = crate::Reg<m3sear::M3SEARrs>;
#[doc = "RAMCFG RAM x ECC single error address register"]
pub mod m3sear;
#[doc = "M3DEAR (r) register accessor: RAMCFG RAM x ECC double error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3dear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3dear`]
module"]
pub type M3DEAR = crate::Reg<m3dear::M3DEARrs>;
#[doc = "RAMCFG RAM x ECC double error address register"]
pub mod m3dear;
#[doc = "M3ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3icr`]
module"]
pub type M3ICR = crate::Reg<m3icr::M3ICRrs>;
#[doc = "RAMCFG RAM x interrupt clear register x"]
pub mod m3icr;
#[doc = "M3ECCKEYR (w) register accessor: RAMCFG SRAM x ECC key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3ecckeyr`]
module"]
pub type M3ECCKEYR = crate::Reg<m3ecckeyr::M3ECCKEYRrs>;
#[doc = "RAMCFG SRAM x ECC key register"]
pub mod m3ecckeyr;
#[doc = "M3ERKEYR (w) register accessor: RAMCFG SRAM x erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3erkeyr`]
module"]
pub type M3ERKEYR = crate::Reg<m3erkeyr::M3ERKEYRrs>;
#[doc = "RAMCFG SRAM x erase key register"]
pub mod m3erkeyr;
#[doc = "M4CR (rw) register accessor: RAMCFG SRAM x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4cr`]
module"]
pub type M4CR = crate::Reg<m4cr::M4CRrs>;
#[doc = "RAMCFG SRAM x control register"]
pub mod m4cr;
#[doc = "M4ISR (r) register accessor: RAMCFG RAMx interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4isr`]
module"]
pub type M4ISR = crate::Reg<m4isr::M4ISRrs>;
#[doc = "RAMCFG RAMx interrupt status register"]
pub mod m4isr;
#[doc = "M4ERKEYR (w) register accessor: RAMCFG SRAM x erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m4erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4erkeyr`]
module"]
pub type M4ERKEYR = crate::Reg<m4erkeyr::M4ERKEYRrs>;
#[doc = "RAMCFG SRAM x erase key register"]
pub mod m4erkeyr;
#[doc = "M5CR (rw) register accessor: RAMCFG SRAM x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5cr`]
module"]
pub type M5CR = crate::Reg<m5cr::M5CRrs>;
#[doc = "RAMCFG SRAM x control register"]
pub mod m5cr;
#[doc = "M5IER (rw) register accessor: RAMCFG SRAM x interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5ier`]
module"]
pub type M5IER = crate::Reg<m5ier::M5IERrs>;
#[doc = "RAMCFG SRAM x interrupt enable register"]
pub mod m5ier;
#[doc = "M5ISR (r) register accessor: RAMCFG RAMx interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5isr`]
module"]
pub type M5ISR = crate::Reg<m5isr::M5ISRrs>;
#[doc = "RAMCFG RAMx interrupt status register"]
pub mod m5isr;
#[doc = "M5SEAR (r) register accessor: RAMCFG RAM x ECC single error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5sear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5sear`]
module"]
pub type M5SEAR = crate::Reg<m5sear::M5SEARrs>;
#[doc = "RAMCFG RAM x ECC single error address register"]
pub mod m5sear;
#[doc = "M5DEAR (r) register accessor: RAMCFG RAM x ECC double error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5dear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5dear`]
module"]
pub type M5DEAR = crate::Reg<m5dear::M5DEARrs>;
#[doc = "RAMCFG RAM x ECC double error address register"]
pub mod m5dear;
#[doc = "M5ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5icr`]
module"]
pub type M5ICR = crate::Reg<m5icr::M5ICRrs>;
#[doc = "RAMCFG RAM x interrupt clear register x"]
pub mod m5icr;
#[doc = "M5ECCKEYR (rw) register accessor: RAMCFG RAM x interrupt clear register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5ecckeyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5ecckeyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5ecckeyr`]
module"]
pub type M5ECCKEYR = crate::Reg<m5ecckeyr::M5ECCKEYRrs>;
#[doc = "RAMCFG RAM x interrupt clear register x"]
pub mod m5ecckeyr;
#[doc = "M5ERKEYR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5erkeyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5erkeyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5erkeyr`]
module"]
pub type M5ERKEYR = crate::Reg<m5erkeyr::M5ERKEYRrs>;
#[doc = ""]
pub mod m5erkeyr;
#[doc = "M6CR (rw) register accessor: memory x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m6cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m6cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m6cr`]
module"]
pub type M6CR = crate::Reg<m6cr::M6CRrs>;
#[doc = "memory x control register"]
pub mod m6cr;
#[doc = "M6ISR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m6isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m6isr`]
module"]
pub type M6ISR = crate::Reg<m6isr::M6ISRrs>;
#[doc = ""]
pub mod m6isr;
#[doc = "M6ERKEYR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m6erkeyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m6erkeyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m6erkeyr`]
module"]
pub type M6ERKEYR = crate::Reg<m6erkeyr::M6ERKEYRrs>;
#[doc = ""]
pub mod m6erkeyr;
