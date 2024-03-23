#[repr(C)]
#[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
pub struct FLT {
    cr1: CR1,
    cr2: CR2,
    isr: ISR,
    icr: ICR,
    jchgr: JCHGR,
    fcr: FCR,
    jdatar: JDATAR,
    rdatar: RDATAR,
    awhtr: AWHTR,
    awltr: AWLTR,
    awsr: AWSR,
    awcfr: AWCFR,
    exmax: EXMAX,
    exmin: EXMIN,
    cnvtimr: CNVTIMR,
    _reserved_end: [u8; 0x44],
}
impl FLT {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn jchgr(&self) -> &JCHGR {
        &self.jchgr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn jdatar(&self) -> &JDATAR {
        &self.jdatar
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn rdatar(&self) -> &RDATAR {
        &self.rdatar
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn awhtr(&self) -> &AWHTR {
        &self.awhtr
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn awltr(&self) -> &AWLTR {
        &self.awltr
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn awsr(&self) -> &AWSR {
        &self.awsr
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn awcfr(&self) -> &AWCFR {
        &self.awcfr
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn exmax(&self) -> &EXMAX {
        &self.exmax
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn exmin(&self) -> &EXMIN {
        &self.exmin
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn cnvtimr(&self) -> &CNVTIMR {
        &self.cnvtimr
    }
}
#[doc = "CR1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = ""]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = ""]
pub mod cr2;
#[doc = "ISR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = ""]
pub mod isr;
#[doc = "ICR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = ""]
pub mod icr;
#[doc = "JCHGR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jchgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jchgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jchgr`]
module"]
pub type JCHGR = crate::Reg<jchgr::JCHGRrs>;
#[doc = ""]
pub mod jchgr;
#[doc = "FCR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = ""]
pub mod fcr;
#[doc = "JDATAR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdatar`]
module"]
pub type JDATAR = crate::Reg<jdatar::JDATARrs>;
#[doc = ""]
pub mod jdatar;
#[doc = "RDATAR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdatar`]
module"]
pub type RDATAR = crate::Reg<rdatar::RDATARrs>;
#[doc = ""]
pub mod rdatar;
#[doc = "AWHTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awhtr`]
module"]
pub type AWHTR = crate::Reg<awhtr::AWHTRrs>;
#[doc = ""]
pub mod awhtr;
#[doc = "AWLTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awltr`]
module"]
pub type AWLTR = crate::Reg<awltr::AWLTRrs>;
#[doc = ""]
pub mod awltr;
#[doc = "AWSR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awsr`]
module"]
pub type AWSR = crate::Reg<awsr::AWSRrs>;
#[doc = ""]
pub mod awsr;
#[doc = "AWCFR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awcfr`]
module"]
pub type AWCFR = crate::Reg<awcfr::AWCFRrs>;
#[doc = ""]
pub mod awcfr;
#[doc = "EXMAX (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exmax::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exmax`]
module"]
pub type EXMAX = crate::Reg<exmax::EXMAXrs>;
#[doc = ""]
pub mod exmax;
#[doc = "EXMIN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exmin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exmin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exmin`]
module"]
pub type EXMIN = crate::Reg<exmin::EXMINrs>;
#[doc = ""]
pub mod exmin;
#[doc = "CNVTIMR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnvtimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnvtimr`]
module"]
pub type CNVTIMR = crate::Reg<cnvtimr::CNVTIMRrs>;
#[doc = ""]
pub mod cnvtimr;
