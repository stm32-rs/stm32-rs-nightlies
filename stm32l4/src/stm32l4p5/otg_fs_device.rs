#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved3: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved7: [u8; 0x08],
    dvbusdis: DVBUSDIS,
    dvbuspulse: DVBUSPULSE,
    _reserved9: [u8; 0x04],
    diepempmsk: DIEPEMPMSK,
    _reserved10: [u8; 0xc8],
    diepctl0: DIEPCTL0,
    _reserved11: [u8; 0x04],
    diepint0: DIEPINT0,
    _reserved12: [u8; 0x04],
    dieptsiz0: DIEPTSIZ0,
    _reserved13: [u8; 0x04],
    dtxfsts0: DTXFSTS0,
    _reserved14: [u8; 0x04],
    diepctl1: DIEPCTL1,
    _reserved15: [u8; 0x04],
    diepint1: DIEPINT1,
    _reserved16: [u8; 0x04],
    dieptsiz1: DIEPTSIZ1,
    _reserved17: [u8; 0x04],
    dtxfsts1: DTXFSTS1,
    _reserved18: [u8; 0x04],
    diepctl2: DIEPCTL2,
    _reserved19: [u8; 0x04],
    diepint2: DIEPINT2,
    _reserved20: [u8; 0x04],
    dieptsiz2: DIEPTSIZ2,
    _reserved21: [u8; 0x04],
    dtxfsts2: DTXFSTS2,
    _reserved22: [u8; 0x04],
    diepctl3: DIEPCTL3,
    _reserved23: [u8; 0x04],
    diepint3: DIEPINT3,
    _reserved24: [u8; 0x04],
    dieptsiz3: DIEPTSIZ3,
    _reserved25: [u8; 0x04],
    dtxfsts3: DTXFSTS3,
    _reserved26: [u8; 0x0184],
    doepctl0: DOEPCTL0,
    _reserved27: [u8; 0x04],
    doepint0: DOEPINT0,
    _reserved28: [u8; 0x04],
    doeptsiz0: DOEPTSIZ0,
    _reserved29: [u8; 0x0c],
    doepctl1: DOEPCTL1,
    _reserved30: [u8; 0x04],
    doepint1: DOEPINT1,
    _reserved31: [u8; 0x04],
    doeptsiz1: DOEPTSIZ1,
    _reserved32: [u8; 0x0c],
    doepctl2: DOEPCTL2,
    _reserved33: [u8; 0x04],
    doepint2: DOEPINT2,
    _reserved34: [u8; 0x04],
    doeptsiz2: DOEPTSIZ2,
    _reserved35: [u8; 0x0c],
    doepctl3: DOEPCTL3,
    _reserved36: [u8; 0x04],
    doepint3: DOEPINT3,
    _reserved37: [u8; 0x04],
    doeptsiz3: DOEPTSIZ3,
}
impl RegisterBlock {
    ///0x00 - OTG_FS device configuration register (OTG_FS_DCFG)
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    ///0x04 - OTG_FS device control register (OTG_FS_DCTL)
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    ///0x08 - OTG_FS device status register (OTG_FS_DSTS)
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    ///0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    ///0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    ///0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    ///0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    ///0x28 - OTG_FS device VBUS discharge time register
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &DVBUSDIS {
        &self.dvbusdis
    }
    ///0x2c - OTG_FS device VBUS pulsing time register
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &DVBUSPULSE {
        &self.dvbuspulse
    }
    ///0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    ///0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
    #[inline(always)]
    pub const fn diepctl0(&self) -> &DIEPCTL0 {
        &self.diepctl0
    }
    ///0x108 - device endpoint-x interrupt register
    #[inline(always)]
    pub const fn diepint0(&self) -> &DIEPINT0 {
        &self.diepint0
    }
    ///0x110 - device endpoint-0 transfer size register
    #[inline(always)]
    pub const fn dieptsiz0(&self) -> &DIEPTSIZ0 {
        &self.dieptsiz0
    }
    ///0x118 - OTG_FS device IN endpoint transmit FIFO status register
    #[inline(always)]
    pub const fn dtxfsts0(&self) -> &DTXFSTS0 {
        &self.dtxfsts0
    }
    ///0x120 - OTG device endpoint-1 control register
    #[inline(always)]
    pub const fn diepctl1(&self) -> &DIEPCTL1 {
        &self.diepctl1
    }
    ///0x128 - device endpoint-1 interrupt register
    #[inline(always)]
    pub const fn diepint1(&self) -> &DIEPINT1 {
        &self.diepint1
    }
    ///0x130 - device endpoint-1 transfer size register
    #[inline(always)]
    pub const fn dieptsiz1(&self) -> &DIEPTSIZ1 {
        &self.dieptsiz1
    }
    ///0x138 - OTG_FS device IN endpoint transmit FIFO status register
    #[inline(always)]
    pub const fn dtxfsts1(&self) -> &DTXFSTS1 {
        &self.dtxfsts1
    }
    ///0x140 - OTG device endpoint-2 control register
    #[inline(always)]
    pub const fn diepctl2(&self) -> &DIEPCTL2 {
        &self.diepctl2
    }
    ///0x148 - device endpoint-2 interrupt register
    #[inline(always)]
    pub const fn diepint2(&self) -> &DIEPINT2 {
        &self.diepint2
    }
    ///0x150 - device endpoint-2 transfer size register
    #[inline(always)]
    pub const fn dieptsiz2(&self) -> &DIEPTSIZ2 {
        &self.dieptsiz2
    }
    ///0x158 - OTG_FS device IN endpoint transmit FIFO status register
    #[inline(always)]
    pub const fn dtxfsts2(&self) -> &DTXFSTS2 {
        &self.dtxfsts2
    }
    ///0x160 - OTG device endpoint-3 control register
    #[inline(always)]
    pub const fn diepctl3(&self) -> &DIEPCTL3 {
        &self.diepctl3
    }
    ///0x168 - device endpoint-3 interrupt register
    #[inline(always)]
    pub const fn diepint3(&self) -> &DIEPINT3 {
        &self.diepint3
    }
    ///0x170 - device endpoint-3 transfer size register
    #[inline(always)]
    pub const fn dieptsiz3(&self) -> &DIEPTSIZ3 {
        &self.dieptsiz3
    }
    ///0x178 - OTG_FS device IN endpoint transmit FIFO status register
    #[inline(always)]
    pub const fn dtxfsts3(&self) -> &DTXFSTS3 {
        &self.dtxfsts3
    }
    ///0x300 - device endpoint-0 control register
    #[inline(always)]
    pub const fn doepctl0(&self) -> &DOEPCTL0 {
        &self.doepctl0
    }
    ///0x308 - device endpoint-0 interrupt register
    #[inline(always)]
    pub const fn doepint0(&self) -> &DOEPINT0 {
        &self.doepint0
    }
    ///0x310 - device OUT endpoint-0 transfer size register
    #[inline(always)]
    pub const fn doeptsiz0(&self) -> &DOEPTSIZ0 {
        &self.doeptsiz0
    }
    ///0x320 - device endpoint-1 control register
    #[inline(always)]
    pub const fn doepctl1(&self) -> &DOEPCTL1 {
        &self.doepctl1
    }
    ///0x328 - device endpoint-1 interrupt register
    #[inline(always)]
    pub const fn doepint1(&self) -> &DOEPINT1 {
        &self.doepint1
    }
    ///0x330 - device OUT endpoint-1 transfer size register
    #[inline(always)]
    pub const fn doeptsiz1(&self) -> &DOEPTSIZ1 {
        &self.doeptsiz1
    }
    ///0x340 - device endpoint-2 control register
    #[inline(always)]
    pub const fn doepctl2(&self) -> &DOEPCTL2 {
        &self.doepctl2
    }
    ///0x348 - device endpoint-2 interrupt register
    #[inline(always)]
    pub const fn doepint2(&self) -> &DOEPINT2 {
        &self.doepint2
    }
    ///0x350 - device OUT endpoint-2 transfer size register
    #[inline(always)]
    pub const fn doeptsiz2(&self) -> &DOEPTSIZ2 {
        &self.doeptsiz2
    }
    ///0x360 - device endpoint-3 control register
    #[inline(always)]
    pub const fn doepctl3(&self) -> &DOEPCTL3 {
        &self.doepctl3
    }
    ///0x368 - device endpoint-3 interrupt register
    #[inline(always)]
    pub const fn doepint3(&self) -> &DOEPINT3 {
        &self.doepint3
    }
    ///0x370 - device OUT endpoint-3 transfer size register
    #[inline(always)]
    pub const fn doeptsiz3(&self) -> &DOEPTSIZ3 {
        &self.doeptsiz3
    }
}
/**DCFG (rw) register accessor: OTG_FS device configuration register (OTG_FS_DCFG)

You can [`read`](crate::Reg::read) this register and get [`dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DCFG)

For information about available fields see [`mod@dcfg`] module*/
pub type DCFG = crate::Reg<dcfg::DCFGrs>;
///OTG_FS device configuration register (OTG_FS_DCFG)
pub mod dcfg;
/**DCTL (rw) register accessor: OTG_FS device control register (OTG_FS_DCTL)

You can [`read`](crate::Reg::read) this register and get [`dctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DCTL)

For information about available fields see [`mod@dctl`] module*/
pub type DCTL = crate::Reg<dctl::DCTLrs>;
///OTG_FS device control register (OTG_FS_DCTL)
pub mod dctl;
/**DSTS (r) register accessor: OTG_FS device status register (OTG_FS_DSTS)

You can [`read`](crate::Reg::read) this register and get [`dsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DSTS)

For information about available fields see [`mod@dsts`] module*/
pub type DSTS = crate::Reg<dsts::DSTSrs>;
///OTG_FS device status register (OTG_FS_DSTS)
pub mod dsts;
/**DIEPMSK (rw) register accessor: OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)

You can [`read`](crate::Reg::read) this register and get [`diepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPMSK)

For information about available fields see [`mod@diepmsk`] module*/
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSKrs>;
///OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
pub mod diepmsk;
/**DOEPMSK (rw) register accessor: OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)

You can [`read`](crate::Reg::read) this register and get [`doepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPMSK)

For information about available fields see [`mod@doepmsk`] module*/
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSKrs>;
///OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
pub mod doepmsk;
/**DAINT (r) register accessor: OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)

You can [`read`](crate::Reg::read) this register and get [`daint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DAINT)

For information about available fields see [`mod@daint`] module*/
pub type DAINT = crate::Reg<daint::DAINTrs>;
///OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
pub mod daint;
/**DAINTMSK (rw) register accessor: OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)

You can [`read`](crate::Reg::read) this register and get [`daintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DAINTMSK)

For information about available fields see [`mod@daintmsk`] module*/
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSKrs>;
///OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
pub mod daintmsk;
/**DVBUSDIS (rw) register accessor: OTG_FS device VBUS discharge time register

You can [`read`](crate::Reg::read) this register and get [`dvbusdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DVBUSDIS)

For information about available fields see [`mod@dvbusdis`] module*/
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDISrs>;
///OTG_FS device VBUS discharge time register
pub mod dvbusdis;
/**DVBUSPULSE (rw) register accessor: OTG_FS device VBUS pulsing time register

You can [`read`](crate::Reg::read) this register and get [`dvbuspulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DVBUSPULSE)

For information about available fields see [`mod@dvbuspulse`] module*/
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSErs>;
///OTG_FS device VBUS pulsing time register
pub mod dvbuspulse;
/**DIEPEMPMSK (rw) register accessor: OTG_FS device IN endpoint FIFO empty interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPEMPMSK)

For information about available fields see [`mod@diepempmsk`] module*/
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSKrs>;
///OTG_FS device IN endpoint FIFO empty interrupt mask register
pub mod diepempmsk;
/**DIEPCTL0 (rw) register accessor: OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)

You can [`read`](crate::Reg::read) this register and get [`diepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPCTL0)

For information about available fields see [`mod@diepctl0`] module*/
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0rs>;
///OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
pub mod diepctl0;
/**DIEPCTL1 (rw) register accessor: OTG device endpoint-1 control register

You can [`read`](crate::Reg::read) this register and get [`diepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPCTL1)

For information about available fields see [`mod@diepctl1`] module*/
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1rs>;
///OTG device endpoint-1 control register
pub mod diepctl1;
/**DIEPCTL2 (rw) register accessor: OTG device endpoint-2 control register

You can [`read`](crate::Reg::read) this register and get [`diepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPCTL2)

For information about available fields see [`mod@diepctl2`] module*/
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2rs>;
///OTG device endpoint-2 control register
pub mod diepctl2;
/**DIEPCTL3 (rw) register accessor: OTG device endpoint-3 control register

You can [`read`](crate::Reg::read) this register and get [`diepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPCTL3)

For information about available fields see [`mod@diepctl3`] module*/
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3rs>;
///OTG device endpoint-3 control register
pub mod diepctl3;
/**DOEPCTL0 (rw) register accessor: device endpoint-0 control register

You can [`read`](crate::Reg::read) this register and get [`doepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPCTL0)

For information about available fields see [`mod@doepctl0`] module*/
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0rs>;
///device endpoint-0 control register
pub mod doepctl0;
/**DOEPCTL1 (rw) register accessor: device endpoint-1 control register

You can [`read`](crate::Reg::read) this register and get [`doepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPCTL1)

For information about available fields see [`mod@doepctl1`] module*/
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1rs>;
///device endpoint-1 control register
pub mod doepctl1;
/**DOEPCTL2 (rw) register accessor: device endpoint-2 control register

You can [`read`](crate::Reg::read) this register and get [`doepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPCTL2)

For information about available fields see [`mod@doepctl2`] module*/
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2rs>;
///device endpoint-2 control register
pub mod doepctl2;
/**DOEPCTL3 (rw) register accessor: device endpoint-3 control register

You can [`read`](crate::Reg::read) this register and get [`doepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPCTL3)

For information about available fields see [`mod@doepctl3`] module*/
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3rs>;
///device endpoint-3 control register
pub mod doepctl3;
/**DIEPINT0 (rw) register accessor: device endpoint-x interrupt register

You can [`read`](crate::Reg::read) this register and get [`diepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPINT0)

For information about available fields see [`mod@diepint0`] module*/
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0rs>;
///device endpoint-x interrupt register
pub mod diepint0;
/**DIEPINT1 (rw) register accessor: device endpoint-1 interrupt register

You can [`read`](crate::Reg::read) this register and get [`diepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPINT1)

For information about available fields see [`mod@diepint1`] module*/
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1rs>;
///device endpoint-1 interrupt register
pub mod diepint1;
/**DIEPINT2 (rw) register accessor: device endpoint-2 interrupt register

You can [`read`](crate::Reg::read) this register and get [`diepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPINT2)

For information about available fields see [`mod@diepint2`] module*/
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2rs>;
///device endpoint-2 interrupt register
pub mod diepint2;
/**DIEPINT3 (rw) register accessor: device endpoint-3 interrupt register

You can [`read`](crate::Reg::read) this register and get [`diepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPINT3)

For information about available fields see [`mod@diepint3`] module*/
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3rs>;
///device endpoint-3 interrupt register
pub mod diepint3;
/**DOEPINT0 (rw) register accessor: device endpoint-0 interrupt register

You can [`read`](crate::Reg::read) this register and get [`doepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPINT0)

For information about available fields see [`mod@doepint0`] module*/
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0rs>;
///device endpoint-0 interrupt register
pub mod doepint0;
/**DOEPINT1 (rw) register accessor: device endpoint-1 interrupt register

You can [`read`](crate::Reg::read) this register and get [`doepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPINT1)

For information about available fields see [`mod@doepint1`] module*/
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1rs>;
///device endpoint-1 interrupt register
pub mod doepint1;
/**DOEPINT2 (rw) register accessor: device endpoint-2 interrupt register

You can [`read`](crate::Reg::read) this register and get [`doepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPINT2)

For information about available fields see [`mod@doepint2`] module*/
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2rs>;
///device endpoint-2 interrupt register
pub mod doepint2;
/**DOEPINT3 (rw) register accessor: device endpoint-3 interrupt register

You can [`read`](crate::Reg::read) this register and get [`doepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPINT3)

For information about available fields see [`mod@doepint3`] module*/
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3rs>;
///device endpoint-3 interrupt register
pub mod doepint3;
/**DIEPTSIZ0 (rw) register accessor: device endpoint-0 transfer size register

You can [`read`](crate::Reg::read) this register and get [`dieptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPTSIZ0)

For information about available fields see [`mod@dieptsiz0`] module*/
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0rs>;
///device endpoint-0 transfer size register
pub mod dieptsiz0;
/**DOEPTSIZ0 (rw) register accessor: device OUT endpoint-0 transfer size register

You can [`read`](crate::Reg::read) this register and get [`doeptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPTSIZ0)

For information about available fields see [`mod@doeptsiz0`] module*/
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0rs>;
///device OUT endpoint-0 transfer size register
pub mod doeptsiz0;
/**DIEPTSIZ1 (rw) register accessor: device endpoint-1 transfer size register

You can [`read`](crate::Reg::read) this register and get [`dieptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPTSIZ1)

For information about available fields see [`mod@dieptsiz1`] module*/
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1rs>;
///device endpoint-1 transfer size register
pub mod dieptsiz1;
/**DIEPTSIZ2 (rw) register accessor: device endpoint-2 transfer size register

You can [`read`](crate::Reg::read) this register and get [`dieptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPTSIZ2)

For information about available fields see [`mod@dieptsiz2`] module*/
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2rs>;
///device endpoint-2 transfer size register
pub mod dieptsiz2;
/**DIEPTSIZ3 (rw) register accessor: device endpoint-3 transfer size register

You can [`read`](crate::Reg::read) this register and get [`dieptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DIEPTSIZ3)

For information about available fields see [`mod@dieptsiz3`] module*/
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3rs>;
///device endpoint-3 transfer size register
pub mod dieptsiz3;
/**DTXFSTS0 (r) register accessor: OTG_FS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`dtxfsts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DTXFSTS0)

For information about available fields see [`mod@dtxfsts0`] module*/
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0rs>;
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts0;
/**DTXFSTS1 (r) register accessor: OTG_FS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`dtxfsts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DTXFSTS1)

For information about available fields see [`mod@dtxfsts1`] module*/
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1rs>;
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts1;
/**DTXFSTS2 (r) register accessor: OTG_FS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`dtxfsts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DTXFSTS2)

For information about available fields see [`mod@dtxfsts2`] module*/
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2rs>;
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts2;
/**DTXFSTS3 (r) register accessor: OTG_FS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`dtxfsts3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DTXFSTS3)

For information about available fields see [`mod@dtxfsts3`] module*/
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3rs>;
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts3;
/**DOEPTSIZ1 (rw) register accessor: device OUT endpoint-1 transfer size register

You can [`read`](crate::Reg::read) this register and get [`doeptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPTSIZ1)

For information about available fields see [`mod@doeptsiz1`] module*/
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1rs>;
///device OUT endpoint-1 transfer size register
pub mod doeptsiz1;
/**DOEPTSIZ2 (rw) register accessor: device OUT endpoint-2 transfer size register

You can [`read`](crate::Reg::read) this register and get [`doeptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPTSIZ2)

For information about available fields see [`mod@doeptsiz2`] module*/
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2rs>;
///device OUT endpoint-2 transfer size register
pub mod doeptsiz2;
/**DOEPTSIZ3 (rw) register accessor: device OUT endpoint-3 transfer size register

You can [`read`](crate::Reg::read) this register and get [`doeptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPTSIZ3)

For information about available fields see [`mod@doeptsiz3`] module*/
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3rs>;
///device OUT endpoint-3 transfer size register
pub mod doeptsiz3;
