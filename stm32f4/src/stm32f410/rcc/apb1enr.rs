#[doc = "Register `APB1ENR` reader"]
pub type R = crate::R<APB1ENRrs>;
#[doc = "Register `APB1ENR` writer"]
pub type W = crate::W<APB1ENRrs>;
#[doc = "TIM5 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM5EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<TIM5EN> for bool {
    #[inline(always)]
    fn from(variant: TIM5EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM5EN` reader - TIM5 clock enable"]
pub type TIM5EN_R = crate::BitReader<TIM5EN>;
impl TIM5EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM5EN {
        match self.bits {
            false => TIM5EN::Disabled,
            true => TIM5EN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM5EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM5EN::Enabled
    }
}
#[doc = "Field `TIM5EN` writer - TIM5 clock enable"]
pub type TIM5EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM5EN>;
impl<'a, REG> TIM5EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM5EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM5EN::Enabled)
    }
}
#[doc = "Field `TIM6EN` reader - TIM6 clock enable"]
pub use TIM5EN_R as TIM6EN_R;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 clock enable"]
pub use TIM5EN_R as LPTIM1EN_R;
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable"]
pub use TIM5EN_R as RTCAPBEN_R;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable"]
pub use TIM5EN_R as WWDGEN_R;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub use TIM5EN_R as SPI2EN_R;
#[doc = "Field `USART2EN` reader - USART 2 clock enable"]
pub use TIM5EN_R as USART2EN_R;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub use TIM5EN_R as I2C1EN_R;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
pub use TIM5EN_R as I2C2EN_R;
#[doc = "Field `FMPI2C1EN` reader - FMPI2C1 clock enable"]
pub use TIM5EN_R as FMPI2C1EN_R;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub use TIM5EN_R as PWREN_R;
#[doc = "Field `DACEN` reader - DAC interface clock enable"]
pub use TIM5EN_R as DACEN_R;
#[doc = "Field `TIM6EN` writer - TIM6 clock enable"]
pub use TIM5EN_W as TIM6EN_W;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 clock enable"]
pub use TIM5EN_W as LPTIM1EN_W;
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable"]
pub use TIM5EN_W as RTCAPBEN_W;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable"]
pub use TIM5EN_W as WWDGEN_W;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub use TIM5EN_W as SPI2EN_W;
#[doc = "Field `USART2EN` writer - USART 2 clock enable"]
pub use TIM5EN_W as USART2EN_W;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub use TIM5EN_W as I2C1EN_W;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
pub use TIM5EN_W as I2C2EN_W;
#[doc = "Field `FMPI2C1EN` writer - FMPI2C1 clock enable"]
pub use TIM5EN_W as FMPI2C1EN_W;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub use TIM5EN_W as PWREN_W;
#[doc = "Field `DACEN` writer - DAC interface clock enable"]
pub use TIM5EN_W as DACEN_W;
impl R {
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
    #[doc = "Bit 9 - LPTIM1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
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
    #[doc = "Bit 24 - FMPI2C1 clock enable"]
    #[inline(always)]
    pub fn fmpi2c1en(&self) -> FMPI2C1EN_R {
        FMPI2C1EN_R::new(((self.bits >> 24) & 1) != 0)
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
}
impl W {
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
    #[doc = "Bit 9 - LPTIM1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<APB1ENRrs> {
        LPTIM1EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APB1ENRrs> {
        RTCAPBEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APB1ENRrs> {
        WWDGEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
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
    #[doc = "Bit 24 - FMPI2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpi2c1en(&mut self) -> FMPI2C1EN_W<APB1ENRrs> {
        FMPI2C1EN_W::new(self, 24)
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
