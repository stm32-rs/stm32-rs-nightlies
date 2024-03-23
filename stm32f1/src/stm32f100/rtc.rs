#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crh: CRH,
    crl: CRL,
    prlh: PRLH,
    prll: PRLL,
    divh: DIVH,
    divl: DIVL,
    cnth: CNTH,
    cntl: CNTL,
    alrh: ALRH,
    alrl: ALRL,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC Control Register High"]
    #[inline(always)]
    pub const fn crh(&self) -> &CRH {
        &self.crh
    }
    #[doc = "0x04 - RTC Control Register Low"]
    #[inline(always)]
    pub const fn crl(&self) -> &CRL {
        &self.crl
    }
    #[doc = "0x08 - RTC Prescaler Load Register High"]
    #[inline(always)]
    pub const fn prlh(&self) -> &PRLH {
        &self.prlh
    }
    #[doc = "0x0c - RTC Prescaler Load Register Low"]
    #[inline(always)]
    pub const fn prll(&self) -> &PRLL {
        &self.prll
    }
    #[doc = "0x10 - RTC Prescaler Divider Register High"]
    #[inline(always)]
    pub const fn divh(&self) -> &DIVH {
        &self.divh
    }
    #[doc = "0x14 - RTC Prescaler Divider Register Low"]
    #[inline(always)]
    pub const fn divl(&self) -> &DIVL {
        &self.divl
    }
    #[doc = "0x18 - RTC Counter Register High"]
    #[inline(always)]
    pub const fn cnth(&self) -> &CNTH {
        &self.cnth
    }
    #[doc = "0x1c - RTC Counter Register Low"]
    #[inline(always)]
    pub const fn cntl(&self) -> &CNTL {
        &self.cntl
    }
    #[doc = "0x20 - RTC Alarm Register High"]
    #[inline(always)]
    pub const fn alrh(&self) -> &ALRH {
        &self.alrh
    }
    #[doc = "0x24 - RTC Alarm Register Low"]
    #[inline(always)]
    pub const fn alrl(&self) -> &ALRL {
        &self.alrl
    }
}
#[doc = "CRH (rw) register accessor: RTC Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crh`]
module"]
pub type CRH = crate::Reg<crh::CRHrs>;
#[doc = "RTC Control Register High"]
pub mod crh;
#[doc = "CRL (rw) register accessor: RTC Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crl`]
module"]
pub type CRL = crate::Reg<crl::CRLrs>;
#[doc = "RTC Control Register Low"]
pub mod crl;
#[doc = "PRLH (w) register accessor: RTC Prescaler Load Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prlh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prlh`]
module"]
pub type PRLH = crate::Reg<prlh::PRLHrs>;
#[doc = "RTC Prescaler Load Register High"]
pub mod prlh;
#[doc = "PRLL (w) register accessor: RTC Prescaler Load Register Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prll::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prll`]
module"]
pub type PRLL = crate::Reg<prll::PRLLrs>;
#[doc = "RTC Prescaler Load Register Low"]
pub mod prll;
#[doc = "DIVH (r) register accessor: RTC Prescaler Divider Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divh`]
module"]
pub type DIVH = crate::Reg<divh::DIVHrs>;
#[doc = "RTC Prescaler Divider Register High"]
pub mod divh;
#[doc = "DIVL (r) register accessor: RTC Prescaler Divider Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divl`]
module"]
pub type DIVL = crate::Reg<divl::DIVLrs>;
#[doc = "RTC Prescaler Divider Register Low"]
pub mod divl;
#[doc = "CNTH (rw) register accessor: RTC Counter Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnth`]
module"]
pub type CNTH = crate::Reg<cnth::CNTHrs>;
#[doc = "RTC Counter Register High"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: RTC Counter Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl`]
module"]
pub type CNTL = crate::Reg<cntl::CNTLrs>;
#[doc = "RTC Counter Register Low"]
pub mod cntl;
#[doc = "ALRH (w) register accessor: RTC Alarm Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrh`]
module"]
pub type ALRH = crate::Reg<alrh::ALRHrs>;
#[doc = "RTC Alarm Register High"]
pub mod alrh;
#[doc = "ALRL (w) register accessor: RTC Alarm Register Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrl`]
module"]
pub type ALRL = crate::Reg<alrl::ALRLrs>;
#[doc = "RTC Alarm Register Low"]
pub mod alrl;
