#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS device configuration register (OTG_FS_DCFG)"]
    pub otg_fs_dcfg: crate::Reg<otg_fs_dcfg::OTG_FS_DCFG_SPEC>,
    #[doc = "0x04 - OTG_FS device control register (OTG_FS_DCTL)"]
    pub otg_fs_dctl: crate::Reg<otg_fs_dctl::OTG_FS_DCTL_SPEC>,
    #[doc = "0x08 - OTG_FS device status register (OTG_FS_DSTS)"]
    pub otg_fs_dsts: crate::Reg<otg_fs_dsts::OTG_FS_DSTS_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
    pub otg_fs_diepmsk: crate::Reg<otg_fs_diepmsk::OTG_FS_DIEPMSK_SPEC>,
    #[doc = "0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
    pub otg_fs_doepmsk: crate::Reg<otg_fs_doepmsk::OTG_FS_DOEPMSK_SPEC>,
    #[doc = "0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
    pub otg_fs_daint: crate::Reg<otg_fs_daint::OTG_FS_DAINT_SPEC>,
    #[doc = "0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
    pub otg_fs_daintmsk: crate::Reg<otg_fs_daintmsk::OTG_FS_DAINTMSK_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - OTG_FS device VBUS discharge time register"]
    pub otg_fs_dvbusdis: crate::Reg<otg_fs_dvbusdis::OTG_FS_DVBUSDIS_SPEC>,
    #[doc = "0x2c - OTG_FS device VBUS pulsing time register"]
    pub otg_fs_dvbuspulse: crate::Reg<otg_fs_dvbuspulse::OTG_FS_DVBUSPULSE_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register"]
    pub otg_fs_diepempmsk: crate::Reg<otg_fs_diepempmsk::OTG_FS_DIEPEMPMSK_SPEC>,
    _reserved10: [u8; 0xc8],
    #[doc = "0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
    pub otg_fs_diepctl0: crate::Reg<otg_fs_diepctl0::OTG_FS_DIEPCTL0_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x108 - device endpoint-x interrupt register"]
    pub otg_fs_diepint0: crate::Reg<otg_fs_diepint0::OTG_FS_DIEPINT0_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x110 - device endpoint-0 transfer size register"]
    pub otg_fs_dieptsiz0: crate::Reg<otg_fs_dieptsiz0::OTG_FS_DIEPTSIZ0_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x118 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts0: crate::Reg<otg_fs_dtxfsts0::OTG_FS_DTXFSTS0_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub otg_fs_diepctl1: crate::Reg<otg_fs_diepctl1::OTG_FS_DIEPCTL1_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    pub otg_fs_diepint1: crate::Reg<otg_fs_diepint1::OTG_FS_DIEPINT1_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x130 - device endpoint-1 transfer size register"]
    pub otg_fs_dieptsiz1: crate::Reg<otg_fs_dieptsiz1::OTG_FS_DIEPTSIZ1_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x138 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts1: crate::Reg<otg_fs_dtxfsts1::OTG_FS_DTXFSTS1_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub otg_fs_diepctl2: crate::Reg<otg_fs_diepctl2::OTG_FS_DIEPCTL2_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x148 - device endpoint-2 interrupt register"]
    pub otg_fs_diepint2: crate::Reg<otg_fs_diepint2::OTG_FS_DIEPINT2_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x150 - device endpoint-2 transfer size register"]
    pub otg_fs_dieptsiz2: crate::Reg<otg_fs_dieptsiz2::OTG_FS_DIEPTSIZ2_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x158 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts2: crate::Reg<otg_fs_dtxfsts2::OTG_FS_DTXFSTS2_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub otg_fs_diepctl3: crate::Reg<otg_fs_diepctl3::OTG_FS_DIEPCTL3_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x168 - device endpoint-3 interrupt register"]
    pub otg_fs_diepint3: crate::Reg<otg_fs_diepint3::OTG_FS_DIEPINT3_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x170 - device endpoint-3 transfer size register"]
    pub otg_fs_dieptsiz3: crate::Reg<otg_fs_dieptsiz3::OTG_FS_DIEPTSIZ3_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x178 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts3: crate::Reg<otg_fs_dtxfsts3::OTG_FS_DTXFSTS3_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x180 - OTG device endpoint-4 control register"]
    pub otg_fs_diepctl4: crate::Reg<otg_fs_diepctl4::OTG_FS_DIEPCTL4_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x188 - device endpoint-4 interrupt register"]
    pub otg_fs_diepint4: crate::Reg<otg_fs_diepint4::OTG_FS_DIEPINT4_SPEC>,
    _reserved28: [u8; 0x08],
    #[doc = "0x194 - device endpoint-4 transfer size register"]
    pub otg_fs_dieptsiz4: crate::Reg<otg_fs_dieptsiz4::OTG_FS_DIEPTSIZ4_SPEC>,
    _reserved29: [u8; 0x04],
    #[doc = "0x19c - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts4: crate::Reg<otg_fs_dtxfsts4::OTG_FS_DTXFSTS4_SPEC>,
    #[doc = "0x1a0 - OTG device endpoint-5 control register"]
    pub otg_fs_diepctl5: crate::Reg<otg_fs_diepctl5::OTG_FS_DIEPCTL5_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x1a8 - device endpoint-5 interrupt register"]
    pub otg_fs_diepint5: crate::Reg<otg_fs_diepint5::OTG_FS_DIEPINT5_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x1b0 - device endpoint-5 transfer size register"]
    pub otg_fs_dieptsiz55: crate::Reg<otg_fs_dieptsiz55::OTG_FS_DIEPTSIZ55_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0x1b8 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts55: crate::Reg<otg_fs_dtxfsts55::OTG_FS_DTXFSTS55_SPEC>,
    _reserved34: [u8; 0x0144],
    #[doc = "0x300 - device endpoint-0 control register"]
    pub otg_fs_doepctl0: crate::Reg<otg_fs_doepctl0::OTG_FS_DOEPCTL0_SPEC>,
    _reserved35: [u8; 0x04],
    #[doc = "0x308 - device endpoint-0 interrupt register"]
    pub otg_fs_doepint0: crate::Reg<otg_fs_doepint0::OTG_FS_DOEPINT0_SPEC>,
    _reserved36: [u8; 0x04],
    #[doc = "0x310 - device OUT endpoint-0 transfer size register"]
    pub otg_fs_doeptsiz0: crate::Reg<otg_fs_doeptsiz0::OTG_FS_DOEPTSIZ0_SPEC>,
    _reserved37: [u8; 0x0c],
    #[doc = "0x320 - device endpoint-1 control register"]
    pub otg_fs_doepctl1: crate::Reg<otg_fs_doepctl1::OTG_FS_DOEPCTL1_SPEC>,
    _reserved38: [u8; 0x04],
    #[doc = "0x328 - device endpoint-1 interrupt register"]
    pub otg_fs_doepint1: crate::Reg<otg_fs_doepint1::OTG_FS_DOEPINT1_SPEC>,
    _reserved39: [u8; 0x04],
    #[doc = "0x330 - device OUT endpoint-1 transfer size register"]
    pub otg_fs_doeptsiz1: crate::Reg<otg_fs_doeptsiz1::OTG_FS_DOEPTSIZ1_SPEC>,
    _reserved40: [u8; 0x0c],
    #[doc = "0x340 - device endpoint-2 control register"]
    pub otg_fs_doepctl2: crate::Reg<otg_fs_doepctl2::OTG_FS_DOEPCTL2_SPEC>,
    _reserved41: [u8; 0x04],
    #[doc = "0x348 - device endpoint-2 interrupt register"]
    pub otg_fs_doepint2: crate::Reg<otg_fs_doepint2::OTG_FS_DOEPINT2_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0x350 - device OUT endpoint-2 transfer size register"]
    pub otg_fs_doeptsiz2: crate::Reg<otg_fs_doeptsiz2::OTG_FS_DOEPTSIZ2_SPEC>,
    _reserved43: [u8; 0x0c],
    #[doc = "0x360 - device endpoint-3 control register"]
    pub otg_fs_doepctl3: crate::Reg<otg_fs_doepctl3::OTG_FS_DOEPCTL3_SPEC>,
    _reserved44: [u8; 0x04],
    #[doc = "0x368 - device endpoint-3 interrupt register"]
    pub otg_fs_doepint3: crate::Reg<otg_fs_doepint3::OTG_FS_DOEPINT3_SPEC>,
    _reserved45: [u8; 0x04],
    #[doc = "0x370 - device OUT endpoint-3 transfer size register"]
    pub otg_fs_doeptsiz3: crate::Reg<otg_fs_doeptsiz3::OTG_FS_DOEPTSIZ3_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0x378 - device endpoint-4 control register"]
    pub otg_fs_doepctl4: crate::Reg<otg_fs_doepctl4::OTG_FS_DOEPCTL4_SPEC>,
    _reserved47: [u8; 0x04],
    #[doc = "0x380 - device endpoint-4 interrupt register"]
    pub otg_fs_doepint4: crate::Reg<otg_fs_doepint4::OTG_FS_DOEPINT4_SPEC>,
    _reserved48: [u8; 0x04],
    #[doc = "0x388 - device OUT endpoint-4 transfer size register"]
    pub otg_fs_doeptsiz4: crate::Reg<otg_fs_doeptsiz4::OTG_FS_DOEPTSIZ4_SPEC>,
    _reserved49: [u8; 0x04],
    #[doc = "0x390 - device endpoint-5 control register"]
    pub otg_fs_doepctl5: crate::Reg<otg_fs_doepctl5::OTG_FS_DOEPCTL5_SPEC>,
    _reserved50: [u8; 0x04],
    #[doc = "0x398 - device endpoint-5 interrupt register"]
    pub otg_fs_doepint5: crate::Reg<otg_fs_doepint5::OTG_FS_DOEPINT5_SPEC>,
    _reserved51: [u8; 0x04],
    #[doc = "0x3a0 - device OUT endpoint-5 transfer size register"]
    pub otg_fs_doeptsiz5: crate::Reg<otg_fs_doeptsiz5::OTG_FS_DOEPTSIZ5_SPEC>,
}
#[doc = "OTG_FS_DCFG register accessor: an alias for `Reg<OTG_FS_DCFG_SPEC>`"]
pub type OTG_FS_DCFG = crate::Reg<otg_fs_dcfg::OTG_FS_DCFG_SPEC>;
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub mod otg_fs_dcfg;
#[doc = "OTG_FS_DCTL register accessor: an alias for `Reg<OTG_FS_DCTL_SPEC>`"]
pub type OTG_FS_DCTL = crate::Reg<otg_fs_dctl::OTG_FS_DCTL_SPEC>;
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub mod otg_fs_dctl;
#[doc = "OTG_FS_DSTS register accessor: an alias for `Reg<OTG_FS_DSTS_SPEC>`"]
pub type OTG_FS_DSTS = crate::Reg<otg_fs_dsts::OTG_FS_DSTS_SPEC>;
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub mod otg_fs_dsts;
#[doc = "OTG_FS_DIEPMSK register accessor: an alias for `Reg<OTG_FS_DIEPMSK_SPEC>`"]
pub type OTG_FS_DIEPMSK = crate::Reg<otg_fs_diepmsk::OTG_FS_DIEPMSK_SPEC>;
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub mod otg_fs_diepmsk;
#[doc = "OTG_FS_DOEPMSK register accessor: an alias for `Reg<OTG_FS_DOEPMSK_SPEC>`"]
pub type OTG_FS_DOEPMSK = crate::Reg<otg_fs_doepmsk::OTG_FS_DOEPMSK_SPEC>;
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub mod otg_fs_doepmsk;
#[doc = "OTG_FS_DAINT register accessor: an alias for `Reg<OTG_FS_DAINT_SPEC>`"]
pub type OTG_FS_DAINT = crate::Reg<otg_fs_daint::OTG_FS_DAINT_SPEC>;
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub mod otg_fs_daint;
#[doc = "OTG_FS_DAINTMSK register accessor: an alias for `Reg<OTG_FS_DAINTMSK_SPEC>`"]
pub type OTG_FS_DAINTMSK = crate::Reg<otg_fs_daintmsk::OTG_FS_DAINTMSK_SPEC>;
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub mod otg_fs_daintmsk;
#[doc = "OTG_FS_DVBUSDIS register accessor: an alias for `Reg<OTG_FS_DVBUSDIS_SPEC>`"]
pub type OTG_FS_DVBUSDIS = crate::Reg<otg_fs_dvbusdis::OTG_FS_DVBUSDIS_SPEC>;
#[doc = "OTG_FS device VBUS discharge time register"]
pub mod otg_fs_dvbusdis;
#[doc = "OTG_FS_DVBUSPULSE register accessor: an alias for `Reg<OTG_FS_DVBUSPULSE_SPEC>`"]
pub type OTG_FS_DVBUSPULSE = crate::Reg<otg_fs_dvbuspulse::OTG_FS_DVBUSPULSE_SPEC>;
#[doc = "OTG_FS device VBUS pulsing time register"]
pub mod otg_fs_dvbuspulse;
#[doc = "OTG_FS_DIEPEMPMSK register accessor: an alias for `Reg<OTG_FS_DIEPEMPMSK_SPEC>`"]
pub type OTG_FS_DIEPEMPMSK = crate::Reg<otg_fs_diepempmsk::OTG_FS_DIEPEMPMSK_SPEC>;
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub mod otg_fs_diepempmsk;
#[doc = "OTG_FS_DIEPCTL0 register accessor: an alias for `Reg<OTG_FS_DIEPCTL0_SPEC>`"]
pub type OTG_FS_DIEPCTL0 = crate::Reg<otg_fs_diepctl0::OTG_FS_DIEPCTL0_SPEC>;
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub mod otg_fs_diepctl0;
#[doc = "OTG_FS_DIEPCTL1 register accessor: an alias for `Reg<OTG_FS_DIEPCTL1_SPEC>`"]
pub type OTG_FS_DIEPCTL1 = crate::Reg<otg_fs_diepctl1::OTG_FS_DIEPCTL1_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod otg_fs_diepctl1;
#[doc = "OTG_FS_DIEPCTL2 register accessor: an alias for `Reg<OTG_FS_DIEPCTL2_SPEC>`"]
pub type OTG_FS_DIEPCTL2 = crate::Reg<otg_fs_diepctl2::OTG_FS_DIEPCTL2_SPEC>;
#[doc = "OTG device endpoint-2 control register"]
pub mod otg_fs_diepctl2;
#[doc = "OTG_FS_DIEPCTL3 register accessor: an alias for `Reg<OTG_FS_DIEPCTL3_SPEC>`"]
pub type OTG_FS_DIEPCTL3 = crate::Reg<otg_fs_diepctl3::OTG_FS_DIEPCTL3_SPEC>;
#[doc = "OTG device endpoint-3 control register"]
pub mod otg_fs_diepctl3;
#[doc = "OTG_FS_DOEPCTL0 register accessor: an alias for `Reg<OTG_FS_DOEPCTL0_SPEC>`"]
pub type OTG_FS_DOEPCTL0 = crate::Reg<otg_fs_doepctl0::OTG_FS_DOEPCTL0_SPEC>;
#[doc = "device endpoint-0 control register"]
pub mod otg_fs_doepctl0;
#[doc = "OTG_FS_DOEPCTL1 register accessor: an alias for `Reg<OTG_FS_DOEPCTL1_SPEC>`"]
pub type OTG_FS_DOEPCTL1 = crate::Reg<otg_fs_doepctl1::OTG_FS_DOEPCTL1_SPEC>;
#[doc = "device endpoint-1 control register"]
pub mod otg_fs_doepctl1;
#[doc = "OTG_FS_DOEPCTL2 register accessor: an alias for `Reg<OTG_FS_DOEPCTL2_SPEC>`"]
pub type OTG_FS_DOEPCTL2 = crate::Reg<otg_fs_doepctl2::OTG_FS_DOEPCTL2_SPEC>;
#[doc = "device endpoint-2 control register"]
pub mod otg_fs_doepctl2;
#[doc = "OTG_FS_DOEPCTL3 register accessor: an alias for `Reg<OTG_FS_DOEPCTL3_SPEC>`"]
pub type OTG_FS_DOEPCTL3 = crate::Reg<otg_fs_doepctl3::OTG_FS_DOEPCTL3_SPEC>;
#[doc = "device endpoint-3 control register"]
pub mod otg_fs_doepctl3;
#[doc = "OTG_FS_DIEPINT0 register accessor: an alias for `Reg<OTG_FS_DIEPINT0_SPEC>`"]
pub type OTG_FS_DIEPINT0 = crate::Reg<otg_fs_diepint0::OTG_FS_DIEPINT0_SPEC>;
#[doc = "device endpoint-x interrupt register"]
pub mod otg_fs_diepint0;
#[doc = "OTG_FS_DIEPINT1 register accessor: an alias for `Reg<OTG_FS_DIEPINT1_SPEC>`"]
pub type OTG_FS_DIEPINT1 = crate::Reg<otg_fs_diepint1::OTG_FS_DIEPINT1_SPEC>;
#[doc = "device endpoint-1 interrupt register"]
pub mod otg_fs_diepint1;
#[doc = "OTG_FS_DIEPINT2 register accessor: an alias for `Reg<OTG_FS_DIEPINT2_SPEC>`"]
pub type OTG_FS_DIEPINT2 = crate::Reg<otg_fs_diepint2::OTG_FS_DIEPINT2_SPEC>;
#[doc = "device endpoint-2 interrupt register"]
pub mod otg_fs_diepint2;
#[doc = "OTG_FS_DIEPINT3 register accessor: an alias for `Reg<OTG_FS_DIEPINT3_SPEC>`"]
pub type OTG_FS_DIEPINT3 = crate::Reg<otg_fs_diepint3::OTG_FS_DIEPINT3_SPEC>;
#[doc = "device endpoint-3 interrupt register"]
pub mod otg_fs_diepint3;
#[doc = "OTG_FS_DOEPINT0 register accessor: an alias for `Reg<OTG_FS_DOEPINT0_SPEC>`"]
pub type OTG_FS_DOEPINT0 = crate::Reg<otg_fs_doepint0::OTG_FS_DOEPINT0_SPEC>;
#[doc = "device endpoint-0 interrupt register"]
pub mod otg_fs_doepint0;
#[doc = "OTG_FS_DOEPINT1 register accessor: an alias for `Reg<OTG_FS_DOEPINT1_SPEC>`"]
pub type OTG_FS_DOEPINT1 = crate::Reg<otg_fs_doepint1::OTG_FS_DOEPINT1_SPEC>;
#[doc = "device endpoint-1 interrupt register"]
pub mod otg_fs_doepint1;
#[doc = "OTG_FS_DOEPINT2 register accessor: an alias for `Reg<OTG_FS_DOEPINT2_SPEC>`"]
pub type OTG_FS_DOEPINT2 = crate::Reg<otg_fs_doepint2::OTG_FS_DOEPINT2_SPEC>;
#[doc = "device endpoint-2 interrupt register"]
pub mod otg_fs_doepint2;
#[doc = "OTG_FS_DOEPINT3 register accessor: an alias for `Reg<OTG_FS_DOEPINT3_SPEC>`"]
pub type OTG_FS_DOEPINT3 = crate::Reg<otg_fs_doepint3::OTG_FS_DOEPINT3_SPEC>;
#[doc = "device endpoint-3 interrupt register"]
pub mod otg_fs_doepint3;
#[doc = "OTG_FS_DIEPTSIZ0 register accessor: an alias for `Reg<OTG_FS_DIEPTSIZ0_SPEC>`"]
pub type OTG_FS_DIEPTSIZ0 = crate::Reg<otg_fs_dieptsiz0::OTG_FS_DIEPTSIZ0_SPEC>;
#[doc = "device endpoint-0 transfer size register"]
pub mod otg_fs_dieptsiz0;
#[doc = "OTG_FS_DOEPTSIZ0 register accessor: an alias for `Reg<OTG_FS_DOEPTSIZ0_SPEC>`"]
pub type OTG_FS_DOEPTSIZ0 = crate::Reg<otg_fs_doeptsiz0::OTG_FS_DOEPTSIZ0_SPEC>;
#[doc = "device OUT endpoint-0 transfer size register"]
pub mod otg_fs_doeptsiz0;
#[doc = "OTG_FS_DIEPTSIZ1 register accessor: an alias for `Reg<OTG_FS_DIEPTSIZ1_SPEC>`"]
pub type OTG_FS_DIEPTSIZ1 = crate::Reg<otg_fs_dieptsiz1::OTG_FS_DIEPTSIZ1_SPEC>;
#[doc = "device endpoint-1 transfer size register"]
pub mod otg_fs_dieptsiz1;
#[doc = "OTG_FS_DIEPTSIZ2 register accessor: an alias for `Reg<OTG_FS_DIEPTSIZ2_SPEC>`"]
pub type OTG_FS_DIEPTSIZ2 = crate::Reg<otg_fs_dieptsiz2::OTG_FS_DIEPTSIZ2_SPEC>;
#[doc = "device endpoint-2 transfer size register"]
pub mod otg_fs_dieptsiz2;
#[doc = "OTG_FS_DIEPTSIZ3 register accessor: an alias for `Reg<OTG_FS_DIEPTSIZ3_SPEC>`"]
pub type OTG_FS_DIEPTSIZ3 = crate::Reg<otg_fs_dieptsiz3::OTG_FS_DIEPTSIZ3_SPEC>;
#[doc = "device endpoint-3 transfer size register"]
pub mod otg_fs_dieptsiz3;
#[doc = "OTG_FS_DTXFSTS0 register accessor: an alias for `Reg<OTG_FS_DTXFSTS0_SPEC>`"]
pub type OTG_FS_DTXFSTS0 = crate::Reg<otg_fs_dtxfsts0::OTG_FS_DTXFSTS0_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts0;
#[doc = "OTG_FS_DTXFSTS1 register accessor: an alias for `Reg<OTG_FS_DTXFSTS1_SPEC>`"]
pub type OTG_FS_DTXFSTS1 = crate::Reg<otg_fs_dtxfsts1::OTG_FS_DTXFSTS1_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts1;
#[doc = "OTG_FS_DTXFSTS2 register accessor: an alias for `Reg<OTG_FS_DTXFSTS2_SPEC>`"]
pub type OTG_FS_DTXFSTS2 = crate::Reg<otg_fs_dtxfsts2::OTG_FS_DTXFSTS2_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts2;
#[doc = "OTG_FS_DTXFSTS3 register accessor: an alias for `Reg<OTG_FS_DTXFSTS3_SPEC>`"]
pub type OTG_FS_DTXFSTS3 = crate::Reg<otg_fs_dtxfsts3::OTG_FS_DTXFSTS3_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts3;
#[doc = "OTG_FS_DOEPTSIZ1 register accessor: an alias for `Reg<OTG_FS_DOEPTSIZ1_SPEC>`"]
pub type OTG_FS_DOEPTSIZ1 = crate::Reg<otg_fs_doeptsiz1::OTG_FS_DOEPTSIZ1_SPEC>;
#[doc = "device OUT endpoint-1 transfer size register"]
pub mod otg_fs_doeptsiz1;
#[doc = "OTG_FS_DOEPTSIZ2 register accessor: an alias for `Reg<OTG_FS_DOEPTSIZ2_SPEC>`"]
pub type OTG_FS_DOEPTSIZ2 = crate::Reg<otg_fs_doeptsiz2::OTG_FS_DOEPTSIZ2_SPEC>;
#[doc = "device OUT endpoint-2 transfer size register"]
pub mod otg_fs_doeptsiz2;
#[doc = "OTG_FS_DOEPTSIZ3 register accessor: an alias for `Reg<OTG_FS_DOEPTSIZ3_SPEC>`"]
pub type OTG_FS_DOEPTSIZ3 = crate::Reg<otg_fs_doeptsiz3::OTG_FS_DOEPTSIZ3_SPEC>;
#[doc = "device OUT endpoint-3 transfer size register"]
pub mod otg_fs_doeptsiz3;
#[doc = "OTG_FS_DIEPCTL4 register accessor: an alias for `Reg<OTG_FS_DIEPCTL4_SPEC>`"]
pub type OTG_FS_DIEPCTL4 = crate::Reg<otg_fs_diepctl4::OTG_FS_DIEPCTL4_SPEC>;
#[doc = "OTG device endpoint-4 control register"]
pub mod otg_fs_diepctl4;
#[doc = "OTG_FS_DIEPINT4 register accessor: an alias for `Reg<OTG_FS_DIEPINT4_SPEC>`"]
pub type OTG_FS_DIEPINT4 = crate::Reg<otg_fs_diepint4::OTG_FS_DIEPINT4_SPEC>;
#[doc = "device endpoint-4 interrupt register"]
pub mod otg_fs_diepint4;
#[doc = "OTG_FS_DIEPTSIZ4 register accessor: an alias for `Reg<OTG_FS_DIEPTSIZ4_SPEC>`"]
pub type OTG_FS_DIEPTSIZ4 = crate::Reg<otg_fs_dieptsiz4::OTG_FS_DIEPTSIZ4_SPEC>;
#[doc = "device endpoint-4 transfer size register"]
pub mod otg_fs_dieptsiz4;
#[doc = "OTG_FS_DTXFSTS4 register accessor: an alias for `Reg<OTG_FS_DTXFSTS4_SPEC>`"]
pub type OTG_FS_DTXFSTS4 = crate::Reg<otg_fs_dtxfsts4::OTG_FS_DTXFSTS4_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts4;
#[doc = "OTG_FS_DIEPCTL5 register accessor: an alias for `Reg<OTG_FS_DIEPCTL5_SPEC>`"]
pub type OTG_FS_DIEPCTL5 = crate::Reg<otg_fs_diepctl5::OTG_FS_DIEPCTL5_SPEC>;
#[doc = "OTG device endpoint-5 control register"]
pub mod otg_fs_diepctl5;
#[doc = "OTG_FS_DIEPINT5 register accessor: an alias for `Reg<OTG_FS_DIEPINT5_SPEC>`"]
pub type OTG_FS_DIEPINT5 = crate::Reg<otg_fs_diepint5::OTG_FS_DIEPINT5_SPEC>;
#[doc = "device endpoint-5 interrupt register"]
pub mod otg_fs_diepint5;
#[doc = "OTG_FS_DIEPTSIZ55 register accessor: an alias for `Reg<OTG_FS_DIEPTSIZ55_SPEC>`"]
pub type OTG_FS_DIEPTSIZ55 = crate::Reg<otg_fs_dieptsiz55::OTG_FS_DIEPTSIZ55_SPEC>;
#[doc = "device endpoint-5 transfer size register"]
pub mod otg_fs_dieptsiz55;
#[doc = "OTG_FS_DTXFSTS55 register accessor: an alias for `Reg<OTG_FS_DTXFSTS55_SPEC>`"]
pub type OTG_FS_DTXFSTS55 = crate::Reg<otg_fs_dtxfsts55::OTG_FS_DTXFSTS55_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts55;
#[doc = "OTG_FS_DOEPCTL4 register accessor: an alias for `Reg<OTG_FS_DOEPCTL4_SPEC>`"]
pub type OTG_FS_DOEPCTL4 = crate::Reg<otg_fs_doepctl4::OTG_FS_DOEPCTL4_SPEC>;
#[doc = "device endpoint-4 control register"]
pub mod otg_fs_doepctl4;
#[doc = "OTG_FS_DOEPINT4 register accessor: an alias for `Reg<OTG_FS_DOEPINT4_SPEC>`"]
pub type OTG_FS_DOEPINT4 = crate::Reg<otg_fs_doepint4::OTG_FS_DOEPINT4_SPEC>;
#[doc = "device endpoint-4 interrupt register"]
pub mod otg_fs_doepint4;
#[doc = "OTG_FS_DOEPTSIZ4 register accessor: an alias for `Reg<OTG_FS_DOEPTSIZ4_SPEC>`"]
pub type OTG_FS_DOEPTSIZ4 = crate::Reg<otg_fs_doeptsiz4::OTG_FS_DOEPTSIZ4_SPEC>;
#[doc = "device OUT endpoint-4 transfer size register"]
pub mod otg_fs_doeptsiz4;
#[doc = "OTG_FS_DOEPCTL5 register accessor: an alias for `Reg<OTG_FS_DOEPCTL5_SPEC>`"]
pub type OTG_FS_DOEPCTL5 = crate::Reg<otg_fs_doepctl5::OTG_FS_DOEPCTL5_SPEC>;
#[doc = "device endpoint-5 control register"]
pub mod otg_fs_doepctl5;
#[doc = "OTG_FS_DOEPINT5 register accessor: an alias for `Reg<OTG_FS_DOEPINT5_SPEC>`"]
pub type OTG_FS_DOEPINT5 = crate::Reg<otg_fs_doepint5::OTG_FS_DOEPINT5_SPEC>;
#[doc = "device endpoint-5 interrupt register"]
pub mod otg_fs_doepint5;
#[doc = "OTG_FS_DOEPTSIZ5 register accessor: an alias for `Reg<OTG_FS_DOEPTSIZ5_SPEC>`"]
pub type OTG_FS_DOEPTSIZ5 = crate::Reg<otg_fs_doeptsiz5::OTG_FS_DOEPTSIZ5_SPEC>;
#[doc = "device OUT endpoint-5 transfer size register"]
pub mod otg_fs_doeptsiz5;
