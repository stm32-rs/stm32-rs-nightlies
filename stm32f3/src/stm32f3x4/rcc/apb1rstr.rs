#[doc = "Register `APB1RSTR` reader"]
pub type R = crate::R<APB1RSTRrs>;
#[doc = "Register `APB1RSTR` writer"]
pub type W = crate::W<APB1RSTRrs>;
#[doc = "Timer 2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<TIM2RST> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2RST` reader - Timer 2 reset"]
pub type TIM2RST_R = crate::BitReader<TIM2RST>;
impl TIM2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIM2RST> {
        match self.bits {
            true => Some(TIM2RST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RST::Reset
    }
}
#[doc = "Field `TIM2RST` writer - Timer 2 reset"]
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM2RST>;
impl<'a, REG> TIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST::Reset)
    }
}
#[doc = "Field `TIM3RST` reader - Timer 3 reset"]
pub use TIM2RST_R as TIM3RST_R;
#[doc = "Field `TIM6RST` reader - Timer 6 reset"]
pub use TIM2RST_R as TIM6RST_R;
#[doc = "Field `TIM7RST` reader - Timer 7 reset"]
pub use TIM2RST_R as TIM7RST_R;
#[doc = "Field `WWDGRST` reader - Window watchdog reset"]
pub use TIM2RST_R as WWDGRST_R;
#[doc = "Field `USART2RST` reader - USART 2 reset"]
pub use TIM2RST_R as USART2RST_R;
#[doc = "Field `USART3RST` reader - USART3 reset"]
pub use TIM2RST_R as USART3RST_R;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub use TIM2RST_R as I2C1RST_R;
#[doc = "Field `CANRST` reader - CAN reset"]
pub use TIM2RST_R as CANRST_R;
#[doc = "Field `DAC2RST` reader - DAC2 interface reset"]
pub use TIM2RST_R as DAC2RST_R;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub use TIM2RST_R as PWRRST_R;
#[doc = "Field `DAC1RST` reader - DAC interface reset"]
pub use TIM2RST_R as DAC1RST_R;
#[doc = "Field `TIM3RST` writer - Timer 3 reset"]
pub use TIM2RST_W as TIM3RST_W;
#[doc = "Field `TIM6RST` writer - Timer 6 reset"]
pub use TIM2RST_W as TIM6RST_W;
#[doc = "Field `TIM7RST` writer - Timer 7 reset"]
pub use TIM2RST_W as TIM7RST_W;
#[doc = "Field `WWDGRST` writer - Window watchdog reset"]
pub use TIM2RST_W as WWDGRST_W;
#[doc = "Field `USART2RST` writer - USART 2 reset"]
pub use TIM2RST_W as USART2RST_W;
#[doc = "Field `USART3RST` writer - USART3 reset"]
pub use TIM2RST_W as USART3RST_W;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub use TIM2RST_W as I2C1RST_W;
#[doc = "Field `CANRST` writer - CAN reset"]
pub use TIM2RST_W as CANRST_W;
#[doc = "Field `DAC2RST` writer - DAC2 interface reset"]
pub use TIM2RST_W as DAC2RST_W;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub use TIM2RST_W as PWRRST_W;
#[doc = "Field `DAC1RST` writer - DAC interface reset"]
pub use TIM2RST_W as DAC1RST_W;
impl R {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN reset"]
    #[inline(always)]
    pub fn canrst(&self) -> CANRST_R {
        CANRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DAC2 interface reset"]
    #[inline(always)]
    pub fn dac2rst(&self) -> DAC2RST_R {
        DAC2RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<APB1RSTRrs> {
        TIM2RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<APB1RSTRrs> {
        TIM3RST_W::new(self, 1)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<APB1RSTRrs> {
        TIM6RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<APB1RSTRrs> {
        TIM7RST_W::new(self, 5)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<APB1RSTRrs> {
        WWDGRST_W::new(self, 11)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<APB1RSTRrs> {
        USART2RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<APB1RSTRrs> {
        USART3RST_W::new(self, 18)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APB1RSTRrs> {
        I2C1RST_W::new(self, 21)
    }
    #[doc = "Bit 25 - CAN reset"]
    #[inline(always)]
    #[must_use]
    pub fn canrst(&mut self) -> CANRST_W<APB1RSTRrs> {
        CANRST_W::new(self, 25)
    }
    #[doc = "Bit 26 - DAC2 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn dac2rst(&mut self) -> DAC2RST_W<APB1RSTRrs> {
        DAC2RST_W::new(self, 26)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<APB1RSTRrs> {
        PWRRST_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn dac1rst(&mut self) -> DAC1RST_W<APB1RSTRrs> {
        DAC1RST_W::new(self, 29)
    }
}
#[doc = "APB1 peripheral reset register (RCC_APB1RSTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
