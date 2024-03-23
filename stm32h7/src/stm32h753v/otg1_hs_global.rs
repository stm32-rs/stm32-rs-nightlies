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
    _reserved12: [u8; 0x08],
    gccfg: GCCFG,
    cid: CID,
    _reserved14: [u8; 0x14],
    glpmcfg: GLPMCFG,
    _reserved15: [u8; 0xa8],
    hptxfsiz: HPTXFSIZ,
    dieptxf: [DIEPTXF; 5],
    dieptxf6: DIEPTXF6,
    dieptxf7: DIEPTXF7,
    dieptxf8: DIEPTXF8,
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_HS control and status register"]
    #[inline(always)]
    pub const fn gotgctl(&self) -> &GOTGCTL {
        &self.gotgctl
    }
    #[doc = "0x04 - OTG_HS interrupt register"]
    #[inline(always)]
    pub const fn gotgint(&self) -> &GOTGINT {
        &self.gotgint
    }
    #[doc = "0x08 - OTG_HS AHB configuration register"]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    #[doc = "0x0c - OTG_HS USB configuration register"]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    #[doc = "0x10 - OTG_HS reset register"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    #[doc = "0x14 - OTG_HS core interrupt register"]
    #[inline(always)]
    pub const fn gintsts(&self) -> &GINTSTS {
        &self.gintsts
    }
    #[doc = "0x18 - OTG_HS interrupt mask register"]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &GINTMSK {
        &self.gintmsk
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (peripheral mode mode)"]
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (peripheral mode)"]
    #[inline(always)]
    pub const fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - OTG_HS Receive FIFO size register"]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    #[doc = "0x28 - Endpoint 0 transmit FIFO size (peripheral mode)"]
    #[inline(always)]
    pub const fn dieptxf0(&self) -> &DIEPTXF0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    #[inline(always)]
    pub const fn hnptxfsiz(&self) -> &HNPTXFSIZ {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - OTG_HS nonperiodic transmit FIFO/queue status register"]
    #[inline(always)]
    pub const fn hnptxsts(&self) -> &HNPTXSTS {
        &self.hnptxsts
    }
    #[doc = "0x38 - OTG_HS general core configuration register"]
    #[inline(always)]
    pub const fn gccfg(&self) -> &GCCFG {
        &self.gccfg
    }
    #[doc = "0x3c - OTG_HS core ID register"]
    #[inline(always)]
    pub const fn cid(&self) -> &CID {
        &self.cid
    }
    #[doc = "0x54 - OTG core LPM configuration register"]
    #[inline(always)]
    pub const fn glpmcfg(&self) -> &GLPMCFG {
        &self.glpmcfg
    }
    #[doc = "0x100 - OTG_HS Host periodic transmit FIFO size register"]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &HPTXFSIZ {
        &self.hptxfsiz
    }
    #[doc = "0x104..0x118 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf(&self, n: usize) -> &DIEPTXF {
        &self.dieptxf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x118 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub fn dieptxf_iter(&self) -> impl Iterator<Item = &DIEPTXF> {
        self.dieptxf.iter()
    }
    #[doc = "0x104 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF {
        self.dieptxf(0)
    }
    #[doc = "0x108 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF {
        self.dieptxf(1)
    }
    #[doc = "0x10c - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF {
        self.dieptxf(2)
    }
    #[doc = "0x110 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF {
        self.dieptxf(3)
    }
    #[doc = "0x114 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &DIEPTXF {
        self.dieptxf(4)
    }
    #[doc = "0x118 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf6(&self) -> &DIEPTXF6 {
        &self.dieptxf6
    }
    #[doc = "0x11c - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf7(&self) -> &DIEPTXF7 {
        &self.dieptxf7
    }
    #[doc = "0x120 - OTG_HS device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf8(&self) -> &DIEPTXF8 {
        &self.dieptxf8
    }
}
#[doc = "GOTGCTL (rw) register accessor: OTG_HS control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`]
module"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTLrs>;
#[doc = "OTG_HS control and status register"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: OTG_HS interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`]
module"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINTrs>;
#[doc = "OTG_HS interrupt register"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: OTG_HS AHB configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`]
module"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFGrs>;
#[doc = "OTG_HS AHB configuration register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: OTG_HS USB configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`]
module"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFGrs>;
#[doc = "OTG_HS USB configuration register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: OTG_HS reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`]
module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTLrs>;
#[doc = "OTG_HS reset register"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: OTG_HS core interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`]
module"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTSrs>;
#[doc = "OTG_HS core interrupt register"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: OTG_HS interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`]
module"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSKrs>;
#[doc = "OTG_HS interrupt mask register"]
pub mod gintmsk;
#[doc = "GRXSTSR_Host (r) register accessor: OTG_HS Receive status debug read register (host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_host`]
module"]
#[doc(alias = "GRXSTSR_Host")]
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOSTrs>;
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub mod grxstsr_host;
#[doc = "GRXSTSP_Host (r) register accessor: OTG_HS status read and pop register (host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_host`]
module"]
#[doc(alias = "GRXSTSP_Host")]
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOSTrs>;
#[doc = "OTG_HS status read and pop register (host mode)"]
pub mod grxstsp_host;
#[doc = "GRXFSIZ (rw) register accessor: OTG_HS Receive FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`]
module"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZrs>;
#[doc = "OTG_HS Receive FIFO size register"]
pub mod grxfsiz;
#[doc = "HNPTXFSIZ (rw) register accessor: OTG_HS nonperiodic transmit FIFO size register (host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hnptxfsiz`]
module"]
pub type HNPTXFSIZ = crate::Reg<hnptxfsiz::HNPTXFSIZrs>;
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub mod hnptxfsiz;
#[doc = "DIEPTXF0 (rw) register accessor: Endpoint 0 transmit FIFO size (peripheral mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf0`]
module"]
pub type DIEPTXF0 = crate::Reg<dieptxf0::DIEPTXF0rs>;
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub mod dieptxf0;
#[doc = "HNPTXSTS (r) register accessor: OTG_HS nonperiodic transmit FIFO/queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hnptxsts`]
module"]
pub type HNPTXSTS = crate::Reg<hnptxsts::HNPTXSTSrs>;
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub mod hnptxsts;
#[doc = "GCCFG (rw) register accessor: OTG_HS general core configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gccfg`]
module"]
pub type GCCFG = crate::Reg<gccfg::GCCFGrs>;
#[doc = "OTG_HS general core configuration register"]
pub mod gccfg;
#[doc = "CID (rw) register accessor: OTG_HS core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid`]
module"]
pub type CID = crate::Reg<cid::CIDrs>;
#[doc = "OTG_HS core ID register"]
pub mod cid;
#[doc = "HPTXFSIZ (rw) register accessor: OTG_HS Host periodic transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`]
module"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZrs>;
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf`]
module"]
pub type DIEPTXF = crate::Reg<dieptxf::DIEPTXFrs>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf;
#[doc = "DIEPTXF6 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf6`]
module"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6rs>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf6;
#[doc = "DIEPTXF7 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf7`]
module"]
pub type DIEPTXF7 = crate::Reg<dieptxf7::DIEPTXF7rs>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf7;
#[doc = "GRXSTSR_Device (r) register accessor: OTG_HS Receive status debug read register (peripheral mode mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_device`]
module"]
#[doc(alias = "GRXSTSR_Device")]
pub type GRXSTSR_DEVICE = crate::Reg<grxstsr_device::GRXSTSR_DEVICErs>;
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub mod grxstsr_device;
#[doc = "GRXSTSP_Device (r) register accessor: OTG_HS status read and pop register (peripheral mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_device`]
module"]
#[doc(alias = "GRXSTSP_Device")]
pub type GRXSTSP_DEVICE = crate::Reg<grxstsp_device::GRXSTSP_DEVICErs>;
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub mod grxstsp_device;
#[doc = "GLPMCFG (rw) register accessor: OTG core LPM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glpmcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glpmcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glpmcfg`]
module"]
pub type GLPMCFG = crate::Reg<glpmcfg::GLPMCFGrs>;
#[doc = "OTG core LPM configuration register"]
pub mod glpmcfg;
#[doc = "DIEPTXF8 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf8`]
module"]
pub type DIEPTXF8 = crate::Reg<dieptxf8::DIEPTXF8rs>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf8;
