#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctrl: CTRL,
    mux: MUX,
    _reserved2: [u8; 0x08],
    val: VAL,
    gposet: GPOSET,
    gpoclr: GPOCLR,
    gpoval: GPOVAL,
    _reserved6: [u8; 0x03d4],
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - HDP Control
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x04 - HDP multiplexing
    #[inline(always)]
    pub const fn mux(&self) -> &MUX {
        &self.mux
    }
    ///0x10 - HDP value
    #[inline(always)]
    pub const fn val(&self) -> &VAL {
        &self.val
    }
    ///0x14 - HDP GPO set
    #[inline(always)]
    pub const fn gposet(&self) -> &GPOSET {
        &self.gposet
    }
    ///0x18 - HDP GPO clear
    #[inline(always)]
    pub const fn gpoclr(&self) -> &GPOCLR {
        &self.gpoclr
    }
    ///0x1c - HDP GPO value
    #[inline(always)]
    pub const fn gpoval(&self) -> &GPOVAL {
        &self.gpoval
    }
    ///0x3f4 - HDP version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - HDP IP identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - HDP size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**CTRL (rw) register accessor: HDP Control

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:CTRL)

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRLrs>;
///HDP Control
pub mod ctrl;
/**MUX (rw) register accessor: HDP multiplexing

You can [`read`](crate::Reg::read) this register and get [`mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:MUX)

For information about available fields see [`mod@mux`] module*/
pub type MUX = crate::Reg<mux::MUXrs>;
///HDP multiplexing
pub mod mux;
/**VAL (r) register accessor: HDP value

You can [`read`](crate::Reg::read) this register and get [`val::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:VAL)

For information about available fields see [`mod@val`] module*/
pub type VAL = crate::Reg<val::VALrs>;
///HDP value
pub mod val;
/**GPOSET (w) register accessor: HDP GPO set

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gposet::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:GPOSET)

For information about available fields see [`mod@gposet`] module*/
pub type GPOSET = crate::Reg<gposet::GPOSETrs>;
///HDP GPO set
pub mod gposet;
/**GPOCLR (w) register accessor: HDP GPO clear

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpoclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:GPOCLR)

For information about available fields see [`mod@gpoclr`] module*/
pub type GPOCLR = crate::Reg<gpoclr::GPOCLRrs>;
///HDP GPO clear
pub mod gpoclr;
/**GPOVAL (rw) register accessor: HDP GPO value

You can [`read`](crate::Reg::read) this register and get [`gpoval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpoval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:GPOVAL)

For information about available fields see [`mod@gpoval`] module*/
pub type GPOVAL = crate::Reg<gpoval::GPOVALrs>;
///HDP GPO value
pub mod gpoval;
/**VERR (r) register accessor: HDP version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///HDP version register
pub mod verr;
/**IPIDR (r) register accessor: HDP IP identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///HDP IP identification register
pub mod ipidr;
/**SIDR (r) register accessor: HDP size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///HDP size identification register
pub mod sidr;
