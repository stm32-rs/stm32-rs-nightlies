#[repr(C)]
#[derive(Debug)]
///DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers
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
    ///0x00 - DFSDM control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x04 - DFSDM control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x08 - DFSDM interrupt and status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x0c - DFSDM interrupt flag clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x10 - DFSDM injected channel group selection register
    #[inline(always)]
    pub const fn jchgr(&self) -> &JCHGR {
        &self.jchgr
    }
    ///0x14 - DFSDM filter control register
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    ///0x18 - DFSDM data register for injected group
    #[inline(always)]
    pub const fn jdatar(&self) -> &JDATAR {
        &self.jdatar
    }
    ///0x1c - DFSDM data register for the regular channel
    #[inline(always)]
    pub const fn rdatar(&self) -> &RDATAR {
        &self.rdatar
    }
    ///0x20 - DFSDM analog watchdog high threshold register
    #[inline(always)]
    pub const fn awhtr(&self) -> &AWHTR {
        &self.awhtr
    }
    ///0x24 - DFSDM analog watchdog low threshold register
    #[inline(always)]
    pub const fn awltr(&self) -> &AWLTR {
        &self.awltr
    }
    ///0x28 - DFSDM analog watchdog status register
    #[inline(always)]
    pub const fn awsr(&self) -> &AWSR {
        &self.awsr
    }
    ///0x2c - DFSDM analog watchdog clear flag register
    #[inline(always)]
    pub const fn awcfr(&self) -> &AWCFR {
        &self.awcfr
    }
    ///0x30 - DFSDM Extremes detector maximum register
    #[inline(always)]
    pub const fn exmax(&self) -> &EXMAX {
        &self.exmax
    }
    ///0x34 - DFSDM Extremes detector minimum register
    #[inline(always)]
    pub const fn exmin(&self) -> &EXMIN {
        &self.exmin
    }
    ///0x38 - DFSDM conversion timer register
    #[inline(always)]
    pub const fn cnvtimr(&self) -> &CNVTIMR {
        &self.cnvtimr
    }
}
/**CR1 (rw) register accessor: DFSDM control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///DFSDM control register 1
pub mod cr1;
/**CR2 (rw) register accessor: DFSDM control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///DFSDM control register 2
pub mod cr2;
/**ISR (r) register accessor: DFSDM interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///DFSDM interrupt and status register
pub mod isr;
/**ICR (rw) register accessor: DFSDM interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///DFSDM interrupt flag clear register
pub mod icr;
/**JCHGR (rw) register accessor: DFSDM injected channel group selection register

You can [`read`](crate::Reg::read) this register and get [`jchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jchgr`] module*/
pub type JCHGR = crate::Reg<jchgr::JCHGRrs>;
///DFSDM injected channel group selection register
pub mod jchgr;
/**FCR (rw) register accessor: DFSDM filter control register

You can [`read`](crate::Reg::read) this register and get [`fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcr`] module*/
pub type FCR = crate::Reg<fcr::FCRrs>;
///DFSDM filter control register
pub mod fcr;
/**JDATAR (r) register accessor: DFSDM data register for injected group

You can [`read`](crate::Reg::read) this register and get [`jdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jdatar`] module*/
pub type JDATAR = crate::Reg<jdatar::JDATARrs>;
///DFSDM data register for injected group
pub mod jdatar;
/**RDATAR (r) register accessor: DFSDM data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`rdatar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdatar`] module*/
pub type RDATAR = crate::Reg<rdatar::RDATARrs>;
///DFSDM data register for the regular channel
pub mod rdatar;
/**AWHTR (rw) register accessor: DFSDM analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`awhtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awhtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awhtr`] module*/
pub type AWHTR = crate::Reg<awhtr::AWHTRrs>;
///DFSDM analog watchdog high threshold register
pub mod awhtr;
/**AWLTR (rw) register accessor: DFSDM analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`awltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awltr`] module*/
pub type AWLTR = crate::Reg<awltr::AWLTRrs>;
///DFSDM analog watchdog low threshold register
pub mod awltr;
/**AWSR (r) register accessor: DFSDM analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`awsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awsr`] module*/
pub type AWSR = crate::Reg<awsr::AWSRrs>;
///DFSDM analog watchdog status register
pub mod awsr;
/**AWCFR (rw) register accessor: DFSDM analog watchdog clear flag register

You can [`read`](crate::Reg::read) this register and get [`awcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@awcfr`] module*/
pub type AWCFR = crate::Reg<awcfr::AWCFRrs>;
///DFSDM analog watchdog clear flag register
pub mod awcfr;
/**EXMAX (r) register accessor: DFSDM Extremes detector maximum register

You can [`read`](crate::Reg::read) this register and get [`exmax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@exmax`] module*/
pub type EXMAX = crate::Reg<exmax::EXMAXrs>;
///DFSDM Extremes detector maximum register
pub mod exmax;
/**EXMIN (r) register accessor: DFSDM Extremes detector minimum register

You can [`read`](crate::Reg::read) this register and get [`exmin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@exmin`] module*/
pub type EXMIN = crate::Reg<exmin::EXMINrs>;
///DFSDM Extremes detector minimum register
pub mod exmin;
/**CNVTIMR (r) register accessor: DFSDM conversion timer register

You can [`read`](crate::Reg::read) this register and get [`cnvtimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cnvtimr`] module*/
pub type CNVTIMR = crate::Reg<cnvtimr::CNVTIMRrs>;
///DFSDM conversion timer register
pub mod cnvtimr;
