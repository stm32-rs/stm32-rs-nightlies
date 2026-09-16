#[repr(C)]
#[derive(Debug)]
///Register block
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
    dieptxf: [DIEPTXF; 8],
}
impl RegisterBlock {
    ///0x00 - OTG_HS control and status register
    #[inline(always)]
    pub const fn gotgctl(&self) -> &GOTGCTL {
        &self.gotgctl
    }
    ///0x04 - OTG_HS interrupt register
    #[inline(always)]
    pub const fn gotgint(&self) -> &GOTGINT {
        &self.gotgint
    }
    ///0x08 - OTG_HS AHB configuration register
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    ///0x0c - OTG_HS USB configuration register
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    ///0x10 - OTG_HS reset register
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    ///0x14 - OTG_HS core interrupt register
    #[inline(always)]
    pub const fn gintsts(&self) -> &GINTSTS {
        &self.gintsts
    }
    ///0x18 - OTG_HS interrupt mask register
    #[inline(always)]
    pub const fn gintmsk(&self) -> &GINTMSK {
        &self.gintmsk
    }
    ///0x1c - OTG_HS Receive status debug read register (peripheral mode mode)
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - OTG_HS Receive status debug read register (host mode)
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - OTG_HS status read and pop register (peripheral mode)
    #[inline(always)]
    pub const fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - OTG_HS status read and pop register (host mode)
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x24 - OTG_HS Receive FIFO size register
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    ///0x28 - Endpoint 0 transmit FIFO size (peripheral mode)
    #[inline(always)]
    pub const fn dieptxf0(&self) -> &DIEPTXF0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)
    #[inline(always)]
    pub const fn hnptxfsiz(&self) -> &HNPTXFSIZ {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2c - OTG_HS nonperiodic transmit FIFO/queue status register
    #[inline(always)]
    pub const fn hnptxsts(&self) -> &HNPTXSTS {
        &self.hnptxsts
    }
    ///0x38 - OTG_HS general core configuration register
    #[inline(always)]
    pub const fn gccfg(&self) -> &GCCFG {
        &self.gccfg
    }
    ///0x3c - OTG_HS core ID register
    #[inline(always)]
    pub const fn cid(&self) -> &CID {
        &self.cid
    }
    ///0x54 - OTG core LPM configuration register
    #[inline(always)]
    pub const fn glpmcfg(&self) -> &GLPMCFG {
        &self.glpmcfg
    }
    ///0x100 - OTG_HS Host periodic transmit FIFO size register
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &HPTXFSIZ {
        &self.hptxfsiz
    }
    ///0x104..0x124 - OTG_HS device IN endpoint transmit FIFO size register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DIEPTXF1` register.</div>
    #[inline(always)]
    pub const fn dieptxf(&self, n: usize) -> &DIEPTXF {
        &self.dieptxf[n]
    }
    ///Iterator for array of:
    ///0x104..0x124 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub fn dieptxf_iter(&self) -> impl Iterator<Item = &DIEPTXF> {
        self.dieptxf.iter()
    }
    ///0x104 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF {
        self.dieptxf(0)
    }
    ///0x108 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF {
        self.dieptxf(1)
    }
    ///0x10c - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF {
        self.dieptxf(2)
    }
    ///0x110 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF {
        self.dieptxf(3)
    }
    ///0x114 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &DIEPTXF {
        self.dieptxf(4)
    }
    ///0x118 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn dieptxf6(&self) -> &DIEPTXF {
        self.dieptxf(5)
    }
    ///0x11c - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn dieptxf7(&self) -> &DIEPTXF {
        self.dieptxf(6)
    }
    ///0x120 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn dieptxf8(&self) -> &DIEPTXF {
        self.dieptxf(7)
    }
}
/**GOTGCTL (rw) register accessor: OTG_HS control and status register

You can [`read`](crate::Reg::read) this register and get [`gotgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GOTGCTL)

For information about available fields see [`mod@gotgctl`] module*/
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTLrs>;
///OTG_HS control and status register
pub mod gotgctl;
/**GOTGINT (rw) register accessor: OTG_HS interrupt register

You can [`read`](crate::Reg::read) this register and get [`gotgint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GOTGINT)

For information about available fields see [`mod@gotgint`] module*/
pub type GOTGINT = crate::Reg<gotgint::GOTGINTrs>;
///OTG_HS interrupt register
pub mod gotgint;
/**GAHBCFG (rw) register accessor: OTG_HS AHB configuration register

You can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GAHBCFG)

For information about available fields see [`mod@gahbcfg`] module*/
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFGrs>;
///OTG_HS AHB configuration register
pub mod gahbcfg;
/**GUSBCFG (rw) register accessor: OTG_HS USB configuration register

You can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GUSBCFG)

For information about available fields see [`mod@gusbcfg`] module*/
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFGrs>;
///OTG_HS USB configuration register
pub mod gusbcfg;
/**GRSTCTL (rw) register accessor: OTG_HS reset register

You can [`read`](crate::Reg::read) this register and get [`grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GRSTCTL)

For information about available fields see [`mod@grstctl`] module*/
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTLrs>;
///OTG_HS reset register
pub mod grstctl;
/**GINTSTS (rw) register accessor: OTG_HS core interrupt register

You can [`read`](crate::Reg::read) this register and get [`gintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GINTSTS)

For information about available fields see [`mod@gintsts`] module*/
pub type GINTSTS = crate::Reg<gintsts::GINTSTSrs>;
///OTG_HS core interrupt register
pub mod gintsts;
/**GINTMSK (rw) register accessor: OTG_HS interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`gintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GINTMSK)

For information about available fields see [`mod@gintmsk`] module*/
pub type GINTMSK = crate::Reg<gintmsk::GINTMSKrs>;
///OTG_HS interrupt mask register
pub mod gintmsk;
/**GRXSTSR_Host (r) register accessor: OTG_HS Receive status debug read register (host mode)

You can [`read`](crate::Reg::read) this register and get [`grxstsr_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GRXSTSR_Host)

For information about available fields see [`mod@grxstsr_host`] module*/
#[doc(alias = "GRXSTSR_Host")]
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOSTrs>;
///OTG_HS Receive status debug read register (host mode)
pub mod grxstsr_host;
/**GRXSTSP_Host (r) register accessor: OTG_HS status read and pop register (host mode)

You can [`read`](crate::Reg::read) this register and get [`grxstsp_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GRXSTSP_Host)

For information about available fields see [`mod@grxstsp_host`] module*/
#[doc(alias = "GRXSTSP_Host")]
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOSTrs>;
///OTG_HS status read and pop register (host mode)
pub mod grxstsp_host;
/**GRXFSIZ (rw) register accessor: OTG_HS Receive FIFO size register

You can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GRXFSIZ)

For information about available fields see [`mod@grxfsiz`] module*/
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZrs>;
///OTG_HS Receive FIFO size register
pub mod grxfsiz;
/**HNPTXFSIZ (rw) register accessor: OTG_HS nonperiodic transmit FIFO size register (host mode)

You can [`read`](crate::Reg::read) this register and get [`hnptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hnptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:HNPTXFSIZ)

For information about available fields see [`mod@hnptxfsiz`] module*/
pub type HNPTXFSIZ = crate::Reg<hnptxfsiz::HNPTXFSIZrs>;
///OTG_HS nonperiodic transmit FIFO size register (host mode)
pub mod hnptxfsiz;
/**DIEPTXF0 (rw) register accessor: Endpoint 0 transmit FIFO size (peripheral mode)

You can [`read`](crate::Reg::read) this register and get [`dieptxf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:DIEPTXF0)

For information about available fields see [`mod@dieptxf0`] module*/
pub type DIEPTXF0 = crate::Reg<dieptxf0::DIEPTXF0rs>;
///Endpoint 0 transmit FIFO size (peripheral mode)
pub mod dieptxf0;
/**HNPTXSTS (r) register accessor: OTG_HS nonperiodic transmit FIFO/queue status register

You can [`read`](crate::Reg::read) this register and get [`hnptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:HNPTXSTS)

For information about available fields see [`mod@hnptxsts`] module*/
pub type HNPTXSTS = crate::Reg<hnptxsts::HNPTXSTSrs>;
///OTG_HS nonperiodic transmit FIFO/queue status register
pub mod hnptxsts;
/**GCCFG (rw) register accessor: OTG_HS general core configuration register

You can [`read`](crate::Reg::read) this register and get [`gccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GCCFG)

For information about available fields see [`mod@gccfg`] module*/
pub type GCCFG = crate::Reg<gccfg::GCCFGrs>;
///OTG_HS general core configuration register
pub mod gccfg;
/**CID (rw) register accessor: OTG_HS core ID register

You can [`read`](crate::Reg::read) this register and get [`cid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:CID)

For information about available fields see [`mod@cid`] module*/
pub type CID = crate::Reg<cid::CIDrs>;
///OTG_HS core ID register
pub mod cid;
/**HPTXFSIZ (rw) register accessor: OTG_HS Host periodic transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:HPTXFSIZ)

For information about available fields see [`mod@hptxfsiz`] module*/
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZrs>;
///OTG_HS Host periodic transmit FIFO size register
pub mod hptxfsiz;
/**DIEPTXF (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`dieptxf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:DIEPTXF[1])

For information about available fields see [`mod@dieptxf`] module*/
pub type DIEPTXF = crate::Reg<dieptxf::DIEPTXFrs>;
///OTG_HS device IN endpoint transmit FIFO size register
pub mod dieptxf;
/**GRXSTSR_Device (r) register accessor: OTG_HS Receive status debug read register (peripheral mode mode)

You can [`read`](crate::Reg::read) this register and get [`grxstsr_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GRXSTSR_Device)

For information about available fields see [`mod@grxstsr_device`] module*/
#[doc(alias = "GRXSTSR_Device")]
pub type GRXSTSR_DEVICE = crate::Reg<grxstsr_device::GRXSTSR_DEVICErs>;
///OTG_HS Receive status debug read register (peripheral mode mode)
pub mod grxstsr_device;
/**GRXSTSP_Device (r) register accessor: OTG_HS status read and pop register (peripheral mode)

You can [`read`](crate::Reg::read) this register and get [`grxstsp_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GRXSTSP_Device)

For information about available fields see [`mod@grxstsp_device`] module*/
#[doc(alias = "GRXSTSP_Device")]
pub type GRXSTSP_DEVICE = crate::Reg<grxstsp_device::GRXSTSP_DEVICErs>;
///OTG_HS status read and pop register (peripheral mode)
pub mod grxstsp_device;
/**GLPMCFG (rw) register accessor: OTG core LPM configuration register

You can [`read`](crate::Reg::read) this register and get [`glpmcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glpmcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#OTG1_HS_GLOBAL:GLPMCFG)

For information about available fields see [`mod@glpmcfg`] module*/
pub type GLPMCFG = crate::Reg<glpmcfg::GLPMCFGrs>;
///OTG core LPM configuration register
pub mod glpmcfg;
