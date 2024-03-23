#[doc = "Register `APB1LLPENR` reader"]
pub type R = crate::R<APB1LLPENRrs>;
#[doc = "Register `APB1LLPENR` writer"]
pub type W = crate::W<APB1LLPENRrs>;
#[doc = "TIM2 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2LPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<TIM2LPEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2LPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2LPEN` reader - TIM2 clock enable during sleep mode Set and reset by software."]
pub type TIM2LPEN_R = crate::BitReader<TIM2LPEN>;
impl TIM2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM2LPEN {
        match self.bits {
            false => TIM2LPEN::Disabled,
            true => TIM2LPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2LPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2LPEN::Enabled
    }
}
#[doc = "Field `TIM2LPEN` writer - TIM2 clock enable during sleep mode Set and reset by software."]
pub type TIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2LPEN>;
impl<'a, REG> TIM2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN::Enabled)
    }
}
#[doc = "Field `TIM3LPEN` reader - TIM3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as TIM3LPEN_R;
#[doc = "Field `TIM6LPEN` reader - TIM6 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as TIM6LPEN_R;
#[doc = "Field `TIM7LPEN` reader - TIM7 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as TIM7LPEN_R;
#[doc = "Field `WWDGLPEN` reader - WWDG clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as WWDGLPEN_R;
#[doc = "Field `OPAMPLPEN` reader - OPAMP clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as OPAMPLPEN_R;
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as SPI2LPEN_R;
#[doc = "Field `SPI3LPEN` reader - SPI3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as SPI3LPEN_R;
#[doc = "Field `COMPLPEN` reader - COMP clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as COMPLPEN_R;
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as USART2LPEN_R;
#[doc = "Field `USART3LPEN` reader - USART3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as USART3LPEN_R;
#[doc = "Field `I2C1LPEN` reader - I2C1 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as I2C1LPEN_R;
#[doc = "Field `I2C2LPEN` reader - I2C2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as I2C2LPEN_R;
#[doc = "Field `I3C1LPEN` reader - I3C1 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as I3C1LPEN_R;
#[doc = "Field `CRSLPEN` reader - CRS clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_R as CRSLPEN_R;
#[doc = "Field `TIM3LPEN` writer - TIM3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as TIM3LPEN_W;
#[doc = "Field `TIM6LPEN` writer - TIM6 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as TIM6LPEN_W;
#[doc = "Field `TIM7LPEN` writer - TIM7 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as TIM7LPEN_W;
#[doc = "Field `WWDGLPEN` writer - WWDG clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as WWDGLPEN_W;
#[doc = "Field `OPAMPLPEN` writer - OPAMP clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as OPAMPLPEN_W;
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as SPI2LPEN_W;
#[doc = "Field `SPI3LPEN` writer - SPI3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as SPI3LPEN_W;
#[doc = "Field `COMPLPEN` writer - COMP clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as COMPLPEN_W;
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as USART2LPEN_W;
#[doc = "Field `USART3LPEN` writer - USART3 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as USART3LPEN_W;
#[doc = "Field `I2C1LPEN` writer - I2C1 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as I2C1LPEN_W;
#[doc = "Field `I2C2LPEN` writer - I2C2 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as I2C2LPEN_W;
#[doc = "Field `I3C1LPEN` writer - I3C1 clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as I3C1LPEN_W;
#[doc = "Field `CRSLPEN` writer - CRS clock enable during sleep mode Set and reset by software."]
pub use TIM2LPEN_W as CRSLPEN_W;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - OPAMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn opamplpen(&self) -> OPAMPLPEN_R {
        OPAMPLPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - COMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn complpen(&self) -> COMPLPEN_R {
        COMPLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I3C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i3c1lpen(&self) -> I3C1LPEN_R {
        I3C1LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn crslpen(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<APB1LLPENRrs> {
        TIM2LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<APB1LLPENRrs> {
        TIM3LPEN_W::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<APB1LLPENRrs> {
        TIM6LPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<APB1LLPENRrs> {
        TIM7LPEN_W::new(self, 5)
    }
    #[doc = "Bit 11 - WWDG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<APB1LLPENRrs> {
        WWDGLPEN_W::new(self, 11)
    }
    #[doc = "Bit 13 - OPAMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn opamplpen(&mut self) -> OPAMPLPEN_W<APB1LLPENRrs> {
        OPAMPLPEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<APB1LLPENRrs> {
        SPI2LPEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<APB1LLPENRrs> {
        SPI3LPEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - COMP clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn complpen(&mut self) -> COMPLPEN_W<APB1LLPENRrs> {
        COMPLPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<APB1LLPENRrs> {
        USART2LPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<APB1LLPENRrs> {
        USART3LPEN_W::new(self, 18)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<APB1LLPENRrs> {
        I2C1LPEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<APB1LLPENRrs> {
        I2C2LPEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I3C1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c1lpen(&mut self) -> I3C1LPEN_W<APB1LLPENRrs> {
        I3C1LPEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - CRS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crslpen(&mut self) -> CRSLPEN_W<APB1LLPENRrs> {
        CRSLPEN_W::new(self, 24)
    }
}
#[doc = "RCC APB1 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1llpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1llpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LLPENRrs;
impl crate::RegisterSpec for APB1LLPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1llpenr::R`](R) reader structure"]
impl crate::Readable for APB1LLPENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1llpenr::W`](W) writer structure"]
impl crate::Writable for APB1LLPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1LLPENR to value 0xdffe_c9ff"]
impl crate::Resettable for APB1LLPENRrs {
    const RESET_VALUE: u32 = 0xdffe_c9ff;
}
