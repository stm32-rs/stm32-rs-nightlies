///Register `APBSMENR2` reader
pub type R = crate::R<APBSMENR2rs>;
///Register `APBSMENR2` writer
pub type W = crate::W<APBSMENR2rs>;
/**SYSCFG, COMP and VREFBUF clock enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGSMEN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<SYSCFGSMEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGSMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGSMEN` reader - SYSCFG, COMP and VREFBUF clock enable during Sleep mode
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
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGSMEN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGSMEN::Enabled
    }
}
///Field `SYSCFGSMEN` writer - SYSCFG, COMP and VREFBUF clock enable during Sleep mode
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGSMEN>;
impl<'a, REG> SYSCFGSMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGSMEN::Enabled)
    }
}
///Field `TIM1SMEN` reader - TIM1 timer clock enable during Sleep mode
pub use SYSCFGSMEN_R as TIM1SMEN_R;
///Field `SPI1SMEN` reader - SPI1 clock enable during Sleep mode
pub use SYSCFGSMEN_R as SPI1SMEN_R;
///Field `USART1SMEN` reader - USART1 clock enable during Sleep mode
pub use SYSCFGSMEN_R as USART1SMEN_R;
///Field `TIM14SMEN` reader - TIM14 timer clock enable during Sleep mode
pub use SYSCFGSMEN_R as TIM14SMEN_R;
///Field `TIM15SMEN` reader - TIM15 timer clock enable during Sleep mode
pub use SYSCFGSMEN_R as TIM15SMEN_R;
///Field `TIM16SMEN` reader - TIM16 timer clock enable during Sleep mode
pub use SYSCFGSMEN_R as TIM16SMEN_R;
///Field `TIM17SMEN` reader - TIM16 timer clock enable during Sleep mode
pub use SYSCFGSMEN_R as TIM17SMEN_R;
///Field `ADCSMEN` reader - ADC clock enable during Sleep mode
pub use SYSCFGSMEN_R as ADCSMEN_R;
///Field `TIM1SMEN` writer - TIM1 timer clock enable during Sleep mode
pub use SYSCFGSMEN_W as TIM1SMEN_W;
///Field `SPI1SMEN` writer - SPI1 clock enable during Sleep mode
pub use SYSCFGSMEN_W as SPI1SMEN_W;
///Field `USART1SMEN` writer - USART1 clock enable during Sleep mode
pub use SYSCFGSMEN_W as USART1SMEN_W;
///Field `TIM14SMEN` writer - TIM14 timer clock enable during Sleep mode
pub use SYSCFGSMEN_W as TIM14SMEN_W;
///Field `TIM15SMEN` writer - TIM15 timer clock enable during Sleep mode
pub use SYSCFGSMEN_W as TIM15SMEN_W;
///Field `TIM16SMEN` writer - TIM16 timer clock enable during Sleep mode
pub use SYSCFGSMEN_W as TIM16SMEN_W;
///Field `TIM17SMEN` writer - TIM16 timer clock enable during Sleep mode
pub use SYSCFGSMEN_W as TIM17SMEN_W;
///Field `ADCSMEN` writer - ADC clock enable during Sleep mode
pub use SYSCFGSMEN_W as ADCSMEN_W;
impl R {
    ///Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep mode
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TIM14 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim14smen(&self) -> TIM14SMEN_R {
        TIM14SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TIM15 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim15smen(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM16 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ADC clock enable during Sleep mode
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBSMENR2")
            .field("syscfgsmen", &self.syscfgsmen())
            .field("tim1smen", &self.tim1smen())
            .field("spi1smen", &self.spi1smen())
            .field("usart1smen", &self.usart1smen())
            .field("tim14smen", &self.tim14smen())
            .field("tim15smen", &self.tim15smen())
            .field("tim16smen", &self.tim16smen())
            .field("tim17smen", &self.tim17smen())
            .field("adcsmen", &self.adcsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep mode
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<'_, APBSMENR2rs> {
        SYSCFGSMEN_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<'_, APBSMENR2rs> {
        TIM1SMEN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<'_, APBSMENR2rs> {
        SPI1SMEN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<'_, APBSMENR2rs> {
        USART1SMEN_W::new(self, 14)
    }
    ///Bit 15 - TIM14 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim14smen(&mut self) -> TIM14SMEN_W<'_, APBSMENR2rs> {
        TIM14SMEN_W::new(self, 15)
    }
    ///Bit 16 - TIM15 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim15smen(&mut self) -> TIM15SMEN_W<'_, APBSMENR2rs> {
        TIM15SMEN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<'_, APBSMENR2rs> {
        TIM16SMEN_W::new(self, 17)
    }
    ///Bit 18 - TIM16 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<'_, APBSMENR2rs> {
        TIM17SMEN_W::new(self, 18)
    }
    ///Bit 20 - ADC clock enable during Sleep mode
    #[inline(always)]
    pub fn adcsmen(&mut self) -> ADCSMEN_W<'_, APBSMENR2rs> {
        ADCSMEN_W::new(self, 20)
    }
}
/**APB peripheral clock enable in Sleep mode register 2

You can [`read`](crate::Reg::read) this register and get [`apbsmenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#RCC:APBSMENR2)*/
pub struct APBSMENR2rs;
impl crate::RegisterSpec for APBSMENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apbsmenr2::R`](R) reader structure
impl crate::Readable for APBSMENR2rs {}
///`write(|w| ..)` method takes [`apbsmenr2::W`](W) writer structure
impl crate::Writable for APBSMENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APBSMENR2 to value 0x0017_d801
impl crate::Resettable for APBSMENR2rs {
    const RESET_VALUE: u32 = 0x0017_d801;
}
