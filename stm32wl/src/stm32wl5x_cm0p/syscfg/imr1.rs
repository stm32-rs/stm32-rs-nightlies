///Register `IMR1` reader
pub type R = crate::R<IMR1rs>;
///Register `IMR1` writer
pub type W = crate::W<IMR1rs>;
///Field `RTCSTAMPTAMPLSECSSIM` reader - RTCSTAMPTAMPLSECSSIM
pub type RTCSTAMPTAMPLSECSSIM_R = crate::BitReader;
///Field `RTCSTAMPTAMPLSECSSIM` writer - RTCSTAMPTAMPLSECSSIM
pub type RTCSTAMPTAMPLSECSSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCSSRUIM` reader - RTCSSRUIM
pub type RTCSSRUIM_R = crate::BitReader;
///Field `RTCSSRUIM` writer - RTCSSRUIM
pub type RTCSSRUIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTI5IM` reader - EXTI5IM
pub type EXTI5IM_R = crate::BitReader;
///Field `EXTI5IM` writer - EXTI5IM
pub type EXTI5IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTI6IM` reader - EXTI6IM
pub type EXTI6IM_R = crate::BitReader;
///Field `EXTI6IM` writer - EXTI6IM
pub type EXTI6IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTI7IM` reader - EXTI7IM
pub type EXTI7IM_R = crate::BitReader;
///Field `EXTI7IM` writer - EXTI7IM
pub type EXTI7IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTI8IM` reader - EXTI8IM
pub type EXTI8IM_R = crate::BitReader;
///Field `EXTI8IM` writer - EXTI8IM
pub type EXTI8IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTI9IM` reader - EXTI9IM
pub type EXTI9IM_R = crate::BitReader;
///Field `EXTI9IM` writer - EXTI9IM
pub type EXTI9IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTI10IM` reader - EXTI10IM
pub type EXTI10IM_R = crate::BitReader;
///Field `EXTI10IM` writer - EXTI10IM
pub type EXTI10IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTI11IM` reader - EXTI11IM
pub type EXTI11IM_R = crate::BitReader;
///Field `EXTI11IM` writer - EXTI11IM
pub type EXTI11IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTI12IM` reader - EXTI12IM
pub type EXTI12IM_R = crate::BitReader;
///Field `EXTI12IM` writer - EXTI12IM
pub type EXTI12IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTI13IM` reader - EXTI13IM
pub type EXTI13IM_R = crate::BitReader;
///Field `EXTI13IM` writer - EXTI13IM
pub type EXTI13IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTI14IM` reader - EXTI14IM
pub type EXTI14IM_R = crate::BitReader;
///Field `EXTI14IM` writer - EXTI14IM
pub type EXTI14IM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTI15IM` reader - EXTI15IM
pub type EXTI15IM_R = crate::BitReader;
///Field `EXTI15IM` writer - EXTI15IM
pub type EXTI15IM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RTCSTAMPTAMPLSECSSIM
    #[inline(always)]
    pub fn rtcstamptamplsecssim(&self) -> RTCSTAMPTAMPLSECSSIM_R {
        RTCSTAMPTAMPLSECSSIM_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - RTCSSRUIM
    #[inline(always)]
    pub fn rtcssruim(&self) -> RTCSSRUIM_R {
        RTCSSRUIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 21 - EXTI5IM
    #[inline(always)]
    pub fn exti5im(&self) -> EXTI5IM_R {
        EXTI5IM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - EXTI6IM
    #[inline(always)]
    pub fn exti6im(&self) -> EXTI6IM_R {
        EXTI6IM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - EXTI7IM
    #[inline(always)]
    pub fn exti7im(&self) -> EXTI7IM_R {
        EXTI7IM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - EXTI8IM
    #[inline(always)]
    pub fn exti8im(&self) -> EXTI8IM_R {
        EXTI8IM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - EXTI9IM
    #[inline(always)]
    pub fn exti9im(&self) -> EXTI9IM_R {
        EXTI9IM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - EXTI10IM
    #[inline(always)]
    pub fn exti10im(&self) -> EXTI10IM_R {
        EXTI10IM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - EXTI11IM
    #[inline(always)]
    pub fn exti11im(&self) -> EXTI11IM_R {
        EXTI11IM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - EXTI12IM
    #[inline(always)]
    pub fn exti12im(&self) -> EXTI12IM_R {
        EXTI12IM_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - EXTI13IM
    #[inline(always)]
    pub fn exti13im(&self) -> EXTI13IM_R {
        EXTI13IM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - EXTI14IM
    #[inline(always)]
    pub fn exti14im(&self) -> EXTI14IM_R {
        EXTI14IM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - EXTI15IM
    #[inline(always)]
    pub fn exti15im(&self) -> EXTI15IM_R {
        EXTI15IM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR1")
            .field("rtcstamptamplsecssim", &self.rtcstamptamplsecssim())
            .field("rtcssruim", &self.rtcssruim())
            .field("exti5im", &self.exti5im())
            .field("exti6im", &self.exti6im())
            .field("exti7im", &self.exti7im())
            .field("exti8im", &self.exti8im())
            .field("exti9im", &self.exti9im())
            .field("exti10im", &self.exti10im())
            .field("exti11im", &self.exti11im())
            .field("exti12im", &self.exti12im())
            .field("exti13im", &self.exti13im())
            .field("exti14im", &self.exti14im())
            .field("exti15im", &self.exti15im())
            .finish()
    }
}
impl W {
    ///Bit 0 - RTCSTAMPTAMPLSECSSIM
    #[inline(always)]
    pub fn rtcstamptamplsecssim(&mut self) -> RTCSTAMPTAMPLSECSSIM_W<'_, IMR1rs> {
        RTCSTAMPTAMPLSECSSIM_W::new(self, 0)
    }
    ///Bit 2 - RTCSSRUIM
    #[inline(always)]
    pub fn rtcssruim(&mut self) -> RTCSSRUIM_W<'_, IMR1rs> {
        RTCSSRUIM_W::new(self, 2)
    }
    ///Bit 21 - EXTI5IM
    #[inline(always)]
    pub fn exti5im(&mut self) -> EXTI5IM_W<'_, IMR1rs> {
        EXTI5IM_W::new(self, 21)
    }
    ///Bit 22 - EXTI6IM
    #[inline(always)]
    pub fn exti6im(&mut self) -> EXTI6IM_W<'_, IMR1rs> {
        EXTI6IM_W::new(self, 22)
    }
    ///Bit 23 - EXTI7IM
    #[inline(always)]
    pub fn exti7im(&mut self) -> EXTI7IM_W<'_, IMR1rs> {
        EXTI7IM_W::new(self, 23)
    }
    ///Bit 24 - EXTI8IM
    #[inline(always)]
    pub fn exti8im(&mut self) -> EXTI8IM_W<'_, IMR1rs> {
        EXTI8IM_W::new(self, 24)
    }
    ///Bit 25 - EXTI9IM
    #[inline(always)]
    pub fn exti9im(&mut self) -> EXTI9IM_W<'_, IMR1rs> {
        EXTI9IM_W::new(self, 25)
    }
    ///Bit 26 - EXTI10IM
    #[inline(always)]
    pub fn exti10im(&mut self) -> EXTI10IM_W<'_, IMR1rs> {
        EXTI10IM_W::new(self, 26)
    }
    ///Bit 27 - EXTI11IM
    #[inline(always)]
    pub fn exti11im(&mut self) -> EXTI11IM_W<'_, IMR1rs> {
        EXTI11IM_W::new(self, 27)
    }
    ///Bit 28 - EXTI12IM
    #[inline(always)]
    pub fn exti12im(&mut self) -> EXTI12IM_W<'_, IMR1rs> {
        EXTI12IM_W::new(self, 28)
    }
    ///Bit 29 - EXTI13IM
    #[inline(always)]
    pub fn exti13im(&mut self) -> EXTI13IM_W<'_, IMR1rs> {
        EXTI13IM_W::new(self, 29)
    }
    ///Bit 30 - EXTI14IM
    #[inline(always)]
    pub fn exti14im(&mut self) -> EXTI14IM_W<'_, IMR1rs> {
        EXTI14IM_W::new(self, 30)
    }
    ///Bit 31 - EXTI15IM
    #[inline(always)]
    pub fn exti15im(&mut self) -> EXTI15IM_W<'_, IMR1rs> {
        EXTI15IM_W::new(self, 31)
    }
}
/**SYSCFG CPU1 interrupt mask register 1

You can [`read`](crate::Reg::read) this register and get [`imr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#SYSCFG:IMR1)*/
pub struct IMR1rs;
impl crate::RegisterSpec for IMR1rs {
    type Ux = u32;
}
///`read()` method returns [`imr1::R`](R) reader structure
impl crate::Readable for IMR1rs {}
///`write(|w| ..)` method takes [`imr1::W`](W) writer structure
impl crate::Writable for IMR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR1 to value 0
impl crate::Resettable for IMR1rs {}
