#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    otg_hs_dcfg: OTG_HS_DCFG,
    otg_hs_dctl: OTG_HS_DCTL,
    otg_hs_dsts: OTG_HS_DSTS,
    _reserved3: [u8; 0x04],
    otg_hs_diepmsk: OTG_HS_DIEPMSK,
    otg_hs_doepmsk: OTG_HS_DOEPMSK,
    otg_hs_daint: OTG_HS_DAINT,
    otg_hs_daintmsk: OTG_HS_DAINTMSK,
    _reserved7: [u8; 0x08],
    otg_hs_dvbusdis: OTG_HS_DVBUSDIS,
    otg_hs_dvbuspulse: OTG_HS_DVBUSPULSE,
    otg_hs_dthrctl: OTG_HS_DTHRCTL,
    otg_hs_diepempmsk: OTG_HS_DIEPEMPMSK,
    otg_hs_deachint: OTG_HS_DEACHINT,
    otg_hs_deachintmsk: OTG_HS_DEACHINTMSK,
    otg_hs_diepeachmsk1: OTG_HS_DIEPEACHMSK1,
    _reserved14: [u8; 0x3c],
    otg_hs_doepeachmsk1: OTG_HS_DOEPEACHMSK1,
    _reserved15: [u8; 0x7c],
    otg_hs_diepctl0: OTG_HS_DIEPCTL0,
    _reserved16: [u8; 0x04],
    otg_hs_diepint0: OTG_HS_DIEPINT0,
    _reserved17: [u8; 0x04],
    otg_hs_dieptsiz0: OTG_HS_DIEPTSIZ0,
    otg_hs_diepdma1: OTG_HS_DIEPDMA1,
    otg_hs_dtxfsts0: OTG_HS_DTXFSTS0,
    _reserved20: [u8; 0x04],
    otg_hs_diepctl1: OTG_HS_DIEPCTL1,
    _reserved21: [u8; 0x04],
    otg_hs_diepint1: OTG_HS_DIEPINT1,
    _reserved22: [u8; 0x04],
    otg_hs_dieptsiz1: OTG_HS_DIEPTSIZ1,
    otg_hs_diepdma2: OTG_HS_DIEPDMA2,
    otg_hs_dtxfsts1: OTG_HS_DTXFSTS1,
    _reserved25: [u8; 0x04],
    otg_hs_diepctl2: OTG_HS_DIEPCTL2,
    _reserved26: [u8; 0x04],
    otg_hs_diepint2: OTG_HS_DIEPINT2,
    _reserved27: [u8; 0x04],
    otg_hs_dieptsiz2: OTG_HS_DIEPTSIZ2,
    otg_hs_diepdma3: OTG_HS_DIEPDMA3,
    otg_hs_dtxfsts2: OTG_HS_DTXFSTS2,
    _reserved30: [u8; 0x04],
    otg_hs_diepctl3: OTG_HS_DIEPCTL3,
    _reserved31: [u8; 0x04],
    otg_hs_diepint3: OTG_HS_DIEPINT3,
    _reserved32: [u8; 0x04],
    otg_hs_dieptsiz3: OTG_HS_DIEPTSIZ3,
    otg_hs_diepdma4: OTG_HS_DIEPDMA4,
    otg_hs_dtxfsts3: OTG_HS_DTXFSTS3,
    _reserved35: [u8; 0x04],
    otg_hs_diepctl4: OTG_HS_DIEPCTL4,
    _reserved36: [u8; 0x04],
    otg_hs_diepint4: OTG_HS_DIEPINT4,
    _reserved37: [u8; 0x04],
    otg_hs_dieptsiz4: OTG_HS_DIEPTSIZ4,
    otg_hs_diepdma5: OTG_HS_DIEPDMA5,
    otg_hs_dtxfsts4: OTG_HS_DTXFSTS4,
    _reserved40: [u8; 0x04],
    otg_hs_diepctl5: OTG_HS_DIEPCTL5,
    _reserved41: [u8; 0x04],
    otg_hs_diepint5: OTG_HS_DIEPINT5,
    _reserved42: [u8; 0x04],
    otg_hs_dieptsiz5: OTG_HS_DIEPTSIZ5,
    _reserved43: [u8; 0x04],
    otg_hs_dtxfsts5: OTG_HS_DTXFSTS5,
    _reserved44: [u8; 0x04],
    otg_hs_diepctl6: OTG_HS_DIEPCTL6,
    _reserved45: [u8; 0x04],
    otg_hs_diepint6: OTG_HS_DIEPINT6,
    _reserved46: [u8; 0x14],
    otg_hs_diepctl7: OTG_HS_DIEPCTL7,
    _reserved47: [u8; 0x04],
    otg_hs_diepint7: OTG_HS_DIEPINT7,
    _reserved48: [u8; 0x0114],
    otg_hs_doepctl0: OTG_HS_DOEPCTL0,
    _reserved49: [u8; 0x04],
    otg_hs_doepint0: OTG_HS_DOEPINT0,
    _reserved50: [u8; 0x04],
    otg_hs_doeptsiz0: OTG_HS_DOEPTSIZ0,
    _reserved51: [u8; 0x0c],
    otg_hs_doepctl1: OTG_HS_DOEPCTL1,
    _reserved52: [u8; 0x04],
    otg_hs_doepint1: OTG_HS_DOEPINT1,
    _reserved53: [u8; 0x04],
    otg_hs_doeptsiz1: OTG_HS_DOEPTSIZ1,
    _reserved54: [u8; 0x0c],
    otg_hs_doepctl2: OTG_HS_DOEPCTL2,
    _reserved55: [u8; 0x04],
    otg_hs_doepint2: OTG_HS_DOEPINT2,
    _reserved56: [u8; 0x04],
    otg_hs_doeptsiz2: OTG_HS_DOEPTSIZ2,
    _reserved57: [u8; 0x0c],
    otg_hs_doepctl3: OTG_HS_DOEPCTL3,
    _reserved58: [u8; 0x04],
    otg_hs_doepint3: OTG_HS_DOEPINT3,
    _reserved59: [u8; 0x04],
    otg_hs_doeptsiz3: OTG_HS_DOEPTSIZ3,
    _reserved60: [u8; 0x14],
    otg_hs_doepint4: OTG_HS_DOEPINT4,
    _reserved61: [u8; 0x04],
    otg_hs_doeptsiz4: OTG_HS_DOEPTSIZ4,
    _reserved62: [u8; 0x14],
    otg_hs_doepint5: OTG_HS_DOEPINT5,
    _reserved63: [u8; 0x1c],
    otg_hs_doepint6: OTG_HS_DOEPINT6,
    _reserved64: [u8; 0x1c],
    otg_hs_doepint7: OTG_HS_DOEPINT7,
}
impl RegisterBlock {
    ///0x00 - OTG_HS device configuration register
    #[inline(always)]
    pub const fn otg_hs_dcfg(&self) -> &OTG_HS_DCFG {
        &self.otg_hs_dcfg
    }
    ///0x04 - OTG_HS device control register
    #[inline(always)]
    pub const fn otg_hs_dctl(&self) -> &OTG_HS_DCTL {
        &self.otg_hs_dctl
    }
    ///0x08 - OTG_HS device status register
    #[inline(always)]
    pub const fn otg_hs_dsts(&self) -> &OTG_HS_DSTS {
        &self.otg_hs_dsts
    }
    ///0x10 - OTG_HS device IN endpoint common interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_diepmsk(&self) -> &OTG_HS_DIEPMSK {
        &self.otg_hs_diepmsk
    }
    ///0x14 - OTG_HS device OUT endpoint common interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_doepmsk(&self) -> &OTG_HS_DOEPMSK {
        &self.otg_hs_doepmsk
    }
    ///0x18 - OTG_HS device all endpoints interrupt register
    #[inline(always)]
    pub const fn otg_hs_daint(&self) -> &OTG_HS_DAINT {
        &self.otg_hs_daint
    }
    ///0x1c - OTG_HS all endpoints interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_daintmsk(&self) -> &OTG_HS_DAINTMSK {
        &self.otg_hs_daintmsk
    }
    ///0x28 - OTG_HS device VBUS discharge time register
    #[inline(always)]
    pub const fn otg_hs_dvbusdis(&self) -> &OTG_HS_DVBUSDIS {
        &self.otg_hs_dvbusdis
    }
    ///0x2c - OTG_HS device VBUS pulsing time register
    #[inline(always)]
    pub const fn otg_hs_dvbuspulse(&self) -> &OTG_HS_DVBUSPULSE {
        &self.otg_hs_dvbuspulse
    }
    ///0x30 - OTG_HS Device threshold control register
    #[inline(always)]
    pub const fn otg_hs_dthrctl(&self) -> &OTG_HS_DTHRCTL {
        &self.otg_hs_dthrctl
    }
    ///0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_diepempmsk(&self) -> &OTG_HS_DIEPEMPMSK {
        &self.otg_hs_diepempmsk
    }
    ///0x38 - OTG_HS device each endpoint interrupt register
    #[inline(always)]
    pub const fn otg_hs_deachint(&self) -> &OTG_HS_DEACHINT {
        &self.otg_hs_deachint
    }
    ///0x3c - OTG_HS device each endpoint interrupt register mask
    #[inline(always)]
    pub const fn otg_hs_deachintmsk(&self) -> &OTG_HS_DEACHINTMSK {
        &self.otg_hs_deachintmsk
    }
    ///0x40 - OTG_HS device each in endpoint-1 interrupt register
    #[inline(always)]
    pub const fn otg_hs_diepeachmsk1(&self) -> &OTG_HS_DIEPEACHMSK1 {
        &self.otg_hs_diepeachmsk1
    }
    ///0x80 - OTG_HS device each OUT endpoint-1 interrupt register
    #[inline(always)]
    pub const fn otg_hs_doepeachmsk1(&self) -> &OTG_HS_DOEPEACHMSK1 {
        &self.otg_hs_doepeachmsk1
    }
    ///0x100 - OTG device endpoint-0 control register
    #[inline(always)]
    pub const fn otg_hs_diepctl0(&self) -> &OTG_HS_DIEPCTL0 {
        &self.otg_hs_diepctl0
    }
    ///0x108 - OTG device endpoint-0 interrupt register
    #[inline(always)]
    pub const fn otg_hs_diepint0(&self) -> &OTG_HS_DIEPINT0 {
        &self.otg_hs_diepint0
    }
    ///0x110 - OTG_HS device IN endpoint 0 transfer size register
    #[inline(always)]
    pub const fn otg_hs_dieptsiz0(&self) -> &OTG_HS_DIEPTSIZ0 {
        &self.otg_hs_dieptsiz0
    }
    ///0x114 - OTG_HS device endpoint-1 DMA address register
    #[inline(always)]
    pub const fn otg_hs_diepdma1(&self) -> &OTG_HS_DIEPDMA1 {
        &self.otg_hs_diepdma1
    }
    ///0x118 - OTG_HS device IN endpoint transmit FIFO status register
    #[inline(always)]
    pub const fn otg_hs_dtxfsts0(&self) -> &OTG_HS_DTXFSTS0 {
        &self.otg_hs_dtxfsts0
    }
    ///0x120 - OTG device endpoint-1 control register
    #[inline(always)]
    pub const fn otg_hs_diepctl1(&self) -> &OTG_HS_DIEPCTL1 {
        &self.otg_hs_diepctl1
    }
    ///0x128 - OTG device endpoint-1 interrupt register
    #[inline(always)]
    pub const fn otg_hs_diepint1(&self) -> &OTG_HS_DIEPINT1 {
        &self.otg_hs_diepint1
    }
    ///0x130 - OTG_HS device endpoint transfer size register
    #[inline(always)]
    pub const fn otg_hs_dieptsiz1(&self) -> &OTG_HS_DIEPTSIZ1 {
        &self.otg_hs_dieptsiz1
    }
    ///0x134 - OTG_HS device endpoint-2 DMA address register
    #[inline(always)]
    pub const fn otg_hs_diepdma2(&self) -> &OTG_HS_DIEPDMA2 {
        &self.otg_hs_diepdma2
    }
    ///0x138 - OTG_HS device IN endpoint transmit FIFO status register
    #[inline(always)]
    pub const fn otg_hs_dtxfsts1(&self) -> &OTG_HS_DTXFSTS1 {
        &self.otg_hs_dtxfsts1
    }
    ///0x140 - OTG device endpoint-2 control register
    #[inline(always)]
    pub const fn otg_hs_diepctl2(&self) -> &OTG_HS_DIEPCTL2 {
        &self.otg_hs_diepctl2
    }
    ///0x148 - OTG device endpoint-2 interrupt register
    #[inline(always)]
    pub const fn otg_hs_diepint2(&self) -> &OTG_HS_DIEPINT2 {
        &self.otg_hs_diepint2
    }
    ///0x150 - OTG_HS device endpoint transfer size register
    #[inline(always)]
    pub const fn otg_hs_dieptsiz2(&self) -> &OTG_HS_DIEPTSIZ2 {
        &self.otg_hs_dieptsiz2
    }
    ///0x154 - OTG_HS device endpoint-3 DMA address register
    #[inline(always)]
    pub const fn otg_hs_diepdma3(&self) -> &OTG_HS_DIEPDMA3 {
        &self.otg_hs_diepdma3
    }
    ///0x158 - OTG_HS device IN endpoint transmit FIFO status register
    #[inline(always)]
    pub const fn otg_hs_dtxfsts2(&self) -> &OTG_HS_DTXFSTS2 {
        &self.otg_hs_dtxfsts2
    }
    ///0x160 - OTG device endpoint-3 control register
    #[inline(always)]
    pub const fn otg_hs_diepctl3(&self) -> &OTG_HS_DIEPCTL3 {
        &self.otg_hs_diepctl3
    }
    ///0x168 - OTG device endpoint-3 interrupt register
    #[inline(always)]
    pub const fn otg_hs_diepint3(&self) -> &OTG_HS_DIEPINT3 {
        &self.otg_hs_diepint3
    }
    ///0x170 - OTG_HS device endpoint transfer size register
    #[inline(always)]
    pub const fn otg_hs_dieptsiz3(&self) -> &OTG_HS_DIEPTSIZ3 {
        &self.otg_hs_dieptsiz3
    }
    ///0x174 - OTG_HS device endpoint-4 DMA address register
    #[inline(always)]
    pub const fn otg_hs_diepdma4(&self) -> &OTG_HS_DIEPDMA4 {
        &self.otg_hs_diepdma4
    }
    ///0x178 - OTG_HS device IN endpoint transmit FIFO status register
    #[inline(always)]
    pub const fn otg_hs_dtxfsts3(&self) -> &OTG_HS_DTXFSTS3 {
        &self.otg_hs_dtxfsts3
    }
    ///0x180 - OTG device endpoint-4 control register
    #[inline(always)]
    pub const fn otg_hs_diepctl4(&self) -> &OTG_HS_DIEPCTL4 {
        &self.otg_hs_diepctl4
    }
    ///0x188 - OTG device endpoint-4 interrupt register
    #[inline(always)]
    pub const fn otg_hs_diepint4(&self) -> &OTG_HS_DIEPINT4 {
        &self.otg_hs_diepint4
    }
    ///0x190 - OTG_HS device endpoint transfer size register
    #[inline(always)]
    pub const fn otg_hs_dieptsiz4(&self) -> &OTG_HS_DIEPTSIZ4 {
        &self.otg_hs_dieptsiz4
    }
    ///0x194 - OTG_HS device endpoint-5 DMA address register
    #[inline(always)]
    pub const fn otg_hs_diepdma5(&self) -> &OTG_HS_DIEPDMA5 {
        &self.otg_hs_diepdma5
    }
    ///0x198 - OTG_HS device IN endpoint transmit FIFO status register
    #[inline(always)]
    pub const fn otg_hs_dtxfsts4(&self) -> &OTG_HS_DTXFSTS4 {
        &self.otg_hs_dtxfsts4
    }
    ///0x1a0 - OTG device endpoint-5 control register
    #[inline(always)]
    pub const fn otg_hs_diepctl5(&self) -> &OTG_HS_DIEPCTL5 {
        &self.otg_hs_diepctl5
    }
    ///0x1a8 - OTG device endpoint-5 interrupt register
    #[inline(always)]
    pub const fn otg_hs_diepint5(&self) -> &OTG_HS_DIEPINT5 {
        &self.otg_hs_diepint5
    }
    ///0x1b0 - OTG_HS device endpoint transfer size register
    #[inline(always)]
    pub const fn otg_hs_dieptsiz5(&self) -> &OTG_HS_DIEPTSIZ5 {
        &self.otg_hs_dieptsiz5
    }
    ///0x1b8 - OTG_HS device IN endpoint transmit FIFO status register
    #[inline(always)]
    pub const fn otg_hs_dtxfsts5(&self) -> &OTG_HS_DTXFSTS5 {
        &self.otg_hs_dtxfsts5
    }
    ///0x1c0 - OTG device endpoint-6 control register
    #[inline(always)]
    pub const fn otg_hs_diepctl6(&self) -> &OTG_HS_DIEPCTL6 {
        &self.otg_hs_diepctl6
    }
    ///0x1c8 - OTG device endpoint-6 interrupt register
    #[inline(always)]
    pub const fn otg_hs_diepint6(&self) -> &OTG_HS_DIEPINT6 {
        &self.otg_hs_diepint6
    }
    ///0x1e0 - OTG device endpoint-7 control register
    #[inline(always)]
    pub const fn otg_hs_diepctl7(&self) -> &OTG_HS_DIEPCTL7 {
        &self.otg_hs_diepctl7
    }
    ///0x1e8 - OTG device endpoint-7 interrupt register
    #[inline(always)]
    pub const fn otg_hs_diepint7(&self) -> &OTG_HS_DIEPINT7 {
        &self.otg_hs_diepint7
    }
    ///0x300 - OTG_HS device control OUT endpoint 0 control register
    #[inline(always)]
    pub const fn otg_hs_doepctl0(&self) -> &OTG_HS_DOEPCTL0 {
        &self.otg_hs_doepctl0
    }
    ///0x308 - OTG_HS device endpoint-0 interrupt register
    #[inline(always)]
    pub const fn otg_hs_doepint0(&self) -> &OTG_HS_DOEPINT0 {
        &self.otg_hs_doepint0
    }
    ///0x310 - OTG_HS device endpoint-1 transfer size register
    #[inline(always)]
    pub const fn otg_hs_doeptsiz0(&self) -> &OTG_HS_DOEPTSIZ0 {
        &self.otg_hs_doeptsiz0
    }
    ///0x320 - OTG device endpoint-1 control register
    #[inline(always)]
    pub const fn otg_hs_doepctl1(&self) -> &OTG_HS_DOEPCTL1 {
        &self.otg_hs_doepctl1
    }
    ///0x328 - OTG_HS device endpoint-1 interrupt register
    #[inline(always)]
    pub const fn otg_hs_doepint1(&self) -> &OTG_HS_DOEPINT1 {
        &self.otg_hs_doepint1
    }
    ///0x330 - OTG_HS device endpoint-2 transfer size register
    #[inline(always)]
    pub const fn otg_hs_doeptsiz1(&self) -> &OTG_HS_DOEPTSIZ1 {
        &self.otg_hs_doeptsiz1
    }
    ///0x340 - OTG device endpoint-2 control register
    #[inline(always)]
    pub const fn otg_hs_doepctl2(&self) -> &OTG_HS_DOEPCTL2 {
        &self.otg_hs_doepctl2
    }
    ///0x348 - OTG_HS device endpoint-2 interrupt register
    #[inline(always)]
    pub const fn otg_hs_doepint2(&self) -> &OTG_HS_DOEPINT2 {
        &self.otg_hs_doepint2
    }
    ///0x350 - OTG_HS device endpoint-3 transfer size register
    #[inline(always)]
    pub const fn otg_hs_doeptsiz2(&self) -> &OTG_HS_DOEPTSIZ2 {
        &self.otg_hs_doeptsiz2
    }
    ///0x360 - OTG device endpoint-3 control register
    #[inline(always)]
    pub const fn otg_hs_doepctl3(&self) -> &OTG_HS_DOEPCTL3 {
        &self.otg_hs_doepctl3
    }
    ///0x368 - OTG_HS device endpoint-3 interrupt register
    #[inline(always)]
    pub const fn otg_hs_doepint3(&self) -> &OTG_HS_DOEPINT3 {
        &self.otg_hs_doepint3
    }
    ///0x370 - OTG_HS device endpoint-4 transfer size register
    #[inline(always)]
    pub const fn otg_hs_doeptsiz3(&self) -> &OTG_HS_DOEPTSIZ3 {
        &self.otg_hs_doeptsiz3
    }
    ///0x388 - OTG_HS device endpoint-4 interrupt register
    #[inline(always)]
    pub const fn otg_hs_doepint4(&self) -> &OTG_HS_DOEPINT4 {
        &self.otg_hs_doepint4
    }
    ///0x390 - OTG_HS device endpoint-5 transfer size register
    #[inline(always)]
    pub const fn otg_hs_doeptsiz4(&self) -> &OTG_HS_DOEPTSIZ4 {
        &self.otg_hs_doeptsiz4
    }
    ///0x3a8 - OTG_HS device endpoint-5 interrupt register
    #[inline(always)]
    pub const fn otg_hs_doepint5(&self) -> &OTG_HS_DOEPINT5 {
        &self.otg_hs_doepint5
    }
    ///0x3c8 - OTG_HS device endpoint-6 interrupt register
    #[inline(always)]
    pub const fn otg_hs_doepint6(&self) -> &OTG_HS_DOEPINT6 {
        &self.otg_hs_doepint6
    }
    ///0x3e8 - OTG_HS device endpoint-7 interrupt register
    #[inline(always)]
    pub const fn otg_hs_doepint7(&self) -> &OTG_HS_DOEPINT7 {
        &self.otg_hs_doepint7
    }
}
/**OTG_HS_DCFG (rw) register accessor: OTG_HS device configuration register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DCFG)

For information about available fields see [`mod@otg_hs_dcfg`] module*/
pub type OTG_HS_DCFG = crate::Reg<otg_hs_dcfg::OTG_HS_DCFGrs>;
///OTG_HS device configuration register
pub mod otg_hs_dcfg;
/**OTG_HS_DCTL (rw) register accessor: OTG_HS device control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DCTL)

For information about available fields see [`mod@otg_hs_dctl`] module*/
pub type OTG_HS_DCTL = crate::Reg<otg_hs_dctl::OTG_HS_DCTLrs>;
///OTG_HS device control register
pub mod otg_hs_dctl;
/**OTG_HS_DSTS (r) register accessor: OTG_HS device status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DSTS)

For information about available fields see [`mod@otg_hs_dsts`] module*/
pub type OTG_HS_DSTS = crate::Reg<otg_hs_dsts::OTG_HS_DSTSrs>;
///OTG_HS device status register
pub mod otg_hs_dsts;
/**OTG_HS_DIEPMSK (rw) register accessor: OTG_HS device IN endpoint common interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPMSK)

For information about available fields see [`mod@otg_hs_diepmsk`] module*/
pub type OTG_HS_DIEPMSK = crate::Reg<otg_hs_diepmsk::OTG_HS_DIEPMSKrs>;
///OTG_HS device IN endpoint common interrupt mask register
pub mod otg_hs_diepmsk;
/**OTG_HS_DOEPMSK (rw) register accessor: OTG_HS device OUT endpoint common interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPMSK)

For information about available fields see [`mod@otg_hs_doepmsk`] module*/
pub type OTG_HS_DOEPMSK = crate::Reg<otg_hs_doepmsk::OTG_HS_DOEPMSKrs>;
///OTG_HS device OUT endpoint common interrupt mask register
pub mod otg_hs_doepmsk;
/**OTG_HS_DAINT (r) register accessor: OTG_HS device all endpoints interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_daint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DAINT)

For information about available fields see [`mod@otg_hs_daint`] module*/
pub type OTG_HS_DAINT = crate::Reg<otg_hs_daint::OTG_HS_DAINTrs>;
///OTG_HS device all endpoints interrupt register
pub mod otg_hs_daint;
/**OTG_HS_DAINTMSK (rw) register accessor: OTG_HS all endpoints interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_daintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_daintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DAINTMSK)

For information about available fields see [`mod@otg_hs_daintmsk`] module*/
pub type OTG_HS_DAINTMSK = crate::Reg<otg_hs_daintmsk::OTG_HS_DAINTMSKrs>;
///OTG_HS all endpoints interrupt mask register
pub mod otg_hs_daintmsk;
/**OTG_HS_DVBUSDIS (rw) register accessor: OTG_HS device VBUS discharge time register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dvbusdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dvbusdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DVBUSDIS)

For information about available fields see [`mod@otg_hs_dvbusdis`] module*/
pub type OTG_HS_DVBUSDIS = crate::Reg<otg_hs_dvbusdis::OTG_HS_DVBUSDISrs>;
///OTG_HS device VBUS discharge time register
pub mod otg_hs_dvbusdis;
/**OTG_HS_DVBUSPULSE (rw) register accessor: OTG_HS device VBUS pulsing time register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dvbuspulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dvbuspulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DVBUSPULSE)

For information about available fields see [`mod@otg_hs_dvbuspulse`] module*/
pub type OTG_HS_DVBUSPULSE = crate::Reg<otg_hs_dvbuspulse::OTG_HS_DVBUSPULSErs>;
///OTG_HS device VBUS pulsing time register
pub mod otg_hs_dvbuspulse;
/**OTG_HS_DTHRCTL (rw) register accessor: OTG_HS Device threshold control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dthrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dthrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DTHRCTL)

For information about available fields see [`mod@otg_hs_dthrctl`] module*/
pub type OTG_HS_DTHRCTL = crate::Reg<otg_hs_dthrctl::OTG_HS_DTHRCTLrs>;
///OTG_HS Device threshold control register
pub mod otg_hs_dthrctl;
/**OTG_HS_DIEPEMPMSK (rw) register accessor: OTG_HS device IN endpoint FIFO empty interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepempmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepempmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPEMPMSK)

For information about available fields see [`mod@otg_hs_diepempmsk`] module*/
pub type OTG_HS_DIEPEMPMSK = crate::Reg<otg_hs_diepempmsk::OTG_HS_DIEPEMPMSKrs>;
///OTG_HS device IN endpoint FIFO empty interrupt mask register
pub mod otg_hs_diepempmsk;
/**OTG_HS_DEACHINT (rw) register accessor: OTG_HS device each endpoint interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_deachint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_deachint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DEACHINT)

For information about available fields see [`mod@otg_hs_deachint`] module*/
pub type OTG_HS_DEACHINT = crate::Reg<otg_hs_deachint::OTG_HS_DEACHINTrs>;
///OTG_HS device each endpoint interrupt register
pub mod otg_hs_deachint;
/**OTG_HS_DEACHINTMSK (rw) register accessor: OTG_HS device each endpoint interrupt register mask

You can [`read`](crate::Reg::read) this register and get [`otg_hs_deachintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_deachintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DEACHINTMSK)

For information about available fields see [`mod@otg_hs_deachintmsk`] module*/
pub type OTG_HS_DEACHINTMSK = crate::Reg<otg_hs_deachintmsk::OTG_HS_DEACHINTMSKrs>;
///OTG_HS device each endpoint interrupt register mask
pub mod otg_hs_deachintmsk;
/**OTG_HS_DIEPEACHMSK1 (rw) register accessor: OTG_HS device each in endpoint-1 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepeachmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepeachmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPEACHMSK1)

For information about available fields see [`mod@otg_hs_diepeachmsk1`] module*/
pub type OTG_HS_DIEPEACHMSK1 = crate::Reg<otg_hs_diepeachmsk1::OTG_HS_DIEPEACHMSK1rs>;
///OTG_HS device each in endpoint-1 interrupt register
pub mod otg_hs_diepeachmsk1;
/**OTG_HS_DOEPEACHMSK1 (rw) register accessor: OTG_HS device each OUT endpoint-1 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepeachmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepeachmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPEACHMSK1)

For information about available fields see [`mod@otg_hs_doepeachmsk1`] module*/
pub type OTG_HS_DOEPEACHMSK1 = crate::Reg<otg_hs_doepeachmsk1::OTG_HS_DOEPEACHMSK1rs>;
///OTG_HS device each OUT endpoint-1 interrupt register
pub mod otg_hs_doepeachmsk1;
/**OTG_HS_DIEPCTL0 (rw) register accessor: OTG device endpoint-0 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPCTL0)

For information about available fields see [`mod@otg_hs_diepctl0`] module*/
pub type OTG_HS_DIEPCTL0 = crate::Reg<otg_hs_diepctl0::OTG_HS_DIEPCTL0rs>;
///OTG device endpoint-0 control register
pub mod otg_hs_diepctl0;
/**OTG_HS_DIEPCTL1 (rw) register accessor: OTG device endpoint-1 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPCTL1)

For information about available fields see [`mod@otg_hs_diepctl1`] module*/
pub type OTG_HS_DIEPCTL1 = crate::Reg<otg_hs_diepctl1::OTG_HS_DIEPCTL1rs>;
///OTG device endpoint-1 control register
pub mod otg_hs_diepctl1;
/**OTG_HS_DIEPCTL2 (rw) register accessor: OTG device endpoint-2 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPCTL2)

For information about available fields see [`mod@otg_hs_diepctl2`] module*/
pub type OTG_HS_DIEPCTL2 = crate::Reg<otg_hs_diepctl2::OTG_HS_DIEPCTL2rs>;
///OTG device endpoint-2 control register
pub mod otg_hs_diepctl2;
/**OTG_HS_DIEPCTL3 (rw) register accessor: OTG device endpoint-3 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPCTL3)

For information about available fields see [`mod@otg_hs_diepctl3`] module*/
pub type OTG_HS_DIEPCTL3 = crate::Reg<otg_hs_diepctl3::OTG_HS_DIEPCTL3rs>;
///OTG device endpoint-3 control register
pub mod otg_hs_diepctl3;
/**OTG_HS_DIEPCTL4 (rw) register accessor: OTG device endpoint-4 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPCTL4)

For information about available fields see [`mod@otg_hs_diepctl4`] module*/
pub type OTG_HS_DIEPCTL4 = crate::Reg<otg_hs_diepctl4::OTG_HS_DIEPCTL4rs>;
///OTG device endpoint-4 control register
pub mod otg_hs_diepctl4;
/**OTG_HS_DIEPCTL5 (rw) register accessor: OTG device endpoint-5 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPCTL5)

For information about available fields see [`mod@otg_hs_diepctl5`] module*/
pub type OTG_HS_DIEPCTL5 = crate::Reg<otg_hs_diepctl5::OTG_HS_DIEPCTL5rs>;
///OTG device endpoint-5 control register
pub mod otg_hs_diepctl5;
/**OTG_HS_DIEPCTL6 (rw) register accessor: OTG device endpoint-6 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPCTL6)

For information about available fields see [`mod@otg_hs_diepctl6`] module*/
pub type OTG_HS_DIEPCTL6 = crate::Reg<otg_hs_diepctl6::OTG_HS_DIEPCTL6rs>;
///OTG device endpoint-6 control register
pub mod otg_hs_diepctl6;
/**OTG_HS_DIEPCTL7 (rw) register accessor: OTG device endpoint-7 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPCTL7)

For information about available fields see [`mod@otg_hs_diepctl7`] module*/
pub type OTG_HS_DIEPCTL7 = crate::Reg<otg_hs_diepctl7::OTG_HS_DIEPCTL7rs>;
///OTG device endpoint-7 control register
pub mod otg_hs_diepctl7;
/**OTG_HS_DIEPINT0 (rw) register accessor: OTG device endpoint-0 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPINT0)

For information about available fields see [`mod@otg_hs_diepint0`] module*/
pub type OTG_HS_DIEPINT0 = crate::Reg<otg_hs_diepint0::OTG_HS_DIEPINT0rs>;
///OTG device endpoint-0 interrupt register
pub mod otg_hs_diepint0;
/**OTG_HS_DIEPINT1 (rw) register accessor: OTG device endpoint-1 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPINT1)

For information about available fields see [`mod@otg_hs_diepint1`] module*/
pub type OTG_HS_DIEPINT1 = crate::Reg<otg_hs_diepint1::OTG_HS_DIEPINT1rs>;
///OTG device endpoint-1 interrupt register
pub mod otg_hs_diepint1;
/**OTG_HS_DIEPINT2 (rw) register accessor: OTG device endpoint-2 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPINT2)

For information about available fields see [`mod@otg_hs_diepint2`] module*/
pub type OTG_HS_DIEPINT2 = crate::Reg<otg_hs_diepint2::OTG_HS_DIEPINT2rs>;
///OTG device endpoint-2 interrupt register
pub mod otg_hs_diepint2;
/**OTG_HS_DIEPINT3 (rw) register accessor: OTG device endpoint-3 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPINT3)

For information about available fields see [`mod@otg_hs_diepint3`] module*/
pub type OTG_HS_DIEPINT3 = crate::Reg<otg_hs_diepint3::OTG_HS_DIEPINT3rs>;
///OTG device endpoint-3 interrupt register
pub mod otg_hs_diepint3;
/**OTG_HS_DIEPINT4 (rw) register accessor: OTG device endpoint-4 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPINT4)

For information about available fields see [`mod@otg_hs_diepint4`] module*/
pub type OTG_HS_DIEPINT4 = crate::Reg<otg_hs_diepint4::OTG_HS_DIEPINT4rs>;
///OTG device endpoint-4 interrupt register
pub mod otg_hs_diepint4;
/**OTG_HS_DIEPINT5 (rw) register accessor: OTG device endpoint-5 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPINT5)

For information about available fields see [`mod@otg_hs_diepint5`] module*/
pub type OTG_HS_DIEPINT5 = crate::Reg<otg_hs_diepint5::OTG_HS_DIEPINT5rs>;
///OTG device endpoint-5 interrupt register
pub mod otg_hs_diepint5;
/**OTG_HS_DIEPINT6 (rw) register accessor: OTG device endpoint-6 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPINT6)

For information about available fields see [`mod@otg_hs_diepint6`] module*/
pub type OTG_HS_DIEPINT6 = crate::Reg<otg_hs_diepint6::OTG_HS_DIEPINT6rs>;
///OTG device endpoint-6 interrupt register
pub mod otg_hs_diepint6;
/**OTG_HS_DIEPINT7 (rw) register accessor: OTG device endpoint-7 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPINT7)

For information about available fields see [`mod@otg_hs_diepint7`] module*/
pub type OTG_HS_DIEPINT7 = crate::Reg<otg_hs_diepint7::OTG_HS_DIEPINT7rs>;
///OTG device endpoint-7 interrupt register
pub mod otg_hs_diepint7;
/**OTG_HS_DIEPTSIZ0 (rw) register accessor: OTG_HS device IN endpoint 0 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPTSIZ0)

For information about available fields see [`mod@otg_hs_dieptsiz0`] module*/
pub type OTG_HS_DIEPTSIZ0 = crate::Reg<otg_hs_dieptsiz0::OTG_HS_DIEPTSIZ0rs>;
///OTG_HS device IN endpoint 0 transfer size register
pub mod otg_hs_dieptsiz0;
/**OTG_HS_DIEPDMA1 (rw) register accessor: OTG_HS device endpoint-1 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepdma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepdma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPDMA1)

For information about available fields see [`mod@otg_hs_diepdma1`] module*/
pub type OTG_HS_DIEPDMA1 = crate::Reg<otg_hs_diepdma1::OTG_HS_DIEPDMA1rs>;
///OTG_HS device endpoint-1 DMA address register
pub mod otg_hs_diepdma1;
/**OTG_HS_DIEPDMA2 (rw) register accessor: OTG_HS device endpoint-2 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepdma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepdma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPDMA2)

For information about available fields see [`mod@otg_hs_diepdma2`] module*/
pub type OTG_HS_DIEPDMA2 = crate::Reg<otg_hs_diepdma2::OTG_HS_DIEPDMA2rs>;
///OTG_HS device endpoint-2 DMA address register
pub mod otg_hs_diepdma2;
/**OTG_HS_DIEPDMA3 (rw) register accessor: OTG_HS device endpoint-3 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepdma3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepdma3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPDMA3)

For information about available fields see [`mod@otg_hs_diepdma3`] module*/
pub type OTG_HS_DIEPDMA3 = crate::Reg<otg_hs_diepdma3::OTG_HS_DIEPDMA3rs>;
///OTG_HS device endpoint-3 DMA address register
pub mod otg_hs_diepdma3;
/**OTG_HS_DIEPDMA4 (rw) register accessor: OTG_HS device endpoint-4 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepdma4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepdma4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPDMA4)

For information about available fields see [`mod@otg_hs_diepdma4`] module*/
pub type OTG_HS_DIEPDMA4 = crate::Reg<otg_hs_diepdma4::OTG_HS_DIEPDMA4rs>;
///OTG_HS device endpoint-4 DMA address register
pub mod otg_hs_diepdma4;
/**OTG_HS_DIEPDMA5 (rw) register accessor: OTG_HS device endpoint-5 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_diepdma5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepdma5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPDMA5)

For information about available fields see [`mod@otg_hs_diepdma5`] module*/
pub type OTG_HS_DIEPDMA5 = crate::Reg<otg_hs_diepdma5::OTG_HS_DIEPDMA5rs>;
///OTG_HS device endpoint-5 DMA address register
pub mod otg_hs_diepdma5;
/**OTG_HS_DTXFSTS0 (r) register accessor: OTG_HS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dtxfsts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DTXFSTS0)

For information about available fields see [`mod@otg_hs_dtxfsts0`] module*/
pub type OTG_HS_DTXFSTS0 = crate::Reg<otg_hs_dtxfsts0::OTG_HS_DTXFSTS0rs>;
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts0;
/**OTG_HS_DTXFSTS1 (r) register accessor: OTG_HS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dtxfsts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DTXFSTS1)

For information about available fields see [`mod@otg_hs_dtxfsts1`] module*/
pub type OTG_HS_DTXFSTS1 = crate::Reg<otg_hs_dtxfsts1::OTG_HS_DTXFSTS1rs>;
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts1;
/**OTG_HS_DTXFSTS2 (r) register accessor: OTG_HS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dtxfsts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DTXFSTS2)

For information about available fields see [`mod@otg_hs_dtxfsts2`] module*/
pub type OTG_HS_DTXFSTS2 = crate::Reg<otg_hs_dtxfsts2::OTG_HS_DTXFSTS2rs>;
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts2;
/**OTG_HS_DTXFSTS3 (r) register accessor: OTG_HS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dtxfsts3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DTXFSTS3)

For information about available fields see [`mod@otg_hs_dtxfsts3`] module*/
pub type OTG_HS_DTXFSTS3 = crate::Reg<otg_hs_dtxfsts3::OTG_HS_DTXFSTS3rs>;
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts3;
/**OTG_HS_DTXFSTS4 (r) register accessor: OTG_HS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dtxfsts4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DTXFSTS4)

For information about available fields see [`mod@otg_hs_dtxfsts4`] module*/
pub type OTG_HS_DTXFSTS4 = crate::Reg<otg_hs_dtxfsts4::OTG_HS_DTXFSTS4rs>;
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts4;
/**OTG_HS_DTXFSTS5 (r) register accessor: OTG_HS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dtxfsts5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DTXFSTS5)

For information about available fields see [`mod@otg_hs_dtxfsts5`] module*/
pub type OTG_HS_DTXFSTS5 = crate::Reg<otg_hs_dtxfsts5::OTG_HS_DTXFSTS5rs>;
///OTG_HS device IN endpoint transmit FIFO status register
pub mod otg_hs_dtxfsts5;
/**OTG_HS_DIEPTSIZ1 (rw) register accessor: OTG_HS device endpoint transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPTSIZ1)

For information about available fields see [`mod@otg_hs_dieptsiz1`] module*/
pub type OTG_HS_DIEPTSIZ1 = crate::Reg<otg_hs_dieptsiz1::OTG_HS_DIEPTSIZ1rs>;
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz1;
/**OTG_HS_DIEPTSIZ2 (rw) register accessor: OTG_HS device endpoint transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPTSIZ2)

For information about available fields see [`mod@otg_hs_dieptsiz2`] module*/
pub type OTG_HS_DIEPTSIZ2 = crate::Reg<otg_hs_dieptsiz2::OTG_HS_DIEPTSIZ2rs>;
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz2;
/**OTG_HS_DIEPTSIZ3 (rw) register accessor: OTG_HS device endpoint transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPTSIZ3)

For information about available fields see [`mod@otg_hs_dieptsiz3`] module*/
pub type OTG_HS_DIEPTSIZ3 = crate::Reg<otg_hs_dieptsiz3::OTG_HS_DIEPTSIZ3rs>;
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz3;
/**OTG_HS_DIEPTSIZ4 (rw) register accessor: OTG_HS device endpoint transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPTSIZ4)

For information about available fields see [`mod@otg_hs_dieptsiz4`] module*/
pub type OTG_HS_DIEPTSIZ4 = crate::Reg<otg_hs_dieptsiz4::OTG_HS_DIEPTSIZ4rs>;
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz4;
/**OTG_HS_DIEPTSIZ5 (rw) register accessor: OTG_HS device endpoint transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DIEPTSIZ5)

For information about available fields see [`mod@otg_hs_dieptsiz5`] module*/
pub type OTG_HS_DIEPTSIZ5 = crate::Reg<otg_hs_dieptsiz5::OTG_HS_DIEPTSIZ5rs>;
///OTG_HS device endpoint transfer size register
pub mod otg_hs_dieptsiz5;
/**OTG_HS_DOEPCTL0 (rw) register accessor: OTG_HS device control OUT endpoint 0 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPCTL0)

For information about available fields see [`mod@otg_hs_doepctl0`] module*/
pub type OTG_HS_DOEPCTL0 = crate::Reg<otg_hs_doepctl0::OTG_HS_DOEPCTL0rs>;
///OTG_HS device control OUT endpoint 0 control register
pub mod otg_hs_doepctl0;
/**OTG_HS_DOEPCTL1 (rw) register accessor: OTG device endpoint-1 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPCTL1)

For information about available fields see [`mod@otg_hs_doepctl1`] module*/
pub type OTG_HS_DOEPCTL1 = crate::Reg<otg_hs_doepctl1::OTG_HS_DOEPCTL1rs>;
///OTG device endpoint-1 control register
pub mod otg_hs_doepctl1;
/**OTG_HS_DOEPCTL2 (rw) register accessor: OTG device endpoint-2 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPCTL2)

For information about available fields see [`mod@otg_hs_doepctl2`] module*/
pub type OTG_HS_DOEPCTL2 = crate::Reg<otg_hs_doepctl2::OTG_HS_DOEPCTL2rs>;
///OTG device endpoint-2 control register
pub mod otg_hs_doepctl2;
/**OTG_HS_DOEPCTL3 (rw) register accessor: OTG device endpoint-3 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPCTL3)

For information about available fields see [`mod@otg_hs_doepctl3`] module*/
pub type OTG_HS_DOEPCTL3 = crate::Reg<otg_hs_doepctl3::OTG_HS_DOEPCTL3rs>;
///OTG device endpoint-3 control register
pub mod otg_hs_doepctl3;
/**OTG_HS_DOEPINT0 (rw) register accessor: OTG_HS device endpoint-0 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPINT0)

For information about available fields see [`mod@otg_hs_doepint0`] module*/
pub type OTG_HS_DOEPINT0 = crate::Reg<otg_hs_doepint0::OTG_HS_DOEPINT0rs>;
///OTG_HS device endpoint-0 interrupt register
pub mod otg_hs_doepint0;
/**OTG_HS_DOEPINT1 (rw) register accessor: OTG_HS device endpoint-1 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPINT1)

For information about available fields see [`mod@otg_hs_doepint1`] module*/
pub type OTG_HS_DOEPINT1 = crate::Reg<otg_hs_doepint1::OTG_HS_DOEPINT1rs>;
///OTG_HS device endpoint-1 interrupt register
pub mod otg_hs_doepint1;
/**OTG_HS_DOEPINT2 (rw) register accessor: OTG_HS device endpoint-2 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPINT2)

For information about available fields see [`mod@otg_hs_doepint2`] module*/
pub type OTG_HS_DOEPINT2 = crate::Reg<otg_hs_doepint2::OTG_HS_DOEPINT2rs>;
///OTG_HS device endpoint-2 interrupt register
pub mod otg_hs_doepint2;
/**OTG_HS_DOEPINT3 (rw) register accessor: OTG_HS device endpoint-3 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPINT3)

For information about available fields see [`mod@otg_hs_doepint3`] module*/
pub type OTG_HS_DOEPINT3 = crate::Reg<otg_hs_doepint3::OTG_HS_DOEPINT3rs>;
///OTG_HS device endpoint-3 interrupt register
pub mod otg_hs_doepint3;
/**OTG_HS_DOEPINT4 (rw) register accessor: OTG_HS device endpoint-4 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPINT4)

For information about available fields see [`mod@otg_hs_doepint4`] module*/
pub type OTG_HS_DOEPINT4 = crate::Reg<otg_hs_doepint4::OTG_HS_DOEPINT4rs>;
///OTG_HS device endpoint-4 interrupt register
pub mod otg_hs_doepint4;
/**OTG_HS_DOEPINT5 (rw) register accessor: OTG_HS device endpoint-5 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPINT5)

For information about available fields see [`mod@otg_hs_doepint5`] module*/
pub type OTG_HS_DOEPINT5 = crate::Reg<otg_hs_doepint5::OTG_HS_DOEPINT5rs>;
///OTG_HS device endpoint-5 interrupt register
pub mod otg_hs_doepint5;
/**OTG_HS_DOEPINT6 (rw) register accessor: OTG_HS device endpoint-6 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPINT6)

For information about available fields see [`mod@otg_hs_doepint6`] module*/
pub type OTG_HS_DOEPINT6 = crate::Reg<otg_hs_doepint6::OTG_HS_DOEPINT6rs>;
///OTG_HS device endpoint-6 interrupt register
pub mod otg_hs_doepint6;
/**OTG_HS_DOEPINT7 (rw) register accessor: OTG_HS device endpoint-7 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPINT7)

For information about available fields see [`mod@otg_hs_doepint7`] module*/
pub type OTG_HS_DOEPINT7 = crate::Reg<otg_hs_doepint7::OTG_HS_DOEPINT7rs>;
///OTG_HS device endpoint-7 interrupt register
pub mod otg_hs_doepint7;
/**OTG_HS_DOEPTSIZ0 (rw) register accessor: OTG_HS device endpoint-1 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doeptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doeptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPTSIZ0)

For information about available fields see [`mod@otg_hs_doeptsiz0`] module*/
pub type OTG_HS_DOEPTSIZ0 = crate::Reg<otg_hs_doeptsiz0::OTG_HS_DOEPTSIZ0rs>;
///OTG_HS device endpoint-1 transfer size register
pub mod otg_hs_doeptsiz0;
/**OTG_HS_DOEPTSIZ1 (rw) register accessor: OTG_HS device endpoint-2 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doeptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doeptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPTSIZ1)

For information about available fields see [`mod@otg_hs_doeptsiz1`] module*/
pub type OTG_HS_DOEPTSIZ1 = crate::Reg<otg_hs_doeptsiz1::OTG_HS_DOEPTSIZ1rs>;
///OTG_HS device endpoint-2 transfer size register
pub mod otg_hs_doeptsiz1;
/**OTG_HS_DOEPTSIZ2 (rw) register accessor: OTG_HS device endpoint-3 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doeptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doeptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPTSIZ2)

For information about available fields see [`mod@otg_hs_doeptsiz2`] module*/
pub type OTG_HS_DOEPTSIZ2 = crate::Reg<otg_hs_doeptsiz2::OTG_HS_DOEPTSIZ2rs>;
///OTG_HS device endpoint-3 transfer size register
pub mod otg_hs_doeptsiz2;
/**OTG_HS_DOEPTSIZ3 (rw) register accessor: OTG_HS device endpoint-4 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doeptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doeptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPTSIZ3)

For information about available fields see [`mod@otg_hs_doeptsiz3`] module*/
pub type OTG_HS_DOEPTSIZ3 = crate::Reg<otg_hs_doeptsiz3::OTG_HS_DOEPTSIZ3rs>;
///OTG_HS device endpoint-4 transfer size register
pub mod otg_hs_doeptsiz3;
/**OTG_HS_DOEPTSIZ4 (rw) register accessor: OTG_HS device endpoint-5 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doeptsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doeptsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DOEPTSIZ4)

For information about available fields see [`mod@otg_hs_doeptsiz4`] module*/
pub type OTG_HS_DOEPTSIZ4 = crate::Reg<otg_hs_doeptsiz4::OTG_HS_DOEPTSIZ4rs>;
///OTG_HS device endpoint-5 transfer size register
pub mod otg_hs_doeptsiz4;
