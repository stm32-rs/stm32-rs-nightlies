#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ram1cr: RAM1CR,
    _reserved1: [u8; 0x04],
    ram1isr: RAM1ISR,
    _reserved2: [u8; 0x1c],
    ram1erkeyr: RAM1ERKEYR,
    _reserved3: [u8; 0x14],
    ram2cr: RAM2CR,
    ram2ier: RAM2IER,
    ram2isr: RAM2ISR,
    ram2sear: RAM2SEAR,
    ram2dear: RAM2DEAR,
    ram2icr: RAM2ICR,
    ram2wpr1: RAM2WPR1,
    ram2wpr2: RAM2WPR2,
    _reserved11: [u8; 0x04],
    ram2ecckeyr: RAM2ECCKEYR,
    ram2erkeyr: RAM2ERKEYR,
    _reserved13: [u8; 0x14],
    ram3cr: RAM3CR,
    ram3ier: RAM3IER,
    ram3isr: RAM3ISR,
    ram3sear: RAM3SEAR,
    ram3dear: RAM3DEAR,
    ram3icr: RAM3ICR,
    _reserved19: [u8; 0x0c],
    ram3ecckeyr: RAM3ECCKEYR,
    ram3erkeyr: RAM3ERKEYR,
    _reserved21: [u8; 0x14],
    ram4cr: RAM4CR,
    _reserved22: [u8; 0x04],
    ram4isr: RAM4ISR,
    _reserved23: [u8; 0x1c],
    ram4erkeyr: RAM4ERKEYR,
    _reserved24: [u8; 0x14],
    ram5cr: RAM5CR,
    ram5ier: RAM5IER,
    ram5isr: RAM5ISR,
    ram5sear: RAM5SEAR,
    ram5dear: RAM5DEAR,
    ram5icr: RAM5ICR,
}
impl RegisterBlock {
    #[doc = "0x00 - RAMCFG SRAM x control register"]
    #[inline(always)]
    pub const fn ram1cr(&self) -> &RAM1CR {
        &self.ram1cr
    }
    #[doc = "0x08 - RAMCFG RAMx interrupt status register"]
    #[inline(always)]
    pub const fn ram1isr(&self) -> &RAM1ISR {
        &self.ram1isr
    }
    #[doc = "0x28 - RAMCFG SRAM x erase key register"]
    #[inline(always)]
    pub const fn ram1erkeyr(&self) -> &RAM1ERKEYR {
        &self.ram1erkeyr
    }
    #[doc = "0x40 - RAMCFG SRAM x control register"]
    #[inline(always)]
    pub const fn ram2cr(&self) -> &RAM2CR {
        &self.ram2cr
    }
    #[doc = "0x44 - RAMCFG SRAM x interrupt enable register"]
    #[inline(always)]
    pub const fn ram2ier(&self) -> &RAM2IER {
        &self.ram2ier
    }
    #[doc = "0x48 - RAMCFG RAMx interrupt status register"]
    #[inline(always)]
    pub const fn ram2isr(&self) -> &RAM2ISR {
        &self.ram2isr
    }
    #[doc = "0x4c - RAMCFG RAM x ECC single error address register"]
    #[inline(always)]
    pub const fn ram2sear(&self) -> &RAM2SEAR {
        &self.ram2sear
    }
    #[doc = "0x50 - RAMCFG RAM x ECC double error address register"]
    #[inline(always)]
    pub const fn ram2dear(&self) -> &RAM2DEAR {
        &self.ram2dear
    }
    #[doc = "0x54 - RAMCFG RAM x interrupt clear register x"]
    #[inline(always)]
    pub const fn ram2icr(&self) -> &RAM2ICR {
        &self.ram2icr
    }
    #[doc = "0x58 - RAMCFG SRAM2 write protection register 1"]
    #[inline(always)]
    pub const fn ram2wpr1(&self) -> &RAM2WPR1 {
        &self.ram2wpr1
    }
    #[doc = "0x5c - RAMCFG SRAM2 write protection register 2"]
    #[inline(always)]
    pub const fn ram2wpr2(&self) -> &RAM2WPR2 {
        &self.ram2wpr2
    }
    #[doc = "0x64 - RAMCFG SRAM x ECC key register"]
    #[inline(always)]
    pub const fn ram2ecckeyr(&self) -> &RAM2ECCKEYR {
        &self.ram2ecckeyr
    }
    #[doc = "0x68 - RAMCFG SRAM x erase key register"]
    #[inline(always)]
    pub const fn ram2erkeyr(&self) -> &RAM2ERKEYR {
        &self.ram2erkeyr
    }
    #[doc = "0x80 - RAMCFG SRAM x control register"]
    #[inline(always)]
    pub const fn ram3cr(&self) -> &RAM3CR {
        &self.ram3cr
    }
    #[doc = "0x84 - RAMCFG SRAM x interrupt enable register"]
    #[inline(always)]
    pub const fn ram3ier(&self) -> &RAM3IER {
        &self.ram3ier
    }
    #[doc = "0x88 - RAMCFG RAMx interrupt status register"]
    #[inline(always)]
    pub const fn ram3isr(&self) -> &RAM3ISR {
        &self.ram3isr
    }
    #[doc = "0x8c - RAMCFG RAM x ECC single error address register"]
    #[inline(always)]
    pub const fn ram3sear(&self) -> &RAM3SEAR {
        &self.ram3sear
    }
    #[doc = "0x90 - RAMCFG RAM x ECC double error address register"]
    #[inline(always)]
    pub const fn ram3dear(&self) -> &RAM3DEAR {
        &self.ram3dear
    }
    #[doc = "0x94 - RAMCFG RAM x interrupt clear register x"]
    #[inline(always)]
    pub const fn ram3icr(&self) -> &RAM3ICR {
        &self.ram3icr
    }
    #[doc = "0xa4 - RAMCFG SRAM x ECC key register"]
    #[inline(always)]
    pub const fn ram3ecckeyr(&self) -> &RAM3ECCKEYR {
        &self.ram3ecckeyr
    }
    #[doc = "0xa8 - RAMCFG SRAM x erase key register"]
    #[inline(always)]
    pub const fn ram3erkeyr(&self) -> &RAM3ERKEYR {
        &self.ram3erkeyr
    }
    #[doc = "0xc0 - RAMCFG SRAM x control register"]
    #[inline(always)]
    pub const fn ram4cr(&self) -> &RAM4CR {
        &self.ram4cr
    }
    #[doc = "0xc8 - RAMCFG RAMx interrupt status register"]
    #[inline(always)]
    pub const fn ram4isr(&self) -> &RAM4ISR {
        &self.ram4isr
    }
    #[doc = "0xe8 - RAMCFG SRAM x erase key register"]
    #[inline(always)]
    pub const fn ram4erkeyr(&self) -> &RAM4ERKEYR {
        &self.ram4erkeyr
    }
    #[doc = "0x100 - RAMCFG SRAM x control register"]
    #[inline(always)]
    pub const fn ram5cr(&self) -> &RAM5CR {
        &self.ram5cr
    }
    #[doc = "0x104 - RAMCFG SRAM x interrupt enable register"]
    #[inline(always)]
    pub const fn ram5ier(&self) -> &RAM5IER {
        &self.ram5ier
    }
    #[doc = "0x108 - RAMCFG RAMx interrupt status register"]
    #[inline(always)]
    pub const fn ram5isr(&self) -> &RAM5ISR {
        &self.ram5isr
    }
    #[doc = "0x10c - RAMCFG RAM x ECC single error address register"]
    #[inline(always)]
    pub const fn ram5sear(&self) -> &RAM5SEAR {
        &self.ram5sear
    }
    #[doc = "0x110 - RAMCFG RAM x ECC double error address register"]
    #[inline(always)]
    pub const fn ram5dear(&self) -> &RAM5DEAR {
        &self.ram5dear
    }
    #[doc = "0x114 - RAMCFG RAM x interrupt clear register x"]
    #[inline(always)]
    pub const fn ram5icr(&self) -> &RAM5ICR {
        &self.ram5icr
    }
}
#[doc = "RAM1CR (rw) register accessor: RAMCFG SRAM x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1cr`]
module"]
pub type RAM1CR = crate::Reg<ram1cr::RAM1CRrs>;
#[doc = "RAMCFG SRAM x control register"]
pub mod ram1cr;
#[doc = "RAM1ISR (r) register accessor: RAMCFG RAMx interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram1isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1isr`]
module"]
pub type RAM1ISR = crate::Reg<ram1isr::RAM1ISRrs>;
#[doc = "RAMCFG RAMx interrupt status register"]
pub mod ram1isr;
#[doc = "RAM1ERKEYR (w) register accessor: RAMCFG SRAM x erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram1erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1erkeyr`]
module"]
pub type RAM1ERKEYR = crate::Reg<ram1erkeyr::RAM1ERKEYRrs>;
#[doc = "RAMCFG SRAM x erase key register"]
pub mod ram1erkeyr;
#[doc = "RAM2CR (rw) register accessor: RAMCFG SRAM x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2cr`]
module"]
pub type RAM2CR = crate::Reg<ram2cr::RAM2CRrs>;
#[doc = "RAMCFG SRAM x control register"]
pub mod ram2cr;
#[doc = "RAM2IER (rw) register accessor: RAMCFG SRAM x interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram2ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2ier`]
module"]
pub type RAM2IER = crate::Reg<ram2ier::RAM2IERrs>;
#[doc = "RAMCFG SRAM x interrupt enable register"]
pub mod ram2ier;
#[doc = "RAM2ISR (r) register accessor: RAMCFG RAMx interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2isr`]
module"]
pub type RAM2ISR = crate::Reg<ram2isr::RAM2ISRrs>;
#[doc = "RAMCFG RAMx interrupt status register"]
pub mod ram2isr;
#[doc = "RAM2SEAR (r) register accessor: RAMCFG RAM x ECC single error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2sear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2sear`]
module"]
pub type RAM2SEAR = crate::Reg<ram2sear::RAM2SEARrs>;
#[doc = "RAMCFG RAM x ECC single error address register"]
pub mod ram2sear;
#[doc = "RAM2DEAR (r) register accessor: RAMCFG RAM x ECC double error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2dear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2dear`]
module"]
pub type RAM2DEAR = crate::Reg<ram2dear::RAM2DEARrs>;
#[doc = "RAMCFG RAM x ECC double error address register"]
pub mod ram2dear;
#[doc = "RAM2ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram2icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2icr`]
module"]
pub type RAM2ICR = crate::Reg<ram2icr::RAM2ICRrs>;
#[doc = "RAMCFG RAM x interrupt clear register x"]
pub mod ram2icr;
#[doc = "RAM2WPR1 (rw) register accessor: RAMCFG SRAM2 write protection register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2wpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram2wpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2wpr1`]
module"]
pub type RAM2WPR1 = crate::Reg<ram2wpr1::RAM2WPR1rs>;
#[doc = "RAMCFG SRAM2 write protection register 1"]
pub mod ram2wpr1;
#[doc = "RAM2WPR2 (rw) register accessor: RAMCFG SRAM2 write protection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram2wpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram2wpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2wpr2`]
module"]
pub type RAM2WPR2 = crate::Reg<ram2wpr2::RAM2WPR2rs>;
#[doc = "RAMCFG SRAM2 write protection register 2"]
pub mod ram2wpr2;
#[doc = "RAM2ECCKEYR (w) register accessor: RAMCFG SRAM x ECC key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram2ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2ecckeyr`]
module"]
pub type RAM2ECCKEYR = crate::Reg<ram2ecckeyr::RAM2ECCKEYRrs>;
#[doc = "RAMCFG SRAM x ECC key register"]
pub mod ram2ecckeyr;
#[doc = "RAM2ERKEYR (w) register accessor: RAMCFG SRAM x erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram2erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2erkeyr`]
module"]
pub type RAM2ERKEYR = crate::Reg<ram2erkeyr::RAM2ERKEYRrs>;
#[doc = "RAMCFG SRAM x erase key register"]
pub mod ram2erkeyr;
#[doc = "RAM3CR (rw) register accessor: RAMCFG SRAM x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram3cr`]
module"]
pub type RAM3CR = crate::Reg<ram3cr::RAM3CRrs>;
#[doc = "RAMCFG SRAM x control register"]
pub mod ram3cr;
#[doc = "RAM3IER (rw) register accessor: RAMCFG SRAM x interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram3ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram3ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram3ier`]
module"]
pub type RAM3IER = crate::Reg<ram3ier::RAM3IERrs>;
#[doc = "RAMCFG SRAM x interrupt enable register"]
pub mod ram3ier;
#[doc = "RAM3ISR (r) register accessor: RAMCFG RAMx interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram3isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram3isr`]
module"]
pub type RAM3ISR = crate::Reg<ram3isr::RAM3ISRrs>;
#[doc = "RAMCFG RAMx interrupt status register"]
pub mod ram3isr;
#[doc = "RAM3SEAR (r) register accessor: RAMCFG RAM x ECC single error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram3sear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram3sear`]
module"]
pub type RAM3SEAR = crate::Reg<ram3sear::RAM3SEARrs>;
#[doc = "RAMCFG RAM x ECC single error address register"]
pub mod ram3sear;
#[doc = "RAM3DEAR (r) register accessor: RAMCFG RAM x ECC double error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram3dear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram3dear`]
module"]
pub type RAM3DEAR = crate::Reg<ram3dear::RAM3DEARrs>;
#[doc = "RAMCFG RAM x ECC double error address register"]
pub mod ram3dear;
#[doc = "RAM3ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram3icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram3icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram3icr`]
module"]
pub type RAM3ICR = crate::Reg<ram3icr::RAM3ICRrs>;
#[doc = "RAMCFG RAM x interrupt clear register x"]
pub mod ram3icr;
#[doc = "RAM3ECCKEYR (w) register accessor: RAMCFG SRAM x ECC key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram3ecckeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram3ecckeyr`]
module"]
pub type RAM3ECCKEYR = crate::Reg<ram3ecckeyr::RAM3ECCKEYRrs>;
#[doc = "RAMCFG SRAM x ECC key register"]
pub mod ram3ecckeyr;
#[doc = "RAM3ERKEYR (w) register accessor: RAMCFG SRAM x erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram3erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram3erkeyr`]
module"]
pub type RAM3ERKEYR = crate::Reg<ram3erkeyr::RAM3ERKEYRrs>;
#[doc = "RAMCFG SRAM x erase key register"]
pub mod ram3erkeyr;
#[doc = "RAM4CR (rw) register accessor: RAMCFG SRAM x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram4cr`]
module"]
pub type RAM4CR = crate::Reg<ram4cr::RAM4CRrs>;
#[doc = "RAMCFG SRAM x control register"]
pub mod ram4cr;
#[doc = "RAM4ISR (r) register accessor: RAMCFG RAMx interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram4isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram4isr`]
module"]
pub type RAM4ISR = crate::Reg<ram4isr::RAM4ISRrs>;
#[doc = "RAMCFG RAMx interrupt status register"]
pub mod ram4isr;
#[doc = "RAM4ERKEYR (w) register accessor: RAMCFG SRAM x erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram4erkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram4erkeyr`]
module"]
pub type RAM4ERKEYR = crate::Reg<ram4erkeyr::RAM4ERKEYRrs>;
#[doc = "RAMCFG SRAM x erase key register"]
pub mod ram4erkeyr;
#[doc = "RAM5CR (rw) register accessor: RAMCFG SRAM x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram5cr`]
module"]
pub type RAM5CR = crate::Reg<ram5cr::RAM5CRrs>;
#[doc = "RAMCFG SRAM x control register"]
pub mod ram5cr;
#[doc = "RAM5IER (rw) register accessor: RAMCFG SRAM x interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram5ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram5ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram5ier`]
module"]
pub type RAM5IER = crate::Reg<ram5ier::RAM5IERrs>;
#[doc = "RAMCFG SRAM x interrupt enable register"]
pub mod ram5ier;
#[doc = "RAM5ISR (r) register accessor: RAMCFG RAMx interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram5isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram5isr`]
module"]
pub type RAM5ISR = crate::Reg<ram5isr::RAM5ISRrs>;
#[doc = "RAMCFG RAMx interrupt status register"]
pub mod ram5isr;
#[doc = "RAM5SEAR (r) register accessor: RAMCFG RAM x ECC single error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram5sear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram5sear`]
module"]
pub type RAM5SEAR = crate::Reg<ram5sear::RAM5SEARrs>;
#[doc = "RAMCFG RAM x ECC single error address register"]
pub mod ram5sear;
#[doc = "RAM5DEAR (r) register accessor: RAMCFG RAM x ECC double error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram5dear::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram5dear`]
module"]
pub type RAM5DEAR = crate::Reg<ram5dear::RAM5DEARrs>;
#[doc = "RAMCFG RAM x ECC double error address register"]
pub mod ram5dear;
#[doc = "RAM5ICR (rw) register accessor: RAMCFG RAM x interrupt clear register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram5icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram5icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram5icr`]
module"]
pub type RAM5ICR = crate::Reg<ram5icr::RAM5ICRrs>;
#[doc = "RAMCFG RAM x interrupt clear register x"]
pub mod ram5icr;
