///Register `IER2` reader
pub type R = crate::R<IER2rs>;
///Register `IER2` writer
pub type W = crate::W<IER2rs>;
///Field `TIM1IE` reader - illegal access interrupt enable for TIM1
pub type TIM1IE_R = crate::BitReader;
///Field `TIM1IE` writer - illegal access interrupt enable for TIM1
pub type TIM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1IE` reader - illegal access interrupt enable for SPI1
pub type SPI1IE_R = crate::BitReader;
///Field `SPI1IE` writer - illegal access interrupt enable for SPI1
pub type SPI1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1IE` reader - illegal access interrupt enable for USART1
pub type USART1IE_R = crate::BitReader;
///Field `USART1IE` writer - illegal access interrupt enable for USART1
pub type USART1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16IE` reader - illegal access interrupt enable for TIM16
pub type TIM16IE_R = crate::BitReader;
///Field `TIM16IE` writer - illegal access interrupt enable for TIM16
pub type TIM16IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17IE` reader - illegal access interrupt enable for TIM17
pub type TIM17IE_R = crate::BitReader;
///Field `TIM17IE` writer - illegal access interrupt enable for TIM17
pub type TIM17IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3IE` reader - illegal access interrupt enable for SPI3
pub type SPI3IE_R = crate::BitReader;
///Field `SPI3IE` writer - illegal access interrupt enable for SPI3
pub type SPI3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1IE` reader - illegal access interrupt enable for LPUART1
pub type LPUART1IE_R = crate::BitReader;
///Field `LPUART1IE` writer - illegal access interrupt enable for LPUART1
pub type LPUART1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3IE` reader - illegal access interrupt enable for I2C3
pub type I2C3IE_R = crate::BitReader;
///Field `I2C3IE` writer - illegal access interrupt enable for I2C3
pub type I2C3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1IE` reader - illegal access interrupt enable for LPTIM1
pub type LPTIM1IE_R = crate::BitReader;
///Field `LPTIM1IE` writer - illegal access interrupt enable for LPTIM1
pub type LPTIM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPIE` reader - illegal access interrupt enable for COMP Note that bit 23 is reserved on sales type STM32WBA52.
pub type COMPIE_R = crate::BitReader;
///Field `COMPIE` writer - illegal access interrupt enable for COMP Note that bit 23 is reserved on sales type STM32WBA52.
pub type COMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC4IE` reader - illegal access interrupt enable for ADC4
pub type ADC4IE_R = crate::BitReader;
///Field `ADC4IE` writer - illegal access interrupt enable for ADC4
pub type ADC4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - illegal access interrupt enable for TIM1
    #[inline(always)]
    pub fn tim1ie(&self) -> TIM1IE_R {
        TIM1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for SPI1
    #[inline(always)]
    pub fn spi1ie(&self) -> SPI1IE_R {
        SPI1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for USART1
    #[inline(always)]
    pub fn usart1ie(&self) -> USART1IE_R {
        USART1IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for TIM16
    #[inline(always)]
    pub fn tim16ie(&self) -> TIM16IE_R {
        TIM16IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for TIM17
    #[inline(always)]
    pub fn tim17ie(&self) -> TIM17IE_R {
        TIM17IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for SPI3
    #[inline(always)]
    pub fn spi3ie(&self) -> SPI3IE_R {
        SPI3IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for LPUART1
    #[inline(always)]
    pub fn lpuart1ie(&self) -> LPUART1IE_R {
        LPUART1IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for I2C3
    #[inline(always)]
    pub fn i2c3ie(&self) -> I2C3IE_R {
        I2C3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for LPTIM1
    #[inline(always)]
    pub fn lptim1ie(&self) -> LPTIM1IE_R {
        LPTIM1IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for COMP Note that bit 23 is reserved on sales type STM32WBA52.
    #[inline(always)]
    pub fn compie(&self) -> COMPIE_R {
        COMPIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for ADC4
    #[inline(always)]
    pub fn adc4ie(&self) -> ADC4IE_R {
        ADC4IE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER2")
            .field("tim1ie", &self.tim1ie())
            .field("spi1ie", &self.spi1ie())
            .field("usart1ie", &self.usart1ie())
            .field("tim16ie", &self.tim16ie())
            .field("tim17ie", &self.tim17ie())
            .field("spi3ie", &self.spi3ie())
            .field("lpuart1ie", &self.lpuart1ie())
            .field("i2c3ie", &self.i2c3ie())
            .field("lptim1ie", &self.lptim1ie())
            .field("compie", &self.compie())
            .field("adc4ie", &self.adc4ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - illegal access interrupt enable for TIM1
    #[inline(always)]
    pub fn tim1ie(&mut self) -> TIM1IE_W<'_, IER2rs> {
        TIM1IE_W::new(self, 0)
    }
    ///Bit 1 - illegal access interrupt enable for SPI1
    #[inline(always)]
    pub fn spi1ie(&mut self) -> SPI1IE_W<'_, IER2rs> {
        SPI1IE_W::new(self, 1)
    }
    ///Bit 3 - illegal access interrupt enable for USART1
    #[inline(always)]
    pub fn usart1ie(&mut self) -> USART1IE_W<'_, IER2rs> {
        USART1IE_W::new(self, 3)
    }
    ///Bit 5 - illegal access interrupt enable for TIM16
    #[inline(always)]
    pub fn tim16ie(&mut self) -> TIM16IE_W<'_, IER2rs> {
        TIM16IE_W::new(self, 5)
    }
    ///Bit 6 - illegal access interrupt enable for TIM17
    #[inline(always)]
    pub fn tim17ie(&mut self) -> TIM17IE_W<'_, IER2rs> {
        TIM17IE_W::new(self, 6)
    }
    ///Bit 16 - illegal access interrupt enable for SPI3
    #[inline(always)]
    pub fn spi3ie(&mut self) -> SPI3IE_W<'_, IER2rs> {
        SPI3IE_W::new(self, 16)
    }
    ///Bit 17 - illegal access interrupt enable for LPUART1
    #[inline(always)]
    pub fn lpuart1ie(&mut self) -> LPUART1IE_W<'_, IER2rs> {
        LPUART1IE_W::new(self, 17)
    }
    ///Bit 18 - illegal access interrupt enable for I2C3
    #[inline(always)]
    pub fn i2c3ie(&mut self) -> I2C3IE_W<'_, IER2rs> {
        I2C3IE_W::new(self, 18)
    }
    ///Bit 19 - illegal access interrupt enable for LPTIM1
    #[inline(always)]
    pub fn lptim1ie(&mut self) -> LPTIM1IE_W<'_, IER2rs> {
        LPTIM1IE_W::new(self, 19)
    }
    ///Bit 23 - illegal access interrupt enable for COMP Note that bit 23 is reserved on sales type STM32WBA52.
    #[inline(always)]
    pub fn compie(&mut self) -> COMPIE_W<'_, IER2rs> {
        COMPIE_W::new(self, 23)
    }
    ///Bit 24 - illegal access interrupt enable for ADC4
    #[inline(always)]
    pub fn adc4ie(&mut self) -> ADC4IE_W<'_, IER2rs> {
        ADC4IE_W::new(self, 24)
    }
}
/**GTZC1 TZIC interrupt enable register 2

You can [`read`](crate::Reg::read) this register and get [`ier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GTZC1_TZIC:IER2)*/
pub struct IER2rs;
impl crate::RegisterSpec for IER2rs {
    type Ux = u32;
}
///`read()` method returns [`ier2::R`](R) reader structure
impl crate::Readable for IER2rs {}
///`write(|w| ..)` method takes [`ier2::W`](W) writer structure
impl crate::Writable for IER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER2 to value 0
impl crate::Resettable for IER2rs {}
