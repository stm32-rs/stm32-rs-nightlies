///Register `APB1ENR` reader
pub type R = crate::R<APB1ENRrs>;
///Register `APB1ENR` writer
pub type W = crate::W<APB1ENRrs>;
///Field `ADCDIGEN` reader - AUXADC clock enable for Aux-ADC digital clock Set and enable by software.
pub type ADCDIGEN_R = crate::BitReader;
///Field `ADCDIGEN` writer - AUXADC clock enable for Aux-ADC digital clock Set and enable by software.
pub type ADCDIGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADCANAEN` reader - ADC clock enable for Aux-ADC analog clock Set and enable by software.
pub type ADCANAEN_R = crate::BitReader;
///Field `ADCANAEN` writer - ADC clock enable for Aux-ADC analog clock Set and enable by software.
pub type ADCANAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUARTEN` reader - LPUART clock enable Set and enable by software.
pub type LPUARTEN_R = crate::BitReader;
///Field `LPUARTEN` writer - LPUART clock enable Set and enable by software.
pub type LPUARTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1EN` reader - USART clock enable Set and enable by software.
pub type USART1EN_R = crate::BitReader;
///Field `USART1EN` writer - USART clock enable Set and enable by software.
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3EN` reader - SPI3 clock enable Set and enable by software.
pub type SPI3EN_R = crate::BitReader;
///Field `SPI3EN` writer - SPI3 clock enable Set and enable by software.
pub type SPI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1EN` reader - I2C1 clock enable Set and enable by software.
pub type I2C1EN_R = crate::BitReader;
///Field `I2C1EN` writer - I2C1 clock enable Set and enable by software.
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - AUXADC clock enable for Aux-ADC digital clock Set and enable by software.
    #[inline(always)]
    pub fn adcdigen(&self) -> ADCDIGEN_R {
        ADCDIGEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC clock enable for Aux-ADC analog clock Set and enable by software.
    #[inline(always)]
    pub fn adcanaen(&self) -> ADCANAEN_R {
        ADCANAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - LPUART clock enable Set and enable by software.
    #[inline(always)]
    pub fn lpuarten(&self) -> LPUARTEN_R {
        LPUARTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - USART clock enable Set and enable by software.
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - SPI3 clock enable Set and enable by software.
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable Set and enable by software.
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR")
            .field("adcdigen", &self.adcdigen())
            .field("adcanaen", &self.adcanaen())
            .field("lpuarten", &self.lpuarten())
            .field("usart1en", &self.usart1en())
            .field("spi3en", &self.spi3en())
            .field("i2c1en", &self.i2c1en())
            .finish()
    }
}
impl W {
    ///Bit 4 - AUXADC clock enable for Aux-ADC digital clock Set and enable by software.
    #[inline(always)]
    pub fn adcdigen(&mut self) -> ADCDIGEN_W<'_, APB1ENRrs> {
        ADCDIGEN_W::new(self, 4)
    }
    ///Bit 5 - ADC clock enable for Aux-ADC analog clock Set and enable by software.
    #[inline(always)]
    pub fn adcanaen(&mut self) -> ADCANAEN_W<'_, APB1ENRrs> {
        ADCANAEN_W::new(self, 5)
    }
    ///Bit 8 - LPUART clock enable Set and enable by software.
    #[inline(always)]
    pub fn lpuarten(&mut self) -> LPUARTEN_W<'_, APB1ENRrs> {
        LPUARTEN_W::new(self, 8)
    }
    ///Bit 10 - USART clock enable Set and enable by software.
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, APB1ENRrs> {
        USART1EN_W::new(self, 10)
    }
    ///Bit 14 - SPI3 clock enable Set and enable by software.
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<'_, APB1ENRrs> {
        SPI3EN_W::new(self, 14)
    }
    ///Bit 21 - I2C1 clock enable Set and enable by software.
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APB1ENRrs> {
        I2C1EN_W::new(self, 21)
    }
}
/**APB1ENR register

You can [`read`](crate::Reg::read) this register and get [`apb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RCC:APB1ENR)*/
pub struct APB1ENRrs;
impl crate::RegisterSpec for APB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1enr::R`](R) reader structure
impl crate::Readable for APB1ENRrs {}
///`write(|w| ..)` method takes [`apb1enr::W`](W) writer structure
impl crate::Writable for APB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1ENR to value 0
impl crate::Resettable for APB1ENRrs {}
