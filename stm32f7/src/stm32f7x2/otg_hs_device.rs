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
    dthrctl: DTHRCTL,
    diepempmsk: DIEPEMPMSK,
    deachint: DEACHINT,
    deachintmsk: DEACHINTMSK,
    _reserved13: [u8; 0x04],
    diepeachmsk1: DIEPEACHMSK1,
    _reserved14: [u8; 0x3c],
    doepeachmsk1: DOEPEACHMSK1,
    _reserved15: [u8; 0x78],
    diep0: DIEP0,
    _reserved16: [u8; 0x04],
    diep: [DIEP; 8],
    _reserved17: [u8; 0xe0],
    doep0: DOEP0,
    _reserved18: [u8; 0x08],
    doep: [DOEP; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    #[doc = "0x04 - OTG_HS device control register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    #[doc = "0x08 - OTG_HS device status register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &DVBUSDIS {
        &self.dvbusdis
    }
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &DVBUSPULSE {
        &self.dvbuspulse
    }
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    #[inline(always)]
    pub const fn dthrctl(&self) -> &DTHRCTL {
        &self.dthrctl
    }
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    #[inline(always)]
    pub const fn deachint(&self) -> &DEACHINT {
        &self.deachint
    }
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    #[inline(always)]
    pub const fn deachintmsk(&self) -> &DEACHINTMSK {
        &self.deachintmsk
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn diepeachmsk1(&self) -> &DIEPEACHMSK1 {
        &self.diepeachmsk1
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn doepeachmsk1(&self) -> &DOEPEACHMSK1 {
        &self.doepeachmsk1
    }
    #[doc = "0x100..0x11c - Device IN endpoint 0"]
    #[inline(always)]
    pub const fn diep0(&self) -> &DIEP0 {
        &self.diep0
    }
    #[doc = "0x120..0x220 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn diep(&self, n: usize) -> &DIEP {
        &self.diep[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x220 - Device IN endpoint X"]
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
    #[doc = "0x1c0..0x1e0 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn diep6(&self) -> &DIEP {
        self.diep(5)
    }
    #[doc = "0x1e0..0x200 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn diep7(&self) -> &DIEP {
        self.diep(6)
    }
    #[doc = "0x200..0x220 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn diep8(&self) -> &DIEP {
        self.diep(7)
    }
    #[doc = "0x300..0x318 - Device OUT endpoint 0"]
    #[inline(always)]
    pub const fn doep0(&self) -> &DOEP0 {
        &self.doep0
    }
    #[doc = "0x320..0x420 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn doep(&self, n: usize) -> &DOEP {
        &self.doep[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x320..0x420 - Device IN endpoint X"]
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
    #[doc = "0x3c0..0x3e0 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn doep6(&self) -> &DOEP {
        self.doep(5)
    }
    #[doc = "0x3e0..0x400 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn doep7(&self) -> &DOEP {
        self.doep(6)
    }
    #[doc = "0x400..0x420 - Device IN endpoint X"]
    #[inline(always)]
    pub const fn doep8(&self) -> &DOEP {
        self.doep(7)
    }
}
#[doc = "DCFG (rw) register accessor: OTG_HS device configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFGrs>;
#[doc = "OTG_HS device configuration register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: OTG_HS device control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
pub type DCTL = crate::Reg<dctl::DCTLrs>;
#[doc = "OTG_HS device control register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: OTG_HS device status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
pub type DSTS = crate::Reg<dsts::DSTSrs>;
#[doc = "OTG_HS device status register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: OTG_HS device IN endpoint common interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSKrs>;
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: OTG_HS device OUT endpoint common interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSKrs>;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: OTG_HS device all endpoints interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
pub type DAINT = crate::Reg<daint::DAINTrs>;
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: OTG_HS all endpoints interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSKrs>;
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: OTG_HS device VBUS discharge time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`]
module"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDISrs>;
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: OTG_HS device VBUS pulsing time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`]
module"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSErs>;
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: OTG_HS Device threshold control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dthrctl`]
module"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTLrs>;
#[doc = "OTG_HS Device threshold control register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: OTG_HS device IN endpoint FIFO empty interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSKrs>;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DEACHINT (rw) register accessor: OTG_HS device each endpoint interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deachint`]
module"]
pub type DEACHINT = crate::Reg<deachint::DEACHINTrs>;
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod deachint;
#[doc = "DEACHINTMSK (rw) register accessor: OTG_HS device each endpoint interrupt register mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deachintmsk`]
module"]
pub type DEACHINTMSK = crate::Reg<deachintmsk::DEACHINTMSKrs>;
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod deachintmsk;
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
#[doc = "DIEPEACHMSK1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepeachmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepeachmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk1`]
module"]
pub type DIEPEACHMSK1 = crate::Reg<diepeachmsk1::DIEPEACHMSK1rs>;
#[doc = ""]
pub mod diepeachmsk1;
#[doc = "DOEPEACHMSK1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepeachmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepeachmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk1`]
module"]
pub type DOEPEACHMSK1 = crate::Reg<doepeachmsk1::DOEPEACHMSK1rs>;
#[doc = ""]
pub mod doepeachmsk1;
