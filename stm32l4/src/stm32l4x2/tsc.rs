#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    ier: IER,
    icr: ICR,
    isr: ISR,
    iohcr: IOHCR,
    _reserved5: [u8; 0x04],
    ioascr: IOASCR,
    _reserved6: [u8; 0x04],
    ioscr: IOSCR,
    _reserved7: [u8; 0x04],
    ioccr: IOCCR,
    _reserved8: [u8; 0x04],
    iogcsr: IOGCSR,
    iogcr: [IOGCR; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x08 - interrupt clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x0c - interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x10 - I/O hysteresis control register"]
    #[inline(always)]
    pub const fn iohcr(&self) -> &IOHCR {
        &self.iohcr
    }
    #[doc = "0x18 - I/O analog switch control register"]
    #[inline(always)]
    pub const fn ioascr(&self) -> &IOASCR {
        &self.ioascr
    }
    #[doc = "0x20 - I/O sampling control register"]
    #[inline(always)]
    pub const fn ioscr(&self) -> &IOSCR {
        &self.ioscr
    }
    #[doc = "0x28 - I/O channel control register"]
    #[inline(always)]
    pub const fn ioccr(&self) -> &IOCCR {
        &self.ioccr
    }
    #[doc = "0x30 - I/O group control status register"]
    #[inline(always)]
    pub const fn iogcsr(&self) -> &IOGCSR {
        &self.iogcsr
    }
    #[doc = "0x34..0x54 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iogcr(&self, n: usize) -> &IOGCR {
        &self.iogcr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x54 - I/O group x counter register"]
    #[inline(always)]
    pub fn iogcr_iter(&self) -> impl Iterator<Item = &IOGCR> {
        self.iogcr.iter()
    }
    #[doc = "0x34 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog1cr(&self) -> &IOGCR {
        self.iogcr(0)
    }
    #[doc = "0x38 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog2cr(&self) -> &IOGCR {
        self.iogcr(1)
    }
    #[doc = "0x3c - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog3cr(&self) -> &IOGCR {
        self.iogcr(2)
    }
    #[doc = "0x40 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog4cr(&self) -> &IOGCR {
        self.iogcr(3)
    }
    #[doc = "0x44 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog5cr(&self) -> &IOGCR {
        self.iogcr(4)
    }
    #[doc = "0x48 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog6cr(&self) -> &IOGCR {
        self.iogcr(5)
    }
    #[doc = "0x4c - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog7cr(&self) -> &IOGCR {
        self.iogcr(6)
    }
    #[doc = "0x50 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog8cr(&self) -> &IOGCR {
        self.iogcr(7)
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "IER (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "ICR (rw) register accessor: interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "ISR (rw) register accessor: interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "IOHCR (rw) register accessor: I/O hysteresis control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iohcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iohcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iohcr`]
module"]
pub type IOHCR = crate::Reg<iohcr::IOHCRrs>;
#[doc = "I/O hysteresis control register"]
pub mod iohcr;
#[doc = "IOASCR (rw) register accessor: I/O analog switch control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioascr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioascr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioascr`]
module"]
pub type IOASCR = crate::Reg<ioascr::IOASCRrs>;
#[doc = "I/O analog switch control register"]
pub mod ioascr;
#[doc = "IOSCR (rw) register accessor: I/O sampling control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioscr`]
module"]
pub type IOSCR = crate::Reg<ioscr::IOSCRrs>;
#[doc = "I/O sampling control register"]
pub mod ioscr;
#[doc = "IOCCR (rw) register accessor: I/O channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioccr`]
module"]
pub type IOCCR = crate::Reg<ioccr::IOCCRrs>;
#[doc = "I/O channel control register"]
pub mod ioccr;
#[doc = "IOGCSR (rw) register accessor: I/O group control status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iogcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iogcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iogcsr`]
module"]
pub type IOGCSR = crate::Reg<iogcsr::IOGCSRrs>;
#[doc = "I/O group control status register"]
pub mod iogcsr;
#[doc = "IOGCR (r) register accessor: I/O group x counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iogcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iogcr`]
module"]
pub type IOGCR = crate::Reg<iogcr::IOGCRrs>;
#[doc = "I/O group x counter register"]
pub mod iogcr;
