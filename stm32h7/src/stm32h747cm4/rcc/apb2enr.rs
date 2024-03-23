#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<APB2ENRrs>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<APB2ENRrs>;
#[doc = "TIM1 peripheral clock enable\n\nValue on reset: 0"]
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
#[doc = "Field `TIM1EN` reader - TIM1 peripheral clock enable"]
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
#[doc = "Field `TIM1EN` writer - TIM1 peripheral clock enable"]
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
#[doc = "Field `TIM8EN` reader - TIM8 peripheral clock enable"]
pub use TIM1EN_R as TIM8EN_R;
#[doc = "Field `USART1EN` reader - USART1 Peripheral Clocks Enable"]
pub use TIM1EN_R as USART1EN_R;
#[doc = "Field `USART6EN` reader - USART6 Peripheral Clocks Enable"]
pub use TIM1EN_R as USART6EN_R;
#[doc = "Field `SPI1EN` reader - SPI1 Peripheral Clocks Enable"]
pub use TIM1EN_R as SPI1EN_R;
#[doc = "Field `SPI4EN` reader - SPI4 Peripheral Clocks Enable"]
pub use TIM1EN_R as SPI4EN_R;
#[doc = "Field `TIM15EN` reader - TIM15 peripheral clock enable"]
pub use TIM1EN_R as TIM15EN_R;
#[doc = "Field `TIM16EN` reader - TIM16 peripheral clock enable"]
pub use TIM1EN_R as TIM16EN_R;
#[doc = "Field `TIM17EN` reader - TIM17 peripheral clock enable"]
pub use TIM1EN_R as TIM17EN_R;
#[doc = "Field `SPI5EN` reader - SPI5 Peripheral Clocks Enable"]
pub use TIM1EN_R as SPI5EN_R;
#[doc = "Field `SAI1EN` reader - SAI1 Peripheral Clocks Enable"]
pub use TIM1EN_R as SAI1EN_R;
#[doc = "Field `SAI2EN` reader - SAI2 Peripheral Clocks Enable"]
pub use TIM1EN_R as SAI2EN_R;
#[doc = "Field `SAI3EN` reader - SAI3 Peripheral Clocks Enable"]
pub use TIM1EN_R as SAI3EN_R;
#[doc = "Field `DFSDM1EN` reader - DFSDM1 Peripheral Clocks Enable"]
pub use TIM1EN_R as DFSDM1EN_R;
#[doc = "Field `HRTIMEN` reader - HRTIM peripheral clock enable"]
pub use TIM1EN_R as HRTIMEN_R;
#[doc = "Field `TIM8EN` writer - TIM8 peripheral clock enable"]
pub use TIM1EN_W as TIM8EN_W;
#[doc = "Field `USART1EN` writer - USART1 Peripheral Clocks Enable"]
pub use TIM1EN_W as USART1EN_W;
#[doc = "Field `USART6EN` writer - USART6 Peripheral Clocks Enable"]
pub use TIM1EN_W as USART6EN_W;
#[doc = "Field `SPI1EN` writer - SPI1 Peripheral Clocks Enable"]
pub use TIM1EN_W as SPI1EN_W;
#[doc = "Field `SPI4EN` writer - SPI4 Peripheral Clocks Enable"]
pub use TIM1EN_W as SPI4EN_W;
#[doc = "Field `TIM15EN` writer - TIM15 peripheral clock enable"]
pub use TIM1EN_W as TIM15EN_W;
#[doc = "Field `TIM16EN` writer - TIM16 peripheral clock enable"]
pub use TIM1EN_W as TIM16EN_W;
#[doc = "Field `TIM17EN` writer - TIM17 peripheral clock enable"]
pub use TIM1EN_W as TIM17EN_W;
#[doc = "Field `SPI5EN` writer - SPI5 Peripheral Clocks Enable"]
pub use TIM1EN_W as SPI5EN_W;
#[doc = "Field `SAI1EN` writer - SAI1 Peripheral Clocks Enable"]
pub use TIM1EN_W as SAI1EN_W;
#[doc = "Field `SAI2EN` writer - SAI2 Peripheral Clocks Enable"]
pub use TIM1EN_W as SAI2EN_W;
#[doc = "Field `SAI3EN` writer - SAI3 Peripheral Clocks Enable"]
pub use TIM1EN_W as SAI3EN_W;
#[doc = "Field `DFSDM1EN` writer - DFSDM1 Peripheral Clocks Enable"]
pub use TIM1EN_W as DFSDM1EN_W;
#[doc = "Field `HRTIMEN` writer - HRTIM peripheral clock enable"]
pub use TIM1EN_W as HRTIMEN_W;
impl R {
    #[doc = "Bit 0 - TIM1 peripheral clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai3en(&self) -> SAI3EN_R {
        SAI3EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn dfsdm1en(&self) -> DFSDM1EN_R {
        DFSDM1EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable"]
    #[inline(always)]
    pub fn hrtimen(&self) -> HRTIMEN_R {
        HRTIMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<APB2ENRrs> {
        TIM1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<APB2ENRrs> {
        TIM8EN_W::new(self, 1)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB2ENRrs> {
        USART1EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<APB2ENRrs> {
        USART6EN_W::new(self, 5)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> SPI4EN_W<APB2ENRrs> {
        SPI4EN_W::new(self, 13)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<APB2ENRrs> {
        TIM15EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi5en(&mut self) -> SPI5EN_W<APB2ENRrs> {
        SPI5EN_W::new(self, 20)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<APB2ENRrs> {
        SAI1EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai2en(&mut self) -> SAI2EN_W<APB2ENRrs> {
        SAI2EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai3en(&mut self) -> SAI3EN_W<APB2ENRrs> {
        SAI3EN_W::new(self, 24)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1en(&mut self) -> DFSDM1EN_W<APB2ENRrs> {
        DFSDM1EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrtimen(&mut self) -> HRTIMEN_W<APB2ENRrs> {
        HRTIMEN_W::new(self, 29)
    }
}
#[doc = "RCC APB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
