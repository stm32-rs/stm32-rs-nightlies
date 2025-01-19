#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    usb_chep0r: USB_CHEP0R,
    usb_chep1r: USB_CHEP1R,
    usb_chep2r: USB_CHEP2R,
    usb_chep3r: USB_CHEP3R,
    usb_chep4r: USB_CHEP4R,
    usb_chep5r: USB_CHEP5R,
    usb_chep6r: USB_CHEP6R,
    usb_chep7r: USB_CHEP7R,
    _reserved8: [u8; 0x20],
    usb_cntr: USB_CNTR,
    usb_istr: USB_ISTR,
    usb_fnr: USB_FNR,
    usb_daddr: USB_DADDR,
    _reserved12: [u8; 0x04],
    usb_lpmcsr: USB_LPMCSR,
    usb_bcdr: USB_BCDR,
}
impl RegisterBlock {
    ///0x00 - USB endpoint/channel 0 register
    #[inline(always)]
    pub const fn usb_chep0r(&self) -> &USB_CHEP0R {
        &self.usb_chep0r
    }
    ///0x04 - USB endpoint/channel 1 register
    #[inline(always)]
    pub const fn usb_chep1r(&self) -> &USB_CHEP1R {
        &self.usb_chep1r
    }
    ///0x08 - USB endpoint/channel 2 register
    #[inline(always)]
    pub const fn usb_chep2r(&self) -> &USB_CHEP2R {
        &self.usb_chep2r
    }
    ///0x0c - USB endpoint/channel 3 register
    #[inline(always)]
    pub const fn usb_chep3r(&self) -> &USB_CHEP3R {
        &self.usb_chep3r
    }
    ///0x10 - USB endpoint/channel 4 register
    #[inline(always)]
    pub const fn usb_chep4r(&self) -> &USB_CHEP4R {
        &self.usb_chep4r
    }
    ///0x14 - USB endpoint/channel 5 register
    #[inline(always)]
    pub const fn usb_chep5r(&self) -> &USB_CHEP5R {
        &self.usb_chep5r
    }
    ///0x18 - USB endpoint/channel 6 register
    #[inline(always)]
    pub const fn usb_chep6r(&self) -> &USB_CHEP6R {
        &self.usb_chep6r
    }
    ///0x1c - USB endpoint/channel 7 register
    #[inline(always)]
    pub const fn usb_chep7r(&self) -> &USB_CHEP7R {
        &self.usb_chep7r
    }
    ///0x40 - USB control register
    #[inline(always)]
    pub const fn usb_cntr(&self) -> &USB_CNTR {
        &self.usb_cntr
    }
    ///0x44 - USB interrupt status register
    #[inline(always)]
    pub const fn usb_istr(&self) -> &USB_ISTR {
        &self.usb_istr
    }
    ///0x48 - USB frame number register
    #[inline(always)]
    pub const fn usb_fnr(&self) -> &USB_FNR {
        &self.usb_fnr
    }
    ///0x4c - USB Device address
    #[inline(always)]
    pub const fn usb_daddr(&self) -> &USB_DADDR {
        &self.usb_daddr
    }
    ///0x54 - LPM control and status register
    #[inline(always)]
    pub const fn usb_lpmcsr(&self) -> &USB_LPMCSR {
        &self.usb_lpmcsr
    }
    ///0x58 - Battery charging detector
    #[inline(always)]
    pub const fn usb_bcdr(&self) -> &USB_BCDR {
        &self.usb_bcdr
    }
}
/**USB_CHEP0R (rw) register accessor: USB endpoint/channel 0 register

You can [`read`](crate::Reg::read) this register and get [`usb_chep0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_chep0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_CHEP0R)

For information about available fields see [`mod@usb_chep0r`]
module*/
pub type USB_CHEP0R = crate::Reg<usb_chep0r::USB_CHEP0Rrs>;
///USB endpoint/channel 0 register
pub mod usb_chep0r;
/**USB_CHEP1R (rw) register accessor: USB endpoint/channel 1 register

You can [`read`](crate::Reg::read) this register and get [`usb_chep1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_chep1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_CHEP1R)

For information about available fields see [`mod@usb_chep1r`]
module*/
pub type USB_CHEP1R = crate::Reg<usb_chep1r::USB_CHEP1Rrs>;
///USB endpoint/channel 1 register
pub mod usb_chep1r;
/**USB_CHEP2R (rw) register accessor: USB endpoint/channel 2 register

You can [`read`](crate::Reg::read) this register and get [`usb_chep2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_chep2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_CHEP2R)

For information about available fields see [`mod@usb_chep2r`]
module*/
pub type USB_CHEP2R = crate::Reg<usb_chep2r::USB_CHEP2Rrs>;
///USB endpoint/channel 2 register
pub mod usb_chep2r;
/**USB_CHEP3R (rw) register accessor: USB endpoint/channel 3 register

You can [`read`](crate::Reg::read) this register and get [`usb_chep3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_chep3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_CHEP3R)

For information about available fields see [`mod@usb_chep3r`]
module*/
pub type USB_CHEP3R = crate::Reg<usb_chep3r::USB_CHEP3Rrs>;
///USB endpoint/channel 3 register
pub mod usb_chep3r;
/**USB_CHEP4R (rw) register accessor: USB endpoint/channel 4 register

You can [`read`](crate::Reg::read) this register and get [`usb_chep4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_chep4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_CHEP4R)

For information about available fields see [`mod@usb_chep4r`]
module*/
pub type USB_CHEP4R = crate::Reg<usb_chep4r::USB_CHEP4Rrs>;
///USB endpoint/channel 4 register
pub mod usb_chep4r;
/**USB_CHEP5R (rw) register accessor: USB endpoint/channel 5 register

You can [`read`](crate::Reg::read) this register and get [`usb_chep5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_chep5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_CHEP5R)

For information about available fields see [`mod@usb_chep5r`]
module*/
pub type USB_CHEP5R = crate::Reg<usb_chep5r::USB_CHEP5Rrs>;
///USB endpoint/channel 5 register
pub mod usb_chep5r;
/**USB_CHEP6R (rw) register accessor: USB endpoint/channel 6 register

You can [`read`](crate::Reg::read) this register and get [`usb_chep6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_chep6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_CHEP6R)

For information about available fields see [`mod@usb_chep6r`]
module*/
pub type USB_CHEP6R = crate::Reg<usb_chep6r::USB_CHEP6Rrs>;
///USB endpoint/channel 6 register
pub mod usb_chep6r;
/**USB_CHEP7R (rw) register accessor: USB endpoint/channel 7 register

You can [`read`](crate::Reg::read) this register and get [`usb_chep7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_chep7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_CHEP7R)

For information about available fields see [`mod@usb_chep7r`]
module*/
pub type USB_CHEP7R = crate::Reg<usb_chep7r::USB_CHEP7Rrs>;
///USB endpoint/channel 7 register
pub mod usb_chep7r;
/**USB_CNTR (rw) register accessor: USB control register

You can [`read`](crate::Reg::read) this register and get [`usb_cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_CNTR)

For information about available fields see [`mod@usb_cntr`]
module*/
pub type USB_CNTR = crate::Reg<usb_cntr::USB_CNTRrs>;
///USB control register
pub mod usb_cntr;
/**USB_ISTR (rw) register accessor: USB interrupt status register

You can [`read`](crate::Reg::read) this register and get [`usb_istr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_istr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_ISTR)

For information about available fields see [`mod@usb_istr`]
module*/
pub type USB_ISTR = crate::Reg<usb_istr::USB_ISTRrs>;
///USB interrupt status register
pub mod usb_istr;
/**USB_FNR (r) register accessor: USB frame number register

You can [`read`](crate::Reg::read) this register and get [`usb_fnr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_FNR)

For information about available fields see [`mod@usb_fnr`]
module*/
pub type USB_FNR = crate::Reg<usb_fnr::USB_FNRrs>;
///USB frame number register
pub mod usb_fnr;
/**USB_DADDR (rw) register accessor: USB Device address

You can [`read`](crate::Reg::read) this register and get [`usb_daddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_daddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_DADDR)

For information about available fields see [`mod@usb_daddr`]
module*/
pub type USB_DADDR = crate::Reg<usb_daddr::USB_DADDRrs>;
///USB Device address
pub mod usb_daddr;
/**USB_LPMCSR (rw) register accessor: LPM control and status register

You can [`read`](crate::Reg::read) this register and get [`usb_lpmcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_lpmcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_LPMCSR)

For information about available fields see [`mod@usb_lpmcsr`]
module*/
pub type USB_LPMCSR = crate::Reg<usb_lpmcsr::USB_LPMCSRrs>;
///LPM control and status register
pub mod usb_lpmcsr;
/**USB_BCDR (rw) register accessor: Battery charging detector

You can [`read`](crate::Reg::read) this register and get [`usb_bcdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_bcdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_BCDR)

For information about available fields see [`mod@usb_bcdr`]
module*/
pub type USB_BCDR = crate::Reg<usb_bcdr::USB_BCDRrs>;
///Battery charging detector
pub mod usb_bcdr;
