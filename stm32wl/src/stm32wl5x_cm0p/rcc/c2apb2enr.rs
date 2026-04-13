///Register `C2APB2ENR` reader
pub type R = crate::R<C2APB2ENRrs>;
///Register `C2APB2ENR` writer
pub type W = crate::W<C2APB2ENRrs>;
/**ADC clocks enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<ADCEN> for bool {
    #[inline(always)]
    fn from(variant: ADCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCEN` reader - ADC clocks enable
pub type ADCEN_R = crate::BitReader<ADCEN>;
impl ADCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCEN {
        match self.bits {
            false => ADCEN::Disabled,
            true => ADCEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCEN::Enabled
    }
}
///Field `ADCEN` writer - ADC clocks enable
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCEN>;
impl<'a, REG> ADCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Enabled)
    }
}
///Field `TIM1EN` reader - CPU2 TIM1 timer clock enable
pub use ADCEN_R as TIM1EN_R;
///Field `SPI1EN` reader - CPU2 SPI1 clock enable
pub use ADCEN_R as SPI1EN_R;
///Field `USART1EN` reader - CPU2 USART1clocks enable
pub use ADCEN_R as USART1EN_R;
///Field `TIM16EN` reader - CPU2 TIM16 timer clock enable
pub use ADCEN_R as TIM16EN_R;
///Field `TIM17EN` reader - CPU2 TIM17 timer clock enable
pub use ADCEN_R as TIM17EN_R;
///Field `TIM1EN` writer - CPU2 TIM1 timer clock enable
pub use ADCEN_W as TIM1EN_W;
///Field `SPI1EN` writer - CPU2 SPI1 clock enable
pub use ADCEN_W as SPI1EN_W;
///Field `USART1EN` writer - CPU2 USART1clocks enable
pub use ADCEN_W as USART1EN_W;
///Field `TIM16EN` writer - CPU2 TIM16 timer clock enable
pub use ADCEN_W as TIM16EN_W;
///Field `TIM17EN` writer - CPU2 TIM17 timer clock enable
pub use ADCEN_W as TIM17EN_W;
impl R {
    ///Bit 9 - ADC clocks enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - CPU2 TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU2 SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - CPU2 USART1clocks enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - CPU2 TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CPU2 TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB2ENR")
            .field("adcen", &self.adcen())
            .field("tim17en", &self.tim17en())
            .field("tim16en", &self.tim16en())
            .field("usart1en", &self.usart1en())
            .field("spi1en", &self.spi1en())
            .field("tim1en", &self.tim1en())
            .finish()
    }
}
impl W {
    ///Bit 9 - ADC clocks enable
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<'_, C2APB2ENRrs> {
        ADCEN_W::new(self, 9)
    }
    ///Bit 11 - CPU2 TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<'_, C2APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    ///Bit 12 - CPU2 SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<'_, C2APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 14 - CPU2 USART1clocks enable
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, C2APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 17 - CPU2 TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<'_, C2APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 18 - CPU2 TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W<'_, C2APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
}
/**CPU2 APB2 peripheral clock enable register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2APB2ENR)*/
pub struct C2APB2ENRrs;
impl crate::RegisterSpec for C2APB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2apb2enr::R`](R) reader structure
impl crate::Readable for C2APB2ENRrs {}
///`write(|w| ..)` method takes [`c2apb2enr::W`](W) writer structure
impl crate::Writable for C2APB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2APB2ENR to value 0
impl crate::Resettable for C2APB2ENRrs {}
