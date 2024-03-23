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
    iog1cr: IOG1CR,
    iog2cr: IOG2CR,
    iog3cr: IOG3CR,
    iog4cr: IOG4CR,
    iog5cr: IOG5CR,
    iog6cr: IOG6CR,
    iog7cr: IOG7CR,
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
    #[doc = "0x34 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog1cr(&self) -> &IOG1CR {
        &self.iog1cr
    }
    #[doc = "0x38 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog2cr(&self) -> &IOG2CR {
        &self.iog2cr
    }
    #[doc = "0x3c - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog3cr(&self) -> &IOG3CR {
        &self.iog3cr
    }
    #[doc = "0x40 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog4cr(&self) -> &IOG4CR {
        &self.iog4cr
    }
    #[doc = "0x44 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog5cr(&self) -> &IOG5CR {
        &self.iog5cr
    }
    #[doc = "0x48 - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog6cr(&self) -> &IOG6CR {
        &self.iog6cr
    }
    #[doc = "0x4c - I/O group x counter register"]
    #[inline(always)]
    pub const fn iog7cr(&self) -> &IOG7CR {
        &self.iog7cr
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
#[doc = "IOG1CR (r) register accessor: I/O group x counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iog1cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iog1cr`]
module"]
pub type IOG1CR = crate::Reg<iog1cr::IOG1CRrs>;
#[doc = "I/O group x counter register"]
pub mod iog1cr;
#[doc = "IOG2CR (r) register accessor: I/O group x counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iog2cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iog2cr`]
module"]
pub type IOG2CR = crate::Reg<iog2cr::IOG2CRrs>;
#[doc = "I/O group x counter register"]
pub mod iog2cr;
#[doc = "IOG3CR (r) register accessor: I/O group x counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iog3cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iog3cr`]
module"]
pub type IOG3CR = crate::Reg<iog3cr::IOG3CRrs>;
#[doc = "I/O group x counter register"]
pub mod iog3cr;
#[doc = "IOG4CR (r) register accessor: I/O group x counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iog4cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iog4cr`]
module"]
pub type IOG4CR = crate::Reg<iog4cr::IOG4CRrs>;
#[doc = "I/O group x counter register"]
pub mod iog4cr;
#[doc = "IOG5CR (r) register accessor: I/O group x counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iog5cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iog5cr`]
module"]
pub type IOG5CR = crate::Reg<iog5cr::IOG5CRrs>;
#[doc = "I/O group x counter register"]
pub mod iog5cr;
#[doc = "IOG6CR (r) register accessor: I/O group x counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iog6cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iog6cr`]
module"]
pub type IOG6CR = crate::Reg<iog6cr::IOG6CRrs>;
#[doc = "I/O group x counter register"]
pub mod iog6cr;
#[doc = "IOG7CR (r) register accessor: I/O group x counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iog7cr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iog7cr`]
module"]
pub type IOG7CR = crate::Reg<iog7cr::IOG7CRrs>;
#[doc = "I/O group x counter register"]
pub mod iog7cr;
