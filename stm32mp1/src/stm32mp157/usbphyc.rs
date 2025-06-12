#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pll: PLL,
    _reserved1: [u8; 0x04],
    misc: MISC,
    _reserved2: [u8; 0x0100],
    tune1: TUNE1,
    _reserved3: [u8; 0xfc],
    tune2: TUNE2,
    _reserved4: [u8; 0x0dec],
    verr: VERR,
}
impl RegisterBlock {
    ///0x00 - This register is used to control the PLL of the HS PHY.
    #[inline(always)]
    pub const fn pll(&self) -> &PLL {
        &self.pll
    }
    ///0x08 - This register is used to control the switch between controllers for the HS PHY.
    #[inline(always)]
    pub const fn misc(&self) -> &MISC {
        &self.misc
    }
    ///0x10c - This register is used to control the tune interface of the HS PHY, port #x.
    #[inline(always)]
    pub const fn tune1(&self) -> &TUNE1 {
        &self.tune1
    }
    ///0x20c - This register is used to control the tune interface of the HS PHY, port #x.
    #[inline(always)]
    pub const fn tune2(&self) -> &TUNE2 {
        &self.tune2
    }
    ///0xffc - This register defines the version of this IP.
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
}
/**PLL (rw) register accessor: This register is used to control the PLL of the HS PHY.

You can [`read`](crate::Reg::read) this register and get [`pll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#USBPHYC:PLL)

For information about available fields see [`mod@pll`] module*/
pub type PLL = crate::Reg<pll::PLLrs>;
///This register is used to control the PLL of the HS PHY.
pub mod pll;
/**MISC (rw) register accessor: This register is used to control the switch between controllers for the HS PHY.

You can [`read`](crate::Reg::read) this register and get [`misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#USBPHYC:MISC)

For information about available fields see [`mod@misc`] module*/
pub type MISC = crate::Reg<misc::MISCrs>;
///This register is used to control the switch between controllers for the HS PHY.
pub mod misc;
/**TUNE1 (rw) register accessor: This register is used to control the tune interface of the HS PHY, port #x.

You can [`read`](crate::Reg::read) this register and get [`tune1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tune1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#USBPHYC:TUNE1)

For information about available fields see [`mod@tune1`] module*/
pub type TUNE1 = crate::Reg<tune1::TUNE1rs>;
///This register is used to control the tune interface of the HS PHY, port #x.
pub mod tune1;
/**TUNE2 (rw) register accessor: This register is used to control the tune interface of the HS PHY, port #x.

You can [`read`](crate::Reg::read) this register and get [`tune2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tune2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#USBPHYC:TUNE2)

For information about available fields see [`mod@tune2`] module*/
pub type TUNE2 = crate::Reg<tune2::TUNE2rs>;
///This register is used to control the tune interface of the HS PHY, port #x.
pub mod tune2;
/**VERR (r) register accessor: This register defines the version of this IP.

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#USBPHYC:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///This register defines the version of this IP.
pub mod verr;
