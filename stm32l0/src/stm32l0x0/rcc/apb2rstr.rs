#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<APB2RSTRrs>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<APB2RSTRrs>;
#[doc = "System configuration controller reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGRSTW {
    #[doc = "1: Reset the module"]
    Reset = 1,
}
impl From<SYSCFGRSTW> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRSTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGRST` reader - System configuration controller reset"]
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRSTW>;
impl SYSCFGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCFGRSTW> {
        match self.bits {
            true => Some(SYSCFGRSTW::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRSTW::Reset
    }
}
#[doc = "Field `SYSCFGRST` writer - System configuration controller reset"]
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGRSTW>;
impl<'a, REG> SYSCFGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGRSTW::Reset)
    }
}
#[doc = "Field `TIM21RST` reader - TIM21 timer reset"]
pub use SYSCFGRST_R as TIM21RST_R;
#[doc = "Field `TIM22RST` reader - TIM22 timer reset"]
pub use SYSCFGRST_R as TIM22RST_R;
#[doc = "Field `ADCRST` reader - ADC interface reset"]
pub use SYSCFGRST_R as ADCRST_R;
#[doc = "Field `SPI1RST` reader - SPI 1 reset"]
pub use SYSCFGRST_R as SPI1RST_R;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub use SYSCFGRST_R as USART1RST_R;
#[doc = "Field `DBGRST` reader - DBG reset"]
pub use SYSCFGRST_R as DBGRST_R;
#[doc = "Field `TIM21RST` writer - TIM21 timer reset"]
pub use SYSCFGRST_W as TIM21RST_W;
#[doc = "Field `TIM22RST` writer - TIM22 timer reset"]
pub use SYSCFGRST_W as TIM22RST_W;
#[doc = "Field `ADCRST` writer - ADC interface reset"]
pub use SYSCFGRST_W as ADCRST_W;
#[doc = "Field `SPI1RST` writer - SPI 1 reset"]
pub use SYSCFGRST_W as SPI1RST_W;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub use SYSCFGRST_W as USART1RST_W;
#[doc = "Field `DBGRST` writer - DBG reset"]
pub use SYSCFGRST_W as DBGRST_W;
impl R {
    #[doc = "Bit 0 - System configuration controller reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TIM21 timer reset"]
    #[inline(always)]
    pub fn tim21rst(&self) -> TIM21RST_R {
        TIM21RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM22 timer reset"]
    #[inline(always)]
    pub fn tim22rst(&self) -> TIM22RST_R {
        TIM22RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC interface reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 22 - DBG reset"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APB2RSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    #[doc = "Bit 2 - TIM21 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim21rst(&mut self) -> TIM21RST_W<APB2RSTRrs> {
        TIM21RST_W::new(self, 2)
    }
    #[doc = "Bit 5 - TIM22 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim22rst(&mut self) -> TIM22RST_W<APB2RSTRrs> {
        TIM22RST_W::new(self, 5)
    }
    #[doc = "Bit 9 - ADC interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<APB2RSTRrs> {
        ADCRST_W::new(self, 9)
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
    #[doc = "Bit 22 - DBG reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrst(&mut self) -> DBGRST_W<APB2RSTRrs> {
        DBGRST_W::new(self, 22)
    }
}
#[doc = "APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
