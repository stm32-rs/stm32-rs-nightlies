#[repr(C)]
#[derive(Debug)]
///DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers
pub struct CH {
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    awscdr: AWSCDR,
    wdatr: WDATR,
    datinr: DATINR,
    _reserved_end: [u8; 0x0c],
}
impl CH {
    ///0x00 - channel configuration y register
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x04 - channel configuration y register
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x08 - analog watchdog and short-circuit detector register
    #[inline(always)]
    pub const fn awscdr(&self) -> &AWSCDR {
        &self.awscdr
    }
    ///0x0c - channel watchdog filter data register
    #[inline(always)]
    pub const fn wdatr(&self) -> &WDATR {
        &self.wdatr
    }
    ///0x10 - channel data input register
    #[inline(always)]
    pub const fn datinr(&self) -> &DATINR {
        &self.datinr
    }
}
/**CFGR1 (rw) register accessor: channel configuration y register

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///channel configuration y register
pub mod cfgr1;
/**CFGR2 (rw) register accessor: channel configuration y register

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///channel configuration y register
pub mod cfgr2;
/**AWSCDR (rw) register accessor: analog watchdog and short-circuit detector register

You can [`read`](crate::Reg::read) this register and get [`awscdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awscdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awscdr`] module*/
pub type AWSCDR = crate::Reg<awscdr::AWSCDRrs>;
///analog watchdog and short-circuit detector register
pub mod awscdr;
/**WDATR (rw) register accessor: channel watchdog filter data register

You can [`read`](crate::Reg::read) this register and get [`wdatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdatr`] module*/
pub type WDATR = crate::Reg<wdatr::WDATRrs>;
///channel watchdog filter data register
pub mod wdatr;
/**DATINR (rw) register accessor: channel data input register

You can [`read`](crate::Reg::read) this register and get [`datinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@datinr`] module*/
pub type DATINR = crate::Reg<datinr::DATINRrs>;
///channel data input register
pub mod datinr;
