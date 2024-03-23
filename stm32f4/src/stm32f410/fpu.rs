#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fpccr: FPCCR,
    fpcar: FPCAR,
    fpscr: FPSCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Floating-point context control register"]
    #[inline(always)]
    pub const fn fpccr(&self) -> &FPCCR {
        &self.fpccr
    }
    #[doc = "0x04 - Floating-point context address register"]
    #[inline(always)]
    pub const fn fpcar(&self) -> &FPCAR {
        &self.fpcar
    }
    #[doc = "0x08 - Floating-point status control register"]
    #[inline(always)]
    pub const fn fpscr(&self) -> &FPSCR {
        &self.fpscr
    }
}
#[doc = "FPCCR (rw) register accessor: Floating-point context control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpccr`]
module"]
pub type FPCCR = crate::Reg<fpccr::FPCCRrs>;
#[doc = "Floating-point context control register"]
pub mod fpccr;
#[doc = "FPCAR (rw) register accessor: Floating-point context address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpcar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpcar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpcar`]
module"]
pub type FPCAR = crate::Reg<fpcar::FPCARrs>;
#[doc = "Floating-point context address register"]
pub mod fpcar;
#[doc = "FPSCR (rw) register accessor: Floating-point status control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpscr`]
module"]
pub type FPSCR = crate::Reg<fpscr::FPSCRrs>;
#[doc = "Floating-point status control register"]
pub mod fpscr;
