#[doc = "Reader of register FDCAN_TTOST"]
pub type R = crate::R<u32, super::FDCAN_TTOST>;
#[doc = "Reader of field `EL`"]
pub type EL_R = crate::R<u8, u8>;
#[doc = "Reader of field `MS`"]
pub type MS_R = crate::R<u8, u8>;
#[doc = "Reader of field `SYS`"]
pub type SYS_R = crate::R<u8, u8>;
#[doc = "Reader of field `QGTP`"]
pub type QGTP_R = crate::R<bool, bool>;
#[doc = "Reader of field `QCS`"]
pub type QCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTO`"]
pub type RTO_R = crate::R<u8, u8>;
#[doc = "Reader of field `WGTD`"]
pub type WGTD_R = crate::R<bool, bool>;
#[doc = "Reader of field `GFI`"]
pub type GFI_R = crate::R<bool, bool>;
#[doc = "Reader of field `TMP`"]
pub type TMP_R = crate::R<u8, u8>;
#[doc = "Reader of field `GSI`"]
pub type GSI_R = crate::R<bool, bool>;
#[doc = "Reader of field `WFE`"]
pub type WFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWE`"]
pub type AWE_R = crate::R<bool, bool>;
#[doc = "Reader of field `WECS`"]
pub type WECS_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPL`"]
pub type SPL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - EL"]
    #[inline(always)]
    pub fn el(&self) -> EL_R {
        EL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - MS"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - SYS"]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - QGTP"]
    #[inline(always)]
    pub fn qgtp(&self) -> QGTP_R {
        QGTP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - QCS"]
    #[inline(always)]
    pub fn qcs(&self) -> QCS_R {
        QCS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - RTO"]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 22 - WGTD"]
    #[inline(always)]
    pub fn wgtd(&self) -> WGTD_R {
        WGTD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GFI"]
    #[inline(always)]
    pub fn gfi(&self) -> GFI_R {
        GFI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - TMP"]
    #[inline(always)]
    pub fn tmp(&self) -> TMP_R {
        TMP_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - GSI"]
    #[inline(always)]
    pub fn gsi(&self) -> GSI_R {
        GSI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - WFE"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - AWE"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - WECS"]
    #[inline(always)]
    pub fn wecs(&self) -> WECS_R {
        WECS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SPL"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
