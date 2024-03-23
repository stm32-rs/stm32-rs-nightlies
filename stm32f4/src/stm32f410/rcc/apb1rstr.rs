#[doc = "Register `APB1RSTR` reader"]
pub type R = crate::R<APB1RSTRrs>;
#[doc = "Register `APB1RSTR` writer"]
pub type W = crate::W<APB1RSTRrs>;
#[doc = "TIM5 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM5RST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<TIM5RST> for bool {
    #[inline(always)]
    fn from(variant: TIM5RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM5RST` reader - TIM5 reset"]
pub type TIM5RST_R = crate::BitReader<TIM5RST>;
impl TIM5RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIM5RST> {
        match self.bits {
            true => Some(TIM5RST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM5RST::Reset
    }
}
#[doc = "Field `TIM5RST` writer - TIM5 reset"]
pub type TIM5RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM5RST>;
impl<'a, REG> TIM5RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM5RST::Reset)
    }
}
#[doc = "Field `TIM6RST` reader - TIM6 reset"]
pub use TIM5RST_R as TIM6RST_R;
#[doc = "Field `LPTIM1RST` reader - LPTIM1 reset"]
pub use TIM5RST_R as LPTIM1RST_R;
#[doc = "Field `WWDGRST` reader - Window watchdog reset"]
pub use TIM5RST_R as WWDGRST_R;
#[doc = "Field `SPI2RST` reader - SPI 2 reset"]
pub use TIM5RST_R as SPI2RST_R;
#[doc = "Field `USART2RST` reader - USART 2 reset"]
pub use TIM5RST_R as USART2RST_R;
#[doc = "Field `I2C1RST` reader - I2C 1 reset"]
pub use TIM5RST_R as I2C1RST_R;
#[doc = "Field `I2C2RST` reader - I2C 2 reset"]
pub use TIM5RST_R as I2C2RST_R;
#[doc = "Field `FMPI2C1RST` reader - FMPI2C1 reset"]
pub use TIM5RST_R as FMPI2C1RST_R;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub use TIM5RST_R as PWRRST_R;
#[doc = "Field `DACRST` reader - DAC reset"]
pub use TIM5RST_R as DACRST_R;
#[doc = "Field `TIM6RST` writer - TIM6 reset"]
pub use TIM5RST_W as TIM6RST_W;
#[doc = "Field `LPTIM1RST` writer - LPTIM1 reset"]
pub use TIM5RST_W as LPTIM1RST_W;
#[doc = "Field `WWDGRST` writer - Window watchdog reset"]
pub use TIM5RST_W as WWDGRST_W;
#[doc = "Field `SPI2RST` writer - SPI 2 reset"]
pub use TIM5RST_W as SPI2RST_W;
#[doc = "Field `USART2RST` writer - USART 2 reset"]
pub use TIM5RST_W as USART2RST_W;
#[doc = "Field `I2C1RST` writer - I2C 1 reset"]
pub use TIM5RST_W as I2C1RST_W;
#[doc = "Field `I2C2RST` writer - I2C 2 reset"]
pub use TIM5RST_W as I2C2RST_W;
#[doc = "Field `FMPI2C1RST` writer - FMPI2C1 reset"]
pub use TIM5RST_W as FMPI2C1RST_W;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub use TIM5RST_W as PWRRST_W;
#[doc = "Field `DACRST` writer - DAC reset"]
pub use TIM5RST_W as DACRST_W;
impl R {
    #[doc = "Bit 3 - TIM5 reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI 2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - FMPI2C1 reset"]
    #[inline(always)]
    pub fn fmpi2c1rst(&self) -> FMPI2C1RST_R {
        FMPI2C1RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TIM5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> TIM5RST_W<APB1RSTRrs> {
        TIM5RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<APB1RSTRrs> {
        TIM6RST_W::new(self, 4)
    }
    #[doc = "Bit 9 - LPTIM1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<APB1RSTRrs> {
        LPTIM1RST_W::new(self, 9)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<APB1RSTRrs> {
        WWDGRST_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<APB1RSTRrs> {
        SPI2RST_W::new(self, 14)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<APB1RSTRrs> {
        USART2RST_W::new(self, 17)
    }
    #[doc = "Bit 21 - I2C 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APB1RSTRrs> {
        I2C1RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<APB1RSTRrs> {
        I2C2RST_W::new(self, 22)
    }
    #[doc = "Bit 24 - FMPI2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn fmpi2c1rst(&mut self) -> FMPI2C1RST_W<APB1RSTRrs> {
        FMPI2C1RST_W::new(self, 24)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<APB1RSTRrs> {
        PWRRST_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    #[must_use]
    pub fn dacrst(&mut self) -> DACRST_W<APB1RSTRrs> {
        DACRST_W::new(self, 29)
    }
}
#[doc = "APB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1RSTRrs;
impl crate::RegisterSpec for APB1RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr::R`](R) reader structure"]
impl crate::Readable for APB1RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr::W`](W) writer structure"]
impl crate::Writable for APB1RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1RSTR to value 0"]
impl crate::Resettable for APB1RSTRrs {
    const RESET_VALUE: u32 = 0;
}
