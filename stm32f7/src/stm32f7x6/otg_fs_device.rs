#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS device configuration register (OTG_FS_DCFG)"]
    pub otg_fs_dcfg: OTG_FS_DCFG,
    #[doc = "0x04 - OTG_FS device control register (OTG_FS_DCTL)"]
    pub otg_fs_dctl: OTG_FS_DCTL,
    #[doc = "0x08 - OTG_FS device status register (OTG_FS_DSTS)"]
    pub otg_fs_dsts: OTG_FS_DSTS,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
    pub otg_fs_diepmsk: OTG_FS_DIEPMSK,
    #[doc = "0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
    pub otg_fs_doepmsk: OTG_FS_DOEPMSK,
    #[doc = "0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
    pub otg_fs_daint: OTG_FS_DAINT,
    #[doc = "0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
    pub otg_fs_daintmsk: OTG_FS_DAINTMSK,
    _reserved7: [u8; 8usize],
    #[doc = "0x28 - OTG_FS device VBUS discharge time register"]
    pub otg_fs_dvbusdis: OTG_FS_DVBUSDIS,
    #[doc = "0x2c - OTG_FS device VBUS pulsing time register"]
    pub otg_fs_dvbuspulse: OTG_FS_DVBUSPULSE,
    _reserved9: [u8; 4usize],
    #[doc = "0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register"]
    pub otg_fs_diepempmsk: OTG_FS_DIEPEMPMSK,
    _reserved10: [u8; 200usize],
    #[doc = "0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
    pub otg_fs_diepctl0: OTG_FS_DIEPCTL0,
    _reserved11: [u8; 4usize],
    #[doc = "0x108 - device endpoint-x interrupt register"]
    pub otg_fs_diepint0: OTG_FS_DIEPINT0,
    _reserved12: [u8; 4usize],
    #[doc = "0x110 - device endpoint-0 transfer size register"]
    pub otg_fs_dieptsiz0: OTG_FS_DIEPTSIZ0,
    _reserved13: [u8; 4usize],
    #[doc = "0x118 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts0: OTG_FS_DTXFSTS0,
    _reserved14: [u8; 4usize],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub otg_fs_diepctl1: OTG_FS_DIEPCTL1,
    _reserved15: [u8; 4usize],
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    pub otg_fs_diepint1: OTG_FS_DIEPINT1,
    _reserved16: [u8; 4usize],
    #[doc = "0x130 - device endpoint-1 transfer size register"]
    pub otg_fs_dieptsiz1: OTG_FS_DIEPTSIZ1,
    _reserved17: [u8; 4usize],
    #[doc = "0x138 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts1: OTG_FS_DTXFSTS1,
    _reserved18: [u8; 4usize],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub otg_fs_diepctl2: OTG_FS_DIEPCTL2,
    _reserved19: [u8; 4usize],
    #[doc = "0x148 - device endpoint-2 interrupt register"]
    pub otg_fs_diepint2: OTG_FS_DIEPINT2,
    _reserved20: [u8; 4usize],
    #[doc = "0x150 - device endpoint-2 transfer size register"]
    pub otg_fs_dieptsiz2: OTG_FS_DIEPTSIZ2,
    _reserved21: [u8; 4usize],
    #[doc = "0x158 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts2: OTG_FS_DTXFSTS2,
    _reserved22: [u8; 4usize],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub otg_fs_diepctl3: OTG_FS_DIEPCTL3,
    _reserved23: [u8; 4usize],
    #[doc = "0x168 - device endpoint-3 interrupt register"]
    pub otg_fs_diepint3: OTG_FS_DIEPINT3,
    _reserved24: [u8; 4usize],
    #[doc = "0x170 - device endpoint-3 transfer size register"]
    pub otg_fs_dieptsiz3: OTG_FS_DIEPTSIZ3,
    _reserved25: [u8; 4usize],
    #[doc = "0x178 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts3: OTG_FS_DTXFSTS3,
    _reserved26: [u8; 4usize],
    #[doc = "0x180 - OTG device endpoint-4 control register"]
    pub otg_fs_diepctl4: OTG_FS_DIEPCTL4,
    _reserved27: [u8; 4usize],
    #[doc = "0x188 - device endpoint-4 interrupt register"]
    pub otg_fs_diepint4: OTG_FS_DIEPINT4,
    _reserved28: [u8; 8usize],
    #[doc = "0x194 - device endpoint-4 transfer size register"]
    pub otg_fs_dieptsiz4: OTG_FS_DIEPTSIZ4,
    _reserved29: [u8; 4usize],
    #[doc = "0x19c - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts4: OTG_FS_DTXFSTS4,
    #[doc = "0x1a0 - OTG device endpoint-5 control register"]
    pub otg_fs_diepctl5: OTG_FS_DIEPCTL5,
    _reserved31: [u8; 4usize],
    #[doc = "0x1a8 - device endpoint-5 interrupt register"]
    pub otg_fs_diepint5: OTG_FS_DIEPINT5,
    _reserved32: [u8; 4usize],
    #[doc = "0x1b0 - device endpoint-5 transfer size register"]
    pub otg_fs_dieptsiz55: OTG_FS_DIEPTSIZ55,
    _reserved33: [u8; 4usize],
    #[doc = "0x1b8 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts55: OTG_FS_DTXFSTS55,
    _reserved34: [u8; 324usize],
    #[doc = "0x300 - device endpoint-0 control register"]
    pub otg_fs_doepctl0: OTG_FS_DOEPCTL0,
    _reserved35: [u8; 4usize],
    #[doc = "0x308 - device endpoint-0 interrupt register"]
    pub otg_fs_doepint0: OTG_FS_DOEPINT0,
    _reserved36: [u8; 4usize],
    #[doc = "0x310 - device OUT endpoint-0 transfer size register"]
    pub otg_fs_doeptsiz0: OTG_FS_DOEPTSIZ0,
    _reserved37: [u8; 12usize],
    #[doc = "0x320 - device endpoint-1 control register"]
    pub otg_fs_doepctl1: OTG_FS_DOEPCTL1,
    _reserved38: [u8; 4usize],
    #[doc = "0x328 - device endpoint-1 interrupt register"]
    pub otg_fs_doepint1: OTG_FS_DOEPINT1,
    _reserved39: [u8; 4usize],
    #[doc = "0x330 - device OUT endpoint-1 transfer size register"]
    pub otg_fs_doeptsiz1: OTG_FS_DOEPTSIZ1,
    _reserved40: [u8; 12usize],
    #[doc = "0x340 - device endpoint-2 control register"]
    pub otg_fs_doepctl2: OTG_FS_DOEPCTL2,
    _reserved41: [u8; 4usize],
    #[doc = "0x348 - device endpoint-2 interrupt register"]
    pub otg_fs_doepint2: OTG_FS_DOEPINT2,
    _reserved42: [u8; 4usize],
    #[doc = "0x350 - device OUT endpoint-2 transfer size register"]
    pub otg_fs_doeptsiz2: OTG_FS_DOEPTSIZ2,
    _reserved43: [u8; 12usize],
    #[doc = "0x360 - device endpoint-3 control register"]
    pub otg_fs_doepctl3: OTG_FS_DOEPCTL3,
    _reserved44: [u8; 4usize],
    #[doc = "0x368 - device endpoint-3 interrupt register"]
    pub otg_fs_doepint3: OTG_FS_DOEPINT3,
    _reserved45: [u8; 4usize],
    #[doc = "0x370 - device OUT endpoint-3 transfer size register"]
    pub otg_fs_doeptsiz3: OTG_FS_DOEPTSIZ3,
    _reserved46: [u8; 4usize],
    #[doc = "0x378 - device endpoint-4 control register"]
    pub otg_fs_doepctl4: OTG_FS_DOEPCTL4,
    _reserved47: [u8; 4usize],
    #[doc = "0x380 - device endpoint-4 interrupt register"]
    pub otg_fs_doepint4: OTG_FS_DOEPINT4,
    _reserved48: [u8; 4usize],
    #[doc = "0x388 - device OUT endpoint-4 transfer size register"]
    pub otg_fs_doeptsiz4: OTG_FS_DOEPTSIZ4,
    _reserved49: [u8; 4usize],
    #[doc = "0x390 - device endpoint-5 control register"]
    pub otg_fs_doepctl5: OTG_FS_DOEPCTL5,
    _reserved50: [u8; 4usize],
    #[doc = "0x398 - device endpoint-5 interrupt register"]
    pub otg_fs_doepint5: OTG_FS_DOEPINT5,
    _reserved51: [u8; 4usize],
    #[doc = "0x3a0 - device OUT endpoint-5 transfer size register"]
    pub otg_fs_doeptsiz5: OTG_FS_DOEPTSIZ5,
}
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dcfg](otg_fs_dcfg) module"]
pub type OTG_FS_DCFG = crate::Reg<u32, _OTG_FS_DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DCFG;
#[doc = "`read()` method returns [otg_fs_dcfg::R](otg_fs_dcfg::R) reader structure"]
impl crate::Readable for OTG_FS_DCFG {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dcfg::W](otg_fs_dcfg::W) writer structure"]
impl crate::Writable for OTG_FS_DCFG {}
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub mod otg_fs_dcfg;
#[doc = "OTG_FS device control register (OTG_FS_DCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dctl](otg_fs_dctl) module"]
pub type OTG_FS_DCTL = crate::Reg<u32, _OTG_FS_DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DCTL;
#[doc = "`read()` method returns [otg_fs_dctl::R](otg_fs_dctl::R) reader structure"]
impl crate::Readable for OTG_FS_DCTL {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dctl::W](otg_fs_dctl::W) writer structure"]
impl crate::Writable for OTG_FS_DCTL {}
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub mod otg_fs_dctl;
#[doc = "OTG_FS device status register (OTG_FS_DSTS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dsts](otg_fs_dsts) module"]
pub type OTG_FS_DSTS = crate::Reg<u32, _OTG_FS_DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DSTS;
#[doc = "`read()` method returns [otg_fs_dsts::R](otg_fs_dsts::R) reader structure"]
impl crate::Readable for OTG_FS_DSTS {}
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub mod otg_fs_dsts;
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepmsk](otg_fs_diepmsk) module"]
pub type OTG_FS_DIEPMSK = crate::Reg<u32, _OTG_FS_DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPMSK;
#[doc = "`read()` method returns [otg_fs_diepmsk::R](otg_fs_diepmsk::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPMSK {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepmsk::W](otg_fs_diepmsk::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPMSK {}
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub mod otg_fs_diepmsk;
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepmsk](otg_fs_doepmsk) module"]
pub type OTG_FS_DOEPMSK = crate::Reg<u32, _OTG_FS_DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPMSK;
#[doc = "`read()` method returns [otg_fs_doepmsk::R](otg_fs_doepmsk::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPMSK {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepmsk::W](otg_fs_doepmsk::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPMSK {}
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub mod otg_fs_doepmsk;
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_daint](otg_fs_daint) module"]
pub type OTG_FS_DAINT = crate::Reg<u32, _OTG_FS_DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DAINT;
#[doc = "`read()` method returns [otg_fs_daint::R](otg_fs_daint::R) reader structure"]
impl crate::Readable for OTG_FS_DAINT {}
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub mod otg_fs_daint;
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_daintmsk](otg_fs_daintmsk) module"]
pub type OTG_FS_DAINTMSK = crate::Reg<u32, _OTG_FS_DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DAINTMSK;
#[doc = "`read()` method returns [otg_fs_daintmsk::R](otg_fs_daintmsk::R) reader structure"]
impl crate::Readable for OTG_FS_DAINTMSK {}
#[doc = "`write(|w| ..)` method takes [otg_fs_daintmsk::W](otg_fs_daintmsk::W) writer structure"]
impl crate::Writable for OTG_FS_DAINTMSK {}
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub mod otg_fs_daintmsk;
#[doc = "OTG_FS device VBUS discharge time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dvbusdis](otg_fs_dvbusdis) module"]
pub type OTG_FS_DVBUSDIS = crate::Reg<u32, _OTG_FS_DVBUSDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DVBUSDIS;
#[doc = "`read()` method returns [otg_fs_dvbusdis::R](otg_fs_dvbusdis::R) reader structure"]
impl crate::Readable for OTG_FS_DVBUSDIS {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dvbusdis::W](otg_fs_dvbusdis::W) writer structure"]
impl crate::Writable for OTG_FS_DVBUSDIS {}
#[doc = "OTG_FS device VBUS discharge time register"]
pub mod otg_fs_dvbusdis;
#[doc = "OTG_FS device VBUS pulsing time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dvbuspulse](otg_fs_dvbuspulse) module"]
pub type OTG_FS_DVBUSPULSE = crate::Reg<u32, _OTG_FS_DVBUSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DVBUSPULSE;
#[doc = "`read()` method returns [otg_fs_dvbuspulse::R](otg_fs_dvbuspulse::R) reader structure"]
impl crate::Readable for OTG_FS_DVBUSPULSE {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dvbuspulse::W](otg_fs_dvbuspulse::W) writer structure"]
impl crate::Writable for OTG_FS_DVBUSPULSE {}
#[doc = "OTG_FS device VBUS pulsing time register"]
pub mod otg_fs_dvbuspulse;
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepempmsk](otg_fs_diepempmsk) module"]
pub type OTG_FS_DIEPEMPMSK = crate::Reg<u32, _OTG_FS_DIEPEMPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPEMPMSK;
#[doc = "`read()` method returns [otg_fs_diepempmsk::R](otg_fs_diepempmsk::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPEMPMSK {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepempmsk::W](otg_fs_diepempmsk::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPEMPMSK {}
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub mod otg_fs_diepempmsk;
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepctl0](otg_fs_diepctl0) module"]
pub type OTG_FS_DIEPCTL0 = crate::Reg<u32, _OTG_FS_DIEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPCTL0;
#[doc = "`read()` method returns [otg_fs_diepctl0::R](otg_fs_diepctl0::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepctl0::W](otg_fs_diepctl0::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPCTL0 {}
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub mod otg_fs_diepctl0;
#[doc = "OTG device endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepctl1](otg_fs_diepctl1) module"]
pub type OTG_FS_DIEPCTL1 = crate::Reg<u32, _OTG_FS_DIEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPCTL1;
#[doc = "`read()` method returns [otg_fs_diepctl1::R](otg_fs_diepctl1::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPCTL1 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepctl1::W](otg_fs_diepctl1::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPCTL1 {}
#[doc = "OTG device endpoint-1 control register"]
pub mod otg_fs_diepctl1;
#[doc = "OTG device endpoint-2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepctl2](otg_fs_diepctl2) module"]
pub type OTG_FS_DIEPCTL2 = crate::Reg<u32, _OTG_FS_DIEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPCTL2;
#[doc = "`read()` method returns [otg_fs_diepctl2::R](otg_fs_diepctl2::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPCTL2 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepctl2::W](otg_fs_diepctl2::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPCTL2 {}
#[doc = "OTG device endpoint-2 control register"]
pub mod otg_fs_diepctl2;
#[doc = "OTG device endpoint-3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepctl3](otg_fs_diepctl3) module"]
pub type OTG_FS_DIEPCTL3 = crate::Reg<u32, _OTG_FS_DIEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPCTL3;
#[doc = "`read()` method returns [otg_fs_diepctl3::R](otg_fs_diepctl3::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPCTL3 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepctl3::W](otg_fs_diepctl3::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPCTL3 {}
#[doc = "OTG device endpoint-3 control register"]
pub mod otg_fs_diepctl3;
#[doc = "device endpoint-0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepctl0](otg_fs_doepctl0) module"]
pub type OTG_FS_DOEPCTL0 = crate::Reg<u32, _OTG_FS_DOEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPCTL0;
#[doc = "`read()` method returns [otg_fs_doepctl0::R](otg_fs_doepctl0::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepctl0::W](otg_fs_doepctl0::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPCTL0 {}
#[doc = "device endpoint-0 control register"]
pub mod otg_fs_doepctl0;
#[doc = "device endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepctl1](otg_fs_doepctl1) module"]
pub type OTG_FS_DOEPCTL1 = crate::Reg<u32, _OTG_FS_DOEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPCTL1;
#[doc = "`read()` method returns [otg_fs_doepctl1::R](otg_fs_doepctl1::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPCTL1 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepctl1::W](otg_fs_doepctl1::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPCTL1 {}
#[doc = "device endpoint-1 control register"]
pub mod otg_fs_doepctl1;
#[doc = "device endpoint-2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepctl2](otg_fs_doepctl2) module"]
pub type OTG_FS_DOEPCTL2 = crate::Reg<u32, _OTG_FS_DOEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPCTL2;
#[doc = "`read()` method returns [otg_fs_doepctl2::R](otg_fs_doepctl2::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPCTL2 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepctl2::W](otg_fs_doepctl2::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPCTL2 {}
#[doc = "device endpoint-2 control register"]
pub mod otg_fs_doepctl2;
#[doc = "device endpoint-3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepctl3](otg_fs_doepctl3) module"]
pub type OTG_FS_DOEPCTL3 = crate::Reg<u32, _OTG_FS_DOEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPCTL3;
#[doc = "`read()` method returns [otg_fs_doepctl3::R](otg_fs_doepctl3::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPCTL3 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepctl3::W](otg_fs_doepctl3::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPCTL3 {}
#[doc = "device endpoint-3 control register"]
pub mod otg_fs_doepctl3;
#[doc = "device endpoint-x interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepint0](otg_fs_diepint0) module"]
pub type OTG_FS_DIEPINT0 = crate::Reg<u32, _OTG_FS_DIEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPINT0;
#[doc = "`read()` method returns [otg_fs_diepint0::R](otg_fs_diepint0::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPINT0 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepint0::W](otg_fs_diepint0::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPINT0 {}
#[doc = "device endpoint-x interrupt register"]
pub mod otg_fs_diepint0;
#[doc = "device endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepint1](otg_fs_diepint1) module"]
pub type OTG_FS_DIEPINT1 = crate::Reg<u32, _OTG_FS_DIEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPINT1;
#[doc = "`read()` method returns [otg_fs_diepint1::R](otg_fs_diepint1::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPINT1 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepint1::W](otg_fs_diepint1::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPINT1 {}
#[doc = "device endpoint-1 interrupt register"]
pub mod otg_fs_diepint1;
#[doc = "device endpoint-2 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepint2](otg_fs_diepint2) module"]
pub type OTG_FS_DIEPINT2 = crate::Reg<u32, _OTG_FS_DIEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPINT2;
#[doc = "`read()` method returns [otg_fs_diepint2::R](otg_fs_diepint2::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPINT2 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepint2::W](otg_fs_diepint2::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPINT2 {}
#[doc = "device endpoint-2 interrupt register"]
pub mod otg_fs_diepint2;
#[doc = "device endpoint-3 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepint3](otg_fs_diepint3) module"]
pub type OTG_FS_DIEPINT3 = crate::Reg<u32, _OTG_FS_DIEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPINT3;
#[doc = "`read()` method returns [otg_fs_diepint3::R](otg_fs_diepint3::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPINT3 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepint3::W](otg_fs_diepint3::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPINT3 {}
#[doc = "device endpoint-3 interrupt register"]
pub mod otg_fs_diepint3;
#[doc = "device endpoint-0 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepint0](otg_fs_doepint0) module"]
pub type OTG_FS_DOEPINT0 = crate::Reg<u32, _OTG_FS_DOEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPINT0;
#[doc = "`read()` method returns [otg_fs_doepint0::R](otg_fs_doepint0::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPINT0 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepint0::W](otg_fs_doepint0::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPINT0 {}
#[doc = "device endpoint-0 interrupt register"]
pub mod otg_fs_doepint0;
#[doc = "device endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepint1](otg_fs_doepint1) module"]
pub type OTG_FS_DOEPINT1 = crate::Reg<u32, _OTG_FS_DOEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPINT1;
#[doc = "`read()` method returns [otg_fs_doepint1::R](otg_fs_doepint1::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPINT1 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepint1::W](otg_fs_doepint1::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPINT1 {}
#[doc = "device endpoint-1 interrupt register"]
pub mod otg_fs_doepint1;
#[doc = "device endpoint-2 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepint2](otg_fs_doepint2) module"]
pub type OTG_FS_DOEPINT2 = crate::Reg<u32, _OTG_FS_DOEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPINT2;
#[doc = "`read()` method returns [otg_fs_doepint2::R](otg_fs_doepint2::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPINT2 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepint2::W](otg_fs_doepint2::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPINT2 {}
#[doc = "device endpoint-2 interrupt register"]
pub mod otg_fs_doepint2;
#[doc = "device endpoint-3 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepint3](otg_fs_doepint3) module"]
pub type OTG_FS_DOEPINT3 = crate::Reg<u32, _OTG_FS_DOEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPINT3;
#[doc = "`read()` method returns [otg_fs_doepint3::R](otg_fs_doepint3::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPINT3 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepint3::W](otg_fs_doepint3::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPINT3 {}
#[doc = "device endpoint-3 interrupt register"]
pub mod otg_fs_doepint3;
#[doc = "device endpoint-0 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptsiz0](otg_fs_dieptsiz0) module"]
pub type OTG_FS_DIEPTSIZ0 = crate::Reg<u32, _OTG_FS_DIEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTSIZ0;
#[doc = "`read()` method returns [otg_fs_dieptsiz0::R](otg_fs_dieptsiz0::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptsiz0::W](otg_fs_dieptsiz0::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTSIZ0 {}
#[doc = "device endpoint-0 transfer size register"]
pub mod otg_fs_dieptsiz0;
#[doc = "device OUT endpoint-0 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doeptsiz0](otg_fs_doeptsiz0) module"]
pub type OTG_FS_DOEPTSIZ0 = crate::Reg<u32, _OTG_FS_DOEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPTSIZ0;
#[doc = "`read()` method returns [otg_fs_doeptsiz0::R](otg_fs_doeptsiz0::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doeptsiz0::W](otg_fs_doeptsiz0::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPTSIZ0 {}
#[doc = "device OUT endpoint-0 transfer size register"]
pub mod otg_fs_doeptsiz0;
#[doc = "device endpoint-1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptsiz1](otg_fs_dieptsiz1) module"]
pub type OTG_FS_DIEPTSIZ1 = crate::Reg<u32, _OTG_FS_DIEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTSIZ1;
#[doc = "`read()` method returns [otg_fs_dieptsiz1::R](otg_fs_dieptsiz1::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptsiz1::W](otg_fs_dieptsiz1::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTSIZ1 {}
#[doc = "device endpoint-1 transfer size register"]
pub mod otg_fs_dieptsiz1;
#[doc = "device endpoint-2 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptsiz2](otg_fs_dieptsiz2) module"]
pub type OTG_FS_DIEPTSIZ2 = crate::Reg<u32, _OTG_FS_DIEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTSIZ2;
#[doc = "`read()` method returns [otg_fs_dieptsiz2::R](otg_fs_dieptsiz2::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptsiz2::W](otg_fs_dieptsiz2::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTSIZ2 {}
#[doc = "device endpoint-2 transfer size register"]
pub mod otg_fs_dieptsiz2;
#[doc = "device endpoint-3 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptsiz3](otg_fs_dieptsiz3) module"]
pub type OTG_FS_DIEPTSIZ3 = crate::Reg<u32, _OTG_FS_DIEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTSIZ3;
#[doc = "`read()` method returns [otg_fs_dieptsiz3::R](otg_fs_dieptsiz3::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptsiz3::W](otg_fs_dieptsiz3::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTSIZ3 {}
#[doc = "device endpoint-3 transfer size register"]
pub mod otg_fs_dieptsiz3;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dtxfsts0](otg_fs_dtxfsts0) module"]
pub type OTG_FS_DTXFSTS0 = crate::Reg<u32, _OTG_FS_DTXFSTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DTXFSTS0;
#[doc = "`read()` method returns [otg_fs_dtxfsts0::R](otg_fs_dtxfsts0::R) reader structure"]
impl crate::Readable for OTG_FS_DTXFSTS0 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts0;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dtxfsts1](otg_fs_dtxfsts1) module"]
pub type OTG_FS_DTXFSTS1 = crate::Reg<u32, _OTG_FS_DTXFSTS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DTXFSTS1;
#[doc = "`read()` method returns [otg_fs_dtxfsts1::R](otg_fs_dtxfsts1::R) reader structure"]
impl crate::Readable for OTG_FS_DTXFSTS1 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts1;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dtxfsts2](otg_fs_dtxfsts2) module"]
pub type OTG_FS_DTXFSTS2 = crate::Reg<u32, _OTG_FS_DTXFSTS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DTXFSTS2;
#[doc = "`read()` method returns [otg_fs_dtxfsts2::R](otg_fs_dtxfsts2::R) reader structure"]
impl crate::Readable for OTG_FS_DTXFSTS2 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts2;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dtxfsts3](otg_fs_dtxfsts3) module"]
pub type OTG_FS_DTXFSTS3 = crate::Reg<u32, _OTG_FS_DTXFSTS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DTXFSTS3;
#[doc = "`read()` method returns [otg_fs_dtxfsts3::R](otg_fs_dtxfsts3::R) reader structure"]
impl crate::Readable for OTG_FS_DTXFSTS3 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts3;
#[doc = "device OUT endpoint-1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doeptsiz1](otg_fs_doeptsiz1) module"]
pub type OTG_FS_DOEPTSIZ1 = crate::Reg<u32, _OTG_FS_DOEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPTSIZ1;
#[doc = "`read()` method returns [otg_fs_doeptsiz1::R](otg_fs_doeptsiz1::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doeptsiz1::W](otg_fs_doeptsiz1::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPTSIZ1 {}
#[doc = "device OUT endpoint-1 transfer size register"]
pub mod otg_fs_doeptsiz1;
#[doc = "device OUT endpoint-2 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doeptsiz2](otg_fs_doeptsiz2) module"]
pub type OTG_FS_DOEPTSIZ2 = crate::Reg<u32, _OTG_FS_DOEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPTSIZ2;
#[doc = "`read()` method returns [otg_fs_doeptsiz2::R](otg_fs_doeptsiz2::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doeptsiz2::W](otg_fs_doeptsiz2::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPTSIZ2 {}
#[doc = "device OUT endpoint-2 transfer size register"]
pub mod otg_fs_doeptsiz2;
#[doc = "device OUT endpoint-3 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doeptsiz3](otg_fs_doeptsiz3) module"]
pub type OTG_FS_DOEPTSIZ3 = crate::Reg<u32, _OTG_FS_DOEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPTSIZ3;
#[doc = "`read()` method returns [otg_fs_doeptsiz3::R](otg_fs_doeptsiz3::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doeptsiz3::W](otg_fs_doeptsiz3::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPTSIZ3 {}
#[doc = "device OUT endpoint-3 transfer size register"]
pub mod otg_fs_doeptsiz3;
#[doc = "OTG device endpoint-4 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepctl4](otg_fs_diepctl4) module"]
pub type OTG_FS_DIEPCTL4 = crate::Reg<u32, _OTG_FS_DIEPCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPCTL4;
#[doc = "`read()` method returns [otg_fs_diepctl4::R](otg_fs_diepctl4::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPCTL4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepctl4::W](otg_fs_diepctl4::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPCTL4 {}
#[doc = "OTG device endpoint-4 control register"]
pub mod otg_fs_diepctl4;
#[doc = "device endpoint-4 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepint4](otg_fs_diepint4) module"]
pub type OTG_FS_DIEPINT4 = crate::Reg<u32, _OTG_FS_DIEPINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPINT4;
#[doc = "`read()` method returns [otg_fs_diepint4::R](otg_fs_diepint4::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPINT4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepint4::W](otg_fs_diepint4::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPINT4 {}
#[doc = "device endpoint-4 interrupt register"]
pub mod otg_fs_diepint4;
#[doc = "device endpoint-4 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptsiz4](otg_fs_dieptsiz4) module"]
pub type OTG_FS_DIEPTSIZ4 = crate::Reg<u32, _OTG_FS_DIEPTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTSIZ4;
#[doc = "`read()` method returns [otg_fs_dieptsiz4::R](otg_fs_dieptsiz4::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptsiz4::W](otg_fs_dieptsiz4::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTSIZ4 {}
#[doc = "device endpoint-4 transfer size register"]
pub mod otg_fs_dieptsiz4;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dtxfsts4](otg_fs_dtxfsts4) module"]
pub type OTG_FS_DTXFSTS4 = crate::Reg<u32, _OTG_FS_DTXFSTS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DTXFSTS4;
#[doc = "`read()` method returns [otg_fs_dtxfsts4::R](otg_fs_dtxfsts4::R) reader structure"]
impl crate::Readable for OTG_FS_DTXFSTS4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dtxfsts4::W](otg_fs_dtxfsts4::W) writer structure"]
impl crate::Writable for OTG_FS_DTXFSTS4 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts4;
#[doc = "OTG device endpoint-5 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepctl5](otg_fs_diepctl5) module"]
pub type OTG_FS_DIEPCTL5 = crate::Reg<u32, _OTG_FS_DIEPCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPCTL5;
#[doc = "`read()` method returns [otg_fs_diepctl5::R](otg_fs_diepctl5::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPCTL5 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepctl5::W](otg_fs_diepctl5::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPCTL5 {}
#[doc = "OTG device endpoint-5 control register"]
pub mod otg_fs_diepctl5;
#[doc = "device endpoint-5 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepint5](otg_fs_diepint5) module"]
pub type OTG_FS_DIEPINT5 = crate::Reg<u32, _OTG_FS_DIEPINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPINT5;
#[doc = "`read()` method returns [otg_fs_diepint5::R](otg_fs_diepint5::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPINT5 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepint5::W](otg_fs_diepint5::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPINT5 {}
#[doc = "device endpoint-5 interrupt register"]
pub mod otg_fs_diepint5;
#[doc = "device endpoint-5 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dieptsiz55](otg_fs_dieptsiz55) module"]
pub type OTG_FS_DIEPTSIZ55 = crate::Reg<u32, _OTG_FS_DIEPTSIZ55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DIEPTSIZ55;
#[doc = "`read()` method returns [otg_fs_dieptsiz55::R](otg_fs_dieptsiz55::R) reader structure"]
impl crate::Readable for OTG_FS_DIEPTSIZ55 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dieptsiz55::W](otg_fs_dieptsiz55::W) writer structure"]
impl crate::Writable for OTG_FS_DIEPTSIZ55 {}
#[doc = "device endpoint-5 transfer size register"]
pub mod otg_fs_dieptsiz55;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dtxfsts55](otg_fs_dtxfsts55) module"]
pub type OTG_FS_DTXFSTS55 = crate::Reg<u32, _OTG_FS_DTXFSTS55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DTXFSTS55;
#[doc = "`read()` method returns [otg_fs_dtxfsts55::R](otg_fs_dtxfsts55::R) reader structure"]
impl crate::Readable for OTG_FS_DTXFSTS55 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_dtxfsts55::W](otg_fs_dtxfsts55::W) writer structure"]
impl crate::Writable for OTG_FS_DTXFSTS55 {}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts55;
#[doc = "device endpoint-4 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepctl4](otg_fs_doepctl4) module"]
pub type OTG_FS_DOEPCTL4 = crate::Reg<u32, _OTG_FS_DOEPCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPCTL4;
#[doc = "`read()` method returns [otg_fs_doepctl4::R](otg_fs_doepctl4::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPCTL4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepctl4::W](otg_fs_doepctl4::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPCTL4 {}
#[doc = "device endpoint-4 control register"]
pub mod otg_fs_doepctl4;
#[doc = "device endpoint-4 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepint4](otg_fs_doepint4) module"]
pub type OTG_FS_DOEPINT4 = crate::Reg<u32, _OTG_FS_DOEPINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPINT4;
#[doc = "`read()` method returns [otg_fs_doepint4::R](otg_fs_doepint4::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPINT4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepint4::W](otg_fs_doepint4::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPINT4 {}
#[doc = "device endpoint-4 interrupt register"]
pub mod otg_fs_doepint4;
#[doc = "device OUT endpoint-4 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doeptsiz4](otg_fs_doeptsiz4) module"]
pub type OTG_FS_DOEPTSIZ4 = crate::Reg<u32, _OTG_FS_DOEPTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPTSIZ4;
#[doc = "`read()` method returns [otg_fs_doeptsiz4::R](otg_fs_doeptsiz4::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doeptsiz4::W](otg_fs_doeptsiz4::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPTSIZ4 {}
#[doc = "device OUT endpoint-4 transfer size register"]
pub mod otg_fs_doeptsiz4;
#[doc = "device endpoint-5 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepctl5](otg_fs_doepctl5) module"]
pub type OTG_FS_DOEPCTL5 = crate::Reg<u32, _OTG_FS_DOEPCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPCTL5;
#[doc = "`read()` method returns [otg_fs_doepctl5::R](otg_fs_doepctl5::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPCTL5 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepctl5::W](otg_fs_doepctl5::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPCTL5 {}
#[doc = "device endpoint-5 control register"]
pub mod otg_fs_doepctl5;
#[doc = "device endpoint-5 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepint5](otg_fs_doepint5) module"]
pub type OTG_FS_DOEPINT5 = crate::Reg<u32, _OTG_FS_DOEPINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPINT5;
#[doc = "`read()` method returns [otg_fs_doepint5::R](otg_fs_doepint5::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPINT5 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepint5::W](otg_fs_doepint5::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPINT5 {}
#[doc = "device endpoint-5 interrupt register"]
pub mod otg_fs_doepint5;
#[doc = "device OUT endpoint-5 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doeptsiz5](otg_fs_doeptsiz5) module"]
pub type OTG_FS_DOEPTSIZ5 = crate::Reg<u32, _OTG_FS_DOEPTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_DOEPTSIZ5;
#[doc = "`read()` method returns [otg_fs_doeptsiz5::R](otg_fs_doeptsiz5::R) reader structure"]
impl crate::Readable for OTG_FS_DOEPTSIZ5 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_doeptsiz5::W](otg_fs_doeptsiz5::W) writer structure"]
impl crate::Writable for OTG_FS_DOEPTSIZ5 {}
#[doc = "device OUT endpoint-5 transfer size register"]
pub mod otg_fs_doeptsiz5;
