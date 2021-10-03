#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS device configuration register (OTG_FS_DCFG)"]
    pub fs_dcfg: crate::Reg<fs_dcfg::FS_DCFG_SPEC>,
    #[doc = "0x04 - OTG_FS device control register (OTG_FS_DCTL)"]
    pub fs_dctl: crate::Reg<fs_dctl::FS_DCTL_SPEC>,
    #[doc = "0x08 - OTG_FS device status register (OTG_FS_DSTS)"]
    pub fs_dsts: crate::Reg<fs_dsts::FS_DSTS_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
    pub fs_diepmsk: crate::Reg<fs_diepmsk::FS_DIEPMSK_SPEC>,
    #[doc = "0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
    pub fs_doepmsk: crate::Reg<fs_doepmsk::FS_DOEPMSK_SPEC>,
    #[doc = "0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
    pub fs_daint: crate::Reg<fs_daint::FS_DAINT_SPEC>,
    #[doc = "0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
    pub fs_daintmsk: crate::Reg<fs_daintmsk::FS_DAINTMSK_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - OTG_FS device VBUS discharge time register"]
    pub dvbusdis: crate::Reg<dvbusdis::DVBUSDIS_SPEC>,
    #[doc = "0x2c - OTG_FS device VBUS pulsing time register"]
    pub dvbuspulse: crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>,
    _reserved10: [u8; 0xc8],
    #[doc = "0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
    pub fs_diepctl0: crate::Reg<fs_diepctl0::FS_DIEPCTL0_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x108 - device endpoint-x interrupt register"]
    pub diepint0: crate::Reg<diepint0::DIEPINT0_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x110 - device endpoint-0 transfer size register"]
    pub dieptsiz0: crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x118 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts0: crate::Reg<dtxfsts0::DTXFSTS0_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub diepctl1: crate::Reg<diepctl1::DIEPCTL1_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    pub diepint1: crate::Reg<diepint1::DIEPINT1_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x130 - device endpoint-1 transfer size register"]
    pub dieptsiz1: crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x138 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts1: crate::Reg<dtxfsts1::DTXFSTS1_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub diepctl2: crate::Reg<diepctl2::DIEPCTL2_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x148 - device endpoint-2 interrupt register"]
    pub diepint2: crate::Reg<diepint2::DIEPINT2_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x150 - device endpoint-2 transfer size register"]
    pub dieptsiz2: crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x158 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts2: crate::Reg<dtxfsts2::DTXFSTS2_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub diepctl3: crate::Reg<diepctl3::DIEPCTL3_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x168 - device endpoint-3 interrupt register"]
    pub diepint3: crate::Reg<diepint3::DIEPINT3_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x170 - device endpoint-3 transfer size register"]
    pub dieptsiz3: crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x178 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts3: crate::Reg<dtxfsts3::DTXFSTS3_SPEC>,
    _reserved26: [u8; 0x0184],
    #[doc = "0x300 - device endpoint-0 control register"]
    pub doepctl0: crate::Reg<doepctl0::DOEPCTL0_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x308 - device endpoint-0 interrupt register"]
    pub doepint0: crate::Reg<doepint0::DOEPINT0_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0x310 - device OUT endpoint-0 transfer size register"]
    pub doeptsiz0: crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>,
    _reserved29: [u8; 0x0c],
    #[doc = "0x320 - device endpoint-1 control register"]
    pub doepctl1: crate::Reg<doepctl1::DOEPCTL1_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x328 - device endpoint-1 interrupt register"]
    pub doepint1: crate::Reg<doepint1::DOEPINT1_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x330 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz1: crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>,
    _reserved32: [u8; 0x0c],
    #[doc = "0x340 - device endpoint-2 control register"]
    pub doepctl2: crate::Reg<doepctl2::DOEPCTL2_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0x348 - device endpoint-2 interrupt register"]
    pub doepint2: crate::Reg<doepint2::DOEPINT2_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0x350 - device OUT endpoint-2 transfer size register"]
    pub doeptsiz2: crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>,
    _reserved35: [u8; 0x0c],
    #[doc = "0x360 - device endpoint-3 control register"]
    pub doepctl3: crate::Reg<doepctl3::DOEPCTL3_SPEC>,
    _reserved36: [u8; 0x04],
    #[doc = "0x368 - device endpoint-3 interrupt register"]
    pub doepint3: crate::Reg<doepint3::DOEPINT3_SPEC>,
    _reserved37: [u8; 0x04],
    #[doc = "0x370 - device OUT endpoint-3 transfer size register"]
    pub doeptsiz3: crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>,
}
#[doc = "FS_DCFG register accessor: an alias for `Reg<FS_DCFG_SPEC>`"]
pub type FS_DCFG = crate::Reg<fs_dcfg::FS_DCFG_SPEC>;
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub mod fs_dcfg;
#[doc = "FS_DCTL register accessor: an alias for `Reg<FS_DCTL_SPEC>`"]
pub type FS_DCTL = crate::Reg<fs_dctl::FS_DCTL_SPEC>;
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub mod fs_dctl;
#[doc = "FS_DSTS register accessor: an alias for `Reg<FS_DSTS_SPEC>`"]
pub type FS_DSTS = crate::Reg<fs_dsts::FS_DSTS_SPEC>;
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub mod fs_dsts;
#[doc = "FS_DIEPMSK register accessor: an alias for `Reg<FS_DIEPMSK_SPEC>`"]
pub type FS_DIEPMSK = crate::Reg<fs_diepmsk::FS_DIEPMSK_SPEC>;
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub mod fs_diepmsk;
#[doc = "FS_DOEPMSK register accessor: an alias for `Reg<FS_DOEPMSK_SPEC>`"]
pub type FS_DOEPMSK = crate::Reg<fs_doepmsk::FS_DOEPMSK_SPEC>;
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub mod fs_doepmsk;
#[doc = "FS_DAINT register accessor: an alias for `Reg<FS_DAINT_SPEC>`"]
pub type FS_DAINT = crate::Reg<fs_daint::FS_DAINT_SPEC>;
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub mod fs_daint;
#[doc = "FS_DAINTMSK register accessor: an alias for `Reg<FS_DAINTMSK_SPEC>`"]
pub type FS_DAINTMSK = crate::Reg<fs_daintmsk::FS_DAINTMSK_SPEC>;
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub mod fs_daintmsk;
#[doc = "DVBUSDIS register accessor: an alias for `Reg<DVBUSDIS_SPEC>`"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "OTG_FS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "OTG_FS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "DIEPEMPMSK register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "FS_DIEPCTL0 register accessor: an alias for `Reg<FS_DIEPCTL0_SPEC>`"]
pub type FS_DIEPCTL0 = crate::Reg<fs_diepctl0::FS_DIEPCTL0_SPEC>;
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub mod fs_diepctl0;
#[doc = "DIEPCTL1 register accessor: an alias for `Reg<DIEPCTL1_SPEC>`"]
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod diepctl1;
#[doc = "DIEPCTL2 register accessor: an alias for `Reg<DIEPCTL2_SPEC>`"]
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2_SPEC>;
#[doc = "OTG device endpoint-2 control register"]
pub mod diepctl2;
#[doc = "DIEPCTL3 register accessor: an alias for `Reg<DIEPCTL3_SPEC>`"]
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3_SPEC>;
#[doc = "OTG device endpoint-3 control register"]
pub mod diepctl3;
#[doc = "DOEPCTL0 register accessor: an alias for `Reg<DOEPCTL0_SPEC>`"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = "device endpoint-0 control register"]
pub mod doepctl0;
#[doc = "DOEPCTL1 register accessor: an alias for `Reg<DOEPCTL1_SPEC>`"]
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1_SPEC>;
#[doc = "device endpoint-1 control register"]
pub mod doepctl1;
#[doc = "DOEPCTL2 register accessor: an alias for `Reg<DOEPCTL2_SPEC>`"]
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2_SPEC>;
#[doc = "device endpoint-2 control register"]
pub mod doepctl2;
#[doc = "DOEPCTL3 register accessor: an alias for `Reg<DOEPCTL3_SPEC>`"]
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3_SPEC>;
#[doc = "device endpoint-3 control register"]
pub mod doepctl3;
#[doc = "DIEPINT0 register accessor: an alias for `Reg<DIEPINT0_SPEC>`"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
#[doc = "device endpoint-x interrupt register"]
pub mod diepint0;
#[doc = "DIEPINT1 register accessor: an alias for `Reg<DIEPINT1_SPEC>`"]
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1_SPEC>;
#[doc = "device endpoint-1 interrupt register"]
pub mod diepint1;
#[doc = "DIEPINT2 register accessor: an alias for `Reg<DIEPINT2_SPEC>`"]
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2_SPEC>;
#[doc = "device endpoint-2 interrupt register"]
pub mod diepint2;
#[doc = "DIEPINT3 register accessor: an alias for `Reg<DIEPINT3_SPEC>`"]
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3_SPEC>;
#[doc = "device endpoint-3 interrupt register"]
pub mod diepint3;
#[doc = "DOEPINT0 register accessor: an alias for `Reg<DOEPINT0_SPEC>`"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
#[doc = "device endpoint-0 interrupt register"]
pub mod doepint0;
#[doc = "DOEPINT1 register accessor: an alias for `Reg<DOEPINT1_SPEC>`"]
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1_SPEC>;
#[doc = "device endpoint-1 interrupt register"]
pub mod doepint1;
#[doc = "DOEPINT2 register accessor: an alias for `Reg<DOEPINT2_SPEC>`"]
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2_SPEC>;
#[doc = "device endpoint-2 interrupt register"]
pub mod doepint2;
#[doc = "DOEPINT3 register accessor: an alias for `Reg<DOEPINT3_SPEC>`"]
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3_SPEC>;
#[doc = "device endpoint-3 interrupt register"]
pub mod doepint3;
#[doc = "DIEPTSIZ0 register accessor: an alias for `Reg<DIEPTSIZ0_SPEC>`"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
#[doc = "device endpoint-0 transfer size register"]
pub mod dieptsiz0;
#[doc = "DOEPTSIZ0 register accessor: an alias for `Reg<DOEPTSIZ0_SPEC>`"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = "device OUT endpoint-0 transfer size register"]
pub mod doeptsiz0;
#[doc = "DIEPTSIZ1 register accessor: an alias for `Reg<DIEPTSIZ1_SPEC>`"]
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>;
#[doc = "device endpoint-1 transfer size register"]
pub mod dieptsiz1;
#[doc = "DIEPTSIZ2 register accessor: an alias for `Reg<DIEPTSIZ2_SPEC>`"]
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>;
#[doc = "device endpoint-2 transfer size register"]
pub mod dieptsiz2;
#[doc = "DIEPTSIZ3 register accessor: an alias for `Reg<DIEPTSIZ3_SPEC>`"]
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>;
#[doc = "device endpoint-3 transfer size register"]
pub mod dieptsiz3;
#[doc = "DTXFSTS0 register accessor: an alias for `Reg<DTXFSTS0_SPEC>`"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts0;
#[doc = "DTXFSTS1 register accessor: an alias for `Reg<DTXFSTS1_SPEC>`"]
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts1;
#[doc = "DTXFSTS2 register accessor: an alias for `Reg<DTXFSTS2_SPEC>`"]
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts2;
#[doc = "DTXFSTS3 register accessor: an alias for `Reg<DTXFSTS3_SPEC>`"]
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts3;
#[doc = "DOEPTSIZ1 register accessor: an alias for `Reg<DOEPTSIZ1_SPEC>`"]
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>;
#[doc = "device OUT endpoint-1 transfer size register"]
pub mod doeptsiz1;
#[doc = "DOEPTSIZ2 register accessor: an alias for `Reg<DOEPTSIZ2_SPEC>`"]
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>;
#[doc = "device OUT endpoint-2 transfer size register"]
pub mod doeptsiz2;
#[doc = "DOEPTSIZ3 register accessor: an alias for `Reg<DOEPTSIZ3_SPEC>`"]
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>;
#[doc = "device OUT endpoint-3 transfer size register"]
pub mod doeptsiz3;
