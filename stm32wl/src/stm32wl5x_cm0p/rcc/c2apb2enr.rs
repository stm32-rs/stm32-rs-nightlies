#[doc = "Register `C2APB2ENR` reader"]
pub type R = crate::R<C2APB2ENRrs>;
#[doc = "Register `C2APB2ENR` writer"]
pub type W = crate::W<C2APB2ENRrs>;
#[doc = "ADC clocks enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCEN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<ADCEN> for bool {
    #[inline(always)]
    fn from(variant: ADCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - ADC clocks enable"]
pub type ADCEN_R = crate::BitReader<ADCEN>;
impl ADCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCEN {
        match self.bits {
            false => ADCEN::Disabled,
            true => ADCEN::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCEN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCEN::Enabled
    }
}
#[doc = "Field `ADCEN` writer - ADC clocks enable"]
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCEN>;
impl<'a, REG> ADCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Enabled)
    }
}
#[doc = "Field `TIM1EN` reader - CPU2 TIM1 timer clock enable"]
pub use ADCEN_R as TIM1EN_R;
#[doc = "Field `SPI1EN` reader - CPU2 SPI1 clock enable"]
pub use ADCEN_R as SPI1EN_R;
#[doc = "Field `USART1EN` reader - CPU2 USART1clocks enable"]
pub use ADCEN_R as USART1EN_R;
#[doc = "Field `TIM16EN` reader - CPU2 TIM16 timer clock enable"]
pub use ADCEN_R as TIM16EN_R;
#[doc = "Field `TIM17EN` reader - CPU2 TIM17 timer clock enable"]
pub use ADCEN_R as TIM17EN_R;
#[doc = "Field `TIM1EN` writer - CPU2 TIM1 timer clock enable"]
pub use ADCEN_W as TIM1EN_W;
#[doc = "Field `SPI1EN` writer - CPU2 SPI1 clock enable"]
pub use ADCEN_W as SPI1EN_W;
#[doc = "Field `USART1EN` writer - CPU2 USART1clocks enable"]
pub use ADCEN_W as USART1EN_W;
#[doc = "Field `TIM16EN` writer - CPU2 TIM16 timer clock enable"]
pub use ADCEN_W as TIM16EN_W;
#[doc = "Field `TIM17EN` writer - CPU2 TIM17 timer clock enable"]
pub use ADCEN_W as TIM17EN_W;
impl R {
    #[doc = "Bit 9 - ADC clocks enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU2 TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU2 SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU2 USART1clocks enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU2 TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU2 TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - ADC clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<C2APB2ENRrs> {
        ADCEN_W::new(self, 9)
    }
    #[doc = "Bit 11 - CPU2 TIM1 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<C2APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - CPU2 SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<C2APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 14 - CPU2 USART1clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<C2APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    #[doc = "Bit 17 - CPU2 TIM16 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<C2APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU2 TIM17 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<C2APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
}
#[doc = "CPU2 APB2 peripheral clock enable register \\[dual core device only\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2apb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2apb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2APB2ENRrs;
impl crate::RegisterSpec for C2APB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2apb2enr::R`](R) reader structure"]
impl crate::Readable for C2APB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2apb2enr::W`](W) writer structure"]
impl crate::Writable for C2APB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2APB2ENR to value 0"]
impl crate::Resettable for C2APB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
