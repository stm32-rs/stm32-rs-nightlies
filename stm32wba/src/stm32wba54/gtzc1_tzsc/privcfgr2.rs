///Register `PRIVCFGR2` reader
pub type R = crate::R<PRIVCFGR2rs>;
///Register `PRIVCFGR2` writer
pub type W = crate::W<PRIVCFGR2rs>;
///Field `TIM1PRIV` reader - privileged access mode for TIM1
pub type TIM1PRIV_R = crate::BitReader;
///Field `TIM1PRIV` writer - privileged access mode for TIM1
pub type TIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1PRIV` reader - privileged access mode for SPI1PRIV
pub type SPI1PRIV_R = crate::BitReader;
///Field `SPI1PRIV` writer - privileged access mode for SPI1PRIV
pub type SPI1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1PRIV` reader - privileged access mode for USART1
pub type USART1PRIV_R = crate::BitReader;
///Field `USART1PRIV` writer - privileged access mode for USART1
pub type USART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16PRIV` reader - privileged access mode for TIM16
pub type TIM16PRIV_R = crate::BitReader;
///Field `TIM16PRIV` writer - privileged access mode for TIM16
pub type TIM16PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17PRIV` reader - privileged access mode for TIM17
pub type TIM17PRIV_R = crate::BitReader;
///Field `TIM17PRIV` writer - privileged access mode for TIM17
pub type TIM17PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3PRIV` reader - privileged access mode for SPI3
pub type SPI3PRIV_R = crate::BitReader;
///Field `SPI3PRIV` writer - privileged access mode for SPI3
pub type SPI3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1PRIV` reader - privileged access mode for LPUART1
pub type LPUART1PRIV_R = crate::BitReader;
///Field `LPUART1PRIV` writer - privileged access mode for LPUART1
pub type LPUART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3PRIV` reader - privileged access mode for I2C3
pub type I2C3PRIV_R = crate::BitReader;
///Field `I2C3PRIV` writer - privileged access mode for I2C3
pub type I2C3PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1PRIV` reader - privileged access mode for LPTIM1
pub type LPTIM1PRIV_R = crate::BitReader;
///Field `LPTIM1PRIV` writer - privileged access mode for LPTIM1
pub type LPTIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPPRIV` reader - privileged access mode for COMP Note that bit 23 is reserved on sales type STM32WBA52.
pub type COMPPRIV_R = crate::BitReader;
///Field `COMPPRIV` writer - privileged access mode for COMP Note that bit 23 is reserved on sales type STM32WBA52.
pub type COMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC4PRIV` reader - privileged access mode for ADC4
pub type ADC4PRIV_R = crate::BitReader;
///Field `ADC4PRIV` writer - privileged access mode for ADC4
pub type ADC4PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - privileged access mode for TIM1
    #[inline(always)]
    pub fn tim1priv(&self) -> TIM1PRIV_R {
        TIM1PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - privileged access mode for SPI1PRIV
    #[inline(always)]
    pub fn spi1priv(&self) -> SPI1PRIV_R {
        SPI1PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - privileged access mode for USART1
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - privileged access mode for TIM16
    #[inline(always)]
    pub fn tim16priv(&self) -> TIM16PRIV_R {
        TIM16PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - privileged access mode for TIM17
    #[inline(always)]
    pub fn tim17priv(&self) -> TIM17PRIV_R {
        TIM17PRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - privileged access mode for SPI3
    #[inline(always)]
    pub fn spi3priv(&self) -> SPI3PRIV_R {
        SPI3PRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - privileged access mode for LPUART1
    #[inline(always)]
    pub fn lpuart1priv(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - privileged access mode for I2C3
    #[inline(always)]
    pub fn i2c3priv(&self) -> I2C3PRIV_R {
        I2C3PRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - privileged access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1priv(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - privileged access mode for COMP Note that bit 23 is reserved on sales type STM32WBA52.
    #[inline(always)]
    pub fn comppriv(&self) -> COMPPRIV_R {
        COMPPRIV_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - privileged access mode for ADC4
    #[inline(always)]
    pub fn adc4priv(&self) -> ADC4PRIV_R {
        ADC4PRIV_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR2")
            .field("tim1priv", &self.tim1priv())
            .field("spi1priv", &self.spi1priv())
            .field("usart1priv", &self.usart1priv())
            .field("tim16priv", &self.tim16priv())
            .field("tim17priv", &self.tim17priv())
            .field("spi3priv", &self.spi3priv())
            .field("lpuart1priv", &self.lpuart1priv())
            .field("i2c3priv", &self.i2c3priv())
            .field("lptim1priv", &self.lptim1priv())
            .field("comppriv", &self.comppriv())
            .field("adc4priv", &self.adc4priv())
            .finish()
    }
}
impl W {
    ///Bit 0 - privileged access mode for TIM1
    #[inline(always)]
    pub fn tim1priv(&mut self) -> TIM1PRIV_W<PRIVCFGR2rs> {
        TIM1PRIV_W::new(self, 0)
    }
    ///Bit 1 - privileged access mode for SPI1PRIV
    #[inline(always)]
    pub fn spi1priv(&mut self) -> SPI1PRIV_W<PRIVCFGR2rs> {
        SPI1PRIV_W::new(self, 1)
    }
    ///Bit 3 - privileged access mode for USART1
    #[inline(always)]
    pub fn usart1priv(&mut self) -> USART1PRIV_W<PRIVCFGR2rs> {
        USART1PRIV_W::new(self, 3)
    }
    ///Bit 5 - privileged access mode for TIM16
    #[inline(always)]
    pub fn tim16priv(&mut self) -> TIM16PRIV_W<PRIVCFGR2rs> {
        TIM16PRIV_W::new(self, 5)
    }
    ///Bit 6 - privileged access mode for TIM17
    #[inline(always)]
    pub fn tim17priv(&mut self) -> TIM17PRIV_W<PRIVCFGR2rs> {
        TIM17PRIV_W::new(self, 6)
    }
    ///Bit 16 - privileged access mode for SPI3
    #[inline(always)]
    pub fn spi3priv(&mut self) -> SPI3PRIV_W<PRIVCFGR2rs> {
        SPI3PRIV_W::new(self, 16)
    }
    ///Bit 17 - privileged access mode for LPUART1
    #[inline(always)]
    pub fn lpuart1priv(&mut self) -> LPUART1PRIV_W<PRIVCFGR2rs> {
        LPUART1PRIV_W::new(self, 17)
    }
    ///Bit 18 - privileged access mode for I2C3
    #[inline(always)]
    pub fn i2c3priv(&mut self) -> I2C3PRIV_W<PRIVCFGR2rs> {
        I2C3PRIV_W::new(self, 18)
    }
    ///Bit 19 - privileged access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1priv(&mut self) -> LPTIM1PRIV_W<PRIVCFGR2rs> {
        LPTIM1PRIV_W::new(self, 19)
    }
    ///Bit 23 - privileged access mode for COMP Note that bit 23 is reserved on sales type STM32WBA52.
    #[inline(always)]
    pub fn comppriv(&mut self) -> COMPPRIV_W<PRIVCFGR2rs> {
        COMPPRIV_W::new(self, 23)
    }
    ///Bit 24 - privileged access mode for ADC4
    #[inline(always)]
    pub fn adc4priv(&mut self) -> ADC4PRIV_W<PRIVCFGR2rs> {
        ADC4PRIV_W::new(self, 24)
    }
}
/**GTZC1 TZSC privilege configuration register 2

You can [`read`](crate::Reg::read) this register and get [`privcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GTZC1_TZSC:PRIVCFGR2)*/
pub struct PRIVCFGR2rs;
impl crate::RegisterSpec for PRIVCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr2::R`](R) reader structure
impl crate::Readable for PRIVCFGR2rs {}
///`write(|w| ..)` method takes [`privcfgr2::W`](W) writer structure
impl crate::Writable for PRIVCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR2 to value 0
impl crate::Resettable for PRIVCFGR2rs {}
