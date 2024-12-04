#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_i3c_: [u8; 0x04],
    i3c_cfgr: I3C_CFGR,
    _reserved2: [u8; 0x08],
    i3c_rdr: I3C_RDR,
    i3c_rdwr: I3C_RDWR,
    i3c_tdr: I3C_TDR,
    i3c_tdwr: I3C_TDWR,
    i3c_ibidr: I3C_IBIDR,
    i3c_tgttdr: I3C_TGTTDR,
    _reserved8: [u8; 0x08],
    i3c_sr: I3C_SR,
    i3c_ser: I3C_SER,
    _reserved10: [u8; 0x08],
    i3c_rmr: I3C_RMR,
    _reserved11: [u8; 0x0c],
    i3c_evr: I3C_EVR,
    i3c_ier: I3C_IER,
    i3c_cevr: I3C_CEVR,
    _reserved14: [u8; 0x04],
    i3c_devr0: I3C_DEVR0,
    i3c_devr1: I3C_DEVR1,
    i3c_devr2: I3C_DEVR2,
    i3c_devr3: I3C_DEVR3,
    i3c_devr4: I3C_DEVR4,
    _reserved19: [u8; 0x1c],
    i3c_maxrlr: I3C_MAXRLR,
    i3c_maxwlr: I3C_MAXWLR,
    _reserved21: [u8; 0x08],
    i3c_timingr0: I3C_TIMINGR0,
    i3c_timingr1: I3C_TIMINGR1,
    i3c_timingr2: I3C_TIMINGR2,
    _reserved24: [u8; 0x14],
    i3c_bcr: I3C_BCR,
    i3c_dcr: I3C_DCR,
    i3c_getcapr: I3C_GETCAPR,
    i3c_crcapr: I3C_CRCAPR,
    i3c_getmxdsr: I3C_GETMXDSR,
    i3c_epidr: I3C_EPIDR,
}
impl RegisterBlock {
    ///0x00 - I3C message control register alternate
    #[inline(always)]
    pub const fn i3c_cr_alternate(&self) -> &I3C_CR_ALTERNATE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - I3C message control register
    #[inline(always)]
    pub const fn i3c_cr(&self) -> &I3C_CR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x04 - I3C configuration register
    #[inline(always)]
    pub const fn i3c_cfgr(&self) -> &I3C_CFGR {
        &self.i3c_cfgr
    }
    ///0x10 - I3C receive data byte register
    #[inline(always)]
    pub const fn i3c_rdr(&self) -> &I3C_RDR {
        &self.i3c_rdr
    }
    ///0x14 - I3C receive data word register
    #[inline(always)]
    pub const fn i3c_rdwr(&self) -> &I3C_RDWR {
        &self.i3c_rdwr
    }
    ///0x18 - I3C transmit data byte register
    #[inline(always)]
    pub const fn i3c_tdr(&self) -> &I3C_TDR {
        &self.i3c_tdr
    }
    ///0x1c - I3C transmit data word register
    #[inline(always)]
    pub const fn i3c_tdwr(&self) -> &I3C_TDWR {
        &self.i3c_tdwr
    }
    ///0x20 - I3C IBI payload data register
    #[inline(always)]
    pub const fn i3c_ibidr(&self) -> &I3C_IBIDR {
        &self.i3c_ibidr
    }
    ///0x24 - I3C target transmit configuration register
    #[inline(always)]
    pub const fn i3c_tgttdr(&self) -> &I3C_TGTTDR {
        &self.i3c_tgttdr
    }
    ///0x30 - I3C status register
    #[inline(always)]
    pub const fn i3c_sr(&self) -> &I3C_SR {
        &self.i3c_sr
    }
    ///0x34 - I3C status error register
    #[inline(always)]
    pub const fn i3c_ser(&self) -> &I3C_SER {
        &self.i3c_ser
    }
    ///0x40 - I3C received message register
    #[inline(always)]
    pub const fn i3c_rmr(&self) -> &I3C_RMR {
        &self.i3c_rmr
    }
    ///0x50 - I3C event register
    #[inline(always)]
    pub const fn i3c_evr(&self) -> &I3C_EVR {
        &self.i3c_evr
    }
    ///0x54 - I3C interrupt enable register
    #[inline(always)]
    pub const fn i3c_ier(&self) -> &I3C_IER {
        &self.i3c_ier
    }
    ///0x58 - I3C clear event register
    #[inline(always)]
    pub const fn i3c_cevr(&self) -> &I3C_CEVR {
        &self.i3c_cevr
    }
    ///0x60 - I3C own device characteristics register
    #[inline(always)]
    pub const fn i3c_devr0(&self) -> &I3C_DEVR0 {
        &self.i3c_devr0
    }
    ///0x64 - I3C device 1 characteristics register
    #[inline(always)]
    pub const fn i3c_devr1(&self) -> &I3C_DEVR1 {
        &self.i3c_devr1
    }
    ///0x68 - I3C device 2 characteristics register
    #[inline(always)]
    pub const fn i3c_devr2(&self) -> &I3C_DEVR2 {
        &self.i3c_devr2
    }
    ///0x6c - I3C device 3 characteristics register
    #[inline(always)]
    pub const fn i3c_devr3(&self) -> &I3C_DEVR3 {
        &self.i3c_devr3
    }
    ///0x70 - I3C device 4 characteristics register
    #[inline(always)]
    pub const fn i3c_devr4(&self) -> &I3C_DEVR4 {
        &self.i3c_devr4
    }
    ///0x90 - I3C maximum read length register
    #[inline(always)]
    pub const fn i3c_maxrlr(&self) -> &I3C_MAXRLR {
        &self.i3c_maxrlr
    }
    ///0x94 - I3C maximum write length register
    #[inline(always)]
    pub const fn i3c_maxwlr(&self) -> &I3C_MAXWLR {
        &self.i3c_maxwlr
    }
    ///0xa0 - I3C timing register 0
    #[inline(always)]
    pub const fn i3c_timingr0(&self) -> &I3C_TIMINGR0 {
        &self.i3c_timingr0
    }
    ///0xa4 - I3C timing register 1
    #[inline(always)]
    pub const fn i3c_timingr1(&self) -> &I3C_TIMINGR1 {
        &self.i3c_timingr1
    }
    ///0xa8 - I3C timing register 2
    #[inline(always)]
    pub const fn i3c_timingr2(&self) -> &I3C_TIMINGR2 {
        &self.i3c_timingr2
    }
    ///0xc0 - I3C bus characteristics register
    #[inline(always)]
    pub const fn i3c_bcr(&self) -> &I3C_BCR {
        &self.i3c_bcr
    }
    ///0xc4 - I3C device characteristics register
    #[inline(always)]
    pub const fn i3c_dcr(&self) -> &I3C_DCR {
        &self.i3c_dcr
    }
    ///0xc8 - I3C get capability register
    #[inline(always)]
    pub const fn i3c_getcapr(&self) -> &I3C_GETCAPR {
        &self.i3c_getcapr
    }
    ///0xcc - I3C controller-role capability register
    #[inline(always)]
    pub const fn i3c_crcapr(&self) -> &I3C_CRCAPR {
        &self.i3c_crcapr
    }
    ///0xd0 - I3C get capability register
    #[inline(always)]
    pub const fn i3c_getmxdsr(&self) -> &I3C_GETMXDSR {
        &self.i3c_getmxdsr
    }
    ///0xd4 - I3C extended provisioned ID register
    #[inline(always)]
    pub const fn i3c_epidr(&self) -> &I3C_EPIDR {
        &self.i3c_epidr
    }
}
/**I3C_CR (w) register accessor: I3C message control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_CR)

For information about available fields see [`mod@i3c_cr`]
module*/
pub type I3C_CR = crate::Reg<i3c_cr::I3C_CRrs>;
///I3C message control register
pub mod i3c_cr;
/**I3C_CR_ALTERNATE (w) register accessor: I3C message control register alternate

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_cr_alternate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_CR_ALTERNATE)

For information about available fields see [`mod@i3c_cr_alternate`]
module*/
pub type I3C_CR_ALTERNATE = crate::Reg<i3c_cr_alternate::I3C_CR_ALTERNATErs>;
///I3C message control register alternate
pub mod i3c_cr_alternate;
/**I3C_CFGR (rw) register accessor: I3C configuration register

You can [`read`](crate::Reg::read) this register and get [`i3c_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_CFGR)

For information about available fields see [`mod@i3c_cfgr`]
module*/
pub type I3C_CFGR = crate::Reg<i3c_cfgr::I3C_CFGRrs>;
///I3C configuration register
pub mod i3c_cfgr;
/**I3C_RDR (r) register accessor: I3C receive data byte register

You can [`read`](crate::Reg::read) this register and get [`i3c_rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_RDR)

For information about available fields see [`mod@i3c_rdr`]
module*/
pub type I3C_RDR = crate::Reg<i3c_rdr::I3C_RDRrs>;
///I3C receive data byte register
pub mod i3c_rdr;
/**I3C_RDWR (r) register accessor: I3C receive data word register

You can [`read`](crate::Reg::read) this register and get [`i3c_rdwr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_RDWR)

For information about available fields see [`mod@i3c_rdwr`]
module*/
pub type I3C_RDWR = crate::Reg<i3c_rdwr::I3C_RDWRrs>;
///I3C receive data word register
pub mod i3c_rdwr;
/**I3C_TDR (w) register accessor: I3C transmit data byte register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_TDR)

For information about available fields see [`mod@i3c_tdr`]
module*/
pub type I3C_TDR = crate::Reg<i3c_tdr::I3C_TDRrs>;
///I3C transmit data byte register
pub mod i3c_tdr;
/**I3C_TDWR (w) register accessor: I3C transmit data word register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_tdwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_TDWR)

For information about available fields see [`mod@i3c_tdwr`]
module*/
pub type I3C_TDWR = crate::Reg<i3c_tdwr::I3C_TDWRrs>;
///I3C transmit data word register
pub mod i3c_tdwr;
/**I3C_IBIDR (rw) register accessor: I3C IBI payload data register

You can [`read`](crate::Reg::read) this register and get [`i3c_ibidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_ibidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_IBIDR)

For information about available fields see [`mod@i3c_ibidr`]
module*/
pub type I3C_IBIDR = crate::Reg<i3c_ibidr::I3C_IBIDRrs>;
///I3C IBI payload data register
pub mod i3c_ibidr;
/**I3C_TGTTDR (rw) register accessor: I3C target transmit configuration register

You can [`read`](crate::Reg::read) this register and get [`i3c_tgttdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_tgttdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_TGTTDR)

For information about available fields see [`mod@i3c_tgttdr`]
module*/
pub type I3C_TGTTDR = crate::Reg<i3c_tgttdr::I3C_TGTTDRrs>;
///I3C target transmit configuration register
pub mod i3c_tgttdr;
/**I3C_SR (r) register accessor: I3C status register

You can [`read`](crate::Reg::read) this register and get [`i3c_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_SR)

For information about available fields see [`mod@i3c_sr`]
module*/
pub type I3C_SR = crate::Reg<i3c_sr::I3C_SRrs>;
///I3C status register
pub mod i3c_sr;
/**I3C_SER (r) register accessor: I3C status error register

You can [`read`](crate::Reg::read) this register and get [`i3c_ser::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_SER)

For information about available fields see [`mod@i3c_ser`]
module*/
pub type I3C_SER = crate::Reg<i3c_ser::I3C_SERrs>;
///I3C status error register
pub mod i3c_ser;
/**I3C_RMR (r) register accessor: I3C received message register

You can [`read`](crate::Reg::read) this register and get [`i3c_rmr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_RMR)

For information about available fields see [`mod@i3c_rmr`]
module*/
pub type I3C_RMR = crate::Reg<i3c_rmr::I3C_RMRrs>;
///I3C received message register
pub mod i3c_rmr;
/**I3C_EVR (r) register accessor: I3C event register

You can [`read`](crate::Reg::read) this register and get [`i3c_evr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_EVR)

For information about available fields see [`mod@i3c_evr`]
module*/
pub type I3C_EVR = crate::Reg<i3c_evr::I3C_EVRrs>;
///I3C event register
pub mod i3c_evr;
/**I3C_IER (r) register accessor: I3C interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`i3c_ier::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_IER)

For information about available fields see [`mod@i3c_ier`]
module*/
pub type I3C_IER = crate::Reg<i3c_ier::I3C_IERrs>;
///I3C interrupt enable register
pub mod i3c_ier;
/**I3C_CEVR (w) register accessor: I3C clear event register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_cevr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_CEVR)

For information about available fields see [`mod@i3c_cevr`]
module*/
pub type I3C_CEVR = crate::Reg<i3c_cevr::I3C_CEVRrs>;
///I3C clear event register
pub mod i3c_cevr;
/**I3C_DEVR0 (rw) register accessor: I3C own device characteristics register

You can [`read`](crate::Reg::read) this register and get [`i3c_devr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_devr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_DEVR0)

For information about available fields see [`mod@i3c_devr0`]
module*/
pub type I3C_DEVR0 = crate::Reg<i3c_devr0::I3C_DEVR0rs>;
///I3C own device characteristics register
pub mod i3c_devr0;
/**I3C_DEVR1 (rw) register accessor: I3C device 1 characteristics register

You can [`read`](crate::Reg::read) this register and get [`i3c_devr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_devr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_DEVR1)

For information about available fields see [`mod@i3c_devr1`]
module*/
pub type I3C_DEVR1 = crate::Reg<i3c_devr1::I3C_DEVR1rs>;
///I3C device 1 characteristics register
pub mod i3c_devr1;
/**I3C_DEVR2 (rw) register accessor: I3C device 2 characteristics register

You can [`read`](crate::Reg::read) this register and get [`i3c_devr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_devr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_DEVR2)

For information about available fields see [`mod@i3c_devr2`]
module*/
pub type I3C_DEVR2 = crate::Reg<i3c_devr2::I3C_DEVR2rs>;
///I3C device 2 characteristics register
pub mod i3c_devr2;
/**I3C_DEVR3 (rw) register accessor: I3C device 3 characteristics register

You can [`read`](crate::Reg::read) this register and get [`i3c_devr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_devr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_DEVR3)

For information about available fields see [`mod@i3c_devr3`]
module*/
pub type I3C_DEVR3 = crate::Reg<i3c_devr3::I3C_DEVR3rs>;
///I3C device 3 characteristics register
pub mod i3c_devr3;
/**I3C_DEVR4 (rw) register accessor: I3C device 4 characteristics register

You can [`read`](crate::Reg::read) this register and get [`i3c_devr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_devr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_DEVR4)

For information about available fields see [`mod@i3c_devr4`]
module*/
pub type I3C_DEVR4 = crate::Reg<i3c_devr4::I3C_DEVR4rs>;
///I3C device 4 characteristics register
pub mod i3c_devr4;
/**I3C_MAXRLR (rw) register accessor: I3C maximum read length register

You can [`read`](crate::Reg::read) this register and get [`i3c_maxrlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_maxrlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_MAXRLR)

For information about available fields see [`mod@i3c_maxrlr`]
module*/
pub type I3C_MAXRLR = crate::Reg<i3c_maxrlr::I3C_MAXRLRrs>;
///I3C maximum read length register
pub mod i3c_maxrlr;
/**I3C_MAXWLR (rw) register accessor: I3C maximum write length register

You can [`read`](crate::Reg::read) this register and get [`i3c_maxwlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_maxwlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_MAXWLR)

For information about available fields see [`mod@i3c_maxwlr`]
module*/
pub type I3C_MAXWLR = crate::Reg<i3c_maxwlr::I3C_MAXWLRrs>;
///I3C maximum write length register
pub mod i3c_maxwlr;
/**I3C_TIMINGR0 (rw) register accessor: I3C timing register 0

You can [`read`](crate::Reg::read) this register and get [`i3c_timingr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_timingr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_TIMINGR0)

For information about available fields see [`mod@i3c_timingr0`]
module*/
pub type I3C_TIMINGR0 = crate::Reg<i3c_timingr0::I3C_TIMINGR0rs>;
///I3C timing register 0
pub mod i3c_timingr0;
/**I3C_TIMINGR1 (rw) register accessor: I3C timing register 1

You can [`read`](crate::Reg::read) this register and get [`i3c_timingr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_timingr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_TIMINGR1)

For information about available fields see [`mod@i3c_timingr1`]
module*/
pub type I3C_TIMINGR1 = crate::Reg<i3c_timingr1::I3C_TIMINGR1rs>;
///I3C timing register 1
pub mod i3c_timingr1;
/**I3C_TIMINGR2 (rw) register accessor: I3C timing register 2

You can [`read`](crate::Reg::read) this register and get [`i3c_timingr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_timingr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_TIMINGR2)

For information about available fields see [`mod@i3c_timingr2`]
module*/
pub type I3C_TIMINGR2 = crate::Reg<i3c_timingr2::I3C_TIMINGR2rs>;
///I3C timing register 2
pub mod i3c_timingr2;
/**I3C_BCR (rw) register accessor: I3C bus characteristics register

You can [`read`](crate::Reg::read) this register and get [`i3c_bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_BCR)

For information about available fields see [`mod@i3c_bcr`]
module*/
pub type I3C_BCR = crate::Reg<i3c_bcr::I3C_BCRrs>;
///I3C bus characteristics register
pub mod i3c_bcr;
/**I3C_DCR (rw) register accessor: I3C device characteristics register

You can [`read`](crate::Reg::read) this register and get [`i3c_dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_DCR)

For information about available fields see [`mod@i3c_dcr`]
module*/
pub type I3C_DCR = crate::Reg<i3c_dcr::I3C_DCRrs>;
///I3C device characteristics register
pub mod i3c_dcr;
/**I3C_GETCAPR (rw) register accessor: I3C get capability register

You can [`read`](crate::Reg::read) this register and get [`i3c_getcapr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_getcapr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_GETCAPR)

For information about available fields see [`mod@i3c_getcapr`]
module*/
pub type I3C_GETCAPR = crate::Reg<i3c_getcapr::I3C_GETCAPRrs>;
///I3C get capability register
pub mod i3c_getcapr;
/**I3C_CRCAPR (rw) register accessor: I3C controller-role capability register

You can [`read`](crate::Reg::read) this register and get [`i3c_crcapr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_crcapr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_CRCAPR)

For information about available fields see [`mod@i3c_crcapr`]
module*/
pub type I3C_CRCAPR = crate::Reg<i3c_crcapr::I3C_CRCAPRrs>;
///I3C controller-role capability register
pub mod i3c_crcapr;
/**I3C_GETMXDSR (rw) register accessor: I3C get capability register

You can [`read`](crate::Reg::read) this register and get [`i3c_getmxdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_getmxdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_GETMXDSR)

For information about available fields see [`mod@i3c_getmxdsr`]
module*/
pub type I3C_GETMXDSR = crate::Reg<i3c_getmxdsr::I3C_GETMXDSRrs>;
///I3C get capability register
pub mod i3c_getmxdsr;
/**I3C_EPIDR (rw) register accessor: I3C extended provisioned ID register

You can [`read`](crate::Reg::read) this register and get [`i3c_epidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_epidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_EPIDR)

For information about available fields see [`mod@i3c_epidr`]
module*/
pub type I3C_EPIDR = crate::Reg<i3c_epidr::I3C_EPIDRrs>;
///I3C extended provisioned ID register
pub mod i3c_epidr;
