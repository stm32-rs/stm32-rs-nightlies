#[doc = "Register `APB1ENR` reader"]
pub type R = crate::R<APB1ENRrs>;
#[doc = "Register `APB1ENR` writer"]
pub type W = crate::W<APB1ENRrs>;
#[doc = "Timer 2 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<TIM2EN> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2EN` reader - Timer 2 clock enable"]
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
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN::Enabled
    }
}
#[doc = "Field `TIM2EN` writer - Timer 2 clock enable"]
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Enabled)
    }
}
#[doc = "Field `TIM3EN` reader - Timer 3 clock enable"]
pub use TIM2EN_R as TIM3EN_R;
#[doc = "Field `TIM4EN` reader - Timer 4 clock enable"]
pub use TIM2EN_R as TIM4EN_R;
#[doc = "Field `TIM6EN` reader - Timer 6 clock enable"]
pub use TIM2EN_R as TIM6EN_R;
#[doc = "Field `TIM7EN` reader - Timer 7 clock enable"]
pub use TIM2EN_R as TIM7EN_R;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable"]
pub use TIM2EN_R as WWDGEN_R;
#[doc = "Field `SPI2EN` reader - SPI 2 clock enable"]
pub use TIM2EN_R as SPI2EN_R;
#[doc = "Field `SPI3EN` reader - SPI 3 clock enable"]
pub use TIM2EN_R as SPI3EN_R;
#[doc = "Field `USART2EN` reader - USART 2 clock enable"]
pub use TIM2EN_R as USART2EN_R;
#[doc = "Field `USART3EN` reader - USART 3 clock enable"]
pub use TIM2EN_R as USART3EN_R;
#[doc = "Field `UART4EN` reader - USART 4 clock enable"]
pub use TIM2EN_R as UART4EN_R;
#[doc = "Field `UART5EN` reader - USART 5 clock enable"]
pub use TIM2EN_R as UART5EN_R;
#[doc = "Field `I2C1EN` reader - I2C 1 clock enable"]
pub use TIM2EN_R as I2C1EN_R;
#[doc = "Field `I2C2EN` reader - I2C 2 clock enable"]
pub use TIM2EN_R as I2C2EN_R;
#[doc = "Field `USBEN` reader - USB clock enable"]
pub use TIM2EN_R as USBEN_R;
#[doc = "Field `CANEN` reader - CAN clock enable"]
pub use TIM2EN_R as CANEN_R;
#[doc = "Field `DAC2EN` reader - DAC2 interface clock enable"]
pub use TIM2EN_R as DAC2EN_R;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub use TIM2EN_R as PWREN_R;
#[doc = "Field `DAC1EN` reader - DAC interface clock enable"]
pub use TIM2EN_R as DAC1EN_R;
#[doc = "Field `I2C3EN` reader - I2C3 clock enable"]
pub use TIM2EN_R as I2C3EN_R;
#[doc = "Field `TIM3EN` writer - Timer 3 clock enable"]
pub use TIM2EN_W as TIM3EN_W;
#[doc = "Field `TIM4EN` writer - Timer 4 clock enable"]
pub use TIM2EN_W as TIM4EN_W;
#[doc = "Field `TIM6EN` writer - Timer 6 clock enable"]
pub use TIM2EN_W as TIM6EN_W;
#[doc = "Field `TIM7EN` writer - Timer 7 clock enable"]
pub use TIM2EN_W as TIM7EN_W;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable"]
pub use TIM2EN_W as WWDGEN_W;
#[doc = "Field `SPI2EN` writer - SPI 2 clock enable"]
pub use TIM2EN_W as SPI2EN_W;
#[doc = "Field `SPI3EN` writer - SPI 3 clock enable"]
pub use TIM2EN_W as SPI3EN_W;
#[doc = "Field `USART2EN` writer - USART 2 clock enable"]
pub use TIM2EN_W as USART2EN_W;
#[doc = "Field `USART3EN` writer - USART 3 clock enable"]
pub use TIM2EN_W as USART3EN_W;
#[doc = "Field `UART4EN` writer - USART 4 clock enable"]
pub use TIM2EN_W as UART4EN_W;
#[doc = "Field `UART5EN` writer - USART 5 clock enable"]
pub use TIM2EN_W as UART5EN_W;
#[doc = "Field `I2C1EN` writer - I2C 1 clock enable"]
pub use TIM2EN_W as I2C1EN_W;
#[doc = "Field `I2C2EN` writer - I2C 2 clock enable"]
pub use TIM2EN_W as I2C2EN_W;
#[doc = "Field `USBEN` writer - USB clock enable"]
pub use TIM2EN_W as USBEN_W;
#[doc = "Field `CANEN` writer - CAN clock enable"]
pub use TIM2EN_W as CANEN_W;
#[doc = "Field `DAC2EN` writer - DAC2 interface clock enable"]
pub use TIM2EN_W as DAC2EN_W;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub use TIM2EN_W as PWREN_W;
#[doc = "Field `DAC1EN` writer - DAC interface clock enable"]
pub use TIM2EN_W as DAC1EN_W;
#[doc = "Field `I2C3EN` writer - I2C3 clock enable"]
pub use TIM2EN_W as I2C3EN_W;
impl R {
    #[doc = "Bit 0 - Timer 2 clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 4 clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 15 - SPI 3 clock enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART 4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART 5 clock enable"]
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
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
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN clock enable"]
    #[inline(always)]
    pub fn canen(&self) -> CANEN_R {
        CANEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DAC2 interface clock enable"]
    #[inline(always)]
    pub fn dac2en(&self) -> DAC2EN_R {
        DAC2EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<APB1ENRrs> {
        TIM2EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<APB1ENRrs> {
        TIM3EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<APB1ENRrs> {
        TIM4EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Timer 6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<APB1ENRrs> {
        TIM6EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer 7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<APB1ENRrs> {
        TIM7EN_W::new(self, 5)
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
    #[doc = "Bit 15 - SPI 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<APB1ENRrs> {
        SPI3EN_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APB1ENRrs> {
        USART2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART 3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<APB1ENRrs> {
        USART3EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - USART 4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<APB1ENRrs> {
        UART4EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - USART 5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<APB1ENRrs> {
        UART5EN_W::new(self, 20)
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
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<APB1ENRrs> {
        USBEN_W::new(self, 23)
    }
    #[doc = "Bit 25 - CAN clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn canen(&mut self) -> CANEN_W<APB1ENRrs> {
        CANEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - DAC2 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac2en(&mut self) -> DAC2EN_W<APB1ENRrs> {
        DAC2EN_W::new(self, 26)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<APB1ENRrs> {
        PWREN_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac1en(&mut self) -> DAC1EN_W<APB1ENRrs> {
        DAC1EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - I2C3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<APB1ENRrs> {
        I2C3EN_W::new(self, 30)
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
