#[doc = "Register `APB1ENR` reader"]
pub type R = crate::R<APB1ENRrs>;
#[doc = "Register `APB1ENR` writer"]
pub type W = crate::W<APB1ENRrs>;
#[doc = "TIM2 clock enable\n\nValue on reset: 0"]
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
#[doc = "Field `TIM2EN` reader - TIM2 clock enable"]
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
#[doc = "Field `TIM2EN` writer - TIM2 clock enable"]
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
#[doc = "Field `TIM3EN` reader - TIM3 clock enable"]
pub use TIM2EN_R as TIM3EN_R;
#[doc = "Field `TIM4EN` reader - TIM4 clock enable"]
pub use TIM2EN_R as TIM4EN_R;
#[doc = "Field `TIM5EN` reader - TIM5 clock enable"]
pub use TIM2EN_R as TIM5EN_R;
#[doc = "Field `TIM6EN` reader - TIM6 clock enable"]
pub use TIM2EN_R as TIM6EN_R;
#[doc = "Field `TIM7EN` reader - TIM7 clock enable"]
pub use TIM2EN_R as TIM7EN_R;
#[doc = "Field `TIM12EN` reader - TIM12 clock enable"]
pub use TIM2EN_R as TIM12EN_R;
#[doc = "Field `TIM13EN` reader - TIM13 clock enable"]
pub use TIM2EN_R as TIM13EN_R;
#[doc = "Field `TIM14EN` reader - TIM14 clock enable"]
pub use TIM2EN_R as TIM14EN_R;
#[doc = "Field `LPTIM1EN` reader - Low power timer 1 clock enable"]
pub use TIM2EN_R as LPTIM1EN_R;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable"]
pub use TIM2EN_R as WWDGEN_R;
#[doc = "Field `CAN3EN` reader - CAN 3 enable"]
pub use TIM2EN_R as CAN3EN_R;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub use TIM2EN_R as SPI2EN_R;
#[doc = "Field `SPI3EN` reader - SPI3 clock enable"]
pub use TIM2EN_R as SPI3EN_R;
#[doc = "Field `SPDIFRXEN` reader - SPDIF-RX clock enable"]
pub use TIM2EN_R as SPDIFRXEN_R;
#[doc = "Field `USART2EN` reader - USART 2 clock enable"]
pub use TIM2EN_R as USART2EN_R;
#[doc = "Field `USART3EN` reader - USART3 clock enable"]
pub use TIM2EN_R as USART3EN_R;
#[doc = "Field `UART4EN` reader - UART4 clock enable"]
pub use TIM2EN_R as UART4EN_R;
#[doc = "Field `UART5EN` reader - UART5 clock enable"]
pub use TIM2EN_R as UART5EN_R;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub use TIM2EN_R as I2C1EN_R;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
pub use TIM2EN_R as I2C2EN_R;
#[doc = "Field `I2C3EN` reader - I2C3 clock enable"]
pub use TIM2EN_R as I2C3EN_R;
#[doc = "Field `I2C4EN` reader - I2C4 clock enable"]
pub use TIM2EN_R as I2C4EN_R;
#[doc = "Field `CAN1EN` reader - CAN 1 clock enable"]
pub use TIM2EN_R as CAN1EN_R;
#[doc = "Field `CAN2EN` reader - CAN 2 clock enable"]
pub use TIM2EN_R as CAN2EN_R;
#[doc = "Field `CECEN` reader - HDMI-CEN clock enable"]
pub use TIM2EN_R as CECEN_R;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub use TIM2EN_R as PWREN_R;
#[doc = "Field `DACEN` reader - DAC interface clock enable"]
pub use TIM2EN_R as DACEN_R;
#[doc = "Field `UART7EN` reader - UART7 clock enable"]
pub use TIM2EN_R as UART7EN_R;
#[doc = "Field `UART8EN` reader - UART8 clock enable"]
pub use TIM2EN_R as UART8EN_R;
#[doc = "Field `TIM3EN` writer - TIM3 clock enable"]
pub use TIM2EN_W as TIM3EN_W;
#[doc = "Field `TIM4EN` writer - TIM4 clock enable"]
pub use TIM2EN_W as TIM4EN_W;
#[doc = "Field `TIM5EN` writer - TIM5 clock enable"]
pub use TIM2EN_W as TIM5EN_W;
#[doc = "Field `TIM6EN` writer - TIM6 clock enable"]
pub use TIM2EN_W as TIM6EN_W;
#[doc = "Field `TIM7EN` writer - TIM7 clock enable"]
pub use TIM2EN_W as TIM7EN_W;
#[doc = "Field `TIM12EN` writer - TIM12 clock enable"]
pub use TIM2EN_W as TIM12EN_W;
#[doc = "Field `TIM13EN` writer - TIM13 clock enable"]
pub use TIM2EN_W as TIM13EN_W;
#[doc = "Field `TIM14EN` writer - TIM14 clock enable"]
pub use TIM2EN_W as TIM14EN_W;
#[doc = "Field `LPTIM1EN` writer - Low power timer 1 clock enable"]
pub use TIM2EN_W as LPTIM1EN_W;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable"]
pub use TIM2EN_W as WWDGEN_W;
#[doc = "Field `CAN3EN` writer - CAN 3 enable"]
pub use TIM2EN_W as CAN3EN_W;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub use TIM2EN_W as SPI2EN_W;
#[doc = "Field `SPI3EN` writer - SPI3 clock enable"]
pub use TIM2EN_W as SPI3EN_W;
#[doc = "Field `SPDIFRXEN` writer - SPDIF-RX clock enable"]
pub use TIM2EN_W as SPDIFRXEN_W;
#[doc = "Field `USART2EN` writer - USART 2 clock enable"]
pub use TIM2EN_W as USART2EN_W;
#[doc = "Field `USART3EN` writer - USART3 clock enable"]
pub use TIM2EN_W as USART3EN_W;
#[doc = "Field `UART4EN` writer - UART4 clock enable"]
pub use TIM2EN_W as UART4EN_W;
#[doc = "Field `UART5EN` writer - UART5 clock enable"]
pub use TIM2EN_W as UART5EN_W;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub use TIM2EN_W as I2C1EN_W;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
pub use TIM2EN_W as I2C2EN_W;
#[doc = "Field `I2C3EN` writer - I2C3 clock enable"]
pub use TIM2EN_W as I2C3EN_W;
#[doc = "Field `I2C4EN` writer - I2C4 clock enable"]
pub use TIM2EN_W as I2C4EN_W;
#[doc = "Field `CAN1EN` writer - CAN 1 clock enable"]
pub use TIM2EN_W as CAN1EN_W;
#[doc = "Field `CAN2EN` writer - CAN 2 clock enable"]
pub use TIM2EN_W as CAN2EN_W;
#[doc = "Field `CECEN` writer - HDMI-CEN clock enable"]
pub use TIM2EN_W as CECEN_W;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub use TIM2EN_W as PWREN_W;
#[doc = "Field `DACEN` writer - DAC interface clock enable"]
pub use TIM2EN_W as DACEN_W;
#[doc = "Field `UART7EN` writer - UART7 clock enable"]
pub use TIM2EN_W as UART7EN_W;
#[doc = "Field `UART8EN` writer - UART8 clock enable"]
pub use TIM2EN_W as UART8EN_W;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 clock enable"]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12 clock enable"]
    #[inline(always)]
    pub fn tim12en(&self) -> TIM12EN_R {
        TIM12EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13 clock enable"]
    #[inline(always)]
    pub fn tim13en(&self) -> TIM13EN_R {
        TIM13EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14 clock enable"]
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low power timer 1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - CAN 3 enable"]
    #[inline(always)]
    pub fn can3en(&self) -> CAN3EN_R {
        CAN3EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPDIF-RX clock enable"]
    #[inline(always)]
    pub fn spdifrxen(&self) -> SPDIFRXEN_R {
        SPDIFRXEN_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C4 clock enable"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN 1 clock enable"]
    #[inline(always)]
    pub fn can1en(&self) -> CAN1EN_R {
        CAN1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN 2 clock enable"]
    #[inline(always)]
    pub fn can2en(&self) -> CAN2EN_R {
        CAN2EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - HDMI-CEN clock enable"]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 clock enable"]
    #[inline(always)]
    pub fn uart7en(&self) -> UART7EN_R {
        UART7EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 clock enable"]
    #[inline(always)]
    pub fn uart8en(&self) -> UART8EN_R {
        UART8EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<APB1ENRrs> {
        TIM2EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<APB1ENRrs> {
        TIM3EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<APB1ENRrs> {
        TIM4EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> TIM5EN_W<APB1ENRrs> {
        TIM5EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<APB1ENRrs> {
        TIM6EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<APB1ENRrs> {
        TIM7EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - TIM12 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim12en(&mut self) -> TIM12EN_W<APB1ENRrs> {
        TIM12EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - TIM13 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim13en(&mut self) -> TIM13EN_W<APB1ENRrs> {
        TIM13EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - TIM14 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim14en(&mut self) -> TIM14EN_W<APB1ENRrs> {
        TIM14EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Low power timer 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<APB1ENRrs> {
        LPTIM1EN_W::new(self, 9)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APB1ENRrs> {
        WWDGEN_W::new(self, 11)
    }
    #[doc = "Bit 13 - CAN 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn can3en(&mut self) -> CAN3EN_W<APB1ENRrs> {
        CAN3EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APB1ENRrs> {
        SPI2EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<APB1ENRrs> {
        SPI3EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - SPDIF-RX clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spdifrxen(&mut self) -> SPDIFRXEN_W<APB1ENRrs> {
        SPDIFRXEN_W::new(self, 16)
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
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<APB1ENRrs> {
        UART4EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<APB1ENRrs> {
        UART5EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1ENRrs> {
        I2C1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<APB1ENRrs> {
        I2C2EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<APB1ENRrs> {
        I2C3EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - I2C4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2C4EN_W<APB1ENRrs> {
        I2C4EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - CAN 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can1en(&mut self) -> CAN1EN_W<APB1ENRrs> {
        CAN1EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - CAN 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can2en(&mut self) -> CAN2EN_W<APB1ENRrs> {
        CAN2EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - HDMI-CEN clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<APB1ENRrs> {
        CECEN_W::new(self, 27)
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
    pub fn dacen(&mut self) -> DACEN_W<APB1ENRrs> {
        DACEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - UART7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart7en(&mut self) -> UART7EN_W<APB1ENRrs> {
        UART7EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - UART8 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart8en(&mut self) -> UART8EN_W<APB1ENRrs> {
        UART8EN_W::new(self, 31)
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
