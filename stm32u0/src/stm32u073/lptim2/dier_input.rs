///Register `DIER_INPUT` reader
pub type R = crate::R<DIER_INPUTrs>;
///Register `DIER_INPUT` writer
pub type W = crate::W<DIER_INPUTrs>;
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
///Field `UPIE` reader - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type UPIE_R = crate::BitReader;
///Field `UPIE` writer - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOWNIE` reader - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type DOWNIE_R = crate::BitReader;
///Field `DOWNIE` writer - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UEIE` reader - Update event interrupt enable
pub type UEIE_R = crate::BitReader;
///Field `UEIE` writer - Update event interrupt enable
pub type UEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REPOKIE` reader - Repetition register update OK interrupt Enable
pub type REPOKIE_R = crate::BitReader;
///Field `REPOKIE` writer - Repetition register update OK interrupt Enable
pub type REPOKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IE` reader - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CC2IE_R = crate::BitReader;
///Field `CC2IE` writer - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3IE` reader - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CC3IE_R = crate::BitReader;
///Field `CC3IE` writer - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CC3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4IE` reader - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CC4IE_R = crate::BitReader;
///Field `CC4IE` writer - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CC4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1OIE` reader - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
pub type CC1OIE_R = crate::BitReader;
///Field `CC1OIE` writer - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
pub type CC1OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2OIE` reader - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CC2OIE_R = crate::BitReader;
///Field `CC2OIE` writer - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CC2OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3OIE` reader - Capture/compare 3 over-capture interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CC3OIE_R = crate::BitReader;
///Field `CC3OIE` writer - Capture/compare 3 over-capture interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CC3OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4OIE` reader - Capture/compare 4 over-capture interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CC4OIE_R = crate::BitReader;
///Field `CC4OIE` writer - Capture/compare 4 over-capture interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CC4OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1DE` reader - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
pub type CC1DE_R = crate::BitReader;
///Field `CC1DE` writer - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UEDE` reader - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
pub type UEDE_R = crate::BitReader;
///Field `UEDE` writer - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
pub type UEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2DE` reader - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CC2DE_R = crate::BitReader;
///Field `CC2DE` writer - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3DE` reader - Capture/compare 3 DMA request enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CC3DE_R = crate::BitReader;
///Field `CC3DE` writer - Capture/compare 3 DMA request enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CC3DE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4DE` reader - Capture/compare 4 DMA request enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CC4DE_R = crate::BitReader;
///Field `CC4DE` writer - Capture/compare 4 DMA request enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CC4DE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
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
    ///Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc1oie(&self) -> CC1OIE_R {
        CC1OIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc2oie(&self) -> CC2OIE_R {
        CC2OIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Capture/compare 3 over-capture interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc3oie(&self) -> CC3OIE_R {
        CC3OIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Capture/compare 4 over-capture interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc4oie(&self) -> CC4OIE_R {
        CC4OIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn uede(&self) -> UEDE_R {
        UEDE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Capture/compare 3 DMA request enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc3de(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Capture/compare 4 DMA request enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc4de(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER_INPUT")
            .field("cc1ie", &self.cc1ie())
            .field("arrmie", &self.arrmie())
            .field("exttrigie", &self.exttrigie())
            .field("arrokie", &self.arrokie())
            .field("upie", &self.upie())
            .field("downie", &self.downie())
            .field("ueie", &self.ueie())
            .field("repokie", &self.repokie())
            .field("cc2ie", &self.cc2ie())
            .field("cc3ie", &self.cc3ie())
            .field("cc4ie", &self.cc4ie())
            .field("cc1oie", &self.cc1oie())
            .field("cc2oie", &self.cc2oie())
            .field("cc3oie", &self.cc3oie())
            .field("cc4oie", &self.cc4oie())
            .field("cc1de", &self.cc1de())
            .field("uede", &self.uede())
            .field("cc2de", &self.cc2de())
            .field("cc3de", &self.cc3de())
            .field("cc4de", &self.cc4de())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<DIER_INPUTrs> {
        CC1IE_W::new(self, 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&mut self) -> ARRMIE_W<DIER_INPUTrs> {
        ARRMIE_W::new(self, 1)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<DIER_INPUTrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&mut self) -> ARROKIE_W<DIER_INPUTrs> {
        ARROKIE_W::new(self, 4)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn upie(&mut self) -> UPIE_W<DIER_INPUTrs> {
        UPIE_W::new(self, 5)
    }
    ///Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn downie(&mut self) -> DOWNIE_W<DIER_INPUTrs> {
        DOWNIE_W::new(self, 6)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&mut self) -> UEIE_W<DIER_INPUTrs> {
        UEIE_W::new(self, 7)
    }
    ///Bit 8 - Repetition register update OK interrupt Enable
    #[inline(always)]
    pub fn repokie(&mut self) -> REPOKIE_W<DIER_INPUTrs> {
        REPOKIE_W::new(self, 8)
    }
    ///Bit 9 - Capture/compare 2 interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<DIER_INPUTrs> {
        CC2IE_W::new(self, 9)
    }
    ///Bit 10 - Capture/compare 3 interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W<DIER_INPUTrs> {
        CC3IE_W::new(self, 10)
    }
    ///Bit 11 - Capture/compare 4 interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W<DIER_INPUTrs> {
        CC4IE_W::new(self, 11)
    }
    ///Bit 12 - Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc1oie(&mut self) -> CC1OIE_W<DIER_INPUTrs> {
        CC1OIE_W::new(self, 12)
    }
    ///Bit 13 - Capture/compare 2 over-capture interrupt enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc2oie(&mut self) -> CC2OIE_W<DIER_INPUTrs> {
        CC2OIE_W::new(self, 13)
    }
    ///Bit 14 - Capture/compare 3 over-capture interrupt enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc3oie(&mut self) -> CC3OIE_W<DIER_INPUTrs> {
        CC3OIE_W::new(self, 14)
    }
    ///Bit 15 - Capture/compare 4 over-capture interrupt enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc4oie(&mut self) -> CC4OIE_W<DIER_INPUTrs> {
        CC4OIE_W::new(self, 15)
    }
    ///Bit 16 - Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<DIER_INPUTrs> {
        CC1DE_W::new(self, 16)
    }
    ///Bit 23 - Update event DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn uede(&mut self) -> UEDE_W<DIER_INPUTrs> {
        UEDE_W::new(self, 23)
    }
    ///Bit 25 - Capture/compare 2 DMA request enable Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W<DIER_INPUTrs> {
        CC2DE_W::new(self, 25)
    }
    ///Bit 26 - Capture/compare 3 DMA request enable Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc3de(&mut self) -> CC3DE_W<DIER_INPUTrs> {
        CC3DE_W::new(self, 26)
    }
    ///Bit 27 - Capture/compare 4 DMA request enable Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc4de(&mut self) -> CC4DE_W<DIER_INPUTrs> {
        CC4DE_W::new(self, 27)
    }
}
/**LPTIM2 interrupt enable register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`dier_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:DIER_INPUT)*/
pub struct DIER_INPUTrs;
impl crate::RegisterSpec for DIER_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`dier_input::R`](R) reader structure
impl crate::Readable for DIER_INPUTrs {}
///`write(|w| ..)` method takes [`dier_input::W`](W) writer structure
impl crate::Writable for DIER_INPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER_INPUT to value 0
impl crate::Resettable for DIER_INPUTrs {}
