///Register `APB2LPENR` reader
pub type R = crate::R<APB2LPENRrs>;
///Register `APB2LPENR` writer
pub type W = crate::W<APB2LPENRrs>;
/**TIM1 clock enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1LPEN {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<TIM1LPEN> for bool {
    #[inline(always)]
    fn from(variant: TIM1LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1LPEN` reader - TIM1 clock enable during Sleep mode
pub type TIM1LPEN_R = crate::BitReader<TIM1LPEN>;
impl TIM1LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1LPEN {
        match self.bits {
            false => TIM1LPEN::DisabledInSleep,
            true => TIM1LPEN::EnabledInSleep,
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == TIM1LPEN::DisabledInSleep
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == TIM1LPEN::EnabledInSleep
    }
}
///Field `TIM1LPEN` writer - TIM1 clock enable during Sleep mode
pub type TIM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1LPEN>;
impl<'a, REG> TIM1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1LPEN::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1LPEN::EnabledInSleep)
    }
}
///Field `USART1LPEN` reader - USART1 clock enable during Sleep mode
pub use TIM1LPEN_R as USART1LPEN_R;
///Field `USART6LPEN` reader - USART6 clock enable during Sleep mode
pub use TIM1LPEN_R as USART6LPEN_R;
///Field `ADC1LPEN` reader - ADC1 clock enable during Sleep mode
pub use TIM1LPEN_R as ADC1LPEN_R;
///Field `SDIOLPEN` reader - SDIO clock enable during Sleep mode
pub use TIM1LPEN_R as SDIOLPEN_R;
///Field `SPI1LPEN` reader - SPI 1 clock enable during Sleep mode
pub use TIM1LPEN_R as SPI1LPEN_R;
///Field `SYSCFGLPEN` reader - System configuration controller clock enable during Sleep mode
pub use TIM1LPEN_R as SYSCFGLPEN_R;
///Field `EXTITLPEN` reader - EXTI and External IT clock enable during sleep mode
pub use TIM1LPEN_R as EXTITLPEN_R;
///Field `TIM9LPEN` reader - TIM9 clock enable during sleep mode
pub use TIM1LPEN_R as TIM9LPEN_R;
///Field `TIM11LPEN` reader - TIM11 clock enable during Sleep mode
pub use TIM1LPEN_R as TIM11LPEN_R;
///Field `SPI5LPEN` reader - SPI5 clock enable during Sleep mode
pub use TIM1LPEN_R as SPI5LPEN_R;
///Field `USART1LPEN` writer - USART1 clock enable during Sleep mode
pub use TIM1LPEN_W as USART1LPEN_W;
///Field `USART6LPEN` writer - USART6 clock enable during Sleep mode
pub use TIM1LPEN_W as USART6LPEN_W;
///Field `ADC1LPEN` writer - ADC1 clock enable during Sleep mode
pub use TIM1LPEN_W as ADC1LPEN_W;
///Field `SDIOLPEN` writer - SDIO clock enable during Sleep mode
pub use TIM1LPEN_W as SDIOLPEN_W;
///Field `SPI1LPEN` writer - SPI 1 clock enable during Sleep mode
pub use TIM1LPEN_W as SPI1LPEN_W;
///Field `SYSCFGLPEN` writer - System configuration controller clock enable during Sleep mode
pub use TIM1LPEN_W as SYSCFGLPEN_W;
///Field `EXTITLPEN` writer - EXTI and External IT clock enable during sleep mode
pub use TIM1LPEN_W as EXTITLPEN_W;
///Field `TIM9LPEN` writer - TIM9 clock enable during sleep mode
pub use TIM1LPEN_W as TIM9LPEN_W;
///Field `TIM11LPEN` writer - TIM11 clock enable during Sleep mode
pub use TIM1LPEN_W as TIM11LPEN_W;
///Field `SPI5LPEN` writer - SPI5 clock enable during Sleep mode
pub use TIM1LPEN_W as SPI5LPEN_W;
impl R {
    ///Bit 0 - TIM1 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - USART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USART6 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - ADC1 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc1lpen(&self) -> ADC1LPEN_R {
        ADC1LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - SDIO clock enable during Sleep mode
    #[inline(always)]
    pub fn sdiolpen(&self) -> SDIOLPEN_R {
        SDIOLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - System configuration controller clock enable during Sleep mode
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EXTI and External IT clock enable during sleep mode
    #[inline(always)]
    pub fn extitlpen(&self) -> EXTITLPEN_R {
        EXTITLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TIM9 clock enable during sleep mode
    #[inline(always)]
    pub fn tim9lpen(&self) -> TIM9LPEN_R {
        TIM9LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - TIM11 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim11lpen(&self) -> TIM11LPEN_R {
        TIM11LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - SPI5 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2LPENR")
            .field("tim1lpen", &self.tim1lpen())
            .field("usart1lpen", &self.usart1lpen())
            .field("usart6lpen", &self.usart6lpen())
            .field("adc1lpen", &self.adc1lpen())
            .field("sdiolpen", &self.sdiolpen())
            .field("spi1lpen", &self.spi1lpen())
            .field("syscfglpen", &self.syscfglpen())
            .field("extitlpen", &self.extitlpen())
            .field("tim9lpen", &self.tim9lpen())
            .field("tim11lpen", &self.tim11lpen())
            .field("spi5lpen", &self.spi5lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<'_, APB2LPENRrs> {
        TIM1LPEN_W::new(self, 0)
    }
    ///Bit 4 - USART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<'_, APB2LPENRrs> {
        USART1LPEN_W::new(self, 4)
    }
    ///Bit 5 - USART6 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<'_, APB2LPENRrs> {
        USART6LPEN_W::new(self, 5)
    }
    ///Bit 8 - ADC1 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc1lpen(&mut self) -> ADC1LPEN_W<'_, APB2LPENRrs> {
        ADC1LPEN_W::new(self, 8)
    }
    ///Bit 11 - SDIO clock enable during Sleep mode
    #[inline(always)]
    pub fn sdiolpen(&mut self) -> SDIOLPEN_W<'_, APB2LPENRrs> {
        SDIOLPEN_W::new(self, 11)
    }
    ///Bit 12 - SPI 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<'_, APB2LPENRrs> {
        SPI1LPEN_W::new(self, 12)
    }
    ///Bit 14 - System configuration controller clock enable during Sleep mode
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<'_, APB2LPENRrs> {
        SYSCFGLPEN_W::new(self, 14)
    }
    ///Bit 15 - EXTI and External IT clock enable during sleep mode
    #[inline(always)]
    pub fn extitlpen(&mut self) -> EXTITLPEN_W<'_, APB2LPENRrs> {
        EXTITLPEN_W::new(self, 15)
    }
    ///Bit 16 - TIM9 clock enable during sleep mode
    #[inline(always)]
    pub fn tim9lpen(&mut self) -> TIM9LPEN_W<'_, APB2LPENRrs> {
        TIM9LPEN_W::new(self, 16)
    }
    ///Bit 18 - TIM11 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim11lpen(&mut self) -> TIM11LPEN_W<'_, APB2LPENRrs> {
        TIM11LPEN_W::new(self, 18)
    }
    ///Bit 20 - SPI5 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<'_, APB2LPENRrs> {
        SPI5LPEN_W::new(self, 20)
    }
}
/**APB2 peripheral clock enabled in low power mode register

You can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F410.html#RCC:APB2LPENR)*/
pub struct APB2LPENRrs;
impl crate::RegisterSpec for APB2LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2lpenr::R`](R) reader structure
impl crate::Readable for APB2LPENRrs {}
///`write(|w| ..)` method takes [`apb2lpenr::W`](W) writer structure
impl crate::Writable for APB2LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2LPENR to value 0x0007_5f33
impl crate::Resettable for APB2LPENRrs {
    const RESET_VALUE: u32 = 0x0007_5f33;
}
