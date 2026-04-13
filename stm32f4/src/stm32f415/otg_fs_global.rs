#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    fs_gotgctl: FS_GOTGCTL,
    fs_gotgint: FS_GOTGINT,
    fs_gahbcfg: FS_GAHBCFG,
    fs_gusbcfg: FS_GUSBCFG,
    fs_grstctl: FS_GRSTCTL,
    fs_gintsts: FS_GINTSTS,
    fs_gintmsk: FS_GINTMSK,
    _reserved_7_fs_grxstsr: [u8; 0x04],
    _reserved8: [u8; 0x04],
    fs_grxfsiz: FS_GRXFSIZ,
    _reserved_9_fs_gnptxfsiz: [u8; 0x04],
    fs_gnptxsts: FS_GNPTXSTS,
    _reserved11: [u8; 0x08],
    fs_gccfg: FS_GCCFG,
    fs_cid: FS_CID,
    _reserved13: [u8; 0xc0],
    fs_hptxfsiz: FS_HPTXFSIZ,
    fs_dieptxf1: FS_DIEPTXF1,
    fs_dieptxf2: FS_DIEPTXF2,
    fs_dieptxf3: FS_DIEPTXF3,
}
impl RegisterBlock {
    ///0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)
    #[inline(always)]
    pub const fn fs_gotgctl(&self) -> &FS_GOTGCTL {
        &self.fs_gotgctl
    }
    ///0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)
    #[inline(always)]
    pub const fn fs_gotgint(&self) -> &FS_GOTGINT {
        &self.fs_gotgint
    }
    ///0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
    #[inline(always)]
    pub const fn fs_gahbcfg(&self) -> &FS_GAHBCFG {
        &self.fs_gahbcfg
    }
    ///0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)
    #[inline(always)]
    pub const fn fs_gusbcfg(&self) -> &FS_GUSBCFG {
        &self.fs_gusbcfg
    }
    ///0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)
    #[inline(always)]
    pub const fn fs_grstctl(&self) -> &FS_GRSTCTL {
        &self.fs_grstctl
    }
    ///0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)
    #[inline(always)]
    pub const fn fs_gintsts(&self) -> &FS_GINTSTS {
        &self.fs_gintsts
    }
    ///0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)
    #[inline(always)]
    pub const fn fs_gintmsk(&self) -> &FS_GINTMSK {
        &self.fs_gintmsk
    }
    ///0x1c - OTG_FS Receive status debug read(Hostmode)
    #[inline(always)]
    pub const fn fs_grxstsr_host(&self) -> &FS_GRXSTSR_HOST {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - OTG_FS Receive status debug read(Device mode)
    #[inline(always)]
    pub const fn fs_grxstsr_device(&self) -> &FS_GRXSTSR_DEVICE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
    #[inline(always)]
    pub const fn fs_grxfsiz(&self) -> &FS_GRXFSIZ {
        &self.fs_grxfsiz
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)
    #[inline(always)]
    pub const fn fs_gnptxfsiz_host(&self) -> &FS_GNPTXFSIZ_HOST {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)
    #[inline(always)]
    pub const fn fs_gnptxfsiz_device(&self) -> &FS_GNPTXFSIZ_DEVICE {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
    #[inline(always)]
    pub const fn fs_gnptxsts(&self) -> &FS_GNPTXSTS {
        &self.fs_gnptxsts
    }
    ///0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)
    #[inline(always)]
    pub const fn fs_gccfg(&self) -> &FS_GCCFG {
        &self.fs_gccfg
    }
    ///0x3c - core ID register
    #[inline(always)]
    pub const fn fs_cid(&self) -> &FS_CID {
        &self.fs_cid
    }
    ///0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
    #[inline(always)]
    pub const fn fs_hptxfsiz(&self) -> &FS_HPTXFSIZ {
        &self.fs_hptxfsiz
    }
    ///0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
    #[inline(always)]
    pub const fn fs_dieptxf1(&self) -> &FS_DIEPTXF1 {
        &self.fs_dieptxf1
    }
    ///0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
    #[inline(always)]
    pub const fn fs_dieptxf2(&self) -> &FS_DIEPTXF2 {
        &self.fs_dieptxf2
    }
    ///0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
    #[inline(always)]
    pub const fn fs_dieptxf3(&self) -> &FS_DIEPTXF3 {
        &self.fs_dieptxf3
    }
}
/**FS_GOTGCTL (rw) register accessor: OTG_FS control and status register (OTG_FS_GOTGCTL)

You can [`read`](crate::Reg::read) this register and get [`fs_gotgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gotgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GOTGCTL)

For information about available fields see [`mod@fs_gotgctl`] module*/
pub type FS_GOTGCTL = crate::Reg<fs_gotgctl::FS_GOTGCTLrs>;
///OTG_FS control and status register (OTG_FS_GOTGCTL)
pub mod fs_gotgctl;
/**FS_GOTGINT (rw) register accessor: OTG_FS interrupt register (OTG_FS_GOTGINT)

You can [`read`](crate::Reg::read) this register and get [`fs_gotgint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gotgint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GOTGINT)

For information about available fields see [`mod@fs_gotgint`] module*/
pub type FS_GOTGINT = crate::Reg<fs_gotgint::FS_GOTGINTrs>;
///OTG_FS interrupt register (OTG_FS_GOTGINT)
pub mod fs_gotgint;
/**FS_GAHBCFG (rw) register accessor: OTG_FS AHB configuration register (OTG_FS_GAHBCFG)

You can [`read`](crate::Reg::read) this register and get [`fs_gahbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gahbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GAHBCFG)

For information about available fields see [`mod@fs_gahbcfg`] module*/
pub type FS_GAHBCFG = crate::Reg<fs_gahbcfg::FS_GAHBCFGrs>;
///OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
pub mod fs_gahbcfg;
/**FS_GUSBCFG (rw) register accessor: OTG_FS USB configuration register (OTG_FS_GUSBCFG)

You can [`read`](crate::Reg::read) this register and get [`fs_gusbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gusbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GUSBCFG)

For information about available fields see [`mod@fs_gusbcfg`] module*/
pub type FS_GUSBCFG = crate::Reg<fs_gusbcfg::FS_GUSBCFGrs>;
///OTG_FS USB configuration register (OTG_FS_GUSBCFG)
pub mod fs_gusbcfg;
/**FS_GRSTCTL (rw) register accessor: OTG_FS reset register (OTG_FS_GRSTCTL)

You can [`read`](crate::Reg::read) this register and get [`fs_grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GRSTCTL)

For information about available fields see [`mod@fs_grstctl`] module*/
pub type FS_GRSTCTL = crate::Reg<fs_grstctl::FS_GRSTCTLrs>;
///OTG_FS reset register (OTG_FS_GRSTCTL)
pub mod fs_grstctl;
/**FS_GINTSTS (rw) register accessor: OTG_FS core interrupt register (OTG_FS_GINTSTS)

You can [`read`](crate::Reg::read) this register and get [`fs_gintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GINTSTS)

For information about available fields see [`mod@fs_gintsts`] module*/
pub type FS_GINTSTS = crate::Reg<fs_gintsts::FS_GINTSTSrs>;
///OTG_FS core interrupt register (OTG_FS_GINTSTS)
pub mod fs_gintsts;
/**FS_GINTMSK (rw) register accessor: OTG_FS interrupt mask register (OTG_FS_GINTMSK)

You can [`read`](crate::Reg::read) this register and get [`fs_gintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GINTMSK)

For information about available fields see [`mod@fs_gintmsk`] module*/
pub type FS_GINTMSK = crate::Reg<fs_gintmsk::FS_GINTMSKrs>;
///OTG_FS interrupt mask register (OTG_FS_GINTMSK)
pub mod fs_gintmsk;
/**FS_GRXSTSR_Device (r) register accessor: OTG_FS Receive status debug read(Device mode)

You can [`read`](crate::Reg::read) this register and get [`fs_grxstsr_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GRXSTSR_Device)

For information about available fields see [`mod@fs_grxstsr_device`] module*/
#[doc(alias = "FS_GRXSTSR_Device")]
pub type FS_GRXSTSR_DEVICE = crate::Reg<fs_grxstsr_device::FS_GRXSTSR_DEVICErs>;
///OTG_FS Receive status debug read(Device mode)
pub mod fs_grxstsr_device;
/**FS_GRXSTSR_Host (r) register accessor: OTG_FS Receive status debug read(Hostmode)

You can [`read`](crate::Reg::read) this register and get [`fs_grxstsr_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GRXSTSR_Host)

For information about available fields see [`mod@fs_grxstsr_host`] module*/
#[doc(alias = "FS_GRXSTSR_Host")]
pub type FS_GRXSTSR_HOST = crate::Reg<fs_grxstsr_host::FS_GRXSTSR_HOSTrs>;
///OTG_FS Receive status debug read(Hostmode)
pub mod fs_grxstsr_host;
/**FS_GRXFSIZ (rw) register accessor: OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)

You can [`read`](crate::Reg::read) this register and get [`fs_grxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_grxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GRXFSIZ)

For information about available fields see [`mod@fs_grxfsiz`] module*/
pub type FS_GRXFSIZ = crate::Reg<fs_grxfsiz::FS_GRXFSIZrs>;
///OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
pub mod fs_grxfsiz;
/**FS_GNPTXFSIZ_Device (rw) register accessor: OTG_FS non-periodic transmit FIFO size register (Device mode)

You can [`read`](crate::Reg::read) this register and get [`fs_gnptxfsiz_device::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gnptxfsiz_device::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GNPTXFSIZ_Device)

For information about available fields see [`mod@fs_gnptxfsiz_device`] module*/
#[doc(alias = "FS_GNPTXFSIZ_Device")]
pub type FS_GNPTXFSIZ_DEVICE = crate::Reg<fs_gnptxfsiz_device::FS_GNPTXFSIZ_DEVICErs>;
///OTG_FS non-periodic transmit FIFO size register (Device mode)
pub mod fs_gnptxfsiz_device;
/**FS_GNPTXFSIZ_Host (rw) register accessor: OTG_FS non-periodic transmit FIFO size register (Host mode)

You can [`read`](crate::Reg::read) this register and get [`fs_gnptxfsiz_host::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gnptxfsiz_host::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GNPTXFSIZ_Host)

For information about available fields see [`mod@fs_gnptxfsiz_host`] module*/
#[doc(alias = "FS_GNPTXFSIZ_Host")]
pub type FS_GNPTXFSIZ_HOST = crate::Reg<fs_gnptxfsiz_host::FS_GNPTXFSIZ_HOSTrs>;
///OTG_FS non-periodic transmit FIFO size register (Host mode)
pub mod fs_gnptxfsiz_host;
/**FS_GNPTXSTS (r) register accessor: OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)

You can [`read`](crate::Reg::read) this register and get [`fs_gnptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GNPTXSTS)

For information about available fields see [`mod@fs_gnptxsts`] module*/
pub type FS_GNPTXSTS = crate::Reg<fs_gnptxsts::FS_GNPTXSTSrs>;
///OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
pub mod fs_gnptxsts;
/**FS_GCCFG (rw) register accessor: OTG_FS general core configuration register (OTG_FS_GCCFG)

You can [`read`](crate::Reg::read) this register and get [`fs_gccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_GCCFG)

For information about available fields see [`mod@fs_gccfg`] module*/
pub type FS_GCCFG = crate::Reg<fs_gccfg::FS_GCCFGrs>;
///OTG_FS general core configuration register (OTG_FS_GCCFG)
pub mod fs_gccfg;
/**FS_CID (rw) register accessor: core ID register

You can [`read`](crate::Reg::read) this register and get [`fs_cid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_cid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_CID)

For information about available fields see [`mod@fs_cid`] module*/
pub type FS_CID = crate::Reg<fs_cid::FS_CIDrs>;
///core ID register
pub mod fs_cid;
/**FS_HPTXFSIZ (rw) register accessor: OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)

You can [`read`](crate::Reg::read) this register and get [`fs_hptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_HPTXFSIZ)

For information about available fields see [`mod@fs_hptxfsiz`] module*/
pub type FS_HPTXFSIZ = crate::Reg<fs_hptxfsiz::FS_HPTXFSIZrs>;
///OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
pub mod fs_hptxfsiz;
/**FS_DIEPTXF1 (rw) register accessor: OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)

You can [`read`](crate::Reg::read) this register and get [`fs_dieptxf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_dieptxf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_DIEPTXF1)

For information about available fields see [`mod@fs_dieptxf1`] module*/
pub type FS_DIEPTXF1 = crate::Reg<fs_dieptxf1::FS_DIEPTXF1rs>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
pub mod fs_dieptxf1;
/**FS_DIEPTXF2 (rw) register accessor: OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)

You can [`read`](crate::Reg::read) this register and get [`fs_dieptxf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_dieptxf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_DIEPTXF2)

For information about available fields see [`mod@fs_dieptxf2`] module*/
pub type FS_DIEPTXF2 = crate::Reg<fs_dieptxf2::FS_DIEPTXF2rs>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
pub mod fs_dieptxf2;
/**FS_DIEPTXF3 (rw) register accessor: OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)

You can [`read`](crate::Reg::read) this register and get [`fs_dieptxf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_dieptxf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#OTG_FS_GLOBAL:FS_DIEPTXF3)

For information about available fields see [`mod@fs_dieptxf3`] module*/
pub type FS_DIEPTXF3 = crate::Reg<fs_dieptxf3::FS_DIEPTXF3rs>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
pub mod fs_dieptxf3;
