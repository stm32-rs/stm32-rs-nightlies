#[doc = "Register `APB1ENR` reader"]
pub type R = crate::R<APB1ENRrs>;
#[doc = "Register `APB1ENR` writer"]
pub type W = crate::W<APB1ENRrs>;
#[doc = "Timer2 clock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<TIM2EN> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2EN` reader - Timer2 clock enable bit"]
pub type TIM2EN_R = crate::BitReader<TIM2EN>;
impl TIM2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM2EN {
        match self.bits {
            false => TIM2EN::Disabled,
            true => TIM2EN::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN::Enabled
    }
}
#[doc = "Field `TIM2EN` writer - Timer2 clock enable bit"]
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Enabled)
    }
}
#[doc = "Field `TIM6EN` reader - Timer 6 clock enable bit"]
pub use TIM2EN_R as TIM6EN_R;
#[doc = "Field `TIM7EN` reader - Timer 7 clock enable bit"]
pub use TIM2EN_R as TIM7EN_R;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable bit"]
pub use TIM2EN_R as WWDGEN_R;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable bit"]
pub use TIM2EN_R as SPI2EN_R;
#[doc = "Field `USART2EN` reader - UART2 clock enable bit"]
pub use TIM2EN_R as USART2EN_R;
#[doc = "Field `LPUART1EN` reader - LPUART1 clock enable bit"]
pub use TIM2EN_R as LPUART1EN_R;
#[doc = "Field `USART4EN` reader - USART4 clock enable bit"]
pub use TIM2EN_R as USART4EN_R;
#[doc = "Field `USART5EN` reader - USART5 clock enable bit"]
pub use TIM2EN_R as USART5EN_R;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable bit"]
pub use TIM2EN_R as I2C1EN_R;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable bit"]
pub use TIM2EN_R as I2C2EN_R;
#[doc = "Field `PWREN` reader - Power interface clock enable bit"]
pub use TIM2EN_R as PWREN_R;
#[doc = "Field `I2C3EN` reader - I2C3 clock enable bit"]
pub use TIM2EN_R as I2C3EN_R;
#[doc = "Field `LPTIM1EN` reader - Low power timer clock enable bit"]
pub use TIM2EN_R as LPTIM1EN_R;
#[doc = "Field `TIM6EN` writer - Timer 6 clock enable bit"]
pub use TIM2EN_W as TIM6EN_W;
#[doc = "Field `TIM7EN` writer - Timer 7 clock enable bit"]
pub use TIM2EN_W as TIM7EN_W;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable bit"]
pub use TIM2EN_W as WWDGEN_W;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable bit"]
pub use TIM2EN_W as SPI2EN_W;
#[doc = "Field `USART2EN` writer - UART2 clock enable bit"]
pub use TIM2EN_W as USART2EN_W;
#[doc = "Field `LPUART1EN` writer - LPUART1 clock enable bit"]
pub use TIM2EN_W as LPUART1EN_W;
#[doc = "Field `USART4EN` writer - USART4 clock enable bit"]
pub use TIM2EN_W as USART4EN_W;
#[doc = "Field `USART5EN` writer - USART5 clock enable bit"]
pub use TIM2EN_W as USART5EN_W;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable bit"]
pub use TIM2EN_W as I2C1EN_W;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable bit"]
pub use TIM2EN_W as I2C2EN_W;
#[doc = "Field `PWREN` writer - Power interface clock enable bit"]
pub use TIM2EN_W as PWREN_W;
#[doc = "Field `I2C3EN` writer - I2C3 clock enable bit"]
pub use TIM2EN_W as I2C3EN_W;
#[doc = "Field `LPTIM1EN` writer - Low power timer clock enable bit"]
pub use TIM2EN_W as LPTIM1EN_W;
impl R {
    #[doc = "Bit 0 - Timer2 clock enable bit"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable bit"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 clock enable bit"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable bit"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable bit"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - UART2 clock enable bit"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LPUART1 clock enable bit"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable bit"]
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART5 clock enable bit"]
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable bit"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable bit"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable bit"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 clock enable bit"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer clock enable bit"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer2 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<APB1ENRrs> {
        TIM2EN_W::new(self, 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<APB1ENRrs> {
        TIM6EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer 7 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<APB1ENRrs> {
        TIM7EN_W::new(self, 5)
    }
    #[doc = "Bit 11 - Window watchdog clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APB1ENRrs> {
        WWDGEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APB1ENRrs> {
        SPI2EN_W::new(self, 14)
    }
    #[doc = "Bit 17 - UART2 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APB1ENRrs> {
        USART2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - LPUART1 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<APB1ENRrs> {
        LPUART1EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart4en(&mut self) -> USART4EN_W<APB1ENRrs> {
        USART4EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - USART5 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn usart5en(&mut self) -> USART5EN_W<APB1ENRrs> {
        USART5EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1ENRrs> {
        I2C1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<APB1ENRrs> {
        I2C2EN_W::new(self, 22)
    }
    #[doc = "Bit 28 - Power interface clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<APB1ENRrs> {
        PWREN_W::new(self, 28)
    }
    #[doc = "Bit 30 - I2C3 clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<APB1ENRrs> {
        I2C3EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low power timer clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<APB1ENRrs> {
        LPTIM1EN_W::new(self, 31)
    }
}
#[doc = "APB1 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
