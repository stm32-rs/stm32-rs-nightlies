///Register `SR2` reader
pub type R = crate::R<SR2rs>;
///Field `TIM1F` reader - illegal access flag for TIM1
pub type TIM1F_R = crate::BitReader;
///Field `SPI1F` reader - illegal access flag for SPI1
pub type SPI1F_R = crate::BitReader;
///Field `USART1F` reader - illegal access flag for USART1
pub type USART1F_R = crate::BitReader;
///Field `TIM16F` reader - illegal access flag for TIM6
pub type TIM16F_R = crate::BitReader;
///Field `TIM17F` reader - illegal access flag for TIM7
pub type TIM17F_R = crate::BitReader;
///Field `SPI3F` reader - SPI3F
pub type SPI3F_R = crate::BitReader;
///Field `LPUART1F` reader - LPUART1F
pub type LPUART1F_R = crate::BitReader;
///Field `I2C3F` reader - I2C3F
pub type I2C3F_R = crate::BitReader;
///Field `LPTIM1F` reader - LPTIM1F
pub type LPTIM1F_R = crate::BitReader;
///Field `COMPF` reader - COMPF
pub type COMPF_R = crate::BitReader;
///Field `ADC4F` reader - ADC4F
pub type ADC4F_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access flag for TIM1
    #[inline(always)]
    pub fn tim1f(&self) -> TIM1F_R {
        TIM1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for SPI1
    #[inline(always)]
    pub fn spi1f(&self) -> SPI1F_R {
        SPI1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for USART1
    #[inline(always)]
    pub fn usart1f(&self) -> USART1F_R {
        USART1F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - illegal access flag for TIM6
    #[inline(always)]
    pub fn tim16f(&self) -> TIM16F_R {
        TIM16F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for TIM7
    #[inline(always)]
    pub fn tim17f(&self) -> TIM17F_R {
        TIM17F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - SPI3F
    #[inline(always)]
    pub fn spi3f(&self) -> SPI3F_R {
        SPI3F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - LPUART1F
    #[inline(always)]
    pub fn lpuart1f(&self) -> LPUART1F_R {
        LPUART1F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - I2C3F
    #[inline(always)]
    pub fn i2c3f(&self) -> I2C3F_R {
        I2C3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - LPTIM1F
    #[inline(always)]
    pub fn lptim1f(&self) -> LPTIM1F_R {
        LPTIM1F_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - COMPF
    #[inline(always)]
    pub fn compf(&self) -> COMPF_R {
        COMPF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ADC4F
    #[inline(always)]
    pub fn adc4f(&self) -> ADC4F_R {
        ADC4F_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR2")
            .field("tim1f", &self.tim1f())
            .field("spi1f", &self.spi1f())
            .field("usart1f", &self.usart1f())
            .field("tim16f", &self.tim16f())
            .field("tim17f", &self.tim17f())
            .field("spi3f", &self.spi3f())
            .field("lpuart1f", &self.lpuart1f())
            .field("i2c3f", &self.i2c3f())
            .field("lptim1f", &self.lptim1f())
            .field("compf", &self.compf())
            .field("adc4f", &self.adc4f())
            .finish()
    }
}
/**TZIC status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZIC:SR2)*/
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
///`read()` method returns [`sr2::R`](R) reader structure
impl crate::Readable for SR2rs {}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2rs {}
