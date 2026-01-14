///Register `APB2SMENR` reader
pub type R = crate::R<APB2SMENRrs>;
///Register `APB2SMENR` writer
pub type W = crate::W<APB2SMENRrs>;
///Field `ADCSMEN` reader - ADC clocks enable during CPU1 Csleep and CStop modes
pub type ADCSMEN_R = crate::BitReader;
///Field `ADCSMEN` writer - ADC clocks enable during CPU1 Csleep and CStop modes
pub type ADCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1SMEN` reader - TIM1 timer clock enable during CPU1 CSleep mode.
pub type TIM1SMEN_R = crate::BitReader;
///Field `TIM1SMEN` writer - TIM1 timer clock enable during CPU1 CSleep mode.
pub type TIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1SMEN` reader - SPI1 clock enable during CPU1 CSleep mode.
pub type SPI1SMEN_R = crate::BitReader;
///Field `SPI1SMEN` writer - SPI1 clock enable during CPU1 CSleep mode.
pub type SPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1SMEN` reader - USART1 clock enable during CPU1 Csleep and CStop modes.
pub type USART1SMEN_R = crate::BitReader;
///Field `USART1SMEN` writer - USART1 clock enable during CPU1 Csleep and CStop modes.
pub type USART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16SMEN` reader - TIM16 timer clock enable during CPU1 CSleep mode.
pub type TIM16SMEN_R = crate::BitReader;
///Field `TIM16SMEN` writer - TIM16 timer clock enable during CPU1 CSleep mode.
pub type TIM16SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17SMEN` reader - TIM17 timer clock enable during CPU1 CSleep mode.
pub type TIM17SMEN_R = crate::BitReader;
///Field `TIM17SMEN` writer - TIM17 timer clock enable during CPU1 CSleep mode.
pub type TIM17SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 9 - ADC clocks enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - TIM1 timer clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable during CPU1 Csleep and CStop modes.
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 timer clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2SMENR")
            .field("tim17smen", &self.tim17smen())
            .field("tim16smen", &self.tim16smen())
            .field("usart1smen", &self.usart1smen())
            .field("spi1smen", &self.spi1smen())
            .field("tim1smen", &self.tim1smen())
            .field("adcsmen", &self.adcsmen())
            .finish()
    }
}
impl W {
    ///Bit 9 - ADC clocks enable during CPU1 Csleep and CStop modes
    #[inline(always)]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<'_, APB2SMENRrs> {
        ADCSMEN_W::new(self, 9)
    }
    ///Bit 11 - TIM1 timer clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<'_, APB2SMENRrs> {
        TIM1SMEN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<'_, APB2SMENRrs> {
        SPI1SMEN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable during CPU1 Csleep and CStop modes.
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<'_, APB2SMENRrs> {
        USART1SMEN_W::new(self, 14)
    }
    ///Bit 17 - TIM16 timer clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<'_, APB2SMENRrs> {
        TIM16SMEN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 timer clock enable during CPU1 CSleep mode.
    #[inline(always)]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<'_, APB2SMENRrs> {
        TIM17SMEN_W::new(self, 18)
    }
}
/**APB2 peripheral clocks enable in Sleep mode register

You can [`read`](crate::Reg::read) this register and get [`apb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#RCC:APB2SMENR)*/
pub struct APB2SMENRrs;
impl crate::RegisterSpec for APB2SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2smenr::R`](R) reader structure
impl crate::Readable for APB2SMENRrs {}
///`write(|w| ..)` method takes [`apb2smenr::W`](W) writer structure
impl crate::Writable for APB2SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2SMENR to value 0x0006_5a00
impl crate::Resettable for APB2SMENRrs {
    const RESET_VALUE: u32 = 0x0006_5a00;
}
