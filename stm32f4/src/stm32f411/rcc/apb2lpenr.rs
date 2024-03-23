#[doc = "Register `APB2LPENR` reader"]
pub type R = crate::R<APB2LPENRrs>;
#[doc = "Register `APB2LPENR` writer"]
pub type W = crate::W<APB2LPENRrs>;
#[doc = "TIM1 clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1LPEN {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<TIM1LPEN> for bool {
    #[inline(always)]
    fn from(variant: TIM1LPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1LPEN` reader - TIM1 clock enable during Sleep mode"]
pub type TIM1LPEN_R = crate::BitReader<TIM1LPEN>;
impl TIM1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM1LPEN {
        match self.bits {
            false => TIM1LPEN::DisabledInSleep,
            true => TIM1LPEN::EnabledInSleep,
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == TIM1LPEN::DisabledInSleep
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == TIM1LPEN::EnabledInSleep
    }
}
#[doc = "Field `TIM1LPEN` writer - TIM1 clock enable during Sleep mode"]
pub type TIM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1LPEN>;
impl<'a, REG> TIM1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1LPEN::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1LPEN::EnabledInSleep)
    }
}
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during Sleep mode"]
pub use TIM1LPEN_R as USART1LPEN_R;
#[doc = "Field `USART6LPEN` reader - USART6 clock enable during Sleep mode"]
pub use TIM1LPEN_R as USART6LPEN_R;
#[doc = "Field `ADC1LPEN` reader - ADC1 clock enable during Sleep mode"]
pub use TIM1LPEN_R as ADC1LPEN_R;
#[doc = "Field `SDIOLPEN` reader - SDIO clock enable during Sleep mode"]
pub use TIM1LPEN_R as SDIOLPEN_R;
#[doc = "Field `SPI1LPEN` reader - SPI 1 clock enable during Sleep mode"]
pub use TIM1LPEN_R as SPI1LPEN_R;
#[doc = "Field `SPI4LPEN` reader - SPI4 clock enable during Sleep mode"]
pub use TIM1LPEN_R as SPI4LPEN_R;
#[doc = "Field `SYSCFGLPEN` reader - System configuration controller clock enable during Sleep mode"]
pub use TIM1LPEN_R as SYSCFGLPEN_R;
#[doc = "Field `TIM9LPEN` reader - TIM9 clock enable during sleep mode"]
pub use TIM1LPEN_R as TIM9LPEN_R;
#[doc = "Field `TIM10LPEN` reader - TIM10 clock enable during Sleep mode"]
pub use TIM1LPEN_R as TIM10LPEN_R;
#[doc = "Field `TIM11LPEN` reader - TIM11 clock enable during Sleep mode"]
pub use TIM1LPEN_R as TIM11LPEN_R;
#[doc = "Field `SPI5LPEN` reader - SPI5 clock enable during Sleep mode"]
pub use TIM1LPEN_R as SPI5LPEN_R;
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during Sleep mode"]
pub use TIM1LPEN_W as USART1LPEN_W;
#[doc = "Field `USART6LPEN` writer - USART6 clock enable during Sleep mode"]
pub use TIM1LPEN_W as USART6LPEN_W;
#[doc = "Field `ADC1LPEN` writer - ADC1 clock enable during Sleep mode"]
pub use TIM1LPEN_W as ADC1LPEN_W;
#[doc = "Field `SDIOLPEN` writer - SDIO clock enable during Sleep mode"]
pub use TIM1LPEN_W as SDIOLPEN_W;
#[doc = "Field `SPI1LPEN` writer - SPI 1 clock enable during Sleep mode"]
pub use TIM1LPEN_W as SPI1LPEN_W;
#[doc = "Field `SPI4LPEN` writer - SPI4 clock enable during Sleep mode"]
pub use TIM1LPEN_W as SPI4LPEN_W;
#[doc = "Field `SYSCFGLPEN` writer - System configuration controller clock enable during Sleep mode"]
pub use TIM1LPEN_W as SYSCFGLPEN_W;
#[doc = "Field `TIM9LPEN` writer - TIM9 clock enable during sleep mode"]
pub use TIM1LPEN_W as TIM9LPEN_W;
#[doc = "Field `TIM10LPEN` writer - TIM10 clock enable during Sleep mode"]
pub use TIM1LPEN_W as TIM10LPEN_W;
#[doc = "Field `TIM11LPEN` writer - TIM11 clock enable during Sleep mode"]
pub use TIM1LPEN_W as TIM11LPEN_W;
#[doc = "Field `SPI5LPEN` writer - SPI5 clock enable during Sleep mode"]
pub use TIM1LPEN_W as SPI5LPEN_W;
impl R {
    #[doc = "Bit 0 - TIM1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adc1lpen(&self) -> ADC1LPEN_R {
        ADC1LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sdiolpen(&self) -> SDIOLPEN_R {
        SDIOLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - System configuration controller clock enable during Sleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM9 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tim9lpen(&self) -> TIM9LPEN_R {
        TIM9LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM10 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim10lpen(&self) -> TIM10LPEN_R {
        TIM10LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM11 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim11lpen(&self) -> TIM11LPEN_R {
        TIM11LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<APB2LPENRrs> {
        TIM1LPEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<APB2LPENRrs> {
        USART1LPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<APB2LPENRrs> {
        USART6LPEN_W::new(self, 5)
    }
    #[doc = "Bit 8 - ADC1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc1lpen(&mut self) -> ADC1LPEN_W<APB2LPENRrs> {
        ADC1LPEN_W::new(self, 8)
    }
    #[doc = "Bit 11 - SDIO clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdiolpen(&mut self) -> SDIOLPEN_W<APB2LPENRrs> {
        SDIOLPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<APB2LPENRrs> {
        SPI1LPEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - SPI4 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<APB2LPENRrs> {
        SPI4LPEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - System configuration controller clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<APB2LPENRrs> {
        SYSCFGLPEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - TIM9 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim9lpen(&mut self) -> TIM9LPEN_W<APB2LPENRrs> {
        TIM9LPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM10 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim10lpen(&mut self) -> TIM10LPEN_W<APB2LPENRrs> {
        TIM10LPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM11 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim11lpen(&mut self) -> TIM11LPEN_W<APB2LPENRrs> {
        TIM11LPEN_W::new(self, 18)
    }
    #[doc = "Bit 20 - SPI5 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<APB2LPENRrs> {
        SPI5LPEN_W::new(self, 20)
    }
}
#[doc = "APB2 peripheral clock enabled in low power mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2LPENRrs;
impl crate::RegisterSpec for APB2LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2lpenr::R`](R) reader structure"]
impl crate::Readable for APB2LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2lpenr::W`](W) writer structure"]
impl crate::Writable for APB2LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2LPENR to value 0x0007_5f33"]
impl crate::Resettable for APB2LPENRrs {
    const RESET_VALUE: u32 = 0x0007_5f33;
}
