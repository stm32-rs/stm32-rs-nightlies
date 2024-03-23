#[repr(C)]
#[doc = "DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers"]
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
    #[doc = "0x00 - DFSDM control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - DFSDM control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - DFSDM interrupt and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x0c - DFSDM interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x10 - DFSDM injected channel group selection register"]
    #[inline(always)]
    pub const fn jchgr(&self) -> &JCHGR {
        &self.jchgr
    }
    #[doc = "0x14 - DFSDM filter control register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x18 - DFSDM data register for injected group"]
    #[inline(always)]
    pub const fn jdatar(&self) -> &JDATAR {
        &self.jdatar
    }
    #[doc = "0x1c - DFSDM data register for the regular channel"]
    #[inline(always)]
    pub const fn rdatar(&self) -> &RDATAR {
        &self.rdatar
    }
    #[doc = "0x20 - DFSDM analog watchdog high threshold register"]
    #[inline(always)]
    pub const fn awhtr(&self) -> &AWHTR {
        &self.awhtr
    }
    #[doc = "0x24 - DFSDM analog watchdog low threshold register"]
    #[inline(always)]
    pub const fn awltr(&self) -> &AWLTR {
        &self.awltr
    }
    #[doc = "0x28 - DFSDM analog watchdog status register"]
    #[inline(always)]
    pub const fn awsr(&self) -> &AWSR {
        &self.awsr
    }
    #[doc = "0x2c - DFSDM analog watchdog clear flag register"]
    #[inline(always)]
    pub const fn awcfr(&self) -> &AWCFR {
        &self.awcfr
    }
    #[doc = "0x30 - DFSDM Extremes detector maximum register"]
    #[inline(always)]
    pub const fn exmax(&self) -> &EXMAX {
        &self.exmax
    }
    #[doc = "0x34 - DFSDM Extremes detector minimum register"]
    #[inline(always)]
    pub const fn exmin(&self) -> &EXMIN {
        &self.exmin
    }
    #[doc = "0x38 - DFSDM conversion timer register"]
    #[inline(always)]
    pub const fn cnvtimr(&self) -> &CNVTIMR {
        &self.cnvtimr
    }
}
#[doc = "CR1 (rw) register accessor: DFSDM control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "DFSDM control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: DFSDM control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "DFSDM control register 2"]
pub mod cr2;
#[doc = "ISR (r) register accessor: DFSDM interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "DFSDM interrupt and status register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: DFSDM interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICRrs>;
#[doc = "DFSDM interrupt flag clear register"]
pub mod icr;
#[doc = "JCHGR (rw) register accessor: DFSDM injected channel group selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jchgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jchgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jchgr`]
module"]
pub type JCHGR = crate::Reg<jchgr::JCHGRrs>;
#[doc = "DFSDM injected channel group selection register"]
pub mod jchgr;
#[doc = "FCR (rw) register accessor: DFSDM filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = "DFSDM filter control register"]
pub mod fcr;
#[doc = "JDATAR (r) register accessor: DFSDM data register for injected group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdatar`]
module"]
pub type JDATAR = crate::Reg<jdatar::JDATARrs>;
#[doc = "DFSDM data register for injected group"]
pub mod jdatar;
#[doc = "RDATAR (r) register accessor: DFSDM data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdatar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdatar`]
module"]
pub type RDATAR = crate::Reg<rdatar::RDATARrs>;
#[doc = "DFSDM data register for the regular channel"]
pub mod rdatar;
#[doc = "AWHTR (rw) register accessor: DFSDM analog watchdog high threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awhtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awhtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awhtr`]
module"]
pub type AWHTR = crate::Reg<awhtr::AWHTRrs>;
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod awhtr;
#[doc = "AWLTR (rw) register accessor: DFSDM analog watchdog low threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awltr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awltr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awltr`]
module"]
pub type AWLTR = crate::Reg<awltr::AWLTRrs>;
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod awltr;
#[doc = "AWSR (r) register accessor: DFSDM analog watchdog status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awsr`]
module"]
pub type AWSR = crate::Reg<awsr::AWSRrs>;
#[doc = "DFSDM analog watchdog status register"]
pub mod awsr;
#[doc = "AWCFR (rw) register accessor: DFSDM analog watchdog clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awcfr`]
module"]
pub type AWCFR = crate::Reg<awcfr::AWCFRrs>;
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod awcfr;
#[doc = "EXMAX (r) register accessor: DFSDM Extremes detector maximum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exmax::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exmax`]
module"]
pub type EXMAX = crate::Reg<exmax::EXMAXrs>;
#[doc = "DFSDM Extremes detector maximum register"]
pub mod exmax;
#[doc = "EXMIN (r) register accessor: DFSDM Extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exmin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exmin`]
module"]
pub type EXMIN = crate::Reg<exmin::EXMINrs>;
#[doc = "DFSDM Extremes detector minimum register"]
pub mod exmin;
#[doc = "CNVTIMR (r) register accessor: DFSDM conversion timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnvtimr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnvtimr`]
module"]
pub type CNVTIMR = crate::Reg<cnvtimr::CNVTIMRrs>;
#[doc = "DFSDM conversion timer register"]
pub mod cnvtimr;
