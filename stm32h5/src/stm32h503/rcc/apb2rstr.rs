#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<APB2RSTRrs>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<APB2RSTRrs>;
#[doc = "TIM1 block reset Set and reset by software.\n\nValue on reset: 0"]
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
#[doc = "Field `TIM1RST` reader - TIM1 block reset Set and reset by software."]
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
#[doc = "Field `TIM1RST` writer - TIM1 block reset Set and reset by software."]
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
#[doc = "Field `SPI1RST` reader - SPI1 block reset Set and reset by software."]
pub use TIM1RST_R as SPI1RST_R;
#[doc = "Field `USART1RST` reader - USART1 block reset Set and reset by software."]
pub use TIM1RST_R as USART1RST_R;
#[doc = "Field `USBRST` reader - USB block reset"]
pub use TIM1RST_R as USBRST_R;
#[doc = "Field `SPI1RST` writer - SPI1 block reset Set and reset by software."]
pub use TIM1RST_W as SPI1RST_W;
#[doc = "Field `USART1RST` writer - USART1 block reset Set and reset by software."]
pub use TIM1RST_W as USART1RST_W;
#[doc = "Field `USBRST` writer - USB block reset"]
pub use TIM1RST_W as USBRST_W;
impl R {
    #[doc = "Bit 11 - TIM1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 24 - USB block reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APB2RSTRrs> {
        TIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    #[doc = "Bit 24 - USB block reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<APB2RSTRrs> {
        USBRST_W::new(self, 24)
    }
}
#[doc = "RCC APB2 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
