#[doc = "Register `TTOST` reader"]
pub type R = crate::R<TTOSTrs>;
#[doc = "Register `TTOST` writer"]
pub type W = crate::W<TTOSTrs>;
#[doc = "Field `EL` reader - Error Level"]
pub type EL_R = crate::FieldReader;
#[doc = "Field `EL` writer - Error Level"]
pub type EL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MS` reader - Master State."]
pub type MS_R = crate::FieldReader;
#[doc = "Field `MS` writer - Master State."]
pub type MS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYS` reader - Synchronization State"]
pub type SYS_R = crate::FieldReader;
#[doc = "Field `SYS` writer - Synchronization State"]
pub type SYS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QGTP` reader - Quality of Global Time Phase"]
pub type QGTP_R = crate::BitReader;
#[doc = "Field `QGTP` writer - Quality of Global Time Phase"]
pub type QGTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QCS` reader - Quality of Clock Speed"]
pub type QCS_R = crate::BitReader;
#[doc = "Field `QCS` writer - Quality of Clock Speed"]
pub type QCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTO` reader - Reference Trigger Offset"]
pub type RTO_R = crate::FieldReader;
#[doc = "Field `RTO` writer - Reference Trigger Offset"]
pub type RTO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WGTD` reader - Wait for Global Time Discontinuity"]
pub type WGTD_R = crate::BitReader;
#[doc = "Field `WGTD` writer - Wait for Global Time Discontinuity"]
pub type WGTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFI` reader - Gap Finished Indicator."]
pub type GFI_R = crate::BitReader;
#[doc = "Field `GFI` writer - Gap Finished Indicator."]
pub type GFI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMP` reader - Time Master Priority"]
pub type TMP_R = crate::FieldReader;
#[doc = "Field `TMP` writer - Time Master Priority"]
pub type TMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GSI` reader - Gap Started Indicator."]
pub type GSI_R = crate::BitReader;
#[doc = "Field `GSI` writer - Gap Started Indicator."]
pub type GSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFE` reader - Wait for Event"]
pub type WFE_R = crate::BitReader;
#[doc = "Field `WFE` writer - Wait for Event"]
pub type WFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWE` reader - Application Watchdog Event"]
pub type AWE_R = crate::BitReader;
#[doc = "Field `AWE` writer - Application Watchdog Event"]
pub type AWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WECS` reader - Wait for External Clock Synchronization"]
pub type WECS_R = crate::BitReader;
#[doc = "Field `WECS` writer - Wait for External Clock Synchronization"]
pub type WECS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPL` reader - Schedule Phase Lock"]
pub type SPL_R = crate::BitReader;
#[doc = "Field `SPL` writer - Schedule Phase Lock"]
pub type SPL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn qgtp(&self) -> QGTP_R {
        QGTP_R::new(((self.bits >> 6) & 1) != 0)
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
impl W {
    #[doc = "Bits 0:1 - Error Level"]
    #[inline(always)]
    #[must_use]
    pub fn el(&mut self) -> EL_W<TTOSTrs> {
        EL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Master State."]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MS_W<TTOSTrs> {
        MS_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Synchronization State"]
    #[inline(always)]
    #[must_use]
    pub fn sys(&mut self) -> SYS_W<TTOSTrs> {
        SYS_W::new(self, 4)
    }
    #[doc = "Bit 6 - Quality of Global Time Phase"]
    #[inline(always)]
    #[must_use]
    pub fn qgtp(&mut self) -> QGTP_W<TTOSTrs> {
        QGTP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Quality of Clock Speed"]
    #[inline(always)]
    #[must_use]
    pub fn qcs(&mut self) -> QCS_W<TTOSTrs> {
        QCS_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset"]
    #[inline(always)]
    #[must_use]
    pub fn rto(&mut self) -> RTO_W<TTOSTrs> {
        RTO_W::new(self, 8)
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity"]
    #[inline(always)]
    #[must_use]
    pub fn wgtd(&mut self) -> WGTD_W<TTOSTrs> {
        WGTD_W::new(self, 22)
    }
    #[doc = "Bit 23 - Gap Finished Indicator."]
    #[inline(always)]
    #[must_use]
    pub fn gfi(&mut self) -> GFI_W<TTOSTrs> {
        GFI_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Time Master Priority"]
    #[inline(always)]
    #[must_use]
    pub fn tmp(&mut self) -> TMP_W<TTOSTrs> {
        TMP_W::new(self, 24)
    }
    #[doc = "Bit 27 - Gap Started Indicator."]
    #[inline(always)]
    #[must_use]
    pub fn gsi(&mut self) -> GSI_W<TTOSTrs> {
        GSI_W::new(self, 27)
    }
    #[doc = "Bit 28 - Wait for Event"]
    #[inline(always)]
    #[must_use]
    pub fn wfe(&mut self) -> WFE_W<TTOSTrs> {
        WFE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Application Watchdog Event"]
    #[inline(always)]
    #[must_use]
    pub fn awe(&mut self) -> AWE_W<TTOSTrs> {
        AWE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn wecs(&mut self) -> WECS_W<TTOSTrs> {
        WECS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Schedule Phase Lock"]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SPL_W<TTOSTrs> {
        SPL_W::new(self, 31)
    }
}
#[doc = "FDCAN TT Operation Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttost::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttost::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTOSTrs;
impl crate::RegisterSpec for TTOSTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttost::R`](R) reader structure"]
impl crate::Readable for TTOSTrs {}
#[doc = "`write(|w| ..)` method takes [`ttost::W`](W) writer structure"]
impl crate::Writable for TTOSTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TTOST to value 0"]
impl crate::Resettable for TTOSTrs {
    const RESET_VALUE: u32 = 0;
}
