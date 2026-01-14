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
}
impl RegisterBlock {
    ///0x00 - HDP control register
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x04 - HDP multiplexer control register
    #[inline(always)]
    pub const fn mux(&self) -> &MUX {
        &self.mux
    }
    ///0x10 - HDP read back value register
    #[inline(always)]
    pub const fn val(&self) -> &VAL {
        &self.val
    }
    ///0x14 - HDP general-purpose output set register
    #[inline(always)]
    pub const fn gposet(&self) -> &GPOSET {
        &self.gposet
    }
    ///0x18 - HDP general purpose output clear register
    #[inline(always)]
    pub const fn gpoclr(&self) -> &GPOCLR {
        &self.gpoclr
    }
    ///0x1c - HDP general purpose output value register
    #[inline(always)]
    pub const fn gpoval(&self) -> &GPOVAL {
        &self.gpoval
    }
}
/**CTRL (rw) register accessor: HDP control register

You can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HDP:CTRL)

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRLrs>;
///HDP control register
pub mod ctrl;
/**MUX (rw) register accessor: HDP multiplexer control register

You can [`read`](crate::Reg::read) this register and get [`mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HDP:MUX)

For information about available fields see [`mod@mux`] module*/
pub type MUX = crate::Reg<mux::MUXrs>;
///HDP multiplexer control register
pub mod mux;
/**VAL (r) register accessor: HDP read back value register

You can [`read`](crate::Reg::read) this register and get [`val::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HDP:VAL)

For information about available fields see [`mod@val`] module*/
pub type VAL = crate::Reg<val::VALrs>;
///HDP read back value register
pub mod val;
/**GPOSET (w) register accessor: HDP general-purpose output set register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gposet::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HDP:GPOSET)

For information about available fields see [`mod@gposet`] module*/
pub type GPOSET = crate::Reg<gposet::GPOSETrs>;
///HDP general-purpose output set register
pub mod gposet;
/**GPOCLR (w) register accessor: HDP general purpose output clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpoclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HDP:GPOCLR)

For information about available fields see [`mod@gpoclr`] module*/
pub type GPOCLR = crate::Reg<gpoclr::GPOCLRrs>;
///HDP general purpose output clear register
pub mod gpoclr;
/**GPOVAL (rw) register accessor: HDP general purpose output value register

You can [`read`](crate::Reg::read) this register and get [`gpoval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpoval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HDP:GPOVAL)

For information about available fields see [`mod@gpoval`] module*/
pub type GPOVAL = crate::Reg<gpoval::GPOVALrs>;
///HDP general purpose output value register
pub mod gpoval;
