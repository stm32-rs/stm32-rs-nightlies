///Register `APB1LPENR` reader
pub type R = crate::R<APB1LPENRrs>;
///Register `APB1LPENR` writer
pub type W = crate::W<APB1LPENRrs>;
/**TIM5 clock enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM5LPEN {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<TIM5LPEN> for bool {
    #[inline(always)]
    fn from(variant: TIM5LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM5LPEN` reader - TIM5 clock enable during Sleep mode
pub type TIM5LPEN_R = crate::BitReader<TIM5LPEN>;
impl TIM5LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM5LPEN {
        match self.bits {
            false => TIM5LPEN::DisabledInSleep,
            true => TIM5LPEN::EnabledInSleep,
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == TIM5LPEN::DisabledInSleep
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == TIM5LPEN::EnabledInSleep
    }
}
///Field `TIM5LPEN` writer - TIM5 clock enable during Sleep mode
pub type TIM5LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM5LPEN>;
impl<'a, REG> TIM5LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(TIM5LPEN::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(TIM5LPEN::EnabledInSleep)
    }
}
///Field `TIM6LPEN` reader - TIM6 clock enable during Sleep mode
pub use TIM5LPEN_R as TIM6LPEN_R;
///Field `LPTIM1LPEN` reader - LPTIM1 clock enable during sleep mode
pub use TIM5LPEN_R as LPTIM1LPEN_R;
///Field `RTCAPBLPEN` reader - RTC APB clock enable during sleep mode
pub use TIM5LPEN_R as RTCAPBLPEN_R;
///Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode
pub use TIM5LPEN_R as WWDGLPEN_R;
///Field `SPI2LPEN` reader - SPI2 clock enable during Sleep mode
pub use TIM5LPEN_R as SPI2LPEN_R;
///Field `USART2LPEN` reader - USART2 clock enable during Sleep mode
pub use TIM5LPEN_R as USART2LPEN_R;
///Field `I2C1LPEN` reader - I2C1 clock enable during Sleep mode
pub use TIM5LPEN_R as I2C1LPEN_R;
///Field `I2C2LPEN` reader - I2C2 clock enable during Sleep mode
pub use TIM5LPEN_R as I2C2LPEN_R;
///Field `FMPI2C1LPEN` reader - FMPI2C1 clock enable during Sleep
pub use TIM5LPEN_R as FMPI2C1LPEN_R;
///Field `PWRLPEN` reader - Power interface clock enable during Sleep mode
pub use TIM5LPEN_R as PWRLPEN_R;
///Field `DACLPEN` reader - DAC interface clock enable during sleep mode
pub use TIM5LPEN_R as DACLPEN_R;
///Field `TIM6LPEN` writer - TIM6 clock enable during Sleep mode
pub use TIM5LPEN_W as TIM6LPEN_W;
///Field `LPTIM1LPEN` writer - LPTIM1 clock enable during sleep mode
pub use TIM5LPEN_W as LPTIM1LPEN_W;
///Field `RTCAPBLPEN` writer - RTC APB clock enable during sleep mode
pub use TIM5LPEN_W as RTCAPBLPEN_W;
///Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode
pub use TIM5LPEN_W as WWDGLPEN_W;
///Field `SPI2LPEN` writer - SPI2 clock enable during Sleep mode
pub use TIM5LPEN_W as SPI2LPEN_W;
///Field `USART2LPEN` writer - USART2 clock enable during Sleep mode
pub use TIM5LPEN_W as USART2LPEN_W;
///Field `I2C1LPEN` writer - I2C1 clock enable during Sleep mode
pub use TIM5LPEN_W as I2C1LPEN_W;
///Field `I2C2LPEN` writer - I2C2 clock enable during Sleep mode
pub use TIM5LPEN_W as I2C2LPEN_W;
///Field `FMPI2C1LPEN` writer - FMPI2C1 clock enable during Sleep
pub use TIM5LPEN_W as FMPI2C1LPEN_W;
///Field `PWRLPEN` writer - Power interface clock enable during Sleep mode
pub use TIM5LPEN_W as PWRLPEN_W;
///Field `DACLPEN` writer - DAC interface clock enable during sleep mode
pub use TIM5LPEN_W as DACLPEN_W;
impl R {
    ///Bit 3 - TIM5 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - LPTIM1 clock enable during sleep mode
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable during sleep mode
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable during Sleep mode
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - FMPI2C1 clock enable during Sleep
    #[inline(always)]
    pub fn fmpi2c1lpen(&self) -> FMPI2C1LPEN_R {
        FMPI2C1LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC interface clock enable during sleep mode
    #[inline(always)]
    pub fn daclpen(&self) -> DACLPEN_R {
        DACLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LPENR")
            .field("tim5lpen", &self.tim5lpen())
            .field("tim6lpen", &self.tim6lpen())
            .field("lptim1lpen", &self.lptim1lpen())
            .field("rtcapblpen", &self.rtcapblpen())
            .field("wwdglpen", &self.wwdglpen())
            .field("spi2lpen", &self.spi2lpen())
            .field("usart2lpen", &self.usart2lpen())
            .field("i2c1lpen", &self.i2c1lpen())
            .field("i2c2lpen", &self.i2c2lpen())
            .field("fmpi2c1lpen", &self.fmpi2c1lpen())
            .field("pwrlpen", &self.pwrlpen())
            .field("daclpen", &self.daclpen())
            .finish()
    }
}
impl W {
    ///Bit 3 - TIM5 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<'_, APB1LPENRrs> {
        TIM5LPEN_W::new(self, 3)
    }
    ///Bit 4 - TIM6 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<'_, APB1LPENRrs> {
        TIM6LPEN_W::new(self, 4)
    }
    ///Bit 9 - LPTIM1 clock enable during sleep mode
    #[inline(always)]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W<'_, APB1LPENRrs> {
        LPTIM1LPEN_W::new(self, 9)
    }
    ///Bit 10 - RTC APB clock enable during sleep mode
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<'_, APB1LPENRrs> {
        RTCAPBLPEN_W::new(self, 10)
    }
    ///Bit 11 - Window watchdog clock enable during Sleep mode
    #[inline(always)]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<'_, APB1LPENRrs> {
        WWDGLPEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<'_, APB1LPENRrs> {
        SPI2LPEN_W::new(self, 14)
    }
    ///Bit 17 - USART2 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<'_, APB1LPENRrs> {
        USART2LPEN_W::new(self, 17)
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<'_, APB1LPENRrs> {
        I2C1LPEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<'_, APB1LPENRrs> {
        I2C2LPEN_W::new(self, 22)
    }
    ///Bit 24 - FMPI2C1 clock enable during Sleep
    #[inline(always)]
    pub fn fmpi2c1lpen(&mut self) -> FMPI2C1LPEN_W<'_, APB1LPENRrs> {
        FMPI2C1LPEN_W::new(self, 24)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode
    #[inline(always)]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W<'_, APB1LPENRrs> {
        PWRLPEN_W::new(self, 28)
    }
    ///Bit 29 - DAC interface clock enable during sleep mode
    #[inline(always)]
    pub fn daclpen(&mut self) -> DACLPEN_W<'_, APB1LPENRrs> {
        DACLPEN_W::new(self, 29)
    }
}
/**APB1 peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`apb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F410.html#RCC:APB1LPENR)*/
pub struct APB1LPENRrs;
impl crate::RegisterSpec for APB1LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1lpenr::R`](R) reader structure
impl crate::Readable for APB1LPENRrs {}
///`write(|w| ..)` method takes [`apb1lpenr::W`](W) writer structure
impl crate::Writable for APB1LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LPENR to value 0x36fe_c9ff
impl crate::Resettable for APB1LPENRrs {
    const RESET_VALUE: u32 = 0x36fe_c9ff;
}
