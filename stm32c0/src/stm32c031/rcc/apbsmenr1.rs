///Register `APBSMENR1` reader
pub type R = crate::R<APBSMENR1rs>;
///Register `APBSMENR1` writer
pub type W = crate::W<APBSMENR1rs>;
///Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM3SMEN_R = crate::BitReader;
///Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode Set and cleared by software.
pub type TIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode Set and cleared by software.
pub type RTCAPBSMEN_R = crate::BitReader;
///Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode Set and cleared by software.
pub type RTCAPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGSMEN` reader - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
pub type WWDGSMEN_R = crate::BitReader;
///Field `WWDGSMEN` writer - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2SMEN` reader - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART2SMEN_R = crate::BitReader;
///Field `USART2SMEN` writer - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
pub type USART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1SMEN` reader - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type I2C1SMEN_R = crate::BitReader;
///Field `I2C1SMEN` writer - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSMEN` reader - Debug support clock enable during Sleep mode Set and cleared by software.
pub type DBGSMEN_R = crate::BitReader;
///Field `DBGSMEN` writer - Debug support clock enable during Sleep mode Set and cleared by software.
pub type DBGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRSMEN` reader - Power interface clock enable during Sleep mode Set and cleared by software.
pub type PWRSMEN_R = crate::BitReader;
///Field `PWRSMEN` writer - Power interface clock enable during Sleep mode Set and cleared by software.
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - TIM3 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 27 - Debug support clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBSMENR1")
            .field("tim3smen", &self.tim3smen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("usart2smen", &self.usart2smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("dbgsmen", &self.dbgsmen())
            .field("pwrsmen", &self.pwrsmen())
            .finish()
    }
}
impl W {
    ///Bit 1 - TIM3 timer clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<APBSMENR1rs> {
        TIM3SMEN_W::new(self, 1)
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<APBSMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    ///Bit 11 - WWDG clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<APBSMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 17 - USART2 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<APBSMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    ///Bit 21 - I2C1 clock enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<APBSMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 27 - Debug support clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W<APBSMENR1rs> {
        DBGSMEN_W::new(self, 27)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<APBSMENR1rs> {
        PWRSMEN_W::new(self, 28)
    }
}
/**RCC APB peripheral clock enable in Sleep/Stop mode register 1

You can [`read`](crate::Reg::read) this register and get [`apbsmenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#RCC:APBSMENR1)*/
pub struct APBSMENR1rs;
impl crate::RegisterSpec for APBSMENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apbsmenr1::R`](R) reader structure
impl crate::Readable for APBSMENR1rs {}
///`write(|w| ..)` method takes [`apbsmenr1::W`](W) writer structure
impl crate::Writable for APBSMENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APBSMENR1 to value 0x18ef_7f36
impl crate::Resettable for APBSMENR1rs {
    const RESET_VALUE: u32 = 0x18ef_7f36;
}
