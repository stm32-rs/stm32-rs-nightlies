#[doc = "Register `DIER_intput` reader"]
pub type R = crate::R<DIER_INTPUTrs>;
#[doc = "Register `DIER_intput` writer"]
pub type W = crate::W<DIER_INTPUTrs>;
#[doc = "Field `CC1IE` reader - Capture/compare 1 interrupt enable"]
pub type CC1IE_R = crate::BitReader;
#[doc = "Field `CC1IE` writer - Capture/compare 1 interrupt enable"]
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMIE` reader - Autoreload match Interrupt Enable"]
pub type ARRMIE_R = crate::BitReader;
#[doc = "Field `ARRMIE` writer - Autoreload match Interrupt Enable"]
pub type ARRMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_R = crate::BitReader;
#[doc = "Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_R = crate::BitReader;
#[doc = "Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPIE` reader - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPIE_R = crate::BitReader;
#[doc = "Field `UPIE` writer - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNIE` reader - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNIE_R = crate::BitReader;
#[doc = "Field `DOWNIE` writer - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEIE` reader - Update event interrupt enable"]
pub type UEIE_R = crate::BitReader;
#[doc = "Field `UEIE` writer - Update event interrupt enable"]
pub type UEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPOKIE` reader - Repetition register update OK interrupt Enable"]
pub type REPOKIE_R = crate::BitReader;
#[doc = "Field `REPOKIE` writer - Repetition register update OK interrupt Enable"]
pub type REPOKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2IE_R = crate::BitReader;
#[doc = "Field `CC2IE` writer - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OIE` reader - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type CC1OIE_R = crate::BitReader;
#[doc = "Field `CC1OIE` writer - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type CC1OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OIE` reader - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2OIE_R = crate::BitReader;
#[doc = "Field `CC2OIE` writer - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1DE` reader - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type CC1DE_R = crate::BitReader;
#[doc = "Field `CC1DE` writer - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEDE` reader - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type UEDE_R = crate::BitReader;
#[doc = "Field `UEDE` writer - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type UEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2DE` reader - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2DE_R = crate::BitReader;
#[doc = "Field `CC2DE` writer - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    pub fn ueie(&self) -> UEIE_R {
        UEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Repetition register update OK interrupt Enable"]
    #[inline(always)]
    pub fn repokie(&self) -> REPOKIE_R {
        REPOKIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc1oie(&self) -> CC1OIE_R {
        CC1OIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc2oie(&self) -> CC2OIE_R {
        CC2OIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn uede(&self) -> UEDE_R {
        UEDE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<DIER_INTPUTrs> {
        CC1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrmie(&mut self) -> ARRMIE_W<DIER_INTPUTrs> {
        ARRMIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<DIER_INTPUTrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrokie(&mut self) -> ARROKIE_W<DIER_INTPUTrs> {
        ARROKIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<DIER_INTPUTrs> {
        UPIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn downie(&mut self) -> DOWNIE_W<DIER_INTPUTrs> {
        DOWNIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ueie(&mut self) -> UEIE_W<DIER_INTPUTrs> {
        UEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Repetition register update OK interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn repokie(&mut self) -> REPOKIE_W<DIER_INTPUTrs> {
        REPOKIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CC2IE_W<DIER_INTPUTrs> {
        CC2IE_W::new(self, 9)
    }
    #[doc = "Bit 12 - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc1oie(&mut self) -> CC1OIE_W<DIER_INTPUTrs> {
        CC1OIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc2oie(&mut self) -> CC2OIE_W<DIER_INTPUTrs> {
        CC2OIE_W::new(self, 13)
    }
    #[doc = "Bit 16 - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> CC1DE_W<DIER_INTPUTrs> {
        CC1DE_W::new(self, 16)
    }
    #[doc = "Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn uede(&mut self) -> UEDE_W<DIER_INTPUTrs> {
        UEDE_W::new(self, 23)
    }
    #[doc = "Bit 25 - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cc2de(&mut self) -> CC2DE_W<DIER_INTPUTrs> {
        CC2DE_W::new(self, 25)
    }
}
#[doc = "LPTIM interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier_intput::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier_intput::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIER_INTPUTrs;
impl crate::RegisterSpec for DIER_INTPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier_intput::R`](R) reader structure"]
impl crate::Readable for DIER_INTPUTrs {}
#[doc = "`write(|w| ..)` method takes [`dier_intput::W`](W) writer structure"]
impl crate::Writable for DIER_INTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIER_intput to value 0"]
impl crate::Resettable for DIER_INTPUTrs {
    const RESET_VALUE: u32 = 0;
}
