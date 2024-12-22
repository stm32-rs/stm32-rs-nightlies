#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    chepr: [CHEPR; 8],
    _reserved1: [u8; 0x20],
    cntr: CNTR,
    istr: ISTR,
    fnr: FNR,
    daddr: DADDR,
    _reserved5: [u8; 0x04],
    lpmcsr: LPMCSR,
    bcdr: BCDR,
}
impl RegisterBlock {
    ///0x00..0x20 - USB endpoint/channel %s register
    #[inline(always)]
    pub const fn chepr(&self, n: usize) -> &CHEPR {
        &self.chepr[n]
    }
    ///Iterator for array of:
    ///0x00..0x20 - USB endpoint/channel %s register
    #[inline(always)]
    pub fn chepr_iter(&self) -> impl Iterator<Item = &CHEPR> {
        self.chepr.iter()
    }
    ///0x00 - USB endpoint/channel 0 register
    #[inline(always)]
    pub const fn chep0r(&self) -> &CHEPR {
        self.chepr(0)
    }
    ///0x04 - USB endpoint/channel 1 register
    #[inline(always)]
    pub const fn chep1r(&self) -> &CHEPR {
        self.chepr(1)
    }
    ///0x08 - USB endpoint/channel 2 register
    #[inline(always)]
    pub const fn chep2r(&self) -> &CHEPR {
        self.chepr(2)
    }
    ///0x0c - USB endpoint/channel 3 register
    #[inline(always)]
    pub const fn chep3r(&self) -> &CHEPR {
        self.chepr(3)
    }
    ///0x10 - USB endpoint/channel 4 register
    #[inline(always)]
    pub const fn chep4r(&self) -> &CHEPR {
        self.chepr(4)
    }
    ///0x14 - USB endpoint/channel 5 register
    #[inline(always)]
    pub const fn chep5r(&self) -> &CHEPR {
        self.chepr(5)
    }
    ///0x18 - USB endpoint/channel 6 register
    #[inline(always)]
    pub const fn chep6r(&self) -> &CHEPR {
        self.chepr(6)
    }
    ///0x1c - USB endpoint/channel 7 register
    #[inline(always)]
    pub const fn chep7r(&self) -> &CHEPR {
        self.chepr(7)
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
/**CHEPR (rw) register accessor: USB endpoint/channel %s register

You can [`read`](crate::Reg::read) this register and get [`chepr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chepr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#USB:CHEP[0]R)

For information about available fields see [`mod@chepr`]
module*/
pub type CHEPR = crate::Reg<chepr::CHEPRrs>;
///USB endpoint/channel %s register
pub mod chepr;
/**CNTR (rw) register accessor: USB control register

You can [`read`](crate::Reg::read) this register and get [`cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#USB:CNTR)

For information about available fields see [`mod@cntr`]
module*/
pub type CNTR = crate::Reg<cntr::CNTRrs>;
///USB control register
pub mod cntr;
/**ISTR (rw) register accessor: USB interrupt status register

You can [`read`](crate::Reg::read) this register and get [`istr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#USB:ISTR)

For information about available fields see [`mod@istr`]
module*/
pub type ISTR = crate::Reg<istr::ISTRrs>;
///USB interrupt status register
pub mod istr;
/**FNR (r) register accessor: USB frame number register

You can [`read`](crate::Reg::read) this register and get [`fnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#USB:FNR)

For information about available fields see [`mod@fnr`]
module*/
pub type FNR = crate::Reg<fnr::FNRrs>;
///USB frame number register
pub mod fnr;
/**DADDR (rw) register accessor: USB Device address

You can [`read`](crate::Reg::read) this register and get [`daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#USB:DADDR)

For information about available fields see [`mod@daddr`]
module*/
pub type DADDR = crate::Reg<daddr::DADDRrs>;
///USB Device address
pub mod daddr;
/**LPMCSR (rw) register accessor: LPM control and status register

You can [`read`](crate::Reg::read) this register and get [`lpmcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#USB:LPMCSR)

For information about available fields see [`mod@lpmcsr`]
module*/
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSRrs>;
///LPM control and status register
pub mod lpmcsr;
/**BCDR (rw) register accessor: Battery charging detector

You can [`read`](crate::Reg::read) this register and get [`bcdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#USB:BCDR)

For information about available fields see [`mod@bcdr`]
module*/
pub type BCDR = crate::Reg<bcdr::BCDRrs>;
///Battery charging detector
pub mod bcdr;
