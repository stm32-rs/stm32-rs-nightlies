///Register `APB1RSTR` reader
pub type R = crate::R<APB1RSTRrs>;
///Register `APB1RSTR` writer
pub type W = crate::W<APB1RSTRrs>;
///Field `AUXADCRST` reader - AUXADC reset for Aux-ADC digital clock Set and reset by software.
pub type AUXADCRST_R = crate::BitReader;
///Field `AUXADCRST` writer - AUXADC reset for Aux-ADC digital clock Set and reset by software.
pub type AUXADCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUARTRST` reader - LPUART reset Set and reset by software.
pub type LPUARTRST_R = crate::BitReader;
///Field `LPUARTRST` writer - LPUART reset Set and reset by software.
pub type LPUARTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USARTRST` reader - USART reset Set and reset by software.
pub type USARTRST_R = crate::BitReader;
///Field `USARTRST` writer - USART reset Set and reset by software.
pub type USARTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3RST` reader - SPI3 reset Set and reset by software.
pub type SPI3RST_R = crate::BitReader;
///Field `SPI3RST` writer - SPI3 reset Set and reset by software.
pub type SPI3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C21RST` reader - I2C1 reset Set and reset by software.
pub type I2C21RST_R = crate::BitReader;
///Field `I2C21RST` writer - I2C1 reset Set and reset by software.
pub type I2C21RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - AUXADC reset for Aux-ADC digital clock Set and reset by software.
    #[inline(always)]
    pub fn auxadcrst(&self) -> AUXADCRST_R {
        AUXADCRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - LPUART reset Set and reset by software.
    #[inline(always)]
    pub fn lpuartrst(&self) -> LPUARTRST_R {
        LPUARTRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - USART reset Set and reset by software.
    #[inline(always)]
    pub fn usartrst(&self) -> USARTRST_R {
        USARTRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - SPI3 reset Set and reset by software.
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 21 - I2C1 reset Set and reset by software.
    #[inline(always)]
    pub fn i2c21rst(&self) -> I2C21RST_R {
        I2C21RST_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR")
            .field("auxadcrst", &self.auxadcrst())
            .field("lpuartrst", &self.lpuartrst())
            .field("usartrst", &self.usartrst())
            .field("spi3rst", &self.spi3rst())
            .field("i2c21rst", &self.i2c21rst())
            .finish()
    }
}
impl W {
    ///Bit 4 - AUXADC reset for Aux-ADC digital clock Set and reset by software.
    #[inline(always)]
    pub fn auxadcrst(&mut self) -> AUXADCRST_W<'_, APB1RSTRrs> {
        AUXADCRST_W::new(self, 4)
    }
    ///Bit 8 - LPUART reset Set and reset by software.
    #[inline(always)]
    pub fn lpuartrst(&mut self) -> LPUARTRST_W<'_, APB1RSTRrs> {
        LPUARTRST_W::new(self, 8)
    }
    ///Bit 10 - USART reset Set and reset by software.
    #[inline(always)]
    pub fn usartrst(&mut self) -> USARTRST_W<'_, APB1RSTRrs> {
        USARTRST_W::new(self, 10)
    }
    ///Bit 14 - SPI3 reset Set and reset by software.
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<'_, APB1RSTRrs> {
        SPI3RST_W::new(self, 14)
    }
    ///Bit 21 - I2C1 reset Set and reset by software.
    #[inline(always)]
    pub fn i2c21rst(&mut self) -> I2C21RST_W<'_, APB1RSTRrs> {
        I2C21RST_W::new(self, 21)
    }
}
/**APB1RSTR register

You can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RCC:APB1RSTR)*/
pub struct APB1RSTRrs;
impl crate::RegisterSpec for APB1RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1rstr::R`](R) reader structure
impl crate::Readable for APB1RSTRrs {}
///`write(|w| ..)` method takes [`apb1rstr::W`](W) writer structure
impl crate::Writable for APB1RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1RSTR to value 0
impl crate::Resettable for APB1RSTRrs {}
