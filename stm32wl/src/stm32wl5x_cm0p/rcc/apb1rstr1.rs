#[doc = "Register `APB1RSTR1` reader"]
pub type R = crate::R<APB1RSTR1rs>;
#[doc = "Register `APB1RSTR1` writer"]
pub type W = crate::W<APB1RSTR1rs>;
#[doc = "TIM2 timer reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST {
    #[doc = "0: No effect"]
    NoReset = 0,
    #[doc = "1: Reset peripheral"]
    Reset = 1,
}
impl From<TIM2RST> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2RST` reader - TIM2 timer reset"]
pub type TIM2RST_R = crate::BitReader<TIM2RST>;
impl TIM2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM2RST {
        match self.bits {
            false => TIM2RST::NoReset,
            true => TIM2RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == TIM2RST::NoReset
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RST::Reset
    }
}
#[doc = "Field `TIM2RST` writer - TIM2 timer reset"]
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM2RST>;
impl<'a, REG> TIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST::NoReset)
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST::Reset)
    }
}
#[doc = "Field `SPI2S2RST` reader - SPI2S2 reset"]
pub use TIM2RST_R as SPI2S2RST_R;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub use TIM2RST_R as USART2RST_R;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub use TIM2RST_R as I2C1RST_R;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub use TIM2RST_R as I2C2RST_R;
#[doc = "Field `I2C3RST` reader - I2C3 reset"]
pub use TIM2RST_R as I2C3RST_R;
#[doc = "Field `DACRST` reader - DAC1 reset"]
pub use TIM2RST_R as DACRST_R;
#[doc = "Field `LPTIM1RST` reader - Low Power Timer 1 reset"]
pub use TIM2RST_R as LPTIM1RST_R;
#[doc = "Field `SPI2S2RST` writer - SPI2S2 reset"]
pub use TIM2RST_W as SPI2S2RST_W;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub use TIM2RST_W as USART2RST_W;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub use TIM2RST_W as I2C1RST_W;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub use TIM2RST_W as I2C2RST_W;
#[doc = "Field `I2C3RST` writer - I2C3 reset"]
pub use TIM2RST_W as I2C3RST_W;
#[doc = "Field `DACRST` writer - DAC1 reset"]
pub use TIM2RST_W as DACRST_W;
#[doc = "Field `LPTIM1RST` writer - Low Power Timer 1 reset"]
pub use TIM2RST_W as LPTIM1RST_W;
impl R {
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2S2 reset"]
    #[inline(always)]
    pub fn spi2s2rst(&self) -> SPI2S2RST_R {
        SPI2S2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 reset"]
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<APB1RSTR1rs> {
        TIM2RST_W::new(self, 0)
    }
    #[doc = "Bit 14 - SPI2S2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2s2rst(&mut self) -> SPI2S2RST_W<APB1RSTR1rs> {
        SPI2S2RST_W::new(self, 14)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<APB1RSTR1rs> {
        USART2RST_W::new(self, 17)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APB1RSTR1rs> {
        I2C1RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<APB1RSTR1rs> {
        I2C2RST_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<APB1RSTR1rs> {
        I2C3RST_W::new(self, 23)
    }
    #[doc = "Bit 29 - DAC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dacrst(&mut self) -> DACRST_W<APB1RSTR1rs> {
        DACRST_W::new(self, 29)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<APB1RSTR1rs> {
        LPTIM1RST_W::new(self, 31)
    }
}
#[doc = "APB1 peripheral reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1RSTR1rs;
impl crate::RegisterSpec for APB1RSTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr1::R`](R) reader structure"]
impl crate::Readable for APB1RSTR1rs {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr1::W`](W) writer structure"]
impl crate::Writable for APB1RSTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1RSTR1 to value 0"]
impl crate::Resettable for APB1RSTR1rs {
    const RESET_VALUE: u32 = 0;
}
