#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gotgctl: GOTGCTL,
    gotgint: GOTGINT,
    gahbcfg: GAHBCFG,
    gusbcfg: GUSBCFG,
    grstctl: GRSTCTL,
    gintsts: GINTSTS,
    gintmsk: GINTMSK,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    grxfsiz: GRXFSIZ,
    _reserved_10_dieptxf0: [u8; 0x04],
    hnptxsts: HNPTXSTS,
    gi2cctl: GI2CCTL,
    _reserved13: [u8; 0x04],
    gccfg: GCCFG,
    cid: CID,
    _reserved15: [u8; 0x14],
    glpmcfg: GLPMCFG,
    gpwrdn: GPWRDN,
    _reserved17: [u8; 0x04],
    gadpctl: GADPCTL,
    _reserved18: [u8; 0x9c],
    hptxfsiz: HPTXFSIZ,
    dieptxf: [DIEPTXF; 5],
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"]
    #[inline(always)]
    pub const fn gotgctl(&self) -> &GOTGCTL {
        &self.gotgctl
    }
    #[doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"]
    #[inline(always)]
    pub const fn gotgint(&self) -> &GOTGINT {
        &self.gotgint
    }
    #[doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    #[doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    #[doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    #[doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
    #[inline(always)]
    pub const fn gintsts(&self) -> &GINTSTS {
        &self.gintsts
    }
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &GINTMSK {
        &self.gintmsk
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - OTG status read and pop register (Host mode)"]
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - OTG status read and pop register (Device mode)"]
    #[inline(always)]
    pub const fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    #[doc = "0x28 - OTG_FS Host non-periodic transmit FIFO size register"]
    #[inline(always)]
    pub const fn hnptxfsiz_host(&self) -> &HNPTXFSIZ_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - OTG_FS Endpoint 0 Transmit FIFO size"]
    #[inline(always)]
    pub const fn dieptxf0(&self) -> &DIEPTXF0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
    #[inline(always)]
    pub const fn hnptxsts(&self) -> &HNPTXSTS {
        &self.hnptxsts
    }
    #[doc = "0x30 - OTG I2C access register"]
    #[inline(always)]
    pub const fn gi2cctl(&self) -> &GI2CCTL {
        &self.gi2cctl
    }
    #[doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"]
    #[inline(always)]
    pub const fn gccfg(&self) -> &GCCFG {
        &self.gccfg
    }
    #[doc = "0x3c - core ID register"]
    #[inline(always)]
    pub const fn cid(&self) -> &CID {
        &self.cid
    }
    #[doc = "0x54 - OTG core LPM configuration register"]
    #[inline(always)]
    pub const fn glpmcfg(&self) -> &GLPMCFG {
        &self.glpmcfg
    }
    #[doc = "0x58 - OTG power down register"]
    #[inline(always)]
    pub const fn gpwrdn(&self) -> &GPWRDN {
        &self.gpwrdn
    }
    #[doc = "0x60 - OTG ADP timer, control and status register"]
    #[inline(always)]
    pub const fn gadpctl(&self) -> &GADPCTL {
        &self.gadpctl
    }
    #[doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &HPTXFSIZ {
        &self.hptxfsiz
    }
    #[doc = "0x104..0x118 - OTF_FS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf(&self, n: usize) -> &DIEPTXF {
        &self.dieptxf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x118 - OTF_FS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub fn dieptxf_iter(&self) -> impl Iterator<Item = &DIEPTXF> {
        self.dieptxf.iter()
    }
    #[doc = "0x104 - OTF_FS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF {
        self.dieptxf(0)
    }
    #[doc = "0x108 - OTF_FS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF {
        self.dieptxf(1)
    }
    #[doc = "0x10c - OTF_FS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF {
        self.dieptxf(2)
    }
    #[doc = "0x110 - OTF_FS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF {
        self.dieptxf(3)
    }
    #[doc = "0x114 - OTF_FS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &DIEPTXF {
        self.dieptxf(4)
    }
}
#[doc = "GOTGCTL (rw) register accessor: OTG_FS control and status register (OTG_FS_GOTGCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`]
module"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTLrs>;
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: OTG_FS interrupt register (OTG_FS_GOTGINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`]
module"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINTrs>;
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: OTG_FS AHB configuration register (OTG_FS_GAHBCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`]
module"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFGrs>;
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: OTG_FS USB configuration register (OTG_FS_GUSBCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`]
module"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFGrs>;
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: OTG_FS reset register (OTG_FS_GRSTCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`]
module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTLrs>;
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: OTG_FS core interrupt register (OTG_FS_GINTSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`]
module"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTSrs>;
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`]
module"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSKrs>;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod gintmsk;
#[doc = "GRXSTSR_Device (r) register accessor: OTG_FS Receive status debug read(Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_device`]
module"]
#[doc(alias = "GRXSTSR_Device")]
pub type GRXSTSR_DEVICE = crate::Reg<grxstsr_device::GRXSTSR_DEVICErs>;
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub mod grxstsr_device;
#[doc = "GRXSTSR_Host (r) register accessor: OTG_FS Receive status debug read(Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_host`]
module"]
#[doc(alias = "GRXSTSR_Host")]
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOSTrs>;
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub mod grxstsr_host;
#[doc = "GRXFSIZ (rw) register accessor: OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`]
module"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZrs>;
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub mod grxfsiz;
#[doc = "DIEPTXF0 (rw) register accessor: OTG_FS Endpoint 0 Transmit FIFO size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf0`]
module"]
pub type DIEPTXF0 = crate::Reg<dieptxf0::DIEPTXF0rs>;
#[doc = "OTG_FS Endpoint 0 Transmit FIFO size"]
pub mod dieptxf0;
#[doc = "HNPTXFSIZ_Host (rw) register accessor: OTG_FS Host non-periodic transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hnptxfsiz_host::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hnptxfsiz_host::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hnptxfsiz_host`]
module"]
#[doc(alias = "HNPTXFSIZ_Host")]
pub type HNPTXFSIZ_HOST = crate::Reg<hnptxfsiz_host::HNPTXFSIZ_HOSTrs>;
#[doc = "OTG_FS Host non-periodic transmit FIFO size register"]
pub mod hnptxfsiz_host;
#[doc = "HNPTXSTS (r) register accessor: OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hnptxsts`]
module"]
pub type HNPTXSTS = crate::Reg<hnptxsts::HNPTXSTSrs>;
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub mod hnptxsts;
#[doc = "GCCFG (rw) register accessor: OTG_FS general core configuration register (OTG_FS_GCCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gccfg`]
module"]
pub type GCCFG = crate::Reg<gccfg::GCCFGrs>;
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub mod gccfg;
#[doc = "CID (rw) register accessor: core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid`]
module"]
pub type CID = crate::Reg<cid::CIDrs>;
#[doc = "core ID register"]
pub mod cid;
#[doc = "HPTXFSIZ (rw) register accessor: OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`]
module"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZrs>;
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub mod hptxfsiz;
#[doc = "DIEPTXF (rw) register accessor: OTF_FS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf`]
module"]
pub type DIEPTXF = crate::Reg<dieptxf::DIEPTXFrs>;
#[doc = "OTF_FS device IN endpoint transmit FIFO size register"]
pub mod dieptxf;
#[doc = "GRXSTSP_Device (r) register accessor: OTG status read and pop register (Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_device`]
module"]
#[doc(alias = "GRXSTSP_Device")]
pub type GRXSTSP_DEVICE = crate::Reg<grxstsp_device::GRXSTSP_DEVICErs>;
#[doc = "OTG status read and pop register (Device mode)"]
pub mod grxstsp_device;
#[doc = "GRXSTSP_Host (r) register accessor: OTG status read and pop register (Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_host`]
module"]
#[doc(alias = "GRXSTSP_Host")]
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOSTrs>;
#[doc = "OTG status read and pop register (Host mode)"]
pub mod grxstsp_host;
#[doc = "GI2CCTL (rw) register accessor: OTG I2C access register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gi2cctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gi2cctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gi2cctl`]
module"]
pub type GI2CCTL = crate::Reg<gi2cctl::GI2CCTLrs>;
#[doc = "OTG I2C access register"]
pub mod gi2cctl;
#[doc = "GPWRDN (rw) register accessor: OTG power down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpwrdn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpwrdn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpwrdn`]
module"]
pub type GPWRDN = crate::Reg<gpwrdn::GPWRDNrs>;
#[doc = "OTG power down register"]
pub mod gpwrdn;
#[doc = "GADPCTL (rw) register accessor: OTG ADP timer, control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gadpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gadpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gadpctl`]
module"]
pub type GADPCTL = crate::Reg<gadpctl::GADPCTLrs>;
#[doc = "OTG ADP timer, control and status register"]
pub mod gadpctl;
#[doc = "GLPMCFG (rw) register accessor: OTG core LPM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glpmcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glpmcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glpmcfg`]
module"]
pub type GLPMCFG = crate::Reg<glpmcfg::GLPMCFGrs>;
#[doc = "OTG core LPM configuration register"]
pub mod glpmcfg;
