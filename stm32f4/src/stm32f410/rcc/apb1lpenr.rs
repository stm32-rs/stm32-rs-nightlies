#[doc = "Register `APB1LPENR` reader"]
pub type R = crate::R<APB1LPENRrs>;
#[doc = "Register `APB1LPENR` writer"]
pub type W = crate::W<APB1LPENRrs>;
#[doc = "TIM5 clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM5LPEN {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<TIM5LPEN> for bool {
    #[inline(always)]
    fn from(variant: TIM5LPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM5LPEN` reader - TIM5 clock enable during Sleep mode"]
pub type TIM5LPEN_R = crate::BitReader<TIM5LPEN>;
impl TIM5LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM5LPEN {
        match self.bits {
            false => TIM5LPEN::DisabledInSleep,
            true => TIM5LPEN::EnabledInSleep,
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == TIM5LPEN::DisabledInSleep
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == TIM5LPEN::EnabledInSleep
    }
}
#[doc = "Field `TIM5LPEN` writer - TIM5 clock enable during Sleep mode"]
pub type TIM5LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM5LPEN>;
impl<'a, REG> TIM5LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(TIM5LPEN::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(TIM5LPEN::EnabledInSleep)
    }
}
#[doc = "Field `TIM6LPEN` reader - TIM6 clock enable during Sleep mode"]
pub use TIM5LPEN_R as TIM6LPEN_R;
#[doc = "Field `LPTIM1LPEN` reader - LPTIM1 clock enable during sleep mode"]
pub use TIM5LPEN_R as LPTIM1LPEN_R;
#[doc = "Field `RTCAPBLPEN` reader - RTC APB clock enable during sleep mode"]
pub use TIM5LPEN_R as RTCAPBLPEN_R;
#[doc = "Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode"]
pub use TIM5LPEN_R as WWDGLPEN_R;
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during Sleep mode"]
pub use TIM5LPEN_R as SPI2LPEN_R;
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during Sleep mode"]
pub use TIM5LPEN_R as USART2LPEN_R;
#[doc = "Field `I2C1LPEN` reader - I2C1 clock enable during Sleep mode"]
pub use TIM5LPEN_R as I2C1LPEN_R;
#[doc = "Field `I2C2LPEN` reader - I2C2 clock enable during Sleep mode"]
pub use TIM5LPEN_R as I2C2LPEN_R;
#[doc = "Field `FMPI2C1LPEN` reader - FMPI2C1 clock enable during Sleep"]
pub use TIM5LPEN_R as FMPI2C1LPEN_R;
#[doc = "Field `PWRLPEN` reader - Power interface clock enable during Sleep mode"]
pub use TIM5LPEN_R as PWRLPEN_R;
#[doc = "Field `DACLPEN` reader - DAC interface clock enable during sleep mode"]
pub use TIM5LPEN_R as DACLPEN_R;
#[doc = "Field `TIM6LPEN` writer - TIM6 clock enable during Sleep mode"]
pub use TIM5LPEN_W as TIM6LPEN_W;
#[doc = "Field `LPTIM1LPEN` writer - LPTIM1 clock enable during sleep mode"]
pub use TIM5LPEN_W as LPTIM1LPEN_W;
#[doc = "Field `RTCAPBLPEN` writer - RTC APB clock enable during sleep mode"]
pub use TIM5LPEN_W as RTCAPBLPEN_W;
#[doc = "Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode"]
pub use TIM5LPEN_W as WWDGLPEN_W;
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during Sleep mode"]
pub use TIM5LPEN_W as SPI2LPEN_W;
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during Sleep mode"]
pub use TIM5LPEN_W as USART2LPEN_W;
#[doc = "Field `I2C1LPEN` writer - I2C1 clock enable during Sleep mode"]
pub use TIM5LPEN_W as I2C1LPEN_W;
#[doc = "Field `I2C2LPEN` writer - I2C2 clock enable during Sleep mode"]
pub use TIM5LPEN_W as I2C2LPEN_W;
#[doc = "Field `FMPI2C1LPEN` writer - FMPI2C1 clock enable during Sleep"]
pub use TIM5LPEN_W as FMPI2C1LPEN_W;
#[doc = "Field `PWRLPEN` writer - Power interface clock enable during Sleep mode"]
pub use TIM5LPEN_W as PWRLPEN_W;
#[doc = "Field `DACLPEN` writer - DAC interface clock enable during sleep mode"]
pub use TIM5LPEN_W as DACLPEN_W;
impl R {
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during sleep mode"]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - FMPI2C1 clock enable during Sleep"]
    #[inline(always)]
    pub fn fmpi2c1lpen(&self) -> FMPI2C1LPEN_R {
        FMPI2C1LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable during sleep mode"]
    #[inline(always)]
    pub fn daclpen(&self) -> DACLPEN_R {
        DACLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<APB1LPENRrs> {
        TIM5LPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<APB1LPENRrs> {
        TIM6LPEN_W::new(self, 4)
    }
    #[doc = "Bit 9 - LPTIM1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W<APB1LPENRrs> {
        LPTIM1LPEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - RTC APB clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<APB1LPENRrs> {
        RTCAPBLPEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<APB1LPENRrs> {
        WWDGLPEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<APB1LPENRrs> {
        SPI2LPEN_W::new(self, 14)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<APB1LPENRrs> {
        USART2LPEN_W::new(self, 17)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<APB1LPENRrs> {
        I2C1LPEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<APB1LPENRrs> {
        I2C2LPEN_W::new(self, 22)
    }
    #[doc = "Bit 24 - FMPI2C1 clock enable during Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn fmpi2c1lpen(&mut self) -> FMPI2C1LPEN_W<APB1LPENRrs> {
        FMPI2C1LPEN_W::new(self, 24)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W<APB1LPENRrs> {
        PWRLPEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC interface clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn daclpen(&mut self) -> DACLPEN_W<APB1LPENRrs> {
        DACLPEN_W::new(self, 29)
    }
}
#[doc = "APB1 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LPENRrs;
impl crate::RegisterSpec for APB1LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lpenr::R`](R) reader structure"]
impl crate::Readable for APB1LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1lpenr::W`](W) writer structure"]
impl crate::Writable for APB1LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1LPENR to value 0x36fe_c9ff"]
impl crate::Resettable for APB1LPENRrs {
    const RESET_VALUE: u32 = 0x36fe_c9ff;
}
