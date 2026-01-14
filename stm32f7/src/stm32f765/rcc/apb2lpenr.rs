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
///Field `TIM8LPEN` reader - TIM8 clock enable during Sleep mode
pub use TIM1LPEN_R as TIM8LPEN_R;
///Field `USART1LPEN` reader - USART1 clock enable during Sleep mode
pub use TIM1LPEN_R as USART1LPEN_R;
///Field `USART6LPEN` reader - USART6 clock enable during Sleep mode
pub use TIM1LPEN_R as USART6LPEN_R;
///Field `SDMMC2LPEN` reader - SDMMC2 clock enable during Sleep mode
pub use TIM1LPEN_R as SDMMC2LPEN_R;
///Field `ADC1LPEN` reader - ADC1 clock enable during Sleep mode
pub use TIM1LPEN_R as ADC1LPEN_R;
///Field `ADC2LPEN` reader - ADC2 clock enable during Sleep mode
pub use TIM1LPEN_R as ADC2LPEN_R;
///Field `ADC3LPEN` reader - ADC 3 clock enable during Sleep mode
pub use TIM1LPEN_R as ADC3LPEN_R;
///Field `SDMMC1LPEN` reader - SDMMC1 clock enable during Sleep mode
pub use TIM1LPEN_R as SDMMC1LPEN_R;
///Field `SPI1LPEN` reader - SPI 1 clock enable during Sleep mode
pub use TIM1LPEN_R as SPI1LPEN_R;
///Field `SPI4LPEN` reader - SPI 4 clock enable during Sleep mode
pub use TIM1LPEN_R as SPI4LPEN_R;
///Field `SYSCFGLPEN` reader - System configuration controller clock enable during Sleep mode
pub use TIM1LPEN_R as SYSCFGLPEN_R;
///Field `TIM9LPEN` reader - TIM9 clock enable during sleep mode
pub use TIM1LPEN_R as TIM9LPEN_R;
///Field `TIM10LPEN` reader - TIM10 clock enable during Sleep mode
pub use TIM1LPEN_R as TIM10LPEN_R;
///Field `TIM11LPEN` reader - TIM11 clock enable during Sleep mode
pub use TIM1LPEN_R as TIM11LPEN_R;
///Field `SPI5LPEN` reader - SPI 5 clock enable during Sleep mode
pub use TIM1LPEN_R as SPI5LPEN_R;
///Field `SPI6LPEN` reader - SPI 6 clock enable during Sleep mode
pub use TIM1LPEN_R as SPI6LPEN_R;
///Field `SAI1LPEN` reader - SAI1 clock enable during sleep mode
pub use TIM1LPEN_R as SAI1LPEN_R;
///Field `SAI2LPEN` reader - SAI2 clock enable during sleep mode
pub use TIM1LPEN_R as SAI2LPEN_R;
///Field `LTDCLPEN` reader - LTDC clock enable during sleep mode
pub use TIM1LPEN_R as LTDCLPEN_R;
///Field `DSILPEN` reader - DSI clock enable during Sleep mode
pub use TIM1LPEN_R as DSILPEN_R;
///Field `DFSDM1LPEN` reader - DFSDM1 clock enable during Sleep mode
pub use TIM1LPEN_R as DFSDM1LPEN_R;
///Field `MDIOLPEN` reader - MDIO clock enable during Sleep mode
pub use TIM1LPEN_R as MDIOLPEN_R;
///Field `TIM8LPEN` writer - TIM8 clock enable during Sleep mode
pub use TIM1LPEN_W as TIM8LPEN_W;
///Field `USART1LPEN` writer - USART1 clock enable during Sleep mode
pub use TIM1LPEN_W as USART1LPEN_W;
///Field `USART6LPEN` writer - USART6 clock enable during Sleep mode
pub use TIM1LPEN_W as USART6LPEN_W;
///Field `SDMMC2LPEN` writer - SDMMC2 clock enable during Sleep mode
pub use TIM1LPEN_W as SDMMC2LPEN_W;
///Field `ADC1LPEN` writer - ADC1 clock enable during Sleep mode
pub use TIM1LPEN_W as ADC1LPEN_W;
///Field `ADC2LPEN` writer - ADC2 clock enable during Sleep mode
pub use TIM1LPEN_W as ADC2LPEN_W;
///Field `ADC3LPEN` writer - ADC 3 clock enable during Sleep mode
pub use TIM1LPEN_W as ADC3LPEN_W;
///Field `SDMMC1LPEN` writer - SDMMC1 clock enable during Sleep mode
pub use TIM1LPEN_W as SDMMC1LPEN_W;
///Field `SPI1LPEN` writer - SPI 1 clock enable during Sleep mode
pub use TIM1LPEN_W as SPI1LPEN_W;
///Field `SPI4LPEN` writer - SPI 4 clock enable during Sleep mode
pub use TIM1LPEN_W as SPI4LPEN_W;
///Field `SYSCFGLPEN` writer - System configuration controller clock enable during Sleep mode
pub use TIM1LPEN_W as SYSCFGLPEN_W;
///Field `TIM9LPEN` writer - TIM9 clock enable during sleep mode
pub use TIM1LPEN_W as TIM9LPEN_W;
///Field `TIM10LPEN` writer - TIM10 clock enable during Sleep mode
pub use TIM1LPEN_W as TIM10LPEN_W;
///Field `TIM11LPEN` writer - TIM11 clock enable during Sleep mode
pub use TIM1LPEN_W as TIM11LPEN_W;
///Field `SPI5LPEN` writer - SPI 5 clock enable during Sleep mode
pub use TIM1LPEN_W as SPI5LPEN_W;
///Field `SPI6LPEN` writer - SPI 6 clock enable during Sleep mode
pub use TIM1LPEN_W as SPI6LPEN_W;
///Field `SAI1LPEN` writer - SAI1 clock enable during sleep mode
pub use TIM1LPEN_W as SAI1LPEN_W;
///Field `SAI2LPEN` writer - SAI2 clock enable during sleep mode
pub use TIM1LPEN_W as SAI2LPEN_W;
///Field `LTDCLPEN` writer - LTDC clock enable during sleep mode
pub use TIM1LPEN_W as LTDCLPEN_W;
///Field `DSILPEN` writer - DSI clock enable during Sleep mode
pub use TIM1LPEN_W as DSILPEN_W;
///Field `DFSDM1LPEN` writer - DFSDM1 clock enable during Sleep mode
pub use TIM1LPEN_W as DFSDM1LPEN_W;
///Field `MDIOLPEN` writer - MDIO clock enable during Sleep mode
pub use TIM1LPEN_W as MDIOLPEN_W;
impl R {
    ///Bit 0 - TIM1 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 1) != 0)
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
    ///Bit 7 - SDMMC2 clock enable during Sleep mode
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ADC1 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc1lpen(&self) -> ADC1LPEN_R {
        ADC1LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ADC2 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc2lpen(&self) -> ADC2LPEN_R {
        ADC2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC 3 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc3lpen(&self) -> ADC3LPEN_R {
        ADC3LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SDMMC1 clock enable during Sleep mode
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI 4 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - System configuration controller clock enable during Sleep mode
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM9 clock enable during sleep mode
    #[inline(always)]
    pub fn tim9lpen(&self) -> TIM9LPEN_R {
        TIM9LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM10 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim10lpen(&self) -> TIM10LPEN_R {
        TIM10LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM11 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim11lpen(&self) -> TIM11LPEN_R {
        TIM11LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - SPI 5 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SPI 6 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI1 clock enable during sleep mode
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SAI2 clock enable during sleep mode
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 26 - LTDC clock enable during sleep mode
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DSI clock enable during Sleep mode
    #[inline(always)]
    pub fn dsilpen(&self) -> DSILPEN_R {
        DSILPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - DFSDM1 clock enable during Sleep mode
    #[inline(always)]
    pub fn dfsdm1lpen(&self) -> DFSDM1LPEN_R {
        DFSDM1LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - MDIO clock enable during Sleep mode
    #[inline(always)]
    pub fn mdiolpen(&self) -> MDIOLPEN_R {
        MDIOLPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2LPENR")
            .field("tim1lpen", &self.tim1lpen())
            .field("tim8lpen", &self.tim8lpen())
            .field("usart1lpen", &self.usart1lpen())
            .field("usart6lpen", &self.usart6lpen())
            .field("adc1lpen", &self.adc1lpen())
            .field("adc2lpen", &self.adc2lpen())
            .field("adc3lpen", &self.adc3lpen())
            .field("spi1lpen", &self.spi1lpen())
            .field("spi4lpen", &self.spi4lpen())
            .field("syscfglpen", &self.syscfglpen())
            .field("tim9lpen", &self.tim9lpen())
            .field("tim10lpen", &self.tim10lpen())
            .field("tim11lpen", &self.tim11lpen())
            .field("spi5lpen", &self.spi5lpen())
            .field("spi6lpen", &self.spi6lpen())
            .field("sai1lpen", &self.sai1lpen())
            .field("ltdclpen", &self.ltdclpen())
            .field("sai2lpen", &self.sai2lpen())
            .field("sdmmc1lpen", &self.sdmmc1lpen())
            .field("mdiolpen", &self.mdiolpen())
            .field("dfsdm1lpen", &self.dfsdm1lpen())
            .field("dsilpen", &self.dsilpen())
            .field("sdmmc2lpen", &self.sdmmc2lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<'_, APB2LPENRrs> {
        TIM1LPEN_W::new(self, 0)
    }
    ///Bit 1 - TIM8 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<'_, APB2LPENRrs> {
        TIM8LPEN_W::new(self, 1)
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
    ///Bit 7 - SDMMC2 clock enable during Sleep mode
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<'_, APB2LPENRrs> {
        SDMMC2LPEN_W::new(self, 7)
    }
    ///Bit 8 - ADC1 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc1lpen(&mut self) -> ADC1LPEN_W<'_, APB2LPENRrs> {
        ADC1LPEN_W::new(self, 8)
    }
    ///Bit 9 - ADC2 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc2lpen(&mut self) -> ADC2LPEN_W<'_, APB2LPENRrs> {
        ADC2LPEN_W::new(self, 9)
    }
    ///Bit 10 - ADC 3 clock enable during Sleep mode
    #[inline(always)]
    pub fn adc3lpen(&mut self) -> ADC3LPEN_W<'_, APB2LPENRrs> {
        ADC3LPEN_W::new(self, 10)
    }
    ///Bit 11 - SDMMC1 clock enable during Sleep mode
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<'_, APB2LPENRrs> {
        SDMMC1LPEN_W::new(self, 11)
    }
    ///Bit 12 - SPI 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<'_, APB2LPENRrs> {
        SPI1LPEN_W::new(self, 12)
    }
    ///Bit 13 - SPI 4 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<'_, APB2LPENRrs> {
        SPI4LPEN_W::new(self, 13)
    }
    ///Bit 14 - System configuration controller clock enable during Sleep mode
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<'_, APB2LPENRrs> {
        SYSCFGLPEN_W::new(self, 14)
    }
    ///Bit 16 - TIM9 clock enable during sleep mode
    #[inline(always)]
    pub fn tim9lpen(&mut self) -> TIM9LPEN_W<'_, APB2LPENRrs> {
        TIM9LPEN_W::new(self, 16)
    }
    ///Bit 17 - TIM10 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim10lpen(&mut self) -> TIM10LPEN_W<'_, APB2LPENRrs> {
        TIM10LPEN_W::new(self, 17)
    }
    ///Bit 18 - TIM11 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim11lpen(&mut self) -> TIM11LPEN_W<'_, APB2LPENRrs> {
        TIM11LPEN_W::new(self, 18)
    }
    ///Bit 20 - SPI 5 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<'_, APB2LPENRrs> {
        SPI5LPEN_W::new(self, 20)
    }
    ///Bit 21 - SPI 6 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<'_, APB2LPENRrs> {
        SPI6LPEN_W::new(self, 21)
    }
    ///Bit 22 - SAI1 clock enable during sleep mode
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<'_, APB2LPENRrs> {
        SAI1LPEN_W::new(self, 22)
    }
    ///Bit 23 - SAI2 clock enable during sleep mode
    #[inline(always)]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W<'_, APB2LPENRrs> {
        SAI2LPEN_W::new(self, 23)
    }
    ///Bit 26 - LTDC clock enable during sleep mode
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<'_, APB2LPENRrs> {
        LTDCLPEN_W::new(self, 26)
    }
    ///Bit 27 - DSI clock enable during Sleep mode
    #[inline(always)]
    pub fn dsilpen(&mut self) -> DSILPEN_W<'_, APB2LPENRrs> {
        DSILPEN_W::new(self, 27)
    }
    ///Bit 29 - DFSDM1 clock enable during Sleep mode
    #[inline(always)]
    pub fn dfsdm1lpen(&mut self) -> DFSDM1LPEN_W<'_, APB2LPENRrs> {
        DFSDM1LPEN_W::new(self, 29)
    }
    ///Bit 30 - MDIO clock enable during Sleep mode
    #[inline(always)]
    pub fn mdiolpen(&mut self) -> MDIOLPEN_W<'_, APB2LPENRrs> {
        MDIOLPEN_W::new(self, 30)
    }
}
/**APB2 peripheral clock enabled in low power mode register

You can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#RCC:APB2LPENR)*/
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
