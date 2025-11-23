///Register `DIER_ALTERNATE1` reader
pub type R = crate::R<DIER_ALTERNATE1rs>;
///Register `DIER_ALTERNATE1` writer
pub type W = crate::W<DIER_ALTERNATE1rs>;
///Field `CC1IE` reader - Capture/compare 1 interrupt enable
pub type CC1IE_R = crate::BitReader;
///Field `CC1IE` writer - Capture/compare 1 interrupt enable
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARRMIE` reader - Autoreload match Interrupt Enable
pub type ARRMIE_R = crate::BitReader;
///Field `ARRMIE` writer - Autoreload match Interrupt Enable
pub type ARRMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable
pub type EXTTRIGIE_R = crate::BitReader;
///Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable
pub type EXTTRIGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable
pub type ARROKIE_R = crate::BitReader;
///Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable
pub type ARROKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPIE` reader - Direction change to UP Interrupt Enable
pub type UPIE_R = crate::BitReader;
///Field `UPIE` writer - Direction change to UP Interrupt Enable
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOWNIE` reader - Direction change to down Interrupt Enable
pub type DOWNIE_R = crate::BitReader;
///Field `DOWNIE` writer - Direction change to down Interrupt Enable
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UEIE` reader - Update event interrupt enable
pub type UEIE_R = crate::BitReader;
///Field `UEIE` writer - Update event interrupt enable
pub type UEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REPOKIE` reader - Repetition register update OK interrupt Enable
pub type REPOKIE_R = crate::BitReader;
///Field `REPOKIE` writer - Repetition register update OK interrupt Enable
pub type REPOKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IE` reader - Capture/compare 2 interrupt enable
pub type CC2IE_R = crate::BitReader;
///Field `CC2IE` writer - Capture/compare 2 interrupt enable
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1OIE` reader - Capture/compare 1 over-capture interrupt enable
pub type CC1OIE_R = crate::BitReader;
///Field `CC1OIE` writer - Capture/compare 1 over-capture interrupt enable
pub type CC1OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2OIE` reader - Capture/compare 2 over-capture interrupt enable
pub type CC2OIE_R = crate::BitReader;
///Field `CC2OIE` writer - Capture/compare 2 over-capture interrupt enable
pub type CC2OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1DE` reader - Capture/compare 1 DMA request enable
pub type CC1DE_R = crate::BitReader;
///Field `CC1DE` writer - Capture/compare 1 DMA request enable
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UEDE` reader - Update event DMA request enable
pub type UEDE_R = crate::BitReader;
///Field `UEDE` writer - Update event DMA request enable
pub type UEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2DE` reader - Capture/compare 2 DMA request enable
pub type CC2DE_R = crate::BitReader;
///Field `CC2DE` writer - Capture/compare 2 DMA request enable
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&self) -> UEIE_R {
        UEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Repetition register update OK interrupt Enable
    #[inline(always)]
    pub fn repokie(&self) -> REPOKIE_R {
        REPOKIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - Capture/compare 1 over-capture interrupt enable
    #[inline(always)]
    pub fn cc1oie(&self) -> CC1OIE_R {
        CC1OIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Capture/compare 2 over-capture interrupt enable
    #[inline(always)]
    pub fn cc2oie(&self) -> CC2OIE_R {
        CC2OIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Capture/compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 23 - Update event DMA request enable
    #[inline(always)]
    pub fn uede(&self) -> UEDE_R {
        UEDE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Capture/compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER_ALTERNATE1")
            .field("cc1ie", &self.cc1ie())
            .field("arrmie", &self.arrmie())
            .field("exttrigie", &self.exttrigie())
            .field("arrokie", &self.arrokie())
            .field("upie", &self.upie())
            .field("downie", &self.downie())
            .field("ueie", &self.ueie())
            .field("repokie", &self.repokie())
            .field("cc2ie", &self.cc2ie())
            .field("cc1oie", &self.cc1oie())
            .field("cc2oie", &self.cc2oie())
            .field("cc1de", &self.cc1de())
            .field("uede", &self.uede())
            .field("cc2de", &self.cc2de())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<'_, DIER_ALTERNATE1rs> {
        CC1IE_W::new(self, 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W<'_, DIER_ALTERNATE1rs> {
        ARRMIE_W::new(self, 1)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<'_, DIER_ALTERNATE1rs> {
        EXTTRIGIE_W::new(self, 2)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W<'_, DIER_ALTERNATE1rs> {
        ARROKIE_W::new(self, 4)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W<'_, DIER_ALTERNATE1rs> {
        UPIE_W::new(self, 5)
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W<'_, DIER_ALTERNATE1rs> {
        DOWNIE_W::new(self, 6)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&mut self) -> UEIE_W<'_, DIER_ALTERNATE1rs> {
        UEIE_W::new(self, 7)
    }
    ///Bit 8 - Repetition register update OK interrupt Enable
    #[inline(always)]
    pub fn repokie(&mut self) -> REPOKIE_W<'_, DIER_ALTERNATE1rs> {
        REPOKIE_W::new(self, 8)
    }
    ///Bit 9 - Capture/compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<'_, DIER_ALTERNATE1rs> {
        CC2IE_W::new(self, 9)
    }
    ///Bit 12 - Capture/compare 1 over-capture interrupt enable
    #[inline(always)]
    pub fn cc1oie(&mut self) -> CC1OIE_W<'_, DIER_ALTERNATE1rs> {
        CC1OIE_W::new(self, 12)
    }
    ///Bit 13 - Capture/compare 2 over-capture interrupt enable
    #[inline(always)]
    pub fn cc2oie(&mut self) -> CC2OIE_W<'_, DIER_ALTERNATE1rs> {
        CC2OIE_W::new(self, 13)
    }
    ///Bit 16 - Capture/compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<'_, DIER_ALTERNATE1rs> {
        CC1DE_W::new(self, 16)
    }
    ///Bit 23 - Update event DMA request enable
    #[inline(always)]
    pub fn uede(&mut self) -> UEDE_W<'_, DIER_ALTERNATE1rs> {
        UEDE_W::new(self, 23)
    }
    ///Bit 25 - Capture/compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W<'_, DIER_ALTERNATE1rs> {
        CC2DE_W::new(self, 25)
    }
}
/**LPTIM1 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier_alternate1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_alternate1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#LPTIM1:DIER_ALTERNATE1)*/
pub struct DIER_ALTERNATE1rs;
impl crate::RegisterSpec for DIER_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`dier_alternate1::R`](R) reader structure
impl crate::Readable for DIER_ALTERNATE1rs {}
///`write(|w| ..)` method takes [`dier_alternate1::W`](W) writer structure
impl crate::Writable for DIER_ALTERNATE1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER_ALTERNATE1 to value 0
impl crate::Resettable for DIER_ALTERNATE1rs {}
