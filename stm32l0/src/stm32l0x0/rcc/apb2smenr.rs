///Register `APB2SMENR` reader
pub type R = crate::R<APB2SMENRrs>;
///Register `APB2SMENR` writer
pub type W = crate::W<APB2SMENRrs>;
/**System configuration controller clock enable during sleep mode bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGSMEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<SYSCFGSMEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGSMEN` reader - System configuration controller clock enable during sleep mode bit
pub type SYSCFGSMEN_R = crate::BitReader<SYSCFGSMEN>;
impl SYSCFGSMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGSMEN {
        match self.bits {
            false => SYSCFGSMEN::Disabled,
            true => SYSCFGSMEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGSMEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGSMEN::Enabled
    }
}
///Field `SYSCFGSMEN` writer - System configuration controller clock enable during sleep mode bit
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGSMEN>;
impl<'a, REG> SYSCFGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::Enabled)
    }
}
///Field `TIM21SMEN` reader - TIM21 timer clock enable during sleep mode bit
pub use SYSCFGSMEN_R as TIM21SMEN_R;
///Field `TIM22SMEN` reader - TIM22 timer clock enable during sleep mode bit
pub use SYSCFGSMEN_R as TIM22SMEN_R;
///Field `ADCSMEN` reader - ADC clock enable during sleep mode bit
pub use SYSCFGSMEN_R as ADCSMEN_R;
///Field `SPI1SMEN` reader - SPI1 clock enable during sleep mode bit
pub use SYSCFGSMEN_R as SPI1SMEN_R;
///Field `USART1SMEN` reader - USART1 clock enable during sleep mode bit
pub use SYSCFGSMEN_R as USART1SMEN_R;
///Field `DBGSMEN` reader - DBG clock enable during sleep mode bit
pub use SYSCFGSMEN_R as DBGSMEN_R;
///Field `TIM21SMEN` writer - TIM21 timer clock enable during sleep mode bit
pub use SYSCFGSMEN_W as TIM21SMEN_W;
///Field `TIM22SMEN` writer - TIM22 timer clock enable during sleep mode bit
pub use SYSCFGSMEN_W as TIM22SMEN_W;
///Field `ADCSMEN` writer - ADC clock enable during sleep mode bit
pub use SYSCFGSMEN_W as ADCSMEN_W;
///Field `SPI1SMEN` writer - SPI1 clock enable during sleep mode bit
pub use SYSCFGSMEN_W as SPI1SMEN_W;
///Field `USART1SMEN` writer - USART1 clock enable during sleep mode bit
pub use SYSCFGSMEN_W as USART1SMEN_W;
///Field `DBGSMEN` writer - DBG clock enable during sleep mode bit
pub use SYSCFGSMEN_W as DBGSMEN_W;
impl R {
    ///Bit 0 - System configuration controller clock enable during sleep mode bit
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - TIM21 timer clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim21smen(&self) -> TIM21SMEN_R {
        TIM21SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - TIM22 timer clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim22smen(&self) -> TIM22SMEN_R {
        TIM22SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - ADC clock enable during sleep mode bit
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 22 - DBG clock enable during sleep mode bit
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2SMENR")
            .field("syscfgsmen", &self.syscfgsmen())
            .field("dbgsmen", &self.dbgsmen())
            .field("usart1smen", &self.usart1smen())
            .field("spi1smen", &self.spi1smen())
            .field("adcsmen", &self.adcsmen())
            .field("tim22smen", &self.tim22smen())
            .field("tim21smen", &self.tim21smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - System configuration controller clock enable during sleep mode bit
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<'_, APB2SMENRrs> {
        SYSCFGSMEN_W::new(self, 0)
    }
    ///Bit 2 - TIM21 timer clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim21smen(&mut self) -> TIM21SMEN_W<'_, APB2SMENRrs> {
        TIM21SMEN_W::new(self, 2)
    }
    ///Bit 5 - TIM22 timer clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim22smen(&mut self) -> TIM22SMEN_W<'_, APB2SMENRrs> {
        TIM22SMEN_W::new(self, 5)
    }
    ///Bit 9 - ADC clock enable during sleep mode bit
    #[inline(always)]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<'_, APB2SMENRrs> {
        ADCSMEN_W::new(self, 9)
    }
    ///Bit 12 - SPI1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<'_, APB2SMENRrs> {
        SPI1SMEN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<'_, APB2SMENRrs> {
        USART1SMEN_W::new(self, 14)
    }
    ///Bit 22 - DBG clock enable during sleep mode bit
    #[inline(always)]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W<'_, APB2SMENRrs> {
        DBGSMEN_W::new(self, 22)
    }
}
/**APB2 peripheral clock enable in sleep mode register

You can [`read`](crate::Reg::read) this register and get [`apb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:APB2SMENR)*/
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
///`reset()` method sets APB2SMENR to value 0x0040_5225
impl crate::Resettable for APB2SMENRrs {
    const RESET_VALUE: u32 = 0x0040_5225;
}
