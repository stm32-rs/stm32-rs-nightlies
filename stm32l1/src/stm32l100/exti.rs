#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    imr: IMR,
    emr: EMR,
    rtsr: RTSR,
    ftsr: FTSR,
    swier: SWIER,
    pr: PR,
}
impl RegisterBlock {
    #[doc = "0x00 - IMR"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x04 - EMR"]
    #[inline(always)]
    pub const fn emr(&self) -> &EMR {
        &self.emr
    }
    #[doc = "0x08 - RTSR"]
    #[inline(always)]
    pub const fn rtsr(&self) -> &RTSR {
        &self.rtsr
    }
    #[doc = "0x0c - FTSR"]
    #[inline(always)]
    pub const fn ftsr(&self) -> &FTSR {
        &self.ftsr
    }
    #[doc = "0x10 - SWIER"]
    #[inline(always)]
    pub const fn swier(&self) -> &SWIER {
        &self.swier
    }
    #[doc = "0x14 - PR"]
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
}
#[doc = "IMR (rw) register accessor: IMR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMRrs>;
#[doc = "IMR"]
pub mod imr;
#[doc = "EMR (rw) register accessor: EMR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr`]
module"]
pub type EMR = crate::Reg<emr::EMRrs>;
#[doc = "EMR"]
pub mod emr;
#[doc = "RTSR (rw) register accessor: RTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr`]
module"]
pub type RTSR = crate::Reg<rtsr::RTSRrs>;
#[doc = "RTSR"]
pub mod rtsr;
#[doc = "FTSR (rw) register accessor: FTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr`]
module"]
pub type FTSR = crate::Reg<ftsr::FTSRrs>;
#[doc = "FTSR"]
pub mod ftsr;
#[doc = "SWIER (rw) register accessor: SWIER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier`]
module"]
pub type SWIER = crate::Reg<swier::SWIERrs>;
#[doc = "SWIER"]
pub mod swier;
#[doc = "PR (rw) register accessor: PR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
pub type PR = crate::Reg<pr::PRrs>;
#[doc = "PR"]
pub mod pr;
