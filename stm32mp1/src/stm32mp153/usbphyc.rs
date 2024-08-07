#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    usbphyc_pll: USBPHYC_PLL,
    _reserved1: [u8; 0x04],
    usbphyc_misc: USBPHYC_MISC,
    _reserved2: [u8; 0x0100],
    usbphyc_tune1: USBPHYC_TUNE1,
    _reserved3: [u8; 0xfc],
    usbphyc_tune2: USBPHYC_TUNE2,
    _reserved4: [u8; 0x0dec],
    usbphyc_verr: USBPHYC_VERR,
}
impl RegisterBlock {
    ///0x00 - This register is used to control the PLL of the HS PHY.
    #[inline(always)]
    pub const fn usbphyc_pll(&self) -> &USBPHYC_PLL {
        &self.usbphyc_pll
    }
    ///0x08 - This register is used to control the switch between controllers for the HS PHY.
    #[inline(always)]
    pub const fn usbphyc_misc(&self) -> &USBPHYC_MISC {
        &self.usbphyc_misc
    }
    ///0x10c - This register is used to control the tune interface of the HS PHY, port #x.
    #[inline(always)]
    pub const fn usbphyc_tune1(&self) -> &USBPHYC_TUNE1 {
        &self.usbphyc_tune1
    }
    ///0x20c - This register is used to control the tune interface of the HS PHY, port #x.
    #[inline(always)]
    pub const fn usbphyc_tune2(&self) -> &USBPHYC_TUNE2 {
        &self.usbphyc_tune2
    }
    ///0xffc - This register defines the version of this IP.
    #[inline(always)]
    pub const fn usbphyc_verr(&self) -> &USBPHYC_VERR {
        &self.usbphyc_verr
    }
}
/**USBPHYC_PLL (rw) register accessor: This register is used to control the PLL of the HS PHY.

You can [`read`](crate::Reg::read) this register and get [`usbphyc_pll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphyc_pll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#USBPHYC:USBPHYC_PLL)

For information about available fields see [`mod@usbphyc_pll`]
module*/
pub type USBPHYC_PLL = crate::Reg<usbphyc_pll::USBPHYC_PLLrs>;
///This register is used to control the PLL of the HS PHY.
pub mod usbphyc_pll;
/**USBPHYC_MISC (rw) register accessor: This register is used to control the switch between controllers for the HS PHY.

You can [`read`](crate::Reg::read) this register and get [`usbphyc_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphyc_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#USBPHYC:USBPHYC_MISC)

For information about available fields see [`mod@usbphyc_misc`]
module*/
pub type USBPHYC_MISC = crate::Reg<usbphyc_misc::USBPHYC_MISCrs>;
///This register is used to control the switch between controllers for the HS PHY.
pub mod usbphyc_misc;
/**USBPHYC_TUNE1 (rw) register accessor: This register is used to control the tune interface of the HS PHY, port #x.

You can [`read`](crate::Reg::read) this register and get [`usbphyc_tune1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphyc_tune1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#USBPHYC:USBPHYC_TUNE1)

For information about available fields see [`mod@usbphyc_tune1`]
module*/
pub type USBPHYC_TUNE1 = crate::Reg<usbphyc_tune1::USBPHYC_TUNE1rs>;
///This register is used to control the tune interface of the HS PHY, port #x.
pub mod usbphyc_tune1;
/**USBPHYC_TUNE2 (rw) register accessor: This register is used to control the tune interface of the HS PHY, port #x.

You can [`read`](crate::Reg::read) this register and get [`usbphyc_tune2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphyc_tune2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#USBPHYC:USBPHYC_TUNE2)

For information about available fields see [`mod@usbphyc_tune2`]
module*/
pub type USBPHYC_TUNE2 = crate::Reg<usbphyc_tune2::USBPHYC_TUNE2rs>;
///This register is used to control the tune interface of the HS PHY, port #x.
pub mod usbphyc_tune2;
/**USBPHYC_VERR (r) register accessor: This register defines the version of this IP.

You can [`read`](crate::Reg::read) this register and get [`usbphyc_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#USBPHYC:USBPHYC_VERR)

For information about available fields see [`mod@usbphyc_verr`]
module*/
pub type USBPHYC_VERR = crate::Reg<usbphyc_verr::USBPHYC_VERRrs>;
///This register defines the version of this IP.
pub mod usbphyc_verr;
