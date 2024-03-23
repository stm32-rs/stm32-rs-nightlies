#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: CSR,
    wdata: WDATA,
    rdata: RDATA,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x04 - Argument register"]
    #[inline(always)]
    pub const fn wdata(&self) -> &WDATA {
        &self.wdata
    }
    #[doc = "0x08 - Result register"]
    #[inline(always)]
    pub const fn rdata(&self) -> &RDATA {
        &self.rdata
    }
}
#[doc = "CSR (rw) register accessor: Control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "Control and status register"]
pub mod csr;
#[doc = "WDATA (rw) register accessor: Argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdata`]
module"]
pub type WDATA = crate::Reg<wdata::WDATArs>;
#[doc = "Argument register"]
pub mod wdata;
#[doc = "RDATA (rw) register accessor: Result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdata`]
module"]
pub type RDATA = crate::Reg<rdata::RDATArs>;
#[doc = "Result register"]
pub mod rdata;
