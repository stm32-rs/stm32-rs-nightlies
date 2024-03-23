#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<APB2ENRrs>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<APB2ENRrs>;
#[doc = "SYSCFG clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<SYSCFGEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGEN` reader - SYSCFG clock enable"]
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN>;
impl SYSCFGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGEN {
        match self.bits {
            false => SYSCFGEN::Disabled,
            true => SYSCFGEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGEN::Enabled
    }
}
#[doc = "Field `SYSCFGEN` writer - SYSCFG clock enable"]
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGEN>;
impl<'a, REG> SYSCFGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Enabled)
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 Timer clock enable"]
pub use SYSCFGEN_R as TIM1EN_R;
#[doc = "Field `SPI1EN` reader - SPI 1 clock enable"]
pub use SYSCFGEN_R as SPI1EN_R;
#[doc = "Field `TIM8EN` reader - TIM8 Timer clock enable"]
pub use SYSCFGEN_R as TIM8EN_R;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub use SYSCFGEN_R as USART1EN_R;
#[doc = "Field `SPI4EN` reader - SPI4 clock enable"]
pub use SYSCFGEN_R as SPI4EN_R;
#[doc = "Field `TIM15EN` reader - TIM15 timer clock enable"]
pub use SYSCFGEN_R as TIM15EN_R;
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable"]
pub use SYSCFGEN_R as TIM16EN_R;
#[doc = "Field `TIM17EN` reader - TIM17 timer clock enable"]
pub use SYSCFGEN_R as TIM17EN_R;
#[doc = "Field `TIM20EN` reader - TIM20 timer clock enable"]
pub use SYSCFGEN_R as TIM20EN_R;
#[doc = "Field `TIM1EN` writer - TIM1 Timer clock enable"]
pub use SYSCFGEN_W as TIM1EN_W;
#[doc = "Field `SPI1EN` writer - SPI 1 clock enable"]
pub use SYSCFGEN_W as SPI1EN_W;
#[doc = "Field `TIM8EN` writer - TIM8 Timer clock enable"]
pub use SYSCFGEN_W as TIM8EN_W;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub use SYSCFGEN_W as USART1EN_W;
#[doc = "Field `SPI4EN` writer - SPI4 clock enable"]
pub use SYSCFGEN_W as SPI4EN_W;
#[doc = "Field `TIM15EN` writer - TIM15 timer clock enable"]
pub use SYSCFGEN_W as TIM15EN_W;
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable"]
pub use SYSCFGEN_W as TIM16EN_W;
#[doc = "Field `TIM17EN` writer - TIM17 timer clock enable"]
pub use SYSCFGEN_W as TIM17EN_W;
#[doc = "Field `TIM20EN` writer - TIM20 timer clock enable"]
pub use SYSCFGEN_W as TIM20EN_W;
impl R {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 Timer clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM20 timer clock enable"]
    #[inline(always)]
    pub fn tim20en(&self) -> TIM20EN_R {
        TIM20EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<APB2ENRrs> {
        SYSCFGEN_W::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 Timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<APB2ENRrs> {
        TIM8EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> SPI4EN_W<APB2ENRrs> {
        SPI4EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<APB2ENRrs> {
        TIM15EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    #[doc = "Bit 20 - TIM20 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim20en(&mut self) -> TIM20EN_W<APB2ENRrs> {
        TIM20EN_W::new(self, 20)
    }
}
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2enr::R`](R) reader structure"]
impl crate::Readable for APB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure"]
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
