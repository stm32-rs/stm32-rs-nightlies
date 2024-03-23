#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crl: CRL,
    crh: CRH,
    idr: IDR,
    odr: ODR,
    bsrr: BSRR,
    brr: BRR,
    lckr: LCKR,
}
impl RegisterBlock {
    #[doc = "0x00 - Port configuration register low (GPIOn_CRL)"]
    #[inline(always)]
    pub const fn crl(&self) -> &CRL {
        &self.crl
    }
    #[doc = "0x04 - Port configuration register high (GPIOn_CRL)"]
    #[inline(always)]
    pub const fn crh(&self) -> &CRH {
        &self.crh
    }
    #[doc = "0x08 - Port input data register (GPIOn_IDR)"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x0c - Port output data register (GPIOn_ODR)"]
    #[inline(always)]
    pub const fn odr(&self) -> &ODR {
        &self.odr
    }
    #[doc = "0x10 - Port bit set/reset register (GPIOn_BSRR)"]
    #[inline(always)]
    pub const fn bsrr(&self) -> &BSRR {
        &self.bsrr
    }
    #[doc = "0x14 - Port bit reset register (GPIOn_BRR)"]
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    #[doc = "0x18 - Port configuration lock register"]
    #[inline(always)]
    pub const fn lckr(&self) -> &LCKR {
        &self.lckr
    }
}
#[doc = "CRL (rw) register accessor: Port configuration register low (GPIOn_CRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crl`]
module"]
pub type CRL = crate::Reg<crl::CRLrs>;
#[doc = "Port configuration register low (GPIOn_CRL)"]
pub mod crl;
#[doc = "CRH (rw) register accessor: Port configuration register high (GPIOn_CRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crh`]
module"]
pub type CRH = crate::Reg<crh::CRHrs>;
#[doc = "Port configuration register high (GPIOn_CRL)"]
pub mod crh;
#[doc = "IDR (r) register accessor: Port input data register (GPIOn_IDR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDRrs>;
#[doc = "Port input data register (GPIOn_IDR)"]
pub mod idr;
#[doc = "ODR (rw) register accessor: Port output data register (GPIOn_ODR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`]
module"]
pub type ODR = crate::Reg<odr::ODRrs>;
#[doc = "Port output data register (GPIOn_ODR)"]
pub mod odr;
#[doc = "BSRR (w) register accessor: Port bit set/reset register (GPIOn_BSRR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`]
module"]
pub type BSRR = crate::Reg<bsrr::BSRRrs>;
#[doc = "Port bit set/reset register (GPIOn_BSRR)"]
pub mod bsrr;
#[doc = "BRR (w) register accessor: Port bit reset register (GPIOn_BRR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
pub type BRR = crate::Reg<brr::BRRrs>;
#[doc = "Port bit reset register (GPIOn_BRR)"]
pub mod brr;
#[doc = "LCKR (rw) register accessor: Port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`]
module"]
pub type LCKR = crate::Reg<lckr::LCKRrs>;
#[doc = "Port configuration lock register"]
pub mod lckr;
