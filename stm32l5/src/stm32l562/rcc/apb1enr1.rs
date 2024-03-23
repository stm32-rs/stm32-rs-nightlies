#[doc = "Register `APB1ENR1` reader"]
pub type R = crate::R<APB1ENR1rs>;
#[doc = "Register `APB1ENR1` writer"]
pub type W = crate::W<APB1ENR1rs>;
#[doc = "TIM2 timer clock enable\n\nValue on reset: 0"]
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
#[doc = "Field `TIM2EN` reader - TIM2 timer clock enable"]
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
#[doc = "Field `TIM2EN` writer - TIM2 timer clock enable"]
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
#[doc = "Field `TIM3EN` reader - TIM3 timer clock enable"]
pub use TIM2EN_R as TIM3EN_R;
#[doc = "Field `TIM4EN` reader - TIM4 timer clock enable"]
pub use TIM2EN_R as TIM4EN_R;
#[doc = "Field `TIM5EN` reader - TIM5 timer clock enable"]
pub use TIM2EN_R as TIM5EN_R;
#[doc = "Field `TIM6EN` reader - TIM6 timer clock enable"]
pub use TIM2EN_R as TIM6EN_R;
#[doc = "Field `TIM7EN` reader - TIM7 timer clock enable"]
pub use TIM2EN_R as TIM7EN_R;
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable"]
pub use TIM2EN_R as RTCAPBEN_R;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable"]
pub use TIM2EN_R as WWDGEN_R;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub use TIM2EN_R as SPI2EN_R;
#[doc = "Field `SP3EN` reader - SPI3 clock enable"]
pub use TIM2EN_R as SP3EN_R;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
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
#[doc = "Field `CRSEN` reader - Clock Recovery System clock enable"]
pub use TIM2EN_R as CRSEN_R;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub use TIM2EN_R as PWREN_R;
#[doc = "Field `DAC1EN` reader - DAC1 interface clock enable"]
pub use TIM2EN_R as DAC1EN_R;
#[doc = "Field `OPAMPEN` reader - OPAMP interface clock enable"]
pub use TIM2EN_R as OPAMPEN_R;
#[doc = "Field `LPTIM1EN` reader - Low power timer 1 clock enable"]
pub use TIM2EN_R as LPTIM1EN_R;
#[doc = "Field `TIM3EN` writer - TIM3 timer clock enable"]
pub use TIM2EN_W as TIM3EN_W;
#[doc = "Field `TIM4EN` writer - TIM4 timer clock enable"]
pub use TIM2EN_W as TIM4EN_W;
#[doc = "Field `TIM5EN` writer - TIM5 timer clock enable"]
pub use TIM2EN_W as TIM5EN_W;
#[doc = "Field `TIM6EN` writer - TIM6 timer clock enable"]
pub use TIM2EN_W as TIM6EN_W;
#[doc = "Field `TIM7EN` writer - TIM7 timer clock enable"]
pub use TIM2EN_W as TIM7EN_W;
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable"]
pub use TIM2EN_W as RTCAPBEN_W;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable"]
pub use TIM2EN_W as WWDGEN_W;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub use TIM2EN_W as SPI2EN_W;
#[doc = "Field `SP3EN` writer - SPI3 clock enable"]
pub use TIM2EN_W as SP3EN_W;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
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
#[doc = "Field `CRSEN` writer - Clock Recovery System clock enable"]
pub use TIM2EN_W as CRSEN_W;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub use TIM2EN_W as PWREN_W;
#[doc = "Field `DAC1EN` writer - DAC1 interface clock enable"]
pub use TIM2EN_W as DAC1EN_W;
#[doc = "Field `OPAMPEN` writer - OPAMP interface clock enable"]
pub use TIM2EN_W as OPAMPEN_W;
#[doc = "Field `LPTIM1EN` writer - Low power timer 1 clock enable"]
pub use TIM2EN_W as LPTIM1EN_W;
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer clock enable"]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    pub fn sp3en(&self) -> SP3EN_R {
        SP3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
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
    #[doc = "Bit 24 - Clock Recovery System clock enable"]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable"]
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP interface clock enable"]
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<APB1ENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<APB1ENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<APB1ENR1rs> {
        TIM4EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> TIM5EN_W<APB1ENR1rs> {
        TIM5EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<APB1ENR1rs> {
        TIM6EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<APB1ENR1rs> {
        TIM7EN_W::new(self, 5)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APB1ENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APB1ENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APB1ENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sp3en(&mut self) -> SP3EN_W<APB1ENR1rs> {
        SP3EN_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APB1ENR1rs> {
        USART2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<APB1ENR1rs> {
        USART3EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<APB1ENR1rs> {
        UART4EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<APB1ENR1rs> {
        UART5EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1ENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<APB1ENR1rs> {
        I2C2EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<APB1ENR1rs> {
        I2C3EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Clock Recovery System clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<APB1ENR1rs> {
        CRSEN_W::new(self, 24)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<APB1ENR1rs> {
        PWREN_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac1en(&mut self) -> DAC1EN_W<APB1ENR1rs> {
        DAC1EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - OPAMP interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn opampen(&mut self) -> OPAMPEN_W<APB1ENR1rs> {
        OPAMPEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low power timer 1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<APB1ENR1rs> {
        LPTIM1EN_W::new(self, 31)
    }
}
#[doc = "APB1ENR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1enr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1enr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1ENR1rs;
impl crate::RegisterSpec for APB1ENR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr1::R`](R) reader structure"]
impl crate::Readable for APB1ENR1rs {}
#[doc = "`write(|w| ..)` method takes [`apb1enr1::W`](W) writer structure"]
impl crate::Writable for APB1ENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1ENR1 to value 0"]
impl crate::Resettable for APB1ENR1rs {
    const RESET_VALUE: u32 = 0;
}
