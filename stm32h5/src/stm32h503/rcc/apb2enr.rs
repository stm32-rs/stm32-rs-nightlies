#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<APB2ENRrs>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<APB2ENRrs>;
#[doc = "TIM1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<TIM1EN> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 clock enable Set and reset by software."]
pub type TIM1EN_R = crate::BitReader<TIM1EN>;
impl TIM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM1EN {
        match self.bits {
            false => TIM1EN::Disabled,
            true => TIM1EN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1EN::Enabled
    }
}
#[doc = "Field `TIM1EN` writer - TIM1 clock enable Set and reset by software."]
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1EN>;
impl<'a, REG> TIM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::Enabled)
    }
}
#[doc = "Field `SPI1EN` reader - SPI1 clock enable Set and reset by software."]
pub use TIM1EN_R as SPI1EN_R;
#[doc = "Field `USART1EN` reader - USART1 clock enable Set and reset by software."]
pub use TIM1EN_R as USART1EN_R;
#[doc = "Field `USBEN` reader - USB clock enable"]
pub use TIM1EN_R as USBEN_R;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable Set and reset by software."]
pub use TIM1EN_W as SPI1EN_W;
#[doc = "Field `USART1EN` writer - USART1 clock enable Set and reset by software."]
pub use TIM1EN_W as USART1EN_W;
#[doc = "Field `USBEN` writer - USB clock enable"]
pub use TIM1EN_W as USBEN_W;
impl R {
    #[doc = "Bit 11 - TIM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 24 - USB clock enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    #[doc = "Bit 24 - USB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<APB2ENRrs> {
        USBEN_W::new(self, 24)
    }
}
#[doc = "RCC APB2 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
