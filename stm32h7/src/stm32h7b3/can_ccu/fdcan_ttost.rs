#[doc = "Register `FDCAN_TTOST` reader"]
pub type R = crate::R<FDCAN_TTOSTrs>;
#[doc = "Field `EL` reader - Error Level"]
pub type EL_R = crate::FieldReader;
#[doc = "Field `MS` reader - Master State."]
pub type MS_R = crate::FieldReader;
#[doc = "Field `SYS` reader - Synchronization State"]
pub type SYS_R = crate::FieldReader;
#[doc = "Field `GTP` reader - Quality of Global Time Phase"]
pub type GTP_R = crate::BitReader;
#[doc = "Field `QCS` reader - Quality of Clock Speed"]
pub type QCS_R = crate::BitReader;
#[doc = "Field `RTO` reader - Reference Trigger Offset"]
pub type RTO_R = crate::FieldReader;
#[doc = "Field `WGTD` reader - Wait for Global Time Discontinuity"]
pub type WGTD_R = crate::BitReader;
#[doc = "Field `GFI` reader - Gap Finished Indicator."]
pub type GFI_R = crate::BitReader;
#[doc = "Field `TMP` reader - Time Master Priority"]
pub type TMP_R = crate::FieldReader;
#[doc = "Field `GSI` reader - Gap Started Indicator."]
pub type GSI_R = crate::BitReader;
#[doc = "Field `WFE` reader - Wait for Event"]
pub type WFE_R = crate::BitReader;
#[doc = "Field `AWE` reader - Application Watchdog Event"]
pub type AWE_R = crate::BitReader;
#[doc = "Field `WECS` reader - Wait for External Clock Synchronization"]
pub type WECS_R = crate::BitReader;
#[doc = "Field `SPL` reader - Schedule Phase Lock"]
pub type SPL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Error Level"]
    #[inline(always)]
    pub fn el(&self) -> EL_R {
        EL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Master State."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronization State"]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Quality of Global Time Phase"]
    #[inline(always)]
    pub fn gtp(&self) -> GTP_R {
        GTP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Quality of Clock Speed"]
    #[inline(always)]
    pub fn qcs(&self) -> QCS_R {
        QCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset"]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity"]
    #[inline(always)]
    pub fn wgtd(&self) -> WGTD_R {
        WGTD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Gap Finished Indicator."]
    #[inline(always)]
    pub fn gfi(&self) -> GFI_R {
        GFI_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Time Master Priority"]
    #[inline(always)]
    pub fn tmp(&self) -> TMP_R {
        TMP_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Gap Started Indicator."]
    #[inline(always)]
    pub fn gsi(&self) -> GSI_R {
        GSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wait for Event"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Application Watchdog Event"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization"]
    #[inline(always)]
    pub fn wecs(&self) -> WECS_R {
        WECS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Schedule Phase Lock"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FDCAN TT Operation Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttost::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTOSTrs;
impl crate::RegisterSpec for FDCAN_TTOSTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttost::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTOSTrs {}
#[doc = "`reset()` method sets FDCAN_TTOST to value 0"]
impl crate::Resettable for FDCAN_TTOSTrs {
    const RESET_VALUE: u32 = 0;
}
