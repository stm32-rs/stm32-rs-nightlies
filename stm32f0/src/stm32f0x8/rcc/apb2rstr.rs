#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<APB2RSTRrs>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<APB2RSTRrs>;
#[doc = "SYSCFG and COMP reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<SYSCFGRST> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGRST` reader - SYSCFG and COMP reset"]
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST>;
impl SYSCFGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCFGRST> {
        match self.bits {
            true => Some(SYSCFGRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRST::Reset
    }
}
#[doc = "Field `SYSCFGRST` writer - SYSCFG and COMP reset"]
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGRST>;
impl<'a, REG> SYSCFGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRST::Reset)
    }
}
#[doc = "Field `USART6RST` reader - USART6 reset"]
pub use SYSCFGRST_R as USART6RST_R;
#[doc = "Field `USART7RST` reader - USART7 reset"]
pub use SYSCFGRST_R as USART7RST_R;
#[doc = "Field `USART8RST` reader - USART8 reset"]
pub use SYSCFGRST_R as USART8RST_R;
#[doc = "Field `ADCRST` reader - ADC interface reset"]
pub use SYSCFGRST_R as ADCRST_R;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub use SYSCFGRST_R as TIM1RST_R;
#[doc = "Field `SPI1RST` reader - SPI 1 reset"]
pub use SYSCFGRST_R as SPI1RST_R;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub use SYSCFGRST_R as USART1RST_R;
#[doc = "Field `TIM15RST` reader - TIM15 timer reset"]
pub use SYSCFGRST_R as TIM15RST_R;
#[doc = "Field `TIM16RST` reader - TIM16 timer reset"]
pub use SYSCFGRST_R as TIM16RST_R;
#[doc = "Field `TIM17RST` reader - TIM17 timer reset"]
pub use SYSCFGRST_R as TIM17RST_R;
#[doc = "Field `DBGMCURST` reader - Debug MCU reset"]
pub use SYSCFGRST_R as DBGMCURST_R;
#[doc = "Field `USART6RST` writer - USART6 reset"]
pub use SYSCFGRST_W as USART6RST_W;
#[doc = "Field `USART7RST` writer - USART7 reset"]
pub use SYSCFGRST_W as USART7RST_W;
#[doc = "Field `USART8RST` writer - USART8 reset"]
pub use SYSCFGRST_W as USART8RST_W;
#[doc = "Field `ADCRST` writer - ADC interface reset"]
pub use SYSCFGRST_W as ADCRST_W;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub use SYSCFGRST_W as TIM1RST_W;
#[doc = "Field `SPI1RST` writer - SPI 1 reset"]
pub use SYSCFGRST_W as SPI1RST_W;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub use SYSCFGRST_W as USART1RST_W;
#[doc = "Field `TIM15RST` writer - TIM15 timer reset"]
pub use SYSCFGRST_W as TIM15RST_W;
#[doc = "Field `TIM16RST` writer - TIM16 timer reset"]
pub use SYSCFGRST_W as TIM16RST_W;
#[doc = "Field `TIM17RST` writer - TIM17 timer reset"]
pub use SYSCFGRST_W as TIM17RST_W;
#[doc = "Field `DBGMCURST` writer - Debug MCU reset"]
pub use SYSCFGRST_W as DBGMCURST_W;
impl R {
    #[doc = "Bit 0 - SYSCFG and COMP reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USART7 reset"]
    #[inline(always)]
    pub fn usart7rst(&self) -> USART7RST_R {
        USART7RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USART8 reset"]
    #[inline(always)]
    pub fn usart8rst(&self) -> USART8RST_R {
        USART8RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC interface reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - Debug MCU reset"]
    #[inline(always)]
    pub fn dbgmcurst(&self) -> DBGMCURST_R {
        DBGMCURST_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG and COMP reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APB2RSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> USART6RST_W<APB2RSTRrs> {
        USART6RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - USART7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart7rst(&mut self) -> USART7RST_W<APB2RSTRrs> {
        USART7RST_W::new(self, 6)
    }
    #[doc = "Bit 7 - USART8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart8rst(&mut self) -> USART8RST_W<APB2RSTRrs> {
        USART8RST_W::new(self, 7)
    }
    #[doc = "Bit 9 - ADC interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<APB2RSTRrs> {
        ADCRST_W::new(self, 9)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APB2RSTRrs> {
        TIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<APB2RSTRrs> {
        TIM15RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<APB2RSTRrs> {
        TIM17RST_W::new(self, 18)
    }
    #[doc = "Bit 22 - Debug MCU reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbgmcurst(&mut self) -> DBGMCURST_W<APB2RSTRrs> {
        DBGMCURST_W::new(self, 22)
    }
}
#[doc = "APB2 peripheral reset register (RCC_APB2RSTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RSTRrs;
impl crate::RegisterSpec for APB2RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for APB2RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for APB2RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTRrs {
    const RESET_VALUE: u32 = 0;
}
