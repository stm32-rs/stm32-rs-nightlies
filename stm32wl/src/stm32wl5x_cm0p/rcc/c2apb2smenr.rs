///Register `C2APB2SMENR` reader
pub type R = crate::R<C2APB2SMENRrs>;
///Register `C2APB2SMENR` writer
pub type W = crate::W<C2APB2SMENRrs>;
/**ADC clocks enable during CPU2 Csleep and CStop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCSMEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<ADCSMEN> for bool {
    #[inline(always)]
    fn from(variant: ADCSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCSMEN` reader - ADC clocks enable during CPU2 Csleep and CStop modes
pub type ADCSMEN_R = crate::BitReader<ADCSMEN>;
impl ADCSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCSMEN {
        match self.bits {
            false => ADCSMEN::Disabled,
            true => ADCSMEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCSMEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCSMEN::Enabled
    }
}
///Field `ADCSMEN` writer - ADC clocks enable during CPU2 Csleep and CStop modes
pub type ADCSMEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCSMEN>;
impl<'a, REG> ADCSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSMEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSMEN::Enabled)
    }
}
///Field `TIM1SMEN` reader - TIM1 timer clock enable during CPU2 CSleep mode
pub use ADCSMEN_R as TIM1SMEN_R;
///Field `SPI1SMEN` reader - SPI1 clock enable during CPU2 CSleep mode
pub use ADCSMEN_R as SPI1SMEN_R;
///Field `USART1SMEN` reader - USART1clock enable during CPU2 CSleep and CStop mode
pub use ADCSMEN_R as USART1SMEN_R;
///Field `TIM16SMEN` reader - TIM16 timer clock enable during CPU2 CSleep mode
pub use ADCSMEN_R as TIM16SMEN_R;
///Field `TIM17SMEN` reader - TIM17 timer clock enable during CPU2 CSleep mode
pub use ADCSMEN_R as TIM17SMEN_R;
///Field `TIM1SMEN` writer - TIM1 timer clock enable during CPU2 CSleep mode
pub use ADCSMEN_W as TIM1SMEN_W;
///Field `SPI1SMEN` writer - SPI1 clock enable during CPU2 CSleep mode
pub use ADCSMEN_W as SPI1SMEN_W;
///Field `USART1SMEN` writer - USART1clock enable during CPU2 CSleep and CStop mode
pub use ADCSMEN_W as USART1SMEN_W;
///Field `TIM16SMEN` writer - TIM16 timer clock enable during CPU2 CSleep mode
pub use ADCSMEN_W as TIM16SMEN_W;
///Field `TIM17SMEN` writer - TIM17 timer clock enable during CPU2 CSleep mode
pub use ADCSMEN_W as TIM17SMEN_W;
impl R {
    ///Bit 9 - ADC clocks enable during CPU2 Csleep and CStop modes
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - TIM1 timer clock enable during CPU2 CSleep mode
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable during CPU2 CSleep mode
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1clock enable during CPU2 CSleep and CStop mode
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clock enable during CPU2 CSleep mode
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 timer clock enable during CPU2 CSleep mode
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB2SMENR")
            .field("adcsmen", &self.adcsmen())
            .field("tim17smen", &self.tim17smen())
            .field("tim16smen", &self.tim16smen())
            .field("usart1smen", &self.usart1smen())
            .field("spi1smen", &self.spi1smen())
            .field("tim1smen", &self.tim1smen())
            .finish()
    }
}
impl W {
    ///Bit 9 - ADC clocks enable during CPU2 Csleep and CStop modes
    #[inline(always)]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<C2APB2SMENRrs> {
        ADCSMEN_W::new(self, 9)
    }
    ///Bit 11 - TIM1 timer clock enable during CPU2 CSleep mode
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<C2APB2SMENRrs> {
        TIM1SMEN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable during CPU2 CSleep mode
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<C2APB2SMENRrs> {
        SPI1SMEN_W::new(self, 12)
    }
    ///Bit 14 - USART1clock enable during CPU2 CSleep and CStop mode
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<C2APB2SMENRrs> {
        USART1SMEN_W::new(self, 14)
    }
    ///Bit 17 - TIM16 timer clock enable during CPU2 CSleep mode
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<C2APB2SMENRrs> {
        TIM16SMEN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 timer clock enable during CPU2 CSleep mode
    #[inline(always)]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<C2APB2SMENRrs> {
        TIM17SMEN_W::new(self, 18)
    }
}
/**CPU2 APB2 peripheral clocks enable in Sleep mode register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2APB2SMENR)*/
pub struct C2APB2SMENRrs;
impl crate::RegisterSpec for C2APB2SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2apb2smenr::R`](R) reader structure
impl crate::Readable for C2APB2SMENRrs {}
///`write(|w| ..)` method takes [`c2apb2smenr::W`](W) writer structure
impl crate::Writable for C2APB2SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2APB2SMENR to value 0x0006_5a00
impl crate::Resettable for C2APB2SMENRrs {
    const RESET_VALUE: u32 = 0x0006_5a00;
}
