///Register `APB2LPENR` reader
pub type R = crate::R<APB2LPENRrs>;
///Register `APB2LPENR` writer
pub type W = crate::W<APB2LPENRrs>;
///Field `TIM1LPEN` reader - TIM1 clock enable during Sleep mode
pub type TIM1LPEN_R = crate::BitReader;
///Field `TIM1LPEN` writer - TIM1 clock enable during Sleep mode
pub type TIM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8LPEN` reader - TIM8 clock enable during Sleep mode
pub type TIM8LPEN_R = crate::BitReader;
///Field `TIM8LPEN` writer - TIM8 clock enable during Sleep mode
pub type TIM8LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1LPEN` reader - USART1 clock enable during Sleep mode
pub type USART1LPEN_R = crate::BitReader;
///Field `USART1LPEN` writer - USART1 clock enable during Sleep mode
pub type USART1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6LPEN` reader - USART6 clock enable during Sleep mode
pub type USART6LPEN_R = crate::BitReader;
///Field `USART6LPEN` writer - USART6 clock enable during Sleep mode
pub type USART6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC1LPEN` reader - ADC1 clock enable during Sleep mode
pub type ADC1LPEN_R = crate::BitReader;
///Field `ADC1LPEN` writer - ADC1 clock enable during Sleep mode
pub type ADC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC2LPEN` reader - ADC2 clock enable during Sleep mode
pub type ADC2LPEN_R = crate::BitReader;
///Field `ADC2LPEN` writer - ADC2 clock enable during Sleep mode
pub type ADC2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC3LPEN` reader - ADC 3 clock enable during Sleep mode
pub type ADC3LPEN_R = crate::BitReader;
///Field `ADC3LPEN` writer - ADC 3 clock enable during Sleep mode
pub type ADC3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIOLPEN` reader - SDIO clock enable during Sleep mode
pub type SDIOLPEN_R = crate::BitReader;
///Field `SDIOLPEN` writer - SDIO clock enable during Sleep mode
pub type SDIOLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1LPEN` reader - SPI 1 clock enable during Sleep mode
pub type SPI1LPEN_R = crate::BitReader;
///Field `SPI1LPEN` writer - SPI 1 clock enable during Sleep mode
pub type SPI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4LPEN` reader - SPI 4 clock enable during Sleep mode
pub type SPI4LPEN_R = crate::BitReader;
///Field `SPI4LPEN` writer - SPI 4 clock enable during Sleep mode
pub type SPI4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGLPEN` reader - System configuration controller clock enable during Sleep mode
pub type SYSCFGLPEN_R = crate::BitReader;
///Field `SYSCFGLPEN` writer - System configuration controller clock enable during Sleep mode
pub type SYSCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9LPEN` reader - TIM9 clock enable during sleep mode
pub type TIM9LPEN_R = crate::BitReader;
///Field `TIM9LPEN` writer - TIM9 clock enable during sleep mode
pub type TIM9LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM10LPEN` reader - TIM10 clock enable during Sleep mode
pub type TIM10LPEN_R = crate::BitReader;
///Field `TIM10LPEN` writer - TIM10 clock enable during Sleep mode
pub type TIM10LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM11LPEN` reader - TIM11 clock enable during Sleep mode
pub type TIM11LPEN_R = crate::BitReader;
///Field `TIM11LPEN` writer - TIM11 clock enable during Sleep mode
pub type TIM11LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5LPEN` reader - SPI 5 clock enable during Sleep mode
pub type SPI5LPEN_R = crate::BitReader;
///Field `SPI5LPEN` writer - SPI 5 clock enable during Sleep mode
pub type SPI5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI6LPEN` reader - SPI 6 clock enable during Sleep mode
pub type SPI6LPEN_R = crate::BitReader;
///Field `SPI6LPEN` writer - SPI 6 clock enable during Sleep mode
pub type SPI6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("sdiolpen", &self.sdiolpen())
            .field("spi1lpen", &self.spi1lpen())
            .field("spi4lpen", &self.spi4lpen())
            .field("syscfglpen", &self.syscfglpen())
            .field("tim9lpen", &self.tim9lpen())
            .field("tim10lpen", &self.tim10lpen())
            .field("tim11lpen", &self.tim11lpen())
            .field("spi5lpen", &self.spi5lpen())
            .field("spi6lpen", &self.spi6lpen())
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
}
/**APB2 peripheral clock enabled in low power mode register

You can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#RCC:APB2LPENR)*/
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
