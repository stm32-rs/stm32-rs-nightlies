#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clidr: CLIDR,
    ctr: CTR,
    ccsidr: CCSIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Cache Level ID register"]
    #[inline(always)]
    pub const fn clidr(&self) -> &CLIDR {
        &self.clidr
    }
    #[doc = "0x04 - Cache Type register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        &self.ctr
    }
    #[doc = "0x08 - Cache Size ID register"]
    #[inline(always)]
    pub const fn ccsidr(&self) -> &CCSIDR {
        &self.ccsidr
    }
}
#[doc = "CLIDR (r) register accessor: Cache Level ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clidr`]
module"]
pub type CLIDR = crate::Reg<clidr::CLIDRrs>;
#[doc = "Cache Level ID register"]
pub mod clidr;
#[doc = "CTR (r) register accessor: Cache Type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
pub type CTR = crate::Reg<ctr::CTRrs>;
#[doc = "Cache Type register"]
pub mod ctr;
#[doc = "CCSIDR (r) register accessor: Cache Size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccsidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccsidr`]
module"]
pub type CCSIDR = crate::Reg<ccsidr::CCSIDRrs>;
#[doc = "Cache Size ID register"]
pub mod ccsidr;
