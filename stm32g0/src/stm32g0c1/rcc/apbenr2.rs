///Register `APBENR2` reader
pub type R = crate::R<APBENR2rs>;
///Register `APBENR2` writer
pub type W = crate::W<APBENR2rs>;
/**SYSCFG, COMP and VREFBUF clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<SYSCFGEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGEN` reader - SYSCFG, COMP and VREFBUF clock enable
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN>;
impl SYSCFGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGEN {
        match self.bits {
            false => SYSCFGEN::Disabled,
            true => SYSCFGEN::Enabled,
        }
    }
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGEN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGEN::Enabled
    }
}
///Field `SYSCFGEN` writer - SYSCFG, COMP and VREFBUF clock enable
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGEN>;
impl<'a, REG> SYSCFGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Enabled)
    }
}
///Field `TIM1EN` reader - TIM1 timer clock enable
pub use SYSCFGEN_R as TIM1EN_R;
///Field `SPI1EN` reader - SPI1 clock enable
pub use SYSCFGEN_R as SPI1EN_R;
///Field `USART1EN` reader - USART1 clock enable
pub use SYSCFGEN_R as USART1EN_R;
///Field `TIM14EN` reader - TIM14 timer clock enable
pub use SYSCFGEN_R as TIM14EN_R;
///Field `TIM15EN` reader - TIM15 timer clock enable
pub use SYSCFGEN_R as TIM15EN_R;
///Field `TIM16EN` reader - TIM16 timer clock enable
pub use SYSCFGEN_R as TIM16EN_R;
///Field `TIM17EN` reader - TIM16 timer clock enable
pub use SYSCFGEN_R as TIM17EN_R;
///Field `ADCEN` reader - ADC clock enable
pub use SYSCFGEN_R as ADCEN_R;
///Field `TIM1EN` writer - TIM1 timer clock enable
pub use SYSCFGEN_W as TIM1EN_W;
///Field `SPI1EN` writer - SPI1 clock enable
pub use SYSCFGEN_W as SPI1EN_W;
///Field `USART1EN` writer - USART1 clock enable
pub use SYSCFGEN_W as USART1EN_W;
///Field `TIM14EN` writer - TIM14 timer clock enable
pub use SYSCFGEN_W as TIM14EN_W;
///Field `TIM15EN` writer - TIM15 timer clock enable
pub use SYSCFGEN_W as TIM15EN_W;
///Field `TIM16EN` writer - TIM16 timer clock enable
pub use SYSCFGEN_W as TIM16EN_W;
///Field `TIM17EN` writer - TIM16 timer clock enable
pub use SYSCFGEN_W as TIM17EN_W;
///Field `ADCEN` writer - ADC clock enable
pub use SYSCFGEN_W as ADCEN_W;
impl R {
    ///Bit 0 - SYSCFG, COMP and VREFBUF clock enable
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TIM14 timer clock enable
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TIM15 timer clock enable
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM16 timer clock enable
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ADC clock enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBENR2")
            .field("syscfgen", &self.syscfgen())
            .field("tim1en", &self.tim1en())
            .field("spi1en", &self.spi1en())
            .field("usart1en", &self.usart1en())
            .field("tim14en", &self.tim14en())
            .field("tim15en", &self.tim15en())
            .field("tim16en", &self.tim16en())
            .field("tim17en", &self.tim17en())
            .field("adcen", &self.adcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG, COMP and VREFBUF clock enable
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, APBENR2rs> {
        SYSCFGEN_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<'_, APBENR2rs> {
        TIM1EN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<'_, APBENR2rs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, APBENR2rs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 15 - TIM14 timer clock enable
    #[inline(always)]
    pub fn tim14en(&mut self) -> TIM14EN_W<'_, APBENR2rs> {
        TIM14EN_W::new(self, 15)
    }
    ///Bit 16 - TIM15 timer clock enable
    #[inline(always)]
    pub fn tim15en(&mut self) -> TIM15EN_W<'_, APBENR2rs> {
        TIM15EN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<'_, APBENR2rs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 18 - TIM16 timer clock enable
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W<'_, APBENR2rs> {
        TIM17EN_W::new(self, 18)
    }
    ///Bit 20 - ADC clock enable
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<'_, APBENR2rs> {
        ADCEN_W::new(self, 20)
    }
}
/**APB peripheral clock enable register 2

You can [`read`](crate::Reg::read) this register and get [`apbenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#RCC:APBENR2)*/
pub struct APBENR2rs;
impl crate::RegisterSpec for APBENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apbenr2::R`](R) reader structure
impl crate::Readable for APBENR2rs {}
///`write(|w| ..)` method takes [`apbenr2::W`](W) writer structure
impl crate::Writable for APBENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APBENR2 to value 0
impl crate::Resettable for APBENR2rs {}
