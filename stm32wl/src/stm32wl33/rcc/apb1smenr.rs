///Register `APB1SMENR` reader
pub type R = crate::R<APB1SMENRrs>;
///Register `APB1SMENR` writer
pub type W = crate::W<APB1SMENRrs>;
///Field `SPI1SMEN` reader - SPI1 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI1 bus clock disabled in Sleep mode - 1: SPI1 bus clock enabled in Sleep mode (if enabled in SPI1EN)
pub type SPI1SMEN_R = crate::BitReader;
///Field `SPI1SMEN` writer - SPI1 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI1 bus clock disabled in Sleep mode - 1: SPI1 bus clock enabled in Sleep mode (if enabled in SPI1EN)
pub type SPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCDIGSMEN` reader - ADCDIG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: ADCDIG bus clock disabled in Sleep mode - 1: ADCDIG bus clock enabled in Sleep mode (if enabled by ADCDIGEN)
pub type ADCDIGSMEN_R = crate::BitReader;
///Field `ADCDIGSMEN` writer - ADCDIG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: ADCDIG bus clock disabled in Sleep mode - 1: ADCDIG bus clock enabled in Sleep mode (if enabled by ADCDIGEN)
pub type ADCDIGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUARTSMEN` reader - LPUART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LPUART bus clock disabled in Sleep mode - 1: LPUART bus clock enabled in Sleep mode (if enabled in LPUARTEN)
pub type LPUARTSMEN_R = crate::BitReader;
///Field `LPUARTSMEN` writer - LPUART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LPUART bus clock disabled in Sleep mode - 1: LPUART bus clock enabled in Sleep mode (if enabled in LPUARTEN)
pub type LPUARTSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USARTSMEN` reader - USART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: USART bus clock disabled in Sleep mode - 1: USART bus clock enabled in Sleep mode (if enabled in USARTEN)
pub type USARTSMEN_R = crate::BitReader;
///Field `USARTSMEN` writer - USART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: USART bus clock disabled in Sleep mode - 1: USART bus clock enabled in Sleep mode (if enabled in USARTEN)
pub type USARTSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3SMEN` reader - SPI3 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI3 bus clock disabled in Sleep mode - 1: SPI3 bus clock enabled in Sleep mode (if enabled in SPI3EN)
pub type SPI3SMEN_R = crate::BitReader;
///Field `SPI3SMEN` writer - SPI3 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI3 bus clock disabled in Sleep mode - 1: SPI3 bus clock enabled in Sleep mode (if enabled in SPI3EN)
pub type SPI3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1SMEN` reader - I2C1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C1 clock disabled in Sleep mode - 1: I2C1 clock enabled in Sleep mode (if enabled in I2C1EN)
pub type I2C1SMEN_R = crate::BitReader;
///Field `I2C1SMEN` writer - I2C1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C1 clock disabled in Sleep mode - 1: I2C1 clock enabled in Sleep mode (if enabled in I2C1EN)
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C2 clock disabled in Sleep mode - 1: I2C2 clock enabled in Sleep mode (if enabled in I2C2EN)
pub type I2C2SMEN_R = crate::BitReader;
///Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C2 clock disabled in Sleep mode - 1: I2C2 clock enabled in Sleep mode (if enabled in I2C2EN)
pub type I2C2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SPI1 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI1 bus clock disabled in Sleep mode - 1: SPI1 bus clock enabled in Sleep mode (if enabled in SPI1EN)
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - ADCDIG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: ADCDIG bus clock disabled in Sleep mode - 1: ADCDIG bus clock enabled in Sleep mode (if enabled by ADCDIGEN)
    #[inline(always)]
    pub fn adcdigsmen(&self) -> ADCDIGSMEN_R {
        ADCDIGSMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - LPUART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LPUART bus clock disabled in Sleep mode - 1: LPUART bus clock enabled in Sleep mode (if enabled in LPUARTEN)
    #[inline(always)]
    pub fn lpuartsmen(&self) -> LPUARTSMEN_R {
        LPUARTSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - USART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: USART bus clock disabled in Sleep mode - 1: USART bus clock enabled in Sleep mode (if enabled in USARTEN)
    #[inline(always)]
    pub fn usartsmen(&self) -> USARTSMEN_R {
        USARTSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - SPI3 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI3 bus clock disabled in Sleep mode - 1: SPI3 bus clock enabled in Sleep mode (if enabled in SPI3EN)
    #[inline(always)]
    pub fn spi3smen(&self) -> SPI3SMEN_R {
        SPI3SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C1 clock disabled in Sleep mode - 1: I2C1 clock enabled in Sleep mode (if enabled in I2C1EN)
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - I2C2 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C2 clock disabled in Sleep mode - 1: I2C2 clock enabled in Sleep mode (if enabled in I2C2EN)
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SMENR")
            .field("spi1smen", &self.spi1smen())
            .field("adcdigsmen", &self.adcdigsmen())
            .field("lpuartsmen", &self.lpuartsmen())
            .field("usartsmen", &self.usartsmen())
            .field("spi3smen", &self.spi3smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("i2c2smen", &self.i2c2smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPI1 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI1 bus clock disabled in Sleep mode - 1: SPI1 bus clock enabled in Sleep mode (if enabled in SPI1EN)
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<'_, APB1SMENRrs> {
        SPI1SMEN_W::new(self, 0)
    }
    ///Bit 4 - ADCDIG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: ADCDIG bus clock disabled in Sleep mode - 1: ADCDIG bus clock enabled in Sleep mode (if enabled by ADCDIGEN)
    #[inline(always)]
    pub fn adcdigsmen(&mut self) -> ADCDIGSMEN_W<'_, APB1SMENRrs> {
        ADCDIGSMEN_W::new(self, 4)
    }
    ///Bit 8 - LPUART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LPUART bus clock disabled in Sleep mode - 1: LPUART bus clock enabled in Sleep mode (if enabled in LPUARTEN)
    #[inline(always)]
    pub fn lpuartsmen(&mut self) -> LPUARTSMEN_W<'_, APB1SMENRrs> {
        LPUARTSMEN_W::new(self, 8)
    }
    ///Bit 10 - USART bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: USART bus clock disabled in Sleep mode - 1: USART bus clock enabled in Sleep mode (if enabled in USARTEN)
    #[inline(always)]
    pub fn usartsmen(&mut self) -> USARTSMEN_W<'_, APB1SMENRrs> {
        USARTSMEN_W::new(self, 10)
    }
    ///Bit 14 - SPI3 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SPI3 bus clock disabled in Sleep mode - 1: SPI3 bus clock enabled in Sleep mode (if enabled in SPI3EN)
    #[inline(always)]
    pub fn spi3smen(&mut self) -> SPI3SMEN_W<'_, APB1SMENRrs> {
        SPI3SMEN_W::new(self, 14)
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C1 clock disabled in Sleep mode - 1: I2C1 clock enabled in Sleep mode (if enabled in I2C1EN)
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<'_, APB1SMENRrs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 23 - I2C2 clock enable during Sleep mode bit This bit is set and reset by software. - 0: I2C2 clock disabled in Sleep mode - 1: I2C2 clock enabled in Sleep mode (if enabled in I2C2EN)
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<'_, APB1SMENRrs> {
        I2C2SMEN_W::new(self, 23)
    }
}
/**APB1SMENR register

You can [`read`](crate::Reg::read) this register and get [`apb1smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:APB1SMENR)*/
pub struct APB1SMENRrs;
impl crate::RegisterSpec for APB1SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1smenr::R`](R) reader structure
impl crate::Readable for APB1SMENRrs {}
///`write(|w| ..)` method takes [`apb1smenr::W`](W) writer structure
impl crate::Writable for APB1SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1SMENR to value 0x00a0_4511
impl crate::Resettable for APB1SMENRrs {
    const RESET_VALUE: u32 = 0x00a0_4511;
}
