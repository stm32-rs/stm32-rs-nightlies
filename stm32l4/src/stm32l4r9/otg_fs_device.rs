#[repr(C)]
#[doc = "Register block"]
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
    diep0: DIEP0,
    _reserved11: [u8; 0x04],
    diep: [DIEP; 5],
    _reserved12: [u8; 0x0140],
    doep0: DOEP0,
    _reserved13: [u8; 0x0c],
    doep: [DOEP; 5],
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_FS device configuration register (OTG_FS_DCFG)"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    #[doc = "0x04 - OTG_FS device control register (OTG_FS_DCTL)"]
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    #[doc = "0x08 - OTG_FS device status register (OTG_FS_DSTS)"]
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    #[doc = "0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    #[doc = "0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    #[doc = "0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    #[doc = "0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    #[doc = "0x28 - OTG_FS device VBUS discharge time register"]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &DVBUSDIS {
        &self.dvbusdis
    }
    #[doc = "0x2c - OTG_FS device VBUS pulsing time register"]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &DVBUSPULSE {
        &self.dvbuspulse
    }
    #[doc = "0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    #[doc = "0x100..0x11c - Device IN endpoint 0"]
    #[inline(always)]
    pub const fn diep0(&self) -> &DIEP0 {
        &self.diep0
    }
    #[doc = "0x120..0x1c0 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn diep(&self, n: usize) -> &DIEP {
        &self.diep[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x1c0 - Device IN endpoint X"]
    #[inline(always)]
    pub fn diep_iter(&self) -> impl Iterator<Item = &DIEP> {
        self.diep.iter()
    }
    #[doc = "0x120..0x140 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn diep1(&self) -> &DIEP {
        self.diep(0)
    }
    #[doc = "0x140..0x160 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn diep2(&self) -> &DIEP {
        self.diep(1)
    }
    #[doc = "0x160..0x180 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn diep3(&self) -> &DIEP {
        self.diep(2)
    }
    #[doc = "0x180..0x1a0 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn diep4(&self) -> &DIEP {
        self.diep(3)
    }
    #[doc = "0x1a0..0x1c0 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn diep5(&self) -> &DIEP {
        self.diep(4)
    }
    #[doc = "0x300..0x314 - Device OUT endpoint 0"]
    #[inline(always)]
    pub const fn doep0(&self) -> &DOEP0 {
        &self.doep0
    }
    #[doc = "0x320..0x3c0 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn doep(&self, n: usize) -> &DOEP {
        &self.doep[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x320..0x3c0 - Device IN endpoint X"]
    #[inline(always)]
    pub fn doep_iter(&self) -> impl Iterator<Item = &DOEP> {
        self.doep.iter()
    }
    #[doc = "0x320..0x340 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn doep1(&self) -> &DOEP {
        self.doep(0)
    }
    #[doc = "0x340..0x360 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn doep2(&self) -> &DOEP {
        self.doep(1)
    }
    #[doc = "0x360..0x380 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn doep3(&self) -> &DOEP {
        self.doep(2)
    }
    #[doc = "0x380..0x3a0 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn doep4(&self) -> &DOEP {
        self.doep(3)
    }
    #[doc = "0x3a0..0x3c0 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn doep5(&self) -> &DOEP {
        self.doep(4)
    }
}
#[doc = "DCFG (rw) register accessor: OTG_FS device configuration register (OTG_FS_DCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFGrs>;
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: OTG_FS device control register (OTG_FS_DCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
pub type DCTL = crate::Reg<dctl::DCTLrs>;
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: OTG_FS device status register (OTG_FS_DSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
pub type DSTS = crate::Reg<dsts::DSTSrs>;
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSKrs>;
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSKrs>;
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
pub type DAINT = crate::Reg<daint::DAINTrs>;
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSKrs>;
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: OTG_FS device VBUS discharge time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`]
module"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDISrs>;
#[doc = "OTG_FS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: OTG_FS device VBUS pulsing time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`]
module"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSErs>;
#[doc = "OTG_FS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "DIEPEMPMSK (rw) register accessor: OTG_FS device IN endpoint FIFO empty interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSKrs>;
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "Device IN endpoint 0"]
pub use self::diep0::DIEP0;
#[doc = r"Cluster"]
#[doc = "Device IN endpoint 0"]
pub mod diep0;
#[doc = "Device IN endpoint X"]
pub use self::diep::DIEP;
#[doc = r"Cluster"]
#[doc = "Device IN endpoint X"]
pub mod diep;
#[doc = "Device OUT endpoint 0"]
pub use self::doep0::DOEP0;
#[doc = r"Cluster"]
#[doc = "Device OUT endpoint 0"]
pub mod doep0;
#[doc = "Device IN endpoint X"]
pub use self::doep::DOEP;
#[doc = r"Cluster"]
#[doc = "Device IN endpoint X"]
pub mod doep;
