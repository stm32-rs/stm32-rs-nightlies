#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    chep0r: CHEP0R,
    chep1r: CHEP1R,
    chep2r: CHEP2R,
    chep3r: CHEP3R,
    chep4r: CHEP4R,
    chep5r: CHEP5R,
    chep6r: CHEP6R,
    chep7r: CHEP7R,
    _reserved8: [u8; 0x20],
    cntr: CNTR,
    istr: ISTR,
    fnr: FNR,
    daddr: DADDR,
    _reserved12: [u8; 0x04],
    lpmcsr: LPMCSR,
    bcdr: BCDR,
}
impl RegisterBlock {
    ///0x00 - USB endpoint/channel 0 register
    #[inline(always)]
    pub const fn chep0r(&self) -> &CHEP0R {
        &self.chep0r
    }
    ///0x04 - USB endpoint/channel 1 register
    #[inline(always)]
    pub const fn chep1r(&self) -> &CHEP1R {
        &self.chep1r
    }
    ///0x08 - USB endpoint/channel 2 register
    #[inline(always)]
    pub const fn chep2r(&self) -> &CHEP2R {
        &self.chep2r
    }
    ///0x0c - USB endpoint/channel 3 register
    #[inline(always)]
    pub const fn chep3r(&self) -> &CHEP3R {
        &self.chep3r
    }
    ///0x10 - USB endpoint/channel 4 register
    #[inline(always)]
    pub const fn chep4r(&self) -> &CHEP4R {
        &self.chep4r
    }
    ///0x14 - USB endpoint/channel 5 register
    #[inline(always)]
    pub const fn chep5r(&self) -> &CHEP5R {
        &self.chep5r
    }
    ///0x18 - USB endpoint/channel 6 register
    #[inline(always)]
    pub const fn chep6r(&self) -> &CHEP6R {
        &self.chep6r
    }
    ///0x1c - USB endpoint/channel 7 register
    #[inline(always)]
    pub const fn chep7r(&self) -> &CHEP7R {
        &self.chep7r
    }
    ///0x40 - USB control register
    #[inline(always)]
    pub const fn cntr(&self) -> &CNTR {
        &self.cntr
    }
    ///0x44 - USB interrupt status register
    #[inline(always)]
    pub const fn istr(&self) -> &ISTR {
        &self.istr
    }
    ///0x48 - USB frame number register
    #[inline(always)]
    pub const fn fnr(&self) -> &FNR {
        &self.fnr
    }
    ///0x4c - USB Device address
    #[inline(always)]
    pub const fn daddr(&self) -> &DADDR {
        &self.daddr
    }
    ///0x54 - LPM control and status register
    #[inline(always)]
    pub const fn lpmcsr(&self) -> &LPMCSR {
        &self.lpmcsr
    }
    ///0x58 - Battery charging detector
    #[inline(always)]
    pub const fn bcdr(&self) -> &BCDR {
        &self.bcdr
    }
}
/**CHEP0R (rw) register accessor: USB endpoint/channel 0 register

You can [`read`](crate::Reg::read) this register and get [`chep0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:CHEP0R)

For information about available fields see [`mod@chep0r`]
module*/
pub type CHEP0R = crate::Reg<chep0r::CHEP0Rrs>;
///USB endpoint/channel 0 register
pub mod chep0r;
/**CHEP1R (rw) register accessor: USB endpoint/channel 1 register

You can [`read`](crate::Reg::read) this register and get [`chep1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:CHEP1R)

For information about available fields see [`mod@chep1r`]
module*/
pub type CHEP1R = crate::Reg<chep1r::CHEP1Rrs>;
///USB endpoint/channel 1 register
pub mod chep1r;
/**CHEP2R (rw) register accessor: USB endpoint/channel 2 register

You can [`read`](crate::Reg::read) this register and get [`chep2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:CHEP2R)

For information about available fields see [`mod@chep2r`]
module*/
pub type CHEP2R = crate::Reg<chep2r::CHEP2Rrs>;
///USB endpoint/channel 2 register
pub mod chep2r;
/**CHEP3R (rw) register accessor: USB endpoint/channel 3 register

You can [`read`](crate::Reg::read) this register and get [`chep3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:CHEP3R)

For information about available fields see [`mod@chep3r`]
module*/
pub type CHEP3R = crate::Reg<chep3r::CHEP3Rrs>;
///USB endpoint/channel 3 register
pub mod chep3r;
/**CHEP4R (rw) register accessor: USB endpoint/channel 4 register

You can [`read`](crate::Reg::read) this register and get [`chep4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:CHEP4R)

For information about available fields see [`mod@chep4r`]
module*/
pub type CHEP4R = crate::Reg<chep4r::CHEP4Rrs>;
///USB endpoint/channel 4 register
pub mod chep4r;
/**CHEP5R (rw) register accessor: USB endpoint/channel 5 register

You can [`read`](crate::Reg::read) this register and get [`chep5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:CHEP5R)

For information about available fields see [`mod@chep5r`]
module*/
pub type CHEP5R = crate::Reg<chep5r::CHEP5Rrs>;
///USB endpoint/channel 5 register
pub mod chep5r;
/**CHEP6R (rw) register accessor: USB endpoint/channel 6 register

You can [`read`](crate::Reg::read) this register and get [`chep6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:CHEP6R)

For information about available fields see [`mod@chep6r`]
module*/
pub type CHEP6R = crate::Reg<chep6r::CHEP6Rrs>;
///USB endpoint/channel 6 register
pub mod chep6r;
/**CHEP7R (rw) register accessor: USB endpoint/channel 7 register

You can [`read`](crate::Reg::read) this register and get [`chep7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chep7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:CHEP7R)

For information about available fields see [`mod@chep7r`]
module*/
pub type CHEP7R = crate::Reg<chep7r::CHEP7Rrs>;
///USB endpoint/channel 7 register
pub mod chep7r;
/**CNTR (rw) register accessor: USB control register

You can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:CNTR)

For information about available fields see [`mod@cntr`]
module*/
pub type CNTR = crate::Reg<cntr::CNTRrs>;
///USB control register
pub mod cntr;
/**ISTR (rw) register accessor: USB interrupt status register

You can [`read`](crate::Reg::read) this register and get [`istr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:ISTR)

For information about available fields see [`mod@istr`]
module*/
pub type ISTR = crate::Reg<istr::ISTRrs>;
///USB interrupt status register
pub mod istr;
/**FNR (r) register accessor: USB frame number register

You can [`read`](crate::Reg::read) this register and get [`fnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:FNR)

For information about available fields see [`mod@fnr`]
module*/
pub type FNR = crate::Reg<fnr::FNRrs>;
///USB frame number register
pub mod fnr;
/**DADDR (rw) register accessor: USB Device address

You can [`read`](crate::Reg::read) this register and get [`daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:DADDR)

For information about available fields see [`mod@daddr`]
module*/
pub type DADDR = crate::Reg<daddr::DADDRrs>;
///USB Device address
pub mod daddr;
/**LPMCSR (rw) register accessor: LPM control and status register

You can [`read`](crate::Reg::read) this register and get [`lpmcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:LPMCSR)

For information about available fields see [`mod@lpmcsr`]
module*/
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSRrs>;
///LPM control and status register
pub mod lpmcsr;
/**BCDR (rw) register accessor: Battery charging detector

You can [`read`](crate::Reg::read) this register and get [`bcdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:BCDR)

For information about available fields see [`mod@bcdr`]
module*/
pub type BCDR = crate::Reg<bcdr::BCDRrs>;
///Battery charging detector
pub mod bcdr;
