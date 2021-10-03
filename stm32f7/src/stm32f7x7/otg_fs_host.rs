#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS host configuration register (OTG_FS_HCFG)"]
    pub otg_fs_hcfg: crate::Reg<otg_fs_hcfg::OTG_FS_HCFG_SPEC>,
    #[doc = "0x04 - OTG_FS Host frame interval register"]
    pub otg_fs_hfir: crate::Reg<otg_fs_hfir::OTG_FS_HFIR_SPEC>,
    #[doc = "0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
    pub otg_fs_hfnum: crate::Reg<otg_fs_hfnum::OTG_FS_HFNUM_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
    pub otg_fs_hptxsts: crate::Reg<otg_fs_hptxsts::OTG_FS_HPTXSTS_SPEC>,
    #[doc = "0x14 - OTG_FS Host all channels interrupt register"]
    pub otg_fs_haint: crate::Reg<otg_fs_haint::OTG_FS_HAINT_SPEC>,
    #[doc = "0x18 - OTG_FS host all channels interrupt mask register"]
    pub otg_fs_haintmsk: crate::Reg<otg_fs_haintmsk::OTG_FS_HAINTMSK_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)"]
    pub otg_fs_hprt: crate::Reg<otg_fs_hprt::OTG_FS_HPRT_SPEC>,
    _reserved7: [u8; 0xbc],
    #[doc = "0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
    pub otg_fs_hcchar0: crate::Reg<otg_fs_hcchar0::OTG_FS_HCCHAR0_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x108 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
    pub otg_fs_hcint0: crate::Reg<otg_fs_hcint0::OTG_FS_HCINT0_SPEC>,
    #[doc = "0x10c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
    pub otg_fs_hcintmsk0: crate::Reg<otg_fs_hcintmsk0::OTG_FS_HCINTMSK0_SPEC>,
    #[doc = "0x110 - OTG_FS host channel-0 transfer size register"]
    pub otg_fs_hctsiz0: crate::Reg<otg_fs_hctsiz0::OTG_FS_HCTSIZ0_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
    pub otg_fs_hcchar1: crate::Reg<otg_fs_hcchar1::OTG_FS_HCCHAR1_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
    pub otg_fs_hcint1: crate::Reg<otg_fs_hcint1::OTG_FS_HCINT1_SPEC>,
    #[doc = "0x12c - OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
    pub otg_fs_hcintmsk1: crate::Reg<otg_fs_hcintmsk1::OTG_FS_HCINTMSK1_SPEC>,
    #[doc = "0x130 - OTG_FS host channel-1 transfer size register"]
    pub otg_fs_hctsiz1: crate::Reg<otg_fs_hctsiz1::OTG_FS_HCTSIZ1_SPEC>,
    _reserved15: [u8; 0x0c],
    #[doc = "0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
    pub otg_fs_hcchar2: crate::Reg<otg_fs_hcchar2::OTG_FS_HCCHAR2_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x148 - OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
    pub otg_fs_hcint2: crate::Reg<otg_fs_hcint2::OTG_FS_HCINT2_SPEC>,
    #[doc = "0x14c - OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
    pub otg_fs_hcintmsk2: crate::Reg<otg_fs_hcintmsk2::OTG_FS_HCINTMSK2_SPEC>,
    #[doc = "0x150 - OTG_FS host channel-2 transfer size register"]
    pub otg_fs_hctsiz2: crate::Reg<otg_fs_hctsiz2::OTG_FS_HCTSIZ2_SPEC>,
    _reserved19: [u8; 0x0c],
    #[doc = "0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
    pub otg_fs_hcchar3: crate::Reg<otg_fs_hcchar3::OTG_FS_HCCHAR3_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x168 - OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
    pub otg_fs_hcint3: crate::Reg<otg_fs_hcint3::OTG_FS_HCINT3_SPEC>,
    #[doc = "0x16c - OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
    pub otg_fs_hcintmsk3: crate::Reg<otg_fs_hcintmsk3::OTG_FS_HCINTMSK3_SPEC>,
    #[doc = "0x170 - OTG_FS host channel-3 transfer size register"]
    pub otg_fs_hctsiz3: crate::Reg<otg_fs_hctsiz3::OTG_FS_HCTSIZ3_SPEC>,
    _reserved23: [u8; 0x0c],
    #[doc = "0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
    pub otg_fs_hcchar4: crate::Reg<otg_fs_hcchar4::OTG_FS_HCCHAR4_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x188 - OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
    pub otg_fs_hcint4: crate::Reg<otg_fs_hcint4::OTG_FS_HCINT4_SPEC>,
    #[doc = "0x18c - OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
    pub otg_fs_hcintmsk4: crate::Reg<otg_fs_hcintmsk4::OTG_FS_HCINTMSK4_SPEC>,
    #[doc = "0x190 - OTG_FS host channel-x transfer size register"]
    pub otg_fs_hctsiz4: crate::Reg<otg_fs_hctsiz4::OTG_FS_HCTSIZ4_SPEC>,
    _reserved27: [u8; 0x0c],
    #[doc = "0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
    pub otg_fs_hcchar5: crate::Reg<otg_fs_hcchar5::OTG_FS_HCCHAR5_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0x1a8 - OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
    pub otg_fs_hcint5: crate::Reg<otg_fs_hcint5::OTG_FS_HCINT5_SPEC>,
    #[doc = "0x1ac - OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
    pub otg_fs_hcintmsk5: crate::Reg<otg_fs_hcintmsk5::OTG_FS_HCINTMSK5_SPEC>,
    #[doc = "0x1b0 - OTG_FS host channel-5 transfer size register"]
    pub otg_fs_hctsiz5: crate::Reg<otg_fs_hctsiz5::OTG_FS_HCTSIZ5_SPEC>,
    _reserved31: [u8; 0x0c],
    #[doc = "0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
    pub otg_fs_hcchar6: crate::Reg<otg_fs_hcchar6::OTG_FS_HCCHAR6_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x1c8 - OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
    pub otg_fs_hcint6: crate::Reg<otg_fs_hcint6::OTG_FS_HCINT6_SPEC>,
    #[doc = "0x1cc - OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
    pub otg_fs_hcintmsk6: crate::Reg<otg_fs_hcintmsk6::OTG_FS_HCINTMSK6_SPEC>,
    #[doc = "0x1d0 - OTG_FS host channel-6 transfer size register"]
    pub otg_fs_hctsiz6: crate::Reg<otg_fs_hctsiz6::OTG_FS_HCTSIZ6_SPEC>,
    _reserved35: [u8; 0x0c],
    #[doc = "0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
    pub otg_fs_hcchar7: crate::Reg<otg_fs_hcchar7::OTG_FS_HCCHAR7_SPEC>,
    _reserved36: [u8; 0x04],
    #[doc = "0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
    pub otg_fs_hcint7: crate::Reg<otg_fs_hcint7::OTG_FS_HCINT7_SPEC>,
    #[doc = "0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
    pub otg_fs_hcintmsk7: crate::Reg<otg_fs_hcintmsk7::OTG_FS_HCINTMSK7_SPEC>,
    #[doc = "0x1f0 - OTG_FS host channel-7 transfer size register"]
    pub otg_fs_hctsiz7: crate::Reg<otg_fs_hctsiz7::OTG_FS_HCTSIZ7_SPEC>,
    #[doc = "0x1f4 - OTG_FS host channel-8 characteristics register"]
    pub otg_fs_hcchar8: crate::Reg<otg_fs_hcchar8::OTG_FS_HCCHAR8_SPEC>,
    #[doc = "0x1f8 - OTG_FS host channel-8 interrupt register"]
    pub otg_fs_hcint8: crate::Reg<otg_fs_hcint8::OTG_FS_HCINT8_SPEC>,
    #[doc = "0x1fc - OTG_FS host channel-8 mask register"]
    pub otg_fs_hcintmsk8: crate::Reg<otg_fs_hcintmsk8::OTG_FS_HCINTMSK8_SPEC>,
    #[doc = "0x200 - OTG_FS host channel-8 transfer size register"]
    pub otg_fs_hctsiz8: crate::Reg<otg_fs_hctsiz8::OTG_FS_HCTSIZ8_SPEC>,
    #[doc = "0x204 - OTG_FS host channel-9 characteristics register"]
    pub otg_fs_hcchar9: crate::Reg<otg_fs_hcchar9::OTG_FS_HCCHAR9_SPEC>,
    #[doc = "0x208 - OTG_FS host channel-9 interrupt register"]
    pub otg_fs_hcint9: crate::Reg<otg_fs_hcint9::OTG_FS_HCINT9_SPEC>,
    #[doc = "0x20c - OTG_FS host channel-9 mask register"]
    pub otg_fs_hcintmsk9: crate::Reg<otg_fs_hcintmsk9::OTG_FS_HCINTMSK9_SPEC>,
    #[doc = "0x210 - OTG_FS host channel-9 transfer size register"]
    pub otg_fs_hctsiz9: crate::Reg<otg_fs_hctsiz9::OTG_FS_HCTSIZ9_SPEC>,
    #[doc = "0x214 - OTG_FS host channel-10 characteristics register"]
    pub otg_fs_hcchar10: crate::Reg<otg_fs_hcchar10::OTG_FS_HCCHAR10_SPEC>,
    #[doc = "0x218 - OTG_FS host channel-10 interrupt register"]
    pub otg_fs_hcint10: crate::Reg<otg_fs_hcint10::OTG_FS_HCINT10_SPEC>,
    #[doc = "0x21c - OTG_FS host channel-10 mask register"]
    pub otg_fs_hcintmsk10: crate::Reg<otg_fs_hcintmsk10::OTG_FS_HCINTMSK10_SPEC>,
    #[doc = "0x220 - OTG_FS host channel-10 transfer size register"]
    pub otg_fs_hctsiz10: crate::Reg<otg_fs_hctsiz10::OTG_FS_HCTSIZ10_SPEC>,
    #[doc = "0x224 - OTG_FS host channel-11 characteristics register"]
    pub otg_fs_hcchar11: crate::Reg<otg_fs_hcchar11::OTG_FS_HCCHAR11_SPEC>,
    #[doc = "0x228 - OTG_FS host channel-11 interrupt register"]
    pub otg_fs_hcint11: crate::Reg<otg_fs_hcint11::OTG_FS_HCINT11_SPEC>,
    #[doc = "0x22c - OTG_FS host channel-11 mask register"]
    pub otg_fs_hcintmsk11: crate::Reg<otg_fs_hcintmsk11::OTG_FS_HCINTMSK11_SPEC>,
    #[doc = "0x230 - OTG_FS host channel-11 transfer size register"]
    pub otg_fs_hctsiz11: crate::Reg<otg_fs_hctsiz11::OTG_FS_HCTSIZ11_SPEC>,
}
#[doc = "OTG_FS_HCFG register accessor: an alias for `Reg<OTG_FS_HCFG_SPEC>`"]
pub type OTG_FS_HCFG = crate::Reg<otg_fs_hcfg::OTG_FS_HCFG_SPEC>;
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)"]
pub mod otg_fs_hcfg;
#[doc = "OTG_FS_HFIR register accessor: an alias for `Reg<OTG_FS_HFIR_SPEC>`"]
pub type OTG_FS_HFIR = crate::Reg<otg_fs_hfir::OTG_FS_HFIR_SPEC>;
#[doc = "OTG_FS Host frame interval register"]
pub mod otg_fs_hfir;
#[doc = "OTG_FS_HFNUM register accessor: an alias for `Reg<OTG_FS_HFNUM_SPEC>`"]
pub type OTG_FS_HFNUM = crate::Reg<otg_fs_hfnum::OTG_FS_HFNUM_SPEC>;
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
pub mod otg_fs_hfnum;
#[doc = "OTG_FS_HPTXSTS register accessor: an alias for `Reg<OTG_FS_HPTXSTS_SPEC>`"]
pub type OTG_FS_HPTXSTS = crate::Reg<otg_fs_hptxsts::OTG_FS_HPTXSTS_SPEC>;
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
pub mod otg_fs_hptxsts;
#[doc = "OTG_FS_HAINT register accessor: an alias for `Reg<OTG_FS_HAINT_SPEC>`"]
pub type OTG_FS_HAINT = crate::Reg<otg_fs_haint::OTG_FS_HAINT_SPEC>;
#[doc = "OTG_FS Host all channels interrupt register"]
pub mod otg_fs_haint;
#[doc = "OTG_FS_HAINTMSK register accessor: an alias for `Reg<OTG_FS_HAINTMSK_SPEC>`"]
pub type OTG_FS_HAINTMSK = crate::Reg<otg_fs_haintmsk::OTG_FS_HAINTMSK_SPEC>;
#[doc = "OTG_FS host all channels interrupt mask register"]
pub mod otg_fs_haintmsk;
#[doc = "OTG_FS_HPRT register accessor: an alias for `Reg<OTG_FS_HPRT_SPEC>`"]
pub type OTG_FS_HPRT = crate::Reg<otg_fs_hprt::OTG_FS_HPRT_SPEC>;
#[doc = "OTG_FS host port control and status register (OTG_FS_HPRT)"]
pub mod otg_fs_hprt;
#[doc = "OTG_FS_HCCHAR0 register accessor: an alias for `Reg<OTG_FS_HCCHAR0_SPEC>`"]
pub type OTG_FS_HCCHAR0 = crate::Reg<otg_fs_hcchar0::OTG_FS_HCCHAR0_SPEC>;
#[doc = "OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
pub mod otg_fs_hcchar0;
#[doc = "OTG_FS_HCCHAR1 register accessor: an alias for `Reg<OTG_FS_HCCHAR1_SPEC>`"]
pub type OTG_FS_HCCHAR1 = crate::Reg<otg_fs_hcchar1::OTG_FS_HCCHAR1_SPEC>;
#[doc = "OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
pub mod otg_fs_hcchar1;
#[doc = "OTG_FS_HCCHAR2 register accessor: an alias for `Reg<OTG_FS_HCCHAR2_SPEC>`"]
pub type OTG_FS_HCCHAR2 = crate::Reg<otg_fs_hcchar2::OTG_FS_HCCHAR2_SPEC>;
#[doc = "OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
pub mod otg_fs_hcchar2;
#[doc = "OTG_FS_HCCHAR3 register accessor: an alias for `Reg<OTG_FS_HCCHAR3_SPEC>`"]
pub type OTG_FS_HCCHAR3 = crate::Reg<otg_fs_hcchar3::OTG_FS_HCCHAR3_SPEC>;
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
pub mod otg_fs_hcchar3;
#[doc = "OTG_FS_HCCHAR4 register accessor: an alias for `Reg<OTG_FS_HCCHAR4_SPEC>`"]
pub type OTG_FS_HCCHAR4 = crate::Reg<otg_fs_hcchar4::OTG_FS_HCCHAR4_SPEC>;
#[doc = "OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
pub mod otg_fs_hcchar4;
#[doc = "OTG_FS_HCCHAR5 register accessor: an alias for `Reg<OTG_FS_HCCHAR5_SPEC>`"]
pub type OTG_FS_HCCHAR5 = crate::Reg<otg_fs_hcchar5::OTG_FS_HCCHAR5_SPEC>;
#[doc = "OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
pub mod otg_fs_hcchar5;
#[doc = "OTG_FS_HCCHAR6 register accessor: an alias for `Reg<OTG_FS_HCCHAR6_SPEC>`"]
pub type OTG_FS_HCCHAR6 = crate::Reg<otg_fs_hcchar6::OTG_FS_HCCHAR6_SPEC>;
#[doc = "OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
pub mod otg_fs_hcchar6;
#[doc = "OTG_FS_HCCHAR7 register accessor: an alias for `Reg<OTG_FS_HCCHAR7_SPEC>`"]
pub type OTG_FS_HCCHAR7 = crate::Reg<otg_fs_hcchar7::OTG_FS_HCCHAR7_SPEC>;
#[doc = "OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
pub mod otg_fs_hcchar7;
#[doc = "OTG_FS_HCINT0 register accessor: an alias for `Reg<OTG_FS_HCINT0_SPEC>`"]
pub type OTG_FS_HCINT0 = crate::Reg<otg_fs_hcint0::OTG_FS_HCINT0_SPEC>;
#[doc = "OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
pub mod otg_fs_hcint0;
#[doc = "OTG_FS_HCINT1 register accessor: an alias for `Reg<OTG_FS_HCINT1_SPEC>`"]
pub type OTG_FS_HCINT1 = crate::Reg<otg_fs_hcint1::OTG_FS_HCINT1_SPEC>;
#[doc = "OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
pub mod otg_fs_hcint1;
#[doc = "OTG_FS_HCINT2 register accessor: an alias for `Reg<OTG_FS_HCINT2_SPEC>`"]
pub type OTG_FS_HCINT2 = crate::Reg<otg_fs_hcint2::OTG_FS_HCINT2_SPEC>;
#[doc = "OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
pub mod otg_fs_hcint2;
#[doc = "OTG_FS_HCINT3 register accessor: an alias for `Reg<OTG_FS_HCINT3_SPEC>`"]
pub type OTG_FS_HCINT3 = crate::Reg<otg_fs_hcint3::OTG_FS_HCINT3_SPEC>;
#[doc = "OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
pub mod otg_fs_hcint3;
#[doc = "OTG_FS_HCINT4 register accessor: an alias for `Reg<OTG_FS_HCINT4_SPEC>`"]
pub type OTG_FS_HCINT4 = crate::Reg<otg_fs_hcint4::OTG_FS_HCINT4_SPEC>;
#[doc = "OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
pub mod otg_fs_hcint4;
#[doc = "OTG_FS_HCINT5 register accessor: an alias for `Reg<OTG_FS_HCINT5_SPEC>`"]
pub type OTG_FS_HCINT5 = crate::Reg<otg_fs_hcint5::OTG_FS_HCINT5_SPEC>;
#[doc = "OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
pub mod otg_fs_hcint5;
#[doc = "OTG_FS_HCINT6 register accessor: an alias for `Reg<OTG_FS_HCINT6_SPEC>`"]
pub type OTG_FS_HCINT6 = crate::Reg<otg_fs_hcint6::OTG_FS_HCINT6_SPEC>;
#[doc = "OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
pub mod otg_fs_hcint6;
#[doc = "OTG_FS_HCINT7 register accessor: an alias for `Reg<OTG_FS_HCINT7_SPEC>`"]
pub type OTG_FS_HCINT7 = crate::Reg<otg_fs_hcint7::OTG_FS_HCINT7_SPEC>;
#[doc = "OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
pub mod otg_fs_hcint7;
#[doc = "OTG_FS_HCINTMSK0 register accessor: an alias for `Reg<OTG_FS_HCINTMSK0_SPEC>`"]
pub type OTG_FS_HCINTMSK0 = crate::Reg<otg_fs_hcintmsk0::OTG_FS_HCINTMSK0_SPEC>;
#[doc = "OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
pub mod otg_fs_hcintmsk0;
#[doc = "OTG_FS_HCINTMSK1 register accessor: an alias for `Reg<OTG_FS_HCINTMSK1_SPEC>`"]
pub type OTG_FS_HCINTMSK1 = crate::Reg<otg_fs_hcintmsk1::OTG_FS_HCINTMSK1_SPEC>;
#[doc = "OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
pub mod otg_fs_hcintmsk1;
#[doc = "OTG_FS_HCINTMSK2 register accessor: an alias for `Reg<OTG_FS_HCINTMSK2_SPEC>`"]
pub type OTG_FS_HCINTMSK2 = crate::Reg<otg_fs_hcintmsk2::OTG_FS_HCINTMSK2_SPEC>;
#[doc = "OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
pub mod otg_fs_hcintmsk2;
#[doc = "OTG_FS_HCINTMSK3 register accessor: an alias for `Reg<OTG_FS_HCINTMSK3_SPEC>`"]
pub type OTG_FS_HCINTMSK3 = crate::Reg<otg_fs_hcintmsk3::OTG_FS_HCINTMSK3_SPEC>;
#[doc = "OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
pub mod otg_fs_hcintmsk3;
#[doc = "OTG_FS_HCINTMSK4 register accessor: an alias for `Reg<OTG_FS_HCINTMSK4_SPEC>`"]
pub type OTG_FS_HCINTMSK4 = crate::Reg<otg_fs_hcintmsk4::OTG_FS_HCINTMSK4_SPEC>;
#[doc = "OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
pub mod otg_fs_hcintmsk4;
#[doc = "OTG_FS_HCINTMSK5 register accessor: an alias for `Reg<OTG_FS_HCINTMSK5_SPEC>`"]
pub type OTG_FS_HCINTMSK5 = crate::Reg<otg_fs_hcintmsk5::OTG_FS_HCINTMSK5_SPEC>;
#[doc = "OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
pub mod otg_fs_hcintmsk5;
#[doc = "OTG_FS_HCINTMSK6 register accessor: an alias for `Reg<OTG_FS_HCINTMSK6_SPEC>`"]
pub type OTG_FS_HCINTMSK6 = crate::Reg<otg_fs_hcintmsk6::OTG_FS_HCINTMSK6_SPEC>;
#[doc = "OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
pub mod otg_fs_hcintmsk6;
#[doc = "OTG_FS_HCINTMSK7 register accessor: an alias for `Reg<OTG_FS_HCINTMSK7_SPEC>`"]
pub type OTG_FS_HCINTMSK7 = crate::Reg<otg_fs_hcintmsk7::OTG_FS_HCINTMSK7_SPEC>;
#[doc = "OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
pub mod otg_fs_hcintmsk7;
#[doc = "OTG_FS_HCTSIZ0 register accessor: an alias for `Reg<OTG_FS_HCTSIZ0_SPEC>`"]
pub type OTG_FS_HCTSIZ0 = crate::Reg<otg_fs_hctsiz0::OTG_FS_HCTSIZ0_SPEC>;
#[doc = "OTG_FS host channel-0 transfer size register"]
pub mod otg_fs_hctsiz0;
#[doc = "OTG_FS_HCTSIZ1 register accessor: an alias for `Reg<OTG_FS_HCTSIZ1_SPEC>`"]
pub type OTG_FS_HCTSIZ1 = crate::Reg<otg_fs_hctsiz1::OTG_FS_HCTSIZ1_SPEC>;
#[doc = "OTG_FS host channel-1 transfer size register"]
pub mod otg_fs_hctsiz1;
#[doc = "OTG_FS_HCTSIZ2 register accessor: an alias for `Reg<OTG_FS_HCTSIZ2_SPEC>`"]
pub type OTG_FS_HCTSIZ2 = crate::Reg<otg_fs_hctsiz2::OTG_FS_HCTSIZ2_SPEC>;
#[doc = "OTG_FS host channel-2 transfer size register"]
pub mod otg_fs_hctsiz2;
#[doc = "OTG_FS_HCTSIZ3 register accessor: an alias for `Reg<OTG_FS_HCTSIZ3_SPEC>`"]
pub type OTG_FS_HCTSIZ3 = crate::Reg<otg_fs_hctsiz3::OTG_FS_HCTSIZ3_SPEC>;
#[doc = "OTG_FS host channel-3 transfer size register"]
pub mod otg_fs_hctsiz3;
#[doc = "OTG_FS_HCTSIZ4 register accessor: an alias for `Reg<OTG_FS_HCTSIZ4_SPEC>`"]
pub type OTG_FS_HCTSIZ4 = crate::Reg<otg_fs_hctsiz4::OTG_FS_HCTSIZ4_SPEC>;
#[doc = "OTG_FS host channel-x transfer size register"]
pub mod otg_fs_hctsiz4;
#[doc = "OTG_FS_HCTSIZ5 register accessor: an alias for `Reg<OTG_FS_HCTSIZ5_SPEC>`"]
pub type OTG_FS_HCTSIZ5 = crate::Reg<otg_fs_hctsiz5::OTG_FS_HCTSIZ5_SPEC>;
#[doc = "OTG_FS host channel-5 transfer size register"]
pub mod otg_fs_hctsiz5;
#[doc = "OTG_FS_HCTSIZ6 register accessor: an alias for `Reg<OTG_FS_HCTSIZ6_SPEC>`"]
pub type OTG_FS_HCTSIZ6 = crate::Reg<otg_fs_hctsiz6::OTG_FS_HCTSIZ6_SPEC>;
#[doc = "OTG_FS host channel-6 transfer size register"]
pub mod otg_fs_hctsiz6;
#[doc = "OTG_FS_HCTSIZ7 register accessor: an alias for `Reg<OTG_FS_HCTSIZ7_SPEC>`"]
pub type OTG_FS_HCTSIZ7 = crate::Reg<otg_fs_hctsiz7::OTG_FS_HCTSIZ7_SPEC>;
#[doc = "OTG_FS host channel-7 transfer size register"]
pub mod otg_fs_hctsiz7;
#[doc = "OTG_FS_HCCHAR8 register accessor: an alias for `Reg<OTG_FS_HCCHAR8_SPEC>`"]
pub type OTG_FS_HCCHAR8 = crate::Reg<otg_fs_hcchar8::OTG_FS_HCCHAR8_SPEC>;
#[doc = "OTG_FS host channel-8 characteristics register"]
pub mod otg_fs_hcchar8;
#[doc = "OTG_FS_HCINT8 register accessor: an alias for `Reg<OTG_FS_HCINT8_SPEC>`"]
pub type OTG_FS_HCINT8 = crate::Reg<otg_fs_hcint8::OTG_FS_HCINT8_SPEC>;
#[doc = "OTG_FS host channel-8 interrupt register"]
pub mod otg_fs_hcint8;
#[doc = "OTG_FS_HCINTMSK8 register accessor: an alias for `Reg<OTG_FS_HCINTMSK8_SPEC>`"]
pub type OTG_FS_HCINTMSK8 = crate::Reg<otg_fs_hcintmsk8::OTG_FS_HCINTMSK8_SPEC>;
#[doc = "OTG_FS host channel-8 mask register"]
pub mod otg_fs_hcintmsk8;
#[doc = "OTG_FS_HCTSIZ8 register accessor: an alias for `Reg<OTG_FS_HCTSIZ8_SPEC>`"]
pub type OTG_FS_HCTSIZ8 = crate::Reg<otg_fs_hctsiz8::OTG_FS_HCTSIZ8_SPEC>;
#[doc = "OTG_FS host channel-8 transfer size register"]
pub mod otg_fs_hctsiz8;
#[doc = "OTG_FS_HCCHAR9 register accessor: an alias for `Reg<OTG_FS_HCCHAR9_SPEC>`"]
pub type OTG_FS_HCCHAR9 = crate::Reg<otg_fs_hcchar9::OTG_FS_HCCHAR9_SPEC>;
#[doc = "OTG_FS host channel-9 characteristics register"]
pub mod otg_fs_hcchar9;
#[doc = "OTG_FS_HCINT9 register accessor: an alias for `Reg<OTG_FS_HCINT9_SPEC>`"]
pub type OTG_FS_HCINT9 = crate::Reg<otg_fs_hcint9::OTG_FS_HCINT9_SPEC>;
#[doc = "OTG_FS host channel-9 interrupt register"]
pub mod otg_fs_hcint9;
#[doc = "OTG_FS_HCINTMSK9 register accessor: an alias for `Reg<OTG_FS_HCINTMSK9_SPEC>`"]
pub type OTG_FS_HCINTMSK9 = crate::Reg<otg_fs_hcintmsk9::OTG_FS_HCINTMSK9_SPEC>;
#[doc = "OTG_FS host channel-9 mask register"]
pub mod otg_fs_hcintmsk9;
#[doc = "OTG_FS_HCTSIZ9 register accessor: an alias for `Reg<OTG_FS_HCTSIZ9_SPEC>`"]
pub type OTG_FS_HCTSIZ9 = crate::Reg<otg_fs_hctsiz9::OTG_FS_HCTSIZ9_SPEC>;
#[doc = "OTG_FS host channel-9 transfer size register"]
pub mod otg_fs_hctsiz9;
#[doc = "OTG_FS_HCCHAR10 register accessor: an alias for `Reg<OTG_FS_HCCHAR10_SPEC>`"]
pub type OTG_FS_HCCHAR10 = crate::Reg<otg_fs_hcchar10::OTG_FS_HCCHAR10_SPEC>;
#[doc = "OTG_FS host channel-10 characteristics register"]
pub mod otg_fs_hcchar10;
#[doc = "OTG_FS_HCINT10 register accessor: an alias for `Reg<OTG_FS_HCINT10_SPEC>`"]
pub type OTG_FS_HCINT10 = crate::Reg<otg_fs_hcint10::OTG_FS_HCINT10_SPEC>;
#[doc = "OTG_FS host channel-10 interrupt register"]
pub mod otg_fs_hcint10;
#[doc = "OTG_FS_HCINTMSK10 register accessor: an alias for `Reg<OTG_FS_HCINTMSK10_SPEC>`"]
pub type OTG_FS_HCINTMSK10 = crate::Reg<otg_fs_hcintmsk10::OTG_FS_HCINTMSK10_SPEC>;
#[doc = "OTG_FS host channel-10 mask register"]
pub mod otg_fs_hcintmsk10;
#[doc = "OTG_FS_HCTSIZ10 register accessor: an alias for `Reg<OTG_FS_HCTSIZ10_SPEC>`"]
pub type OTG_FS_HCTSIZ10 = crate::Reg<otg_fs_hctsiz10::OTG_FS_HCTSIZ10_SPEC>;
#[doc = "OTG_FS host channel-10 transfer size register"]
pub mod otg_fs_hctsiz10;
#[doc = "OTG_FS_HCCHAR11 register accessor: an alias for `Reg<OTG_FS_HCCHAR11_SPEC>`"]
pub type OTG_FS_HCCHAR11 = crate::Reg<otg_fs_hcchar11::OTG_FS_HCCHAR11_SPEC>;
#[doc = "OTG_FS host channel-11 characteristics register"]
pub mod otg_fs_hcchar11;
#[doc = "OTG_FS_HCINT11 register accessor: an alias for `Reg<OTG_FS_HCINT11_SPEC>`"]
pub type OTG_FS_HCINT11 = crate::Reg<otg_fs_hcint11::OTG_FS_HCINT11_SPEC>;
#[doc = "OTG_FS host channel-11 interrupt register"]
pub mod otg_fs_hcint11;
#[doc = "OTG_FS_HCINTMSK11 register accessor: an alias for `Reg<OTG_FS_HCINTMSK11_SPEC>`"]
pub type OTG_FS_HCINTMSK11 = crate::Reg<otg_fs_hcintmsk11::OTG_FS_HCINTMSK11_SPEC>;
#[doc = "OTG_FS host channel-11 mask register"]
pub mod otg_fs_hcintmsk11;
#[doc = "OTG_FS_HCTSIZ11 register accessor: an alias for `Reg<OTG_FS_HCTSIZ11_SPEC>`"]
pub type OTG_FS_HCTSIZ11 = crate::Reg<otg_fs_hctsiz11::OTG_FS_HCTSIZ11_SPEC>;
#[doc = "OTG_FS host channel-11 transfer size register"]
pub mod otg_fs_hctsiz11;
