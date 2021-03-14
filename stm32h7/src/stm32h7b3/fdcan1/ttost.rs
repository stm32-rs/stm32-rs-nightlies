#[doc = "Reader of register TTOST"]
pub type R = crate::R<u32, super::TTOST>;
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
    #[doc = "Bits 0:1 - Error Level"]
    #[inline(always)]
    pub fn el(&self) -> EL_R {
        EL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Master State."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Synchronization State"]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Quality of Global Time Phase"]
    #[inline(always)]
    pub fn qgtp(&self) -> QGTP_R {
        QGTP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Quality of Clock Speed"]
    #[inline(always)]
    pub fn qcs(&self) -> QCS_R {
        QCS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset"]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity"]
    #[inline(always)]
    pub fn wgtd(&self) -> WGTD_R {
        WGTD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Gap Finished Indicator."]
    #[inline(always)]
    pub fn gfi(&self) -> GFI_R {
        GFI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Time Master Priority"]
    #[inline(always)]
    pub fn tmp(&self) -> TMP_R {
        TMP_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Gap Started Indicator."]
    #[inline(always)]
    pub fn gsi(&self) -> GSI_R {
        GSI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Wait for Event"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Application Watchdog Event"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization"]
    #[inline(always)]
    pub fn wecs(&self) -> WECS_R {
        WECS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Schedule Phase Lock"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
