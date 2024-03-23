#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: CSR,
    wdata: WDATA,
    rdata: RDATA,
}
impl RegisterBlock {
    #[doc = "0x00 - CORDIC Control Status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x04 - FMAC Write Data register"]
    #[inline(always)]
    pub const fn wdata(&self) -> &WDATA {
        &self.wdata
    }
    #[doc = "0x08 - FMAC Read Data register"]
    #[inline(always)]
    pub const fn rdata(&self) -> &RDATA {
        &self.rdata
    }
}
#[doc = "CSR (rw) register accessor: CORDIC Control Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "CORDIC Control Status register"]
pub mod csr;
#[doc = "WDATA (w) register accessor: FMAC Write Data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdata`]
module"]
pub type WDATA = crate::Reg<wdata::WDATArs>;
#[doc = "FMAC Write Data register"]
pub mod wdata;
#[doc = "RDATA (r) register accessor: FMAC Read Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdata`]
module"]
pub type RDATA = crate::Reg<rdata::RDATArs>;
#[doc = "FMAC Read Data register"]
pub mod rdata;
