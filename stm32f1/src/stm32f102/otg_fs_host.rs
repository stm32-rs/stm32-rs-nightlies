#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS host configuration register (OTG_FS_HCFG)"]
    pub fs_hcfg: FS_HCFG,
    #[doc = "0x04 - OTG_FS Host frame interval register"]
    pub hfir: HFIR,
    #[doc = "0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
    pub fs_hfnum: FS_HFNUM,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
    pub fs_hptxsts: FS_HPTXSTS,
    #[doc = "0x14 - OTG_FS Host all channels interrupt register"]
    pub haint: HAINT,
    #[doc = "0x18 - OTG_FS host all channels interrupt mask register"]
    pub haintmsk: HAINTMSK,
    _reserved6: [u8; 36usize],
    #[doc = "0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)"]
    pub fs_hprt: FS_HPRT,
    _reserved7: [u8; 188usize],
    #[doc = "0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
    pub fs_hcchar0: FS_HCCHAR0,
    _reserved8: [u8; 4usize],
    #[doc = "0x108 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
    pub fs_hcint0: FS_HCINT0,
    #[doc = "0x10c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
    pub fs_hcintmsk0: FS_HCINTMSK0,
    #[doc = "0x110 - OTG_FS host channel-0 transfer size register"]
    pub fs_hctsiz0: FS_HCTSIZ0,
    _reserved11: [u8; 12usize],
    #[doc = "0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
    pub fs_hcchar1: FS_HCCHAR1,
    _reserved12: [u8; 4usize],
    #[doc = "0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
    pub fs_hcint1: FS_HCINT1,
    #[doc = "0x12c - OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
    pub fs_hcintmsk1: FS_HCINTMSK1,
    #[doc = "0x130 - OTG_FS host channel-1 transfer size register"]
    pub fs_hctsiz1: FS_HCTSIZ1,
    _reserved15: [u8; 12usize],
    #[doc = "0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
    pub fs_hcchar2: FS_HCCHAR2,
    _reserved16: [u8; 4usize],
    #[doc = "0x148 - OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
    pub fs_hcint2: FS_HCINT2,
    #[doc = "0x14c - OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
    pub fs_hcintmsk2: FS_HCINTMSK2,
    #[doc = "0x150 - OTG_FS host channel-2 transfer size register"]
    pub fs_hctsiz2: FS_HCTSIZ2,
    _reserved19: [u8; 12usize],
    #[doc = "0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
    pub fs_hcchar3: FS_HCCHAR3,
    _reserved20: [u8; 4usize],
    #[doc = "0x168 - OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
    pub fs_hcint3: FS_HCINT3,
    #[doc = "0x16c - OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
    pub fs_hcintmsk3: FS_HCINTMSK3,
    #[doc = "0x170 - OTG_FS host channel-3 transfer size register"]
    pub fs_hctsiz3: FS_HCTSIZ3,
    _reserved23: [u8; 12usize],
    #[doc = "0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
    pub fs_hcchar4: FS_HCCHAR4,
    _reserved24: [u8; 4usize],
    #[doc = "0x188 - OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
    pub fs_hcint4: FS_HCINT4,
    #[doc = "0x18c - OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
    pub fs_hcintmsk4: FS_HCINTMSK4,
    #[doc = "0x190 - OTG_FS host channel-x transfer size register"]
    pub fs_hctsiz4: FS_HCTSIZ4,
    _reserved27: [u8; 12usize],
    #[doc = "0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
    pub fs_hcchar5: FS_HCCHAR5,
    _reserved28: [u8; 4usize],
    #[doc = "0x1a8 - OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
    pub fs_hcint5: FS_HCINT5,
    #[doc = "0x1ac - OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
    pub fs_hcintmsk5: FS_HCINTMSK5,
    #[doc = "0x1b0 - OTG_FS host channel-5 transfer size register"]
    pub fs_hctsiz5: FS_HCTSIZ5,
    _reserved31: [u8; 12usize],
    #[doc = "0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
    pub fs_hcchar6: FS_HCCHAR6,
    _reserved32: [u8; 4usize],
    #[doc = "0x1c8 - OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
    pub fs_hcint6: FS_HCINT6,
    #[doc = "0x1cc - OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
    pub fs_hcintmsk6: FS_HCINTMSK6,
    #[doc = "0x1d0 - OTG_FS host channel-6 transfer size register"]
    pub fs_hctsiz6: FS_HCTSIZ6,
    _reserved35: [u8; 12usize],
    #[doc = "0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
    pub fs_hcchar7: FS_HCCHAR7,
    _reserved36: [u8; 4usize],
    #[doc = "0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
    pub fs_hcint7: FS_HCINT7,
    #[doc = "0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
    pub fs_hcintmsk7: FS_HCINTMSK7,
    #[doc = "0x1f0 - OTG_FS host channel-7 transfer size register"]
    pub fs_hctsiz7: FS_HCTSIZ7,
}
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcfg](fs_hcfg) module"]
pub type FS_HCFG = crate::Reg<u32, _FS_HCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCFG;
#[doc = "`read()` method returns [fs_hcfg::R](fs_hcfg::R) reader structure"]
impl crate::Readable for FS_HCFG {}
#[doc = "`write(|w| ..)` method takes [fs_hcfg::W](fs_hcfg::W) writer structure"]
impl crate::Writable for FS_HCFG {}
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)"]
pub mod fs_hcfg;
#[doc = "OTG_FS Host frame interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfir](hfir) module"]
pub type HFIR = crate::Reg<u32, _HFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFIR;
#[doc = "`read()` method returns [hfir::R](hfir::R) reader structure"]
impl crate::Readable for HFIR {}
#[doc = "`write(|w| ..)` method takes [hfir::W](hfir::W) writer structure"]
impl crate::Writable for HFIR {}
#[doc = "OTG_FS Host frame interval register"]
pub mod hfir;
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hfnum](fs_hfnum) module"]
pub type FS_HFNUM = crate::Reg<u32, _FS_HFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HFNUM;
#[doc = "`read()` method returns [fs_hfnum::R](fs_hfnum::R) reader structure"]
impl crate::Readable for FS_HFNUM {}
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
pub mod fs_hfnum;
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hptxsts](fs_hptxsts) module"]
pub type FS_HPTXSTS = crate::Reg<u32, _FS_HPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HPTXSTS;
#[doc = "`read()` method returns [fs_hptxsts::R](fs_hptxsts::R) reader structure"]
impl crate::Readable for FS_HPTXSTS {}
#[doc = "`write(|w| ..)` method takes [fs_hptxsts::W](fs_hptxsts::W) writer structure"]
impl crate::Writable for FS_HPTXSTS {}
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
pub mod fs_hptxsts;
#[doc = "OTG_FS Host all channels interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haint](haint) module"]
pub type HAINT = crate::Reg<u32, _HAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HAINT;
#[doc = "`read()` method returns [haint::R](haint::R) reader structure"]
impl crate::Readable for HAINT {}
#[doc = "OTG_FS Host all channels interrupt register"]
pub mod haint;
#[doc = "OTG_FS host all channels interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haintmsk](haintmsk) module"]
pub type HAINTMSK = crate::Reg<u32, _HAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HAINTMSK;
#[doc = "`read()` method returns [haintmsk::R](haintmsk::R) reader structure"]
impl crate::Readable for HAINTMSK {}
#[doc = "`write(|w| ..)` method takes [haintmsk::W](haintmsk::W) writer structure"]
impl crate::Writable for HAINTMSK {}
#[doc = "OTG_FS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "OTG_FS host port control and status register (OTG_FS_HPRT)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hprt](fs_hprt) module"]
pub type FS_HPRT = crate::Reg<u32, _FS_HPRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HPRT;
#[doc = "`read()` method returns [fs_hprt::R](fs_hprt::R) reader structure"]
impl crate::Readable for FS_HPRT {}
#[doc = "`write(|w| ..)` method takes [fs_hprt::W](fs_hprt::W) writer structure"]
impl crate::Writable for FS_HPRT {}
#[doc = "OTG_FS host port control and status register (OTG_FS_HPRT)"]
pub mod fs_hprt;
#[doc = "OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcchar0](fs_hcchar0) module"]
pub type FS_HCCHAR0 = crate::Reg<u32, _FS_HCCHAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCCHAR0;
#[doc = "`read()` method returns [fs_hcchar0::R](fs_hcchar0::R) reader structure"]
impl crate::Readable for FS_HCCHAR0 {}
#[doc = "`write(|w| ..)` method takes [fs_hcchar0::W](fs_hcchar0::W) writer structure"]
impl crate::Writable for FS_HCCHAR0 {}
#[doc = "OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
pub mod fs_hcchar0;
#[doc = "OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcchar1](fs_hcchar1) module"]
pub type FS_HCCHAR1 = crate::Reg<u32, _FS_HCCHAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCCHAR1;
#[doc = "`read()` method returns [fs_hcchar1::R](fs_hcchar1::R) reader structure"]
impl crate::Readable for FS_HCCHAR1 {}
#[doc = "`write(|w| ..)` method takes [fs_hcchar1::W](fs_hcchar1::W) writer structure"]
impl crate::Writable for FS_HCCHAR1 {}
#[doc = "OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
pub mod fs_hcchar1;
#[doc = "OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcchar2](fs_hcchar2) module"]
pub type FS_HCCHAR2 = crate::Reg<u32, _FS_HCCHAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCCHAR2;
#[doc = "`read()` method returns [fs_hcchar2::R](fs_hcchar2::R) reader structure"]
impl crate::Readable for FS_HCCHAR2 {}
#[doc = "`write(|w| ..)` method takes [fs_hcchar2::W](fs_hcchar2::W) writer structure"]
impl crate::Writable for FS_HCCHAR2 {}
#[doc = "OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
pub mod fs_hcchar2;
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcchar3](fs_hcchar3) module"]
pub type FS_HCCHAR3 = crate::Reg<u32, _FS_HCCHAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCCHAR3;
#[doc = "`read()` method returns [fs_hcchar3::R](fs_hcchar3::R) reader structure"]
impl crate::Readable for FS_HCCHAR3 {}
#[doc = "`write(|w| ..)` method takes [fs_hcchar3::W](fs_hcchar3::W) writer structure"]
impl crate::Writable for FS_HCCHAR3 {}
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
pub mod fs_hcchar3;
#[doc = "OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcchar4](fs_hcchar4) module"]
pub type FS_HCCHAR4 = crate::Reg<u32, _FS_HCCHAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCCHAR4;
#[doc = "`read()` method returns [fs_hcchar4::R](fs_hcchar4::R) reader structure"]
impl crate::Readable for FS_HCCHAR4 {}
#[doc = "`write(|w| ..)` method takes [fs_hcchar4::W](fs_hcchar4::W) writer structure"]
impl crate::Writable for FS_HCCHAR4 {}
#[doc = "OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
pub mod fs_hcchar4;
#[doc = "OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcchar5](fs_hcchar5) module"]
pub type FS_HCCHAR5 = crate::Reg<u32, _FS_HCCHAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCCHAR5;
#[doc = "`read()` method returns [fs_hcchar5::R](fs_hcchar5::R) reader structure"]
impl crate::Readable for FS_HCCHAR5 {}
#[doc = "`write(|w| ..)` method takes [fs_hcchar5::W](fs_hcchar5::W) writer structure"]
impl crate::Writable for FS_HCCHAR5 {}
#[doc = "OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
pub mod fs_hcchar5;
#[doc = "OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcchar6](fs_hcchar6) module"]
pub type FS_HCCHAR6 = crate::Reg<u32, _FS_HCCHAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCCHAR6;
#[doc = "`read()` method returns [fs_hcchar6::R](fs_hcchar6::R) reader structure"]
impl crate::Readable for FS_HCCHAR6 {}
#[doc = "`write(|w| ..)` method takes [fs_hcchar6::W](fs_hcchar6::W) writer structure"]
impl crate::Writable for FS_HCCHAR6 {}
#[doc = "OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
pub mod fs_hcchar6;
#[doc = "OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcchar7](fs_hcchar7) module"]
pub type FS_HCCHAR7 = crate::Reg<u32, _FS_HCCHAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCCHAR7;
#[doc = "`read()` method returns [fs_hcchar7::R](fs_hcchar7::R) reader structure"]
impl crate::Readable for FS_HCCHAR7 {}
#[doc = "`write(|w| ..)` method takes [fs_hcchar7::W](fs_hcchar7::W) writer structure"]
impl crate::Writable for FS_HCCHAR7 {}
#[doc = "OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
pub mod fs_hcchar7;
#[doc = "OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcint0](fs_hcint0) module"]
pub type FS_HCINT0 = crate::Reg<u32, _FS_HCINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINT0;
#[doc = "`read()` method returns [fs_hcint0::R](fs_hcint0::R) reader structure"]
impl crate::Readable for FS_HCINT0 {}
#[doc = "`write(|w| ..)` method takes [fs_hcint0::W](fs_hcint0::W) writer structure"]
impl crate::Writable for FS_HCINT0 {}
#[doc = "OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
pub mod fs_hcint0;
#[doc = "OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcint1](fs_hcint1) module"]
pub type FS_HCINT1 = crate::Reg<u32, _FS_HCINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINT1;
#[doc = "`read()` method returns [fs_hcint1::R](fs_hcint1::R) reader structure"]
impl crate::Readable for FS_HCINT1 {}
#[doc = "`write(|w| ..)` method takes [fs_hcint1::W](fs_hcint1::W) writer structure"]
impl crate::Writable for FS_HCINT1 {}
#[doc = "OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
pub mod fs_hcint1;
#[doc = "OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcint2](fs_hcint2) module"]
pub type FS_HCINT2 = crate::Reg<u32, _FS_HCINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINT2;
#[doc = "`read()` method returns [fs_hcint2::R](fs_hcint2::R) reader structure"]
impl crate::Readable for FS_HCINT2 {}
#[doc = "`write(|w| ..)` method takes [fs_hcint2::W](fs_hcint2::W) writer structure"]
impl crate::Writable for FS_HCINT2 {}
#[doc = "OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
pub mod fs_hcint2;
#[doc = "OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcint3](fs_hcint3) module"]
pub type FS_HCINT3 = crate::Reg<u32, _FS_HCINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINT3;
#[doc = "`read()` method returns [fs_hcint3::R](fs_hcint3::R) reader structure"]
impl crate::Readable for FS_HCINT3 {}
#[doc = "`write(|w| ..)` method takes [fs_hcint3::W](fs_hcint3::W) writer structure"]
impl crate::Writable for FS_HCINT3 {}
#[doc = "OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
pub mod fs_hcint3;
#[doc = "OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcint4](fs_hcint4) module"]
pub type FS_HCINT4 = crate::Reg<u32, _FS_HCINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINT4;
#[doc = "`read()` method returns [fs_hcint4::R](fs_hcint4::R) reader structure"]
impl crate::Readable for FS_HCINT4 {}
#[doc = "`write(|w| ..)` method takes [fs_hcint4::W](fs_hcint4::W) writer structure"]
impl crate::Writable for FS_HCINT4 {}
#[doc = "OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
pub mod fs_hcint4;
#[doc = "OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcint5](fs_hcint5) module"]
pub type FS_HCINT5 = crate::Reg<u32, _FS_HCINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINT5;
#[doc = "`read()` method returns [fs_hcint5::R](fs_hcint5::R) reader structure"]
impl crate::Readable for FS_HCINT5 {}
#[doc = "`write(|w| ..)` method takes [fs_hcint5::W](fs_hcint5::W) writer structure"]
impl crate::Writable for FS_HCINT5 {}
#[doc = "OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
pub mod fs_hcint5;
#[doc = "OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcint6](fs_hcint6) module"]
pub type FS_HCINT6 = crate::Reg<u32, _FS_HCINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINT6;
#[doc = "`read()` method returns [fs_hcint6::R](fs_hcint6::R) reader structure"]
impl crate::Readable for FS_HCINT6 {}
#[doc = "`write(|w| ..)` method takes [fs_hcint6::W](fs_hcint6::W) writer structure"]
impl crate::Writable for FS_HCINT6 {}
#[doc = "OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
pub mod fs_hcint6;
#[doc = "OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcint7](fs_hcint7) module"]
pub type FS_HCINT7 = crate::Reg<u32, _FS_HCINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINT7;
#[doc = "`read()` method returns [fs_hcint7::R](fs_hcint7::R) reader structure"]
impl crate::Readable for FS_HCINT7 {}
#[doc = "`write(|w| ..)` method takes [fs_hcint7::W](fs_hcint7::W) writer structure"]
impl crate::Writable for FS_HCINT7 {}
#[doc = "OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
pub mod fs_hcint7;
#[doc = "OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcintmsk0](fs_hcintmsk0) module"]
pub type FS_HCINTMSK0 = crate::Reg<u32, _FS_HCINTMSK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINTMSK0;
#[doc = "`read()` method returns [fs_hcintmsk0::R](fs_hcintmsk0::R) reader structure"]
impl crate::Readable for FS_HCINTMSK0 {}
#[doc = "`write(|w| ..)` method takes [fs_hcintmsk0::W](fs_hcintmsk0::W) writer structure"]
impl crate::Writable for FS_HCINTMSK0 {}
#[doc = "OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
pub mod fs_hcintmsk0;
#[doc = "OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcintmsk1](fs_hcintmsk1) module"]
pub type FS_HCINTMSK1 = crate::Reg<u32, _FS_HCINTMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINTMSK1;
#[doc = "`read()` method returns [fs_hcintmsk1::R](fs_hcintmsk1::R) reader structure"]
impl crate::Readable for FS_HCINTMSK1 {}
#[doc = "`write(|w| ..)` method takes [fs_hcintmsk1::W](fs_hcintmsk1::W) writer structure"]
impl crate::Writable for FS_HCINTMSK1 {}
#[doc = "OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
pub mod fs_hcintmsk1;
#[doc = "OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcintmsk2](fs_hcintmsk2) module"]
pub type FS_HCINTMSK2 = crate::Reg<u32, _FS_HCINTMSK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINTMSK2;
#[doc = "`read()` method returns [fs_hcintmsk2::R](fs_hcintmsk2::R) reader structure"]
impl crate::Readable for FS_HCINTMSK2 {}
#[doc = "`write(|w| ..)` method takes [fs_hcintmsk2::W](fs_hcintmsk2::W) writer structure"]
impl crate::Writable for FS_HCINTMSK2 {}
#[doc = "OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
pub mod fs_hcintmsk2;
#[doc = "OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcintmsk3](fs_hcintmsk3) module"]
pub type FS_HCINTMSK3 = crate::Reg<u32, _FS_HCINTMSK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINTMSK3;
#[doc = "`read()` method returns [fs_hcintmsk3::R](fs_hcintmsk3::R) reader structure"]
impl crate::Readable for FS_HCINTMSK3 {}
#[doc = "`write(|w| ..)` method takes [fs_hcintmsk3::W](fs_hcintmsk3::W) writer structure"]
impl crate::Writable for FS_HCINTMSK3 {}
#[doc = "OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
pub mod fs_hcintmsk3;
#[doc = "OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcintmsk4](fs_hcintmsk4) module"]
pub type FS_HCINTMSK4 = crate::Reg<u32, _FS_HCINTMSK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINTMSK4;
#[doc = "`read()` method returns [fs_hcintmsk4::R](fs_hcintmsk4::R) reader structure"]
impl crate::Readable for FS_HCINTMSK4 {}
#[doc = "`write(|w| ..)` method takes [fs_hcintmsk4::W](fs_hcintmsk4::W) writer structure"]
impl crate::Writable for FS_HCINTMSK4 {}
#[doc = "OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
pub mod fs_hcintmsk4;
#[doc = "OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcintmsk5](fs_hcintmsk5) module"]
pub type FS_HCINTMSK5 = crate::Reg<u32, _FS_HCINTMSK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINTMSK5;
#[doc = "`read()` method returns [fs_hcintmsk5::R](fs_hcintmsk5::R) reader structure"]
impl crate::Readable for FS_HCINTMSK5 {}
#[doc = "`write(|w| ..)` method takes [fs_hcintmsk5::W](fs_hcintmsk5::W) writer structure"]
impl crate::Writable for FS_HCINTMSK5 {}
#[doc = "OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
pub mod fs_hcintmsk5;
#[doc = "OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcintmsk6](fs_hcintmsk6) module"]
pub type FS_HCINTMSK6 = crate::Reg<u32, _FS_HCINTMSK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINTMSK6;
#[doc = "`read()` method returns [fs_hcintmsk6::R](fs_hcintmsk6::R) reader structure"]
impl crate::Readable for FS_HCINTMSK6 {}
#[doc = "`write(|w| ..)` method takes [fs_hcintmsk6::W](fs_hcintmsk6::W) writer structure"]
impl crate::Writable for FS_HCINTMSK6 {}
#[doc = "OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
pub mod fs_hcintmsk6;
#[doc = "OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcintmsk7](fs_hcintmsk7) module"]
pub type FS_HCINTMSK7 = crate::Reg<u32, _FS_HCINTMSK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCINTMSK7;
#[doc = "`read()` method returns [fs_hcintmsk7::R](fs_hcintmsk7::R) reader structure"]
impl crate::Readable for FS_HCINTMSK7 {}
#[doc = "`write(|w| ..)` method takes [fs_hcintmsk7::W](fs_hcintmsk7::W) writer structure"]
impl crate::Writable for FS_HCINTMSK7 {}
#[doc = "OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
pub mod fs_hcintmsk7;
#[doc = "OTG_FS host channel-0 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hctsiz0](fs_hctsiz0) module"]
pub type FS_HCTSIZ0 = crate::Reg<u32, _FS_HCTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCTSIZ0;
#[doc = "`read()` method returns [fs_hctsiz0::R](fs_hctsiz0::R) reader structure"]
impl crate::Readable for FS_HCTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [fs_hctsiz0::W](fs_hctsiz0::W) writer structure"]
impl crate::Writable for FS_HCTSIZ0 {}
#[doc = "OTG_FS host channel-0 transfer size register"]
pub mod fs_hctsiz0;
#[doc = "OTG_FS host channel-1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hctsiz1](fs_hctsiz1) module"]
pub type FS_HCTSIZ1 = crate::Reg<u32, _FS_HCTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCTSIZ1;
#[doc = "`read()` method returns [fs_hctsiz1::R](fs_hctsiz1::R) reader structure"]
impl crate::Readable for FS_HCTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [fs_hctsiz1::W](fs_hctsiz1::W) writer structure"]
impl crate::Writable for FS_HCTSIZ1 {}
#[doc = "OTG_FS host channel-1 transfer size register"]
pub mod fs_hctsiz1;
#[doc = "OTG_FS host channel-2 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hctsiz2](fs_hctsiz2) module"]
pub type FS_HCTSIZ2 = crate::Reg<u32, _FS_HCTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCTSIZ2;
#[doc = "`read()` method returns [fs_hctsiz2::R](fs_hctsiz2::R) reader structure"]
impl crate::Readable for FS_HCTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [fs_hctsiz2::W](fs_hctsiz2::W) writer structure"]
impl crate::Writable for FS_HCTSIZ2 {}
#[doc = "OTG_FS host channel-2 transfer size register"]
pub mod fs_hctsiz2;
#[doc = "OTG_FS host channel-3 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hctsiz3](fs_hctsiz3) module"]
pub type FS_HCTSIZ3 = crate::Reg<u32, _FS_HCTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCTSIZ3;
#[doc = "`read()` method returns [fs_hctsiz3::R](fs_hctsiz3::R) reader structure"]
impl crate::Readable for FS_HCTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [fs_hctsiz3::W](fs_hctsiz3::W) writer structure"]
impl crate::Writable for FS_HCTSIZ3 {}
#[doc = "OTG_FS host channel-3 transfer size register"]
pub mod fs_hctsiz3;
#[doc = "OTG_FS host channel-x transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hctsiz4](fs_hctsiz4) module"]
pub type FS_HCTSIZ4 = crate::Reg<u32, _FS_HCTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCTSIZ4;
#[doc = "`read()` method returns [fs_hctsiz4::R](fs_hctsiz4::R) reader structure"]
impl crate::Readable for FS_HCTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [fs_hctsiz4::W](fs_hctsiz4::W) writer structure"]
impl crate::Writable for FS_HCTSIZ4 {}
#[doc = "OTG_FS host channel-x transfer size register"]
pub mod fs_hctsiz4;
#[doc = "OTG_FS host channel-5 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hctsiz5](fs_hctsiz5) module"]
pub type FS_HCTSIZ5 = crate::Reg<u32, _FS_HCTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCTSIZ5;
#[doc = "`read()` method returns [fs_hctsiz5::R](fs_hctsiz5::R) reader structure"]
impl crate::Readable for FS_HCTSIZ5 {}
#[doc = "`write(|w| ..)` method takes [fs_hctsiz5::W](fs_hctsiz5::W) writer structure"]
impl crate::Writable for FS_HCTSIZ5 {}
#[doc = "OTG_FS host channel-5 transfer size register"]
pub mod fs_hctsiz5;
#[doc = "OTG_FS host channel-6 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hctsiz6](fs_hctsiz6) module"]
pub type FS_HCTSIZ6 = crate::Reg<u32, _FS_HCTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCTSIZ6;
#[doc = "`read()` method returns [fs_hctsiz6::R](fs_hctsiz6::R) reader structure"]
impl crate::Readable for FS_HCTSIZ6 {}
#[doc = "`write(|w| ..)` method takes [fs_hctsiz6::W](fs_hctsiz6::W) writer structure"]
impl crate::Writable for FS_HCTSIZ6 {}
#[doc = "OTG_FS host channel-6 transfer size register"]
pub mod fs_hctsiz6;
#[doc = "OTG_FS host channel-7 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hctsiz7](fs_hctsiz7) module"]
pub type FS_HCTSIZ7 = crate::Reg<u32, _FS_HCTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HCTSIZ7;
#[doc = "`read()` method returns [fs_hctsiz7::R](fs_hctsiz7::R) reader structure"]
impl crate::Readable for FS_HCTSIZ7 {}
#[doc = "`write(|w| ..)` method takes [fs_hctsiz7::W](fs_hctsiz7::W) writer structure"]
impl crate::Writable for FS_HCTSIZ7 {}
#[doc = "OTG_FS host channel-7 transfer size register"]
pub mod fs_hctsiz7;
