#[doc = "Register `APB1ENR` reader"]
pub type R = crate::R<APB1ENRrs>;
#[doc = "Register `APB1ENR` writer"]
pub type W = crate::W<APB1ENRrs>;
#[doc = "Timer 3 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<TIM3EN> for bool {
    #[inline(always)]
    fn from(variant: TIM3EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM3EN` reader - Timer 3 clock enable"]
pub type TIM3EN_R = crate::BitReader<TIM3EN>;
impl TIM3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM3EN {
        match self.bits {
            false => TIM3EN::Disabled,
            true => TIM3EN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM3EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM3EN::Enabled
    }
}
#[doc = "Field `TIM3EN` writer - Timer 3 clock enable"]
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM3EN>;
impl<'a, REG> TIM3EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3EN::Enabled)
    }
}
#[doc = "Field `TIM6EN` reader - Timer 6 clock enable"]
pub use TIM3EN_R as TIM6EN_R;
#[doc = "Field `TIM7EN` reader - TIM7 timer clock enable"]
pub use TIM3EN_R as TIM7EN_R;
#[doc = "Field `TIM14EN` reader - Timer 14 clock enable"]
pub use TIM3EN_R as TIM14EN_R;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable"]
pub use TIM3EN_R as WWDGEN_R;
#[doc = "Field `SPI2EN` reader - SPI 2 clock enable"]
pub use TIM3EN_R as SPI2EN_R;
#[doc = "Field `USART2EN` reader - USART 2 clock enable"]
pub use TIM3EN_R as USART2EN_R;
#[doc = "Field `USART3EN` reader - USART3 clock enable"]
pub use TIM3EN_R as USART3EN_R;
#[doc = "Field `USART4EN` reader - USART4 clock enable"]
pub use TIM3EN_R as USART4EN_R;
#[doc = "Field `USART5EN` reader - USART5 clock enable"]
pub use TIM3EN_R as USART5EN_R;
#[doc = "Field `I2C1EN` reader - I2C 1 clock enable"]
pub use TIM3EN_R as I2C1EN_R;
#[doc = "Field `I2C2EN` reader - I2C 2 clock enable"]
pub use TIM3EN_R as I2C2EN_R;
#[doc = "Field `USBEN` reader - USB interface clock enable"]
pub use TIM3EN_R as USBEN_R;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub use TIM3EN_R as PWREN_R;
#[doc = "Field `TIM6EN` writer - Timer 6 clock enable"]
pub use TIM3EN_W as TIM6EN_W;
#[doc = "Field `TIM7EN` writer - TIM7 timer clock enable"]
pub use TIM3EN_W as TIM7EN_W;
#[doc = "Field `TIM14EN` writer - Timer 14 clock enable"]
pub use TIM3EN_W as TIM14EN_W;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable"]
pub use TIM3EN_W as WWDGEN_W;
#[doc = "Field `SPI2EN` writer - SPI 2 clock enable"]
pub use TIM3EN_W as SPI2EN_W;
#[doc = "Field `USART2EN` writer - USART 2 clock enable"]
pub use TIM3EN_W as USART2EN_W;
#[doc = "Field `USART3EN` writer - USART3 clock enable"]
pub use TIM3EN_W as USART3EN_W;
#[doc = "Field `USART4EN` writer - USART4 clock enable"]
pub use TIM3EN_W as USART4EN_W;
#[doc = "Field `USART5EN` writer - USART5 clock enable"]
pub use TIM3EN_W as USART5EN_W;
#[doc = "Field `I2C1EN` writer - I2C 1 clock enable"]
pub use TIM3EN_W as I2C1EN_W;
#[doc = "Field `I2C2EN` writer - I2C 2 clock enable"]
pub use TIM3EN_W as I2C2EN_W;
#[doc = "Field `USBEN` writer - USB interface clock enable"]
pub use TIM3EN_W as USBEN_W;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub use TIM3EN_W as PWREN_W;
impl R {
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer 14 clock enable"]
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI 2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable"]
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART5 clock enable"]
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB interface clock enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<APB1ENRrs> {
        TIM3EN_W::new(self, 1)
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<APB1ENRrs> {
        TIM6EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<APB1ENRrs> {
        TIM7EN_W::new(self, 5)
    }
    #[doc = "Bit 8 - Timer 14 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim14en(&mut self) -> TIM14EN_W<APB1ENRrs> {
        TIM14EN_W::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APB1ENRrs> {
        WWDGEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APB1ENRrs> {
        SPI2EN_W::new(self, 14)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APB1ENRrs> {
        USART2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<APB1ENRrs> {
        USART3EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart4en(&mut self) -> USART4EN_W<APB1ENRrs> {
        USART4EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - USART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart5en(&mut self) -> USART5EN_W<APB1ENRrs> {
        USART5EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1ENRrs> {
        I2C1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<APB1ENRrs> {
        I2C2EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - USB interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<APB1ENRrs> {
        USBEN_W::new(self, 23)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<APB1ENRrs> {
        PWREN_W::new(self, 28)
    }
}
#[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1ENRrs;
impl crate::RegisterSpec for APB1ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr::R`](R) reader structure"]
impl crate::Readable for APB1ENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1enr::W`](W) writer structure"]
impl crate::Writable for APB1ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1ENR to value 0"]
impl crate::Resettable for APB1ENRrs {
    const RESET_VALUE: u32 = 0;
}
