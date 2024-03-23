#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<APB2RSTRrs>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<APB2RSTRrs>;
#[doc = "TIM1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1RST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<TIM1RST> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1RST` reader - TIM1 reset"]
pub type TIM1RST_R = crate::BitReader<TIM1RST>;
impl TIM1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIM1RST> {
        match self.bits {
            true => Some(TIM1RST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM1RST::Reset
    }
}
#[doc = "Field `TIM1RST` writer - TIM1 reset"]
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM1RST>;
impl<'a, REG> TIM1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST::Reset)
    }
}
#[doc = "Field `TIM8RST` reader - TIM8 reset"]
pub use TIM1RST_R as TIM8RST_R;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub use TIM1RST_R as USART1RST_R;
#[doc = "Field `USART6RST` reader - USART6 reset"]
pub use TIM1RST_R as USART6RST_R;
#[doc = "Field `ADCRST` reader - ADC interface reset (common to all ADCs)"]
pub use TIM1RST_R as ADCRST_R;
#[doc = "Field `SDIORST` reader - SDIO reset"]
pub use TIM1RST_R as SDIORST_R;
#[doc = "Field `SPI1RST` reader - SPI 1 reset"]
pub use TIM1RST_R as SPI1RST_R;
#[doc = "Field `SYSCFGRST` reader - System configuration controller reset"]
pub use TIM1RST_R as SYSCFGRST_R;
#[doc = "Field `TIM9RST` reader - TIM9 reset"]
pub use TIM1RST_R as TIM9RST_R;
#[doc = "Field `TIM10RST` reader - TIM10 reset"]
pub use TIM1RST_R as TIM10RST_R;
#[doc = "Field `TIM11RST` reader - TIM11 reset"]
pub use TIM1RST_R as TIM11RST_R;
#[doc = "Field `TIM8RST` writer - TIM8 reset"]
pub use TIM1RST_W as TIM8RST_W;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub use TIM1RST_W as USART1RST_W;
#[doc = "Field `USART6RST` writer - USART6 reset"]
pub use TIM1RST_W as USART6RST_W;
#[doc = "Field `ADCRST` writer - ADC interface reset (common to all ADCs)"]
pub use TIM1RST_W as ADCRST_W;
#[doc = "Field `SDIORST` writer - SDIO reset"]
pub use TIM1RST_W as SDIORST_W;
#[doc = "Field `SPI1RST` writer - SPI 1 reset"]
pub use TIM1RST_W as SPI1RST_W;
#[doc = "Field `SYSCFGRST` writer - System configuration controller reset"]
pub use TIM1RST_W as SYSCFGRST_W;
#[doc = "Field `TIM9RST` writer - TIM9 reset"]
pub use TIM1RST_W as TIM9RST_W;
#[doc = "Field `TIM10RST` writer - TIM10 reset"]
pub use TIM1RST_W as TIM10RST_W;
#[doc = "Field `TIM11RST` writer - TIM11 reset"]
pub use TIM1RST_W as TIM11RST_W;
impl R {
    #[doc = "Bit 0 - TIM1 reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 reset"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC interface reset (common to all ADCs)"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO reset"]
    #[inline(always)]
    pub fn sdiorst(&self) -> SDIORST_R {
        SDIORST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - System configuration controller reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM9 reset"]
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM10 reset"]
    #[inline(always)]
    pub fn tim10rst(&self) -> TIM10RST_R {
        TIM10RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM11 reset"]
    #[inline(always)]
    pub fn tim11rst(&self) -> TIM11RST_R {
        TIM11RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APB2RSTRrs> {
        TIM1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<APB2RSTRrs> {
        TIM8RST_W::new(self, 1)
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB2RSTRrs> {
        USART1RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> USART6RST_W<APB2RSTRrs> {
        USART6RST_W::new(self, 5)
    }
    #[doc = "Bit 8 - ADC interface reset (common to all ADCs)"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<APB2RSTRrs> {
        ADCRST_W::new(self, 8)
    }
    #[doc = "Bit 11 - SDIO reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdiorst(&mut self) -> SDIORST_W<APB2RSTRrs> {
        SDIORST_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    #[doc = "Bit 14 - System configuration controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APB2RSTRrs> {
        SYSCFGRST_W::new(self, 14)
    }
    #[doc = "Bit 16 - TIM9 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim9rst(&mut self) -> TIM9RST_W<APB2RSTRrs> {
        TIM9RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM10 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim10rst(&mut self) -> TIM10RST_W<APB2RSTRrs> {
        TIM10RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM11 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim11rst(&mut self) -> TIM11RST_W<APB2RSTRrs> {
        TIM11RST_W::new(self, 18)
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
