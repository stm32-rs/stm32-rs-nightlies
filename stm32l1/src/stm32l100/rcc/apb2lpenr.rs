///Register `APB2LPENR` reader
pub type R = crate::R<APB2LPENRrs>;
///Register `APB2LPENR` writer
pub type W = crate::W<APB2LPENRrs>;
/**System configuration controller clock enable during Sleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGLPEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<SYSCFGLPEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGLPEN` reader - System configuration controller clock enable during Sleep mode
pub type SYSCFGLPEN_R = crate::BitReader<SYSCFGLPEN>;
impl SYSCFGLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGLPEN {
        match self.bits {
            false => SYSCFGLPEN::Disabled,
            true => SYSCFGLPEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGLPEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGLPEN::Enabled
    }
}
///Field `SYSCFGLPEN` writer - System configuration controller clock enable during Sleep mode
pub type SYSCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGLPEN>;
impl<'a, REG> SYSCFGLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGLPEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGLPEN::Enabled)
    }
}
///Field `TIM9LPEN` reader - TIM9 timer clock enable during Sleep mode
pub use SYSCFGLPEN_R as TIM9LPEN_R;
///Field `TIM10LPEN` reader - TIM10 timer clock enable during Sleep mode
pub use SYSCFGLPEN_R as TIM10LPEN_R;
///Field `TIM11LPEN` reader - TIM11 timer clock enable during Sleep mode
pub use SYSCFGLPEN_R as TIM11LPEN_R;
///Field `ADC1LPEN` reader - ADC1 interface clock enable during Sleep mode
pub use SYSCFGLPEN_R as ADC1LPEN_R;
///Field `SDIOLPEN` reader - SDIO clock enable during Sleep mode
pub use SYSCFGLPEN_R as SDIOLPEN_R;
///Field `SPI1LPEN` reader - SPI 1 clock enable during Sleep mode
pub use SYSCFGLPEN_R as SPI1LPEN_R;
///Field `USART1LPEN` reader - USART1 clock enable during Sleep mode
pub use SYSCFGLPEN_R as USART1LPEN_R;
///Field `TIM9LPEN` writer - TIM9 timer clock enable during Sleep mode
pub use SYSCFGLPEN_W as TIM9LPEN_W;
///Field `TIM10LPEN` writer - TIM10 timer clock enable during Sleep mode
pub use SYSCFGLPEN_W as TIM10LPEN_W;
///Field `TIM11LPEN` writer - TIM11 timer clock enable during Sleep mode
pub use SYSCFGLPEN_W as TIM11LPEN_W;
///Field `ADC1LPEN` writer - ADC1 interface clock enable during Sleep mode
pub use SYSCFGLPEN_W as ADC1LPEN_W;
///Field `SDIOLPEN` writer - SDIO clock enable during Sleep mode
pub use SYSCFGLPEN_W as SDIOLPEN_W;
///Field `SPI1LPEN` writer - SPI 1 clock enable during Sleep mode
pub use SYSCFGLPEN_W as SPI1LPEN_W;
///Field `USART1LPEN` writer - USART1 clock enable during Sleep mode
pub use SYSCFGLPEN_W as USART1LPEN_W;
impl R {
    ///Bit 0 - System configuration controller clock enable during Sleep mode
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - TIM9 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim9lpen(&self) -> TIM9LPEN_R {
        TIM9LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM10 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim10lpen(&self) -> TIM10LPEN_R {
        TIM10LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM11 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim11lpen(&self) -> TIM11LPEN_R {
        TIM11LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - ADC1 interface clock enable during Sleep mode
    #[inline(always)]
    pub fn adc1lpen(&self) -> ADC1LPEN_R {
        ADC1LPEN_R::new(((self.bits >> 9) & 1) != 0)
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
    ///Bit 14 - USART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2LPENR")
            .field("syscfglpen", &self.syscfglpen())
            .field("usart1lpen", &self.usart1lpen())
            .field("spi1lpen", &self.spi1lpen())
            .field("sdiolpen", &self.sdiolpen())
            .field("adc1lpen", &self.adc1lpen())
            .field("tim11lpen", &self.tim11lpen())
            .field("tim10lpen", &self.tim10lpen())
            .field("tim9lpen", &self.tim9lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - System configuration controller clock enable during Sleep mode
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<'_, APB2LPENRrs> {
        SYSCFGLPEN_W::new(self, 0)
    }
    ///Bit 2 - TIM9 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim9lpen(&mut self) -> TIM9LPEN_W<'_, APB2LPENRrs> {
        TIM9LPEN_W::new(self, 2)
    }
    ///Bit 3 - TIM10 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim10lpen(&mut self) -> TIM10LPEN_W<'_, APB2LPENRrs> {
        TIM10LPEN_W::new(self, 3)
    }
    ///Bit 4 - TIM11 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim11lpen(&mut self) -> TIM11LPEN_W<'_, APB2LPENRrs> {
        TIM11LPEN_W::new(self, 4)
    }
    ///Bit 9 - ADC1 interface clock enable during Sleep mode
    #[inline(always)]
    pub fn adc1lpen(&mut self) -> ADC1LPEN_W<'_, APB2LPENRrs> {
        ADC1LPEN_W::new(self, 9)
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
    ///Bit 14 - USART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<'_, APB2LPENRrs> {
        USART1LPEN_W::new(self, 14)
    }
}
/**APB2 peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RCC:APB2LPENR)*/
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
///`reset()` method sets APB2LPENR to value 0
impl crate::Resettable for APB2LPENRrs {}
