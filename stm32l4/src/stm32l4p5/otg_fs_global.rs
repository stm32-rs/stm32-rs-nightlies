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
    gpwrdn: GPWRDN,
    _reserved16: [u8; 0x04],
    gadpctl: GADPCTL,
    _reserved17: [u8; 0x9c],
    hptxfsiz: HPTXFSIZ,
    dieptxf1: DIEPTXF1,
    dieptxf2: DIEPTXF2,
    dieptxf3: DIEPTXF3,
    dieptxf4: DIEPTXF4,
    dieptxf5: DIEPTXF5,
}
impl RegisterBlock {
    ///0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)
    #[inline(always)]
    pub const fn gotgctl(&self) -> &GOTGCTL {
        &self.gotgctl
    }
    ///0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)
    #[inline(always)]
    pub const fn gotgint(&self) -> &GOTGINT {
        &self.gotgint
    }
    ///0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    ///0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    ///0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    ///0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)
    #[inline(always)]
    pub const fn gintsts(&self) -> &GINTSTS {
        &self.gintsts
    }
    ///0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)
    #[inline(always)]
    pub const fn gintmsk(&self) -> &GINTMSK {
        &self.gintmsk
    }
    ///0x1c - OTG_FS Receive status debug read(Host mode)
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - OTG_FS Receive status debug read(Device mode)
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - OTG status read and pop (host mode)
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - OTG status read and pop (device mode)
    #[inline(always)]
    pub const fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    ///0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)
    #[inline(always)]
    pub const fn hnptxfsiz(&self) -> &HNPTXFSIZ {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)
    #[inline(always)]
    pub const fn dieptxf0(&self) -> &DIEPTXF0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    ///0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
    #[inline(always)]
    pub const fn hnptxsts(&self) -> &HNPTXSTS {
        &self.hnptxsts
    }
    ///0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)
    #[inline(always)]
    pub const fn gccfg(&self) -> &GCCFG {
        &self.gccfg
    }
    ///0x3c - core ID register
    #[inline(always)]
    pub const fn cid(&self) -> &CID {
        &self.cid
    }
    ///0x54 - OTG core LPM configuration register
    #[inline(always)]
    pub const fn glpmcfg(&self) -> &GLPMCFG {
        &self.glpmcfg
    }
    ///0x58 - OTG power down register
    #[inline(always)]
    pub const fn gpwrdn(&self) -> &GPWRDN {
        &self.gpwrdn
    }
    ///0x60 - OTG ADP timer, control and status register
    #[inline(always)]
    pub const fn gadpctl(&self) -> &GADPCTL {
        &self.gadpctl
    }
    ///0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &HPTXFSIZ {
        &self.hptxfsiz
    }
    ///0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF1 {
        &self.dieptxf1
    }
    ///0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF2 {
        &self.dieptxf2
    }
    ///0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF3 {
        &self.dieptxf3
    }
    ///0x110 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF4 {
        &self.dieptxf4
    }
    ///0x114 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF6)
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &DIEPTXF5 {
        &self.dieptxf5
    }
}
/**GOTGCTL (rw) register accessor: OTG_FS control and status register (OTG_FS_GOTGCTL)

You can [`read`](crate::Reg::read) this register and get [`gotgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GOTGCTL)

For information about available fields see [`mod@gotgctl`]
module*/
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTLrs>;
///OTG_FS control and status register (OTG_FS_GOTGCTL)
pub mod gotgctl;
/**GOTGINT (rw) register accessor: OTG_FS interrupt register (OTG_FS_GOTGINT)

You can [`read`](crate::Reg::read) this register and get [`gotgint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GOTGINT)

For information about available fields see [`mod@gotgint`]
module*/
pub type GOTGINT = crate::Reg<gotgint::GOTGINTrs>;
///OTG_FS interrupt register (OTG_FS_GOTGINT)
pub mod gotgint;
/**GAHBCFG (rw) register accessor: OTG_FS AHB configuration register (OTG_FS_GAHBCFG)

You can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GAHBCFG)

For information about available fields see [`mod@gahbcfg`]
module*/
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFGrs>;
///OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
pub mod gahbcfg;
/**GUSBCFG (rw) register accessor: OTG_FS USB configuration register (OTG_FS_GUSBCFG)

You can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GUSBCFG)

For information about available fields see [`mod@gusbcfg`]
module*/
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFGrs>;
///OTG_FS USB configuration register (OTG_FS_GUSBCFG)
pub mod gusbcfg;
/**GRSTCTL (rw) register accessor: OTG_FS reset register (OTG_FS_GRSTCTL)

You can [`read`](crate::Reg::read) this register and get [`grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GRSTCTL)

For information about available fields see [`mod@grstctl`]
module*/
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTLrs>;
///OTG_FS reset register (OTG_FS_GRSTCTL)
pub mod grstctl;
/**GINTSTS (rw) register accessor: OTG_FS core interrupt register (OTG_FS_GINTSTS)

You can [`read`](crate::Reg::read) this register and get [`gintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GINTSTS)

For information about available fields see [`mod@gintsts`]
module*/
pub type GINTSTS = crate::Reg<gintsts::GINTSTSrs>;
///OTG_FS core interrupt register (OTG_FS_GINTSTS)
pub mod gintsts;
/**GINTMSK (rw) register accessor: OTG_FS interrupt mask register (OTG_FS_GINTMSK)

You can [`read`](crate::Reg::read) this register and get [`gintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GINTMSK)

For information about available fields see [`mod@gintmsk`]
module*/
pub type GINTMSK = crate::Reg<gintmsk::GINTMSKrs>;
///OTG_FS interrupt mask register (OTG_FS_GINTMSK)
pub mod gintmsk;
/**GRXSTSR_Device (r) register accessor: OTG_FS Receive status debug read(Device mode)

You can [`read`](crate::Reg::read) this register and get [`grxstsr_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GRXSTSR_Device)

For information about available fields see [`mod@grxstsr_device`]
module*/
#[doc(alias = "GRXSTSR_Device")]
pub type GRXSTSR_DEVICE = crate::Reg<grxstsr_device::GRXSTSR_DEVICErs>;
///OTG_FS Receive status debug read(Device mode)
pub mod grxstsr_device;
/**GRXSTSR_Host (r) register accessor: OTG_FS Receive status debug read(Host mode)

You can [`read`](crate::Reg::read) this register and get [`grxstsr_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GRXSTSR_Host)

For information about available fields see [`mod@grxstsr_host`]
module*/
#[doc(alias = "GRXSTSR_Host")]
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOSTrs>;
///OTG_FS Receive status debug read(Host mode)
pub mod grxstsr_host;
/**GRXFSIZ (rw) register accessor: OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)

You can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GRXFSIZ)

For information about available fields see [`mod@grxfsiz`]
module*/
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZrs>;
///OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
pub mod grxfsiz;
/**DIEPTXF0 (rw) register accessor: OTG_FS non-periodic transmit FIFO size register (Device mode)

You can [`read`](crate::Reg::read) this register and get [`dieptxf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:DIEPTXF0)

For information about available fields see [`mod@dieptxf0`]
module*/
pub type DIEPTXF0 = crate::Reg<dieptxf0::DIEPTXF0rs>;
///OTG_FS non-periodic transmit FIFO size register (Device mode)
pub mod dieptxf0;
/**HNPTXFSIZ (rw) register accessor: OTG_FS non-periodic transmit FIFO size register (Host mode)

You can [`read`](crate::Reg::read) this register and get [`hnptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hnptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:HNPTXFSIZ)

For information about available fields see [`mod@hnptxfsiz`]
module*/
pub type HNPTXFSIZ = crate::Reg<hnptxfsiz::HNPTXFSIZrs>;
///OTG_FS non-periodic transmit FIFO size register (Host mode)
pub mod hnptxfsiz;
/**HNPTXSTS (r) register accessor: OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)

You can [`read`](crate::Reg::read) this register and get [`hnptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:HNPTXSTS)

For information about available fields see [`mod@hnptxsts`]
module*/
pub type HNPTXSTS = crate::Reg<hnptxsts::HNPTXSTSrs>;
///OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
pub mod hnptxsts;
/**GCCFG (rw) register accessor: OTG_FS general core configuration register (OTG_FS_GCCFG)

You can [`read`](crate::Reg::read) this register and get [`gccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GCCFG)

For information about available fields see [`mod@gccfg`]
module*/
pub type GCCFG = crate::Reg<gccfg::GCCFGrs>;
///OTG_FS general core configuration register (OTG_FS_GCCFG)
pub mod gccfg;
/**CID (rw) register accessor: core ID register

You can [`read`](crate::Reg::read) this register and get [`cid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:CID)

For information about available fields see [`mod@cid`]
module*/
pub type CID = crate::Reg<cid::CIDrs>;
///core ID register
pub mod cid;
/**HPTXFSIZ (rw) register accessor: OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)

You can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:HPTXFSIZ)

For information about available fields see [`mod@hptxfsiz`]
module*/
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZrs>;
///OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
pub mod hptxfsiz;
/**DIEPTXF1 (rw) register accessor: OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)

You can [`read`](crate::Reg::read) this register and get [`dieptxf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:DIEPTXF1)

For information about available fields see [`mod@dieptxf1`]
module*/
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1rs>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
pub mod dieptxf1;
/**DIEPTXF2 (rw) register accessor: OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)

You can [`read`](crate::Reg::read) this register and get [`dieptxf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:DIEPTXF2)

For information about available fields see [`mod@dieptxf2`]
module*/
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2rs>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
pub mod dieptxf2;
/**DIEPTXF3 (rw) register accessor: OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)

You can [`read`](crate::Reg::read) this register and get [`dieptxf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:DIEPTXF3)

For information about available fields see [`mod@dieptxf3`]
module*/
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3rs>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
pub mod dieptxf3;
/**GRXSTSP_Device (r) register accessor: OTG status read and pop (device mode)

You can [`read`](crate::Reg::read) this register and get [`grxstsp_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GRXSTSP_Device)

For information about available fields see [`mod@grxstsp_device`]
module*/
#[doc(alias = "GRXSTSP_Device")]
pub type GRXSTSP_DEVICE = crate::Reg<grxstsp_device::GRXSTSP_DEVICErs>;
///OTG status read and pop (device mode)
pub mod grxstsp_device;
/**GRXSTSP_Host (r) register accessor: OTG status read and pop (host mode)

You can [`read`](crate::Reg::read) this register and get [`grxstsp_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GRXSTSP_Host)

For information about available fields see [`mod@grxstsp_host`]
module*/
#[doc(alias = "GRXSTSP_Host")]
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOSTrs>;
///OTG status read and pop (host mode)
pub mod grxstsp_host;
/**DIEPTXF4 (rw) register accessor: OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)

You can [`read`](crate::Reg::read) this register and get [`dieptxf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:DIEPTXF4)

For information about available fields see [`mod@dieptxf4`]
module*/
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4rs>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)
pub mod dieptxf4;
/**DIEPTXF5 (rw) register accessor: OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF6)

You can [`read`](crate::Reg::read) this register and get [`dieptxf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:DIEPTXF5)

For information about available fields see [`mod@dieptxf5`]
module*/
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5rs>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF6)
pub mod dieptxf5;
/**GLPMCFG (rw) register accessor: OTG core LPM configuration register

You can [`read`](crate::Reg::read) this register and get [`glpmcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glpmcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GLPMCFG)

For information about available fields see [`mod@glpmcfg`]
module*/
pub type GLPMCFG = crate::Reg<glpmcfg::GLPMCFGrs>;
///OTG core LPM configuration register
pub mod glpmcfg;
/**GPWRDN (rw) register accessor: OTG power down register

You can [`read`](crate::Reg::read) this register and get [`gpwrdn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpwrdn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GPWRDN)

For information about available fields see [`mod@gpwrdn`]
module*/
pub type GPWRDN = crate::Reg<gpwrdn::GPWRDNrs>;
///OTG power down register
pub mod gpwrdn;
/**GADPCTL (rw) register accessor: OTG ADP timer, control and status register

You can [`read`](crate::Reg::read) this register and get [`gadpctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gadpctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_GLOBAL:GADPCTL)

For information about available fields see [`mod@gadpctl`]
module*/
pub type GADPCTL = crate::Reg<gadpctl::GADPCTLrs>;
///OTG ADP timer, control and status register
pub mod gadpctl;
