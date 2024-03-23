#[repr(C)]
#[doc = "DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
pub struct CH {
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    awscdr: AWSCDR,
    wdatr: WDATR,
    datinr: DATINR,
    dlyr: DLYR,
    _reserved_end: [u8; 0x08],
}
impl CH {
    #[doc = "0x00 - DFSDM channel 0 configuration register"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    #[doc = "0x04 - DFSDM channel 0 configuration register"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x08 - DFSDM channel 0 analog watchdog and short-circuit detector register"]
    #[inline(always)]
    pub const fn awscdr(&self) -> &AWSCDR {
        &self.awscdr
    }
    #[doc = "0x0c - DFSDM channel 0 watchdog filter data register"]
    #[inline(always)]
    pub const fn wdatr(&self) -> &WDATR {
        &self.wdatr
    }
    #[doc = "0x10 - DFSDM channel 0 data input register"]
    #[inline(always)]
    pub const fn datinr(&self) -> &DATINR {
        &self.datinr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn dlyr(&self) -> &DLYR {
        &self.dlyr
    }
}
#[doc = "CFGR1 (rw) register accessor: DFSDM channel 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
#[doc = "DFSDM channel 0 configuration register"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: DFSDM channel 0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "DFSDM channel 0 configuration register"]
pub mod cfgr2;
#[doc = "AWSCDR (rw) register accessor: DFSDM channel 0 analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awscdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awscdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awscdr`]
module"]
pub type AWSCDR = crate::Reg<awscdr::AWSCDRrs>;
#[doc = "DFSDM channel 0 analog watchdog and short-circuit detector register"]
pub mod awscdr;
#[doc = "WDATR (r) register accessor: DFSDM channel 0 watchdog filter data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdatr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdatr`]
module"]
pub type WDATR = crate::Reg<wdatr::WDATRrs>;
#[doc = "DFSDM channel 0 watchdog filter data register"]
pub mod wdatr;
#[doc = "DATINR (rw) register accessor: DFSDM channel 0 data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datinr`]
module"]
pub type DATINR = crate::Reg<datinr::DATINRrs>;
#[doc = "DFSDM channel 0 data input register"]
pub mod datinr;
#[doc = "DLYR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlyr`]
module"]
pub type DLYR = crate::Reg<dlyr::DLYRrs>;
#[doc = ""]
pub mod dlyr;
