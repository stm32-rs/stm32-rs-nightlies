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
    #[doc = "0x04 - CORDIC argument register"]
    #[inline(always)]
    pub const fn wdata(&self) -> &WDATA {
        &self.wdata
    }
    #[doc = "0x08 - CORDIC result register"]
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
#[doc = "WDATA (rw) register accessor: CORDIC argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdata`]
module"]
pub type WDATA = crate::Reg<wdata::WDATArs>;
#[doc = "CORDIC argument register"]
pub mod wdata;
#[doc = "RDATA (r) register accessor: CORDIC result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdata`]
module"]
pub type RDATA = crate::Reg<rdata::RDATArs>;
#[doc = "CORDIC result register"]
pub mod rdata;
