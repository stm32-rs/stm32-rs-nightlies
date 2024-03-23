#[doc = "Register `APB1LRSTR` reader"]
pub type R = crate::R<APB1LRSTRrs>;
#[doc = "Register `APB1LRSTR` writer"]
pub type W = crate::W<APB1LRSTRrs>;
#[doc = "TIM2 block reset Set and reset by software.\n\nValue on reset: 0"]
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
#[doc = "Field `TIM2RST` reader - TIM2 block reset Set and reset by software."]
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
#[doc = "Field `TIM2RST` writer - TIM2 block reset Set and reset by software."]
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
#[doc = "Field `TIM3RST` reader - TIM3 block reset Set and reset by software."]
pub use TIM2RST_R as TIM3RST_R;
#[doc = "Field `TIM6RST` reader - TIM6 block reset Set and reset by software."]
pub use TIM2RST_R as TIM6RST_R;
#[doc = "Field `TIM7RST` reader - TIM7 block reset Set and reset by software."]
pub use TIM2RST_R as TIM7RST_R;
#[doc = "Field `OPAMPRST` reader - OPAMP block reset Set and reset by software."]
pub use TIM2RST_R as OPAMPRST_R;
#[doc = "Field `SPI2RST` reader - SPI2 block reset Set and reset by software."]
pub use TIM2RST_R as SPI2RST_R;
#[doc = "Field `SPI3RST` reader - SPI3 block reset Set and reset by software."]
pub use TIM2RST_R as SPI3RST_R;
#[doc = "Field `COMPRST` reader - COMP block reset Set and reset by software."]
pub use TIM2RST_R as COMPRST_R;
#[doc = "Field `USART2RST` reader - USART2 block reset Set and reset by software."]
pub use TIM2RST_R as USART2RST_R;
#[doc = "Field `USART3RST` reader - USART3 block reset Set and reset by software."]
pub use TIM2RST_R as USART3RST_R;
#[doc = "Field `I2C1RST` reader - I2C1 block reset Set and reset by software."]
pub use TIM2RST_R as I2C1RST_R;
#[doc = "Field `I2C2RST` reader - I2C2 block reset Set and reset by software."]
pub use TIM2RST_R as I2C2RST_R;
#[doc = "Field `I3C1RST` reader - I3C1 block reset Set and reset by software."]
pub use TIM2RST_R as I3C1RST_R;
#[doc = "Field `CRSRST` reader - CRS block reset Set and reset by software."]
pub use TIM2RST_R as CRSRST_R;
#[doc = "Field `TIM3RST` writer - TIM3 block reset Set and reset by software."]
pub use TIM2RST_W as TIM3RST_W;
#[doc = "Field `TIM6RST` writer - TIM6 block reset Set and reset by software."]
pub use TIM2RST_W as TIM6RST_W;
#[doc = "Field `TIM7RST` writer - TIM7 block reset Set and reset by software."]
pub use TIM2RST_W as TIM7RST_W;
#[doc = "Field `OPAMPRST` writer - OPAMP block reset Set and reset by software."]
pub use TIM2RST_W as OPAMPRST_W;
#[doc = "Field `SPI2RST` writer - SPI2 block reset Set and reset by software."]
pub use TIM2RST_W as SPI2RST_W;
#[doc = "Field `SPI3RST` writer - SPI3 block reset Set and reset by software."]
pub use TIM2RST_W as SPI3RST_W;
#[doc = "Field `COMPRST` writer - COMP block reset Set and reset by software."]
pub use TIM2RST_W as COMPRST_W;
#[doc = "Field `USART2RST` writer - USART2 block reset Set and reset by software."]
pub use TIM2RST_W as USART2RST_W;
#[doc = "Field `USART3RST` writer - USART3 block reset Set and reset by software."]
pub use TIM2RST_W as USART3RST_W;
#[doc = "Field `I2C1RST` writer - I2C1 block reset Set and reset by software."]
pub use TIM2RST_W as I2C1RST_W;
#[doc = "Field `I2C2RST` writer - I2C2 block reset Set and reset by software."]
pub use TIM2RST_W as I2C2RST_W;
#[doc = "Field `I3C1RST` writer - I3C1 block reset Set and reset by software."]
pub use TIM2RST_W as I3C1RST_W;
#[doc = "Field `CRSRST` writer - CRS block reset Set and reset by software."]
pub use TIM2RST_W as CRSRST_W;
impl R {
    #[doc = "Bit 0 - TIM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 block reset Set and reset by software."]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 13 - OPAMP block reset Set and reset by software."]
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - COMP block reset Set and reset by software."]
    #[inline(always)]
    pub fn comprst(&self) -> COMPRST_R {
        COMPRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I3C1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i3c1rst(&self) -> I3C1RST_R {
        I3C1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS block reset Set and reset by software."]
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<APB1LRSTRrs> {
        TIM2RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<APB1LRSTRrs> {
        TIM3RST_W::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<APB1LRSTRrs> {
        TIM6RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<APB1LRSTRrs> {
        TIM7RST_W::new(self, 5)
    }
    #[doc = "Bit 13 - OPAMP block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn opamprst(&mut self) -> OPAMPRST_W<APB1LRSTRrs> {
        OPAMPRST_W::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<APB1LRSTRrs> {
        SPI2RST_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<APB1LRSTRrs> {
        SPI3RST_W::new(self, 15)
    }
    #[doc = "Bit 16 - COMP block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn comprst(&mut self) -> COMPRST_W<APB1LRSTRrs> {
        COMPRST_W::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<APB1LRSTRrs> {
        USART2RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<APB1LRSTRrs> {
        USART3RST_W::new(self, 18)
    }
    #[doc = "Bit 21 - I2C1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APB1LRSTRrs> {
        I2C1RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<APB1LRSTRrs> {
        I2C2RST_W::new(self, 22)
    }
    #[doc = "Bit 23 - I3C1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c1rst(&mut self) -> I3C1RST_W<APB1LRSTRrs> {
        I3C1RST_W::new(self, 23)
    }
    #[doc = "Bit 24 - CRS block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<APB1LRSTRrs> {
        CRSRST_W::new(self, 24)
    }
}
#[doc = "RCC APB1 peripheral low reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LRSTRrs;
impl crate::RegisterSpec for APB1LRSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lrstr::R`](R) reader structure"]
impl crate::Readable for APB1LRSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1lrstr::W`](W) writer structure"]
impl crate::Writable for APB1LRSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1LRSTR to value 0"]
impl crate::Resettable for APB1LRSTRrs {
    const RESET_VALUE: u32 = 0;
}
