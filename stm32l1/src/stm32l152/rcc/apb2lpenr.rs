///Register `APB2LPENR` reader
pub type R = crate::R<APB2LPENRrs>;
///Register `APB2LPENR` writer
pub type W = crate::W<APB2LPENRrs>;
///Field `SYSCFGLPEN` reader - System configuration controller clock enable during Sleep mode
pub type SYSCFGLPEN_R = crate::BitReader;
///Field `SYSCFGLPEN` writer - System configuration controller clock enable during Sleep mode
pub type SYSCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9LPEN` reader - TIM9 timer clock enable during Sleep mode
pub type TIM9LPEN_R = crate::BitReader;
///Field `TIM9LPEN` writer - TIM9 timer clock enable during Sleep mode
pub type TIM9LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM10LPEN` reader - TIM10 timer clock enable during Sleep mode
pub type TIM10LPEN_R = crate::BitReader;
///Field `TIM10LPEN` writer - TIM10 timer clock enable during Sleep mode
pub type TIM10LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM11LPEN` reader - TIM11 timer clock enable during Sleep mode
pub type TIM11LPEN_R = crate::BitReader;
///Field `TIM11LPEN` writer - TIM11 timer clock enable during Sleep mode
pub type TIM11LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC1LPEN` reader - ADC1 interface clock enable during Sleep mode
pub type ADC1LPEN_R = crate::BitReader;
///Field `ADC1LPEN` writer - ADC1 interface clock enable during Sleep mode
pub type ADC1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIOLPEN` reader - SDIO clock enable during Sleep mode
pub type SDIOLPEN_R = crate::BitReader;
///Field `SDIOLPEN` writer - SDIO clock enable during Sleep mode
pub type SDIOLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1LPEN` reader - SPI 1 clock enable during Sleep mode
pub type SPI1LPEN_R = crate::BitReader;
///Field `SPI1LPEN` writer - SPI 1 clock enable during Sleep mode
pub type SPI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1LPEN` reader - USART1 clock enable during Sleep mode
pub type USART1LPEN_R = crate::BitReader;
///Field `USART1LPEN` writer - USART1 clock enable during Sleep mode
pub type USART1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("usart1lpen", &self.usart1lpen())
            .field("spi1lpen", &self.spi1lpen())
            .field("sdiolpen", &self.sdiolpen())
            .field("adc1lpen", &self.adc1lpen())
            .field("tim11lpen", &self.tim11lpen())
            .field("tim10lpen", &self.tim10lpen())
            .field("tim9lpen", &self.tim9lpen())
            .field("syscfglpen", &self.syscfglpen())
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#RCC:APB2LPENR)*/
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
