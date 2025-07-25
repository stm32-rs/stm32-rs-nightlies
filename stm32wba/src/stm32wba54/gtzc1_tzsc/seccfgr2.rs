///Register `SECCFGR2` reader
pub type R = crate::R<SECCFGR2rs>;
///Register `SECCFGR2` writer
pub type W = crate::W<SECCFGR2rs>;
///Field `TIM1SEC` reader - secure access mode for TIM1
pub type TIM1SEC_R = crate::BitReader;
///Field `TIM1SEC` writer - secure access mode for TIM1
pub type TIM1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1SEC` reader - secure access mode for SPI1
pub type SPI1SEC_R = crate::BitReader;
///Field `SPI1SEC` writer - secure access mode for SPI1
pub type SPI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1SEC` reader - secure access mode for USART1
pub type USART1SEC_R = crate::BitReader;
///Field `USART1SEC` writer - secure access mode for USART1
pub type USART1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16SEC` reader - secure access mode for TIM16
pub type TIM16SEC_R = crate::BitReader;
///Field `TIM16SEC` writer - secure access mode for TIM16
pub type TIM16SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17SEC` reader - secure access mode for TIM17
pub type TIM17SEC_R = crate::BitReader;
///Field `TIM17SEC` writer - secure access mode for TIM17
pub type TIM17SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3SEC` reader - secure access mode for SPI3
pub type SPI3SEC_R = crate::BitReader;
///Field `SPI3SEC` writer - secure access mode for SPI3
pub type SPI3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1SEC` reader - secure access mode for LPUART1
pub type LPUART1SEC_R = crate::BitReader;
///Field `LPUART1SEC` writer - secure access mode for LPUART1
pub type LPUART1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3SEC` reader - secure access mode for I2C3
pub type I2C3SEC_R = crate::BitReader;
///Field `I2C3SEC` writer - secure access mode for I2C3
pub type I2C3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1SEC` reader - secure access mode for LPTIM1
pub type LPTIM1SEC_R = crate::BitReader;
///Field `LPTIM1SEC` writer - secure access mode for LPTIM1
pub type LPTIM1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPSEC` reader - secure access mode for COMP Note that bit 23 is reserved on sales type STM32WBA52.
pub type COMPSEC_R = crate::BitReader;
///Field `COMPSEC` writer - secure access mode for COMP Note that bit 23 is reserved on sales type STM32WBA52.
pub type COMPSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC4SEC` reader - secure access mode for ADC4
pub type ADC4SEC_R = crate::BitReader;
///Field `ADC4SEC` writer - secure access mode for ADC4
pub type ADC4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - secure access mode for TIM1
    #[inline(always)]
    pub fn tim1sec(&self) -> TIM1SEC_R {
        TIM1SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure access mode for SPI1
    #[inline(always)]
    pub fn spi1sec(&self) -> SPI1SEC_R {
        SPI1SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - secure access mode for USART1
    #[inline(always)]
    pub fn usart1sec(&self) -> USART1SEC_R {
        USART1SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - secure access mode for TIM16
    #[inline(always)]
    pub fn tim16sec(&self) -> TIM16SEC_R {
        TIM16SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - secure access mode for TIM17
    #[inline(always)]
    pub fn tim17sec(&self) -> TIM17SEC_R {
        TIM17SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - secure access mode for SPI3
    #[inline(always)]
    pub fn spi3sec(&self) -> SPI3SEC_R {
        SPI3SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - secure access mode for LPUART1
    #[inline(always)]
    pub fn lpuart1sec(&self) -> LPUART1SEC_R {
        LPUART1SEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - secure access mode for I2C3
    #[inline(always)]
    pub fn i2c3sec(&self) -> I2C3SEC_R {
        I2C3SEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - secure access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1sec(&self) -> LPTIM1SEC_R {
        LPTIM1SEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - secure access mode for COMP Note that bit 23 is reserved on sales type STM32WBA52.
    #[inline(always)]
    pub fn compsec(&self) -> COMPSEC_R {
        COMPSEC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - secure access mode for ADC4
    #[inline(always)]
    pub fn adc4sec(&self) -> ADC4SEC_R {
        ADC4SEC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR2")
            .field("tim1sec", &self.tim1sec())
            .field("spi1sec", &self.spi1sec())
            .field("usart1sec", &self.usart1sec())
            .field("tim16sec", &self.tim16sec())
            .field("tim17sec", &self.tim17sec())
            .field("spi3sec", &self.spi3sec())
            .field("lpuart1sec", &self.lpuart1sec())
            .field("i2c3sec", &self.i2c3sec())
            .field("lptim1sec", &self.lptim1sec())
            .field("compsec", &self.compsec())
            .field("adc4sec", &self.adc4sec())
            .finish()
    }
}
impl W {
    ///Bit 0 - secure access mode for TIM1
    #[inline(always)]
    pub fn tim1sec(&mut self) -> TIM1SEC_W<SECCFGR2rs> {
        TIM1SEC_W::new(self, 0)
    }
    ///Bit 1 - secure access mode for SPI1
    #[inline(always)]
    pub fn spi1sec(&mut self) -> SPI1SEC_W<SECCFGR2rs> {
        SPI1SEC_W::new(self, 1)
    }
    ///Bit 3 - secure access mode for USART1
    #[inline(always)]
    pub fn usart1sec(&mut self) -> USART1SEC_W<SECCFGR2rs> {
        USART1SEC_W::new(self, 3)
    }
    ///Bit 5 - secure access mode for TIM16
    #[inline(always)]
    pub fn tim16sec(&mut self) -> TIM16SEC_W<SECCFGR2rs> {
        TIM16SEC_W::new(self, 5)
    }
    ///Bit 6 - secure access mode for TIM17
    #[inline(always)]
    pub fn tim17sec(&mut self) -> TIM17SEC_W<SECCFGR2rs> {
        TIM17SEC_W::new(self, 6)
    }
    ///Bit 16 - secure access mode for SPI3
    #[inline(always)]
    pub fn spi3sec(&mut self) -> SPI3SEC_W<SECCFGR2rs> {
        SPI3SEC_W::new(self, 16)
    }
    ///Bit 17 - secure access mode for LPUART1
    #[inline(always)]
    pub fn lpuart1sec(&mut self) -> LPUART1SEC_W<SECCFGR2rs> {
        LPUART1SEC_W::new(self, 17)
    }
    ///Bit 18 - secure access mode for I2C3
    #[inline(always)]
    pub fn i2c3sec(&mut self) -> I2C3SEC_W<SECCFGR2rs> {
        I2C3SEC_W::new(self, 18)
    }
    ///Bit 19 - secure access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1sec(&mut self) -> LPTIM1SEC_W<SECCFGR2rs> {
        LPTIM1SEC_W::new(self, 19)
    }
    ///Bit 23 - secure access mode for COMP Note that bit 23 is reserved on sales type STM32WBA52.
    #[inline(always)]
    pub fn compsec(&mut self) -> COMPSEC_W<SECCFGR2rs> {
        COMPSEC_W::new(self, 23)
    }
    ///Bit 24 - secure access mode for ADC4
    #[inline(always)]
    pub fn adc4sec(&mut self) -> ADC4SEC_W<SECCFGR2rs> {
        ADC4SEC_W::new(self, 24)
    }
}
/**GTZC1 TZSC secure configuration register 2

You can [`read`](crate::Reg::read) this register and get [`seccfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GTZC1_TZSC:SECCFGR2)*/
pub struct SECCFGR2rs;
impl crate::RegisterSpec for SECCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr2::R`](R) reader structure
impl crate::Readable for SECCFGR2rs {}
///`write(|w| ..)` method takes [`seccfgr2::W`](W) writer structure
impl crate::Writable for SECCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR2 to value 0
impl crate::Resettable for SECCFGR2rs {}
