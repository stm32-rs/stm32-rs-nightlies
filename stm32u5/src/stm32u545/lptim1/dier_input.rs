#[doc = "Register `DIER_input` reader"]
pub type R = crate::R<DIER_INPUTrs>;
#[doc = "Register `DIER_input` writer"]
pub type W = crate::W<DIER_INPUTrs>;
#[doc = "Field `CC1IF` reader - Capture/compare 1 clear flag"]
pub type CC1IF_R = crate::BitReader;
#[doc = "Field `CC1IF` writer - Capture/compare 1 clear flag"]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `UPIE` reader - Direction change to UP Interrupt Enable"]
pub type UPIE_R = crate::BitReader;
#[doc = "Field `UPIE` writer - Direction change to UP Interrupt Enable"]
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNIE` reader - Direction change to down Interrupt Enable"]
pub type DOWNIE_R = crate::BitReader;
#[doc = "Field `DOWNIE` writer - Direction change to down Interrupt Enable"]
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEIE` reader - Update event interrupt enable"]
pub type UEIE_R = crate::BitReader;
#[doc = "Field `UEIE` writer - Update event interrupt enable"]
pub type UEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPOKIE` reader - REPOKIE"]
pub type REPOKIE_R = crate::BitReader;
#[doc = "Field `REPOKIE` writer - REPOKIE"]
pub type REPOKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - Capture/compare 2 interrupt enable"]
pub type CC2IE_R = crate::BitReader;
#[doc = "Field `CC2IE` writer - Capture/compare 2 interrupt enable"]
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OIE` reader - Capture/compare 1 over-capture interrupt enable"]
pub type CC1OIE_R = crate::BitReader;
#[doc = "Field `CC1OIE` writer - Capture/compare 1 over-capture interrupt enable"]
pub type CC1OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OIE` reader - Capture/compare 2 over-capture interrupt enable"]
pub type CC2OIE_R = crate::BitReader;
#[doc = "Field `CC2OIE` writer - Capture/compare 2 over-capture interrupt enable"]
pub type CC2OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1DE` reader - Capture/compare 1 DMA request enable"]
pub type CC1DE_R = crate::BitReader;
#[doc = "Field `CC1DE` writer - Capture/compare 1 DMA request enable"]
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2DE` reader - Capture/compare 2 DMA request enable"]
pub type CC2DE_R = crate::BitReader;
#[doc = "Field `CC2DE` writer - Capture/compare 2 DMA request enable"]
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/compare 1 clear flag"]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable"]
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    pub fn ueie(&self) -> UEIE_R {
        UEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REPOKIE"]
    #[inline(always)]
    pub fn repokie(&self) -> REPOKIE_R {
        REPOKIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/compare 1 over-capture interrupt enable"]
    #[inline(always)]
    pub fn cc1oie(&self) -> CC1OIE_R {
        CC1OIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/compare 2 over-capture interrupt enable"]
    #[inline(always)]
    pub fn cc2oie(&self) -> CC2OIE_R {
        CC2OIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture/compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 25 - Capture/compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare 1 clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<DIER_INPUTrs> {
        CC1IF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrmie(&mut self) -> ARRMIE_W<DIER_INPUTrs> {
        ARRMIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<DIER_INPUTrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrokie(&mut self) -> ARROKIE_W<DIER_INPUTrs> {
        ARROKIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<DIER_INPUTrs> {
        UPIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn downie(&mut self) -> DOWNIE_W<DIER_INPUTrs> {
        DOWNIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Update event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ueie(&mut self) -> UEIE_W<DIER_INPUTrs> {
        UEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - REPOKIE"]
    #[inline(always)]
    #[must_use]
    pub fn repokie(&mut self) -> REPOKIE_W<DIER_INPUTrs> {
        REPOKIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CC2IE_W<DIER_INPUTrs> {
        CC2IE_W::new(self, 9)
    }
    #[doc = "Bit 12 - Capture/compare 1 over-capture interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1oie(&mut self) -> CC1OIE_W<DIER_INPUTrs> {
        CC1OIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Capture/compare 2 over-capture interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2oie(&mut self) -> CC2OIE_W<DIER_INPUTrs> {
        CC2OIE_W::new(self, 13)
    }
    #[doc = "Bit 16 - Capture/compare 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> CC1DE_W<DIER_INPUTrs> {
        CC1DE_W::new(self, 16)
    }
    #[doc = "Bit 25 - Capture/compare 2 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2de(&mut self) -> CC2DE_W<DIER_INPUTrs> {
        CC2DE_W::new(self, 25)
    }
}
#[doc = "LPTIM interrupt Enable Register (intput mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIER_INPUTrs;
impl crate::RegisterSpec for DIER_INPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier_input::R`](R) reader structure"]
impl crate::Readable for DIER_INPUTrs {}
#[doc = "`write(|w| ..)` method takes [`dier_input::W`](W) writer structure"]
impl crate::Writable for DIER_INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIER_input to value 0"]
impl crate::Resettable for DIER_INPUTrs {
    const RESET_VALUE: u32 = 0;
}
