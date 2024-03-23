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
#[doc = "Field `TIM8RST` reader - TIM8RST"]
pub use TIM1RST_R as TIM8RST_R;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub use TIM1RST_R as USART1RST_R;
#[doc = "Field `USART6RST` reader - USART6 reset"]
pub use TIM1RST_R as USART6RST_R;
#[doc = "Field `UART9RST` reader - UART9 reset"]
pub use TIM1RST_R as UART9RST_R;
#[doc = "Field `UART10RST` reader - UART10 reset"]
pub use TIM1RST_R as UART10RST_R;
#[doc = "Field `ADCRST` reader - ADC interface reset (common to all ADCs)"]
pub use TIM1RST_R as ADCRST_R;
#[doc = "Field `SDIORST` reader - SDIO reset"]
pub use TIM1RST_R as SDIORST_R;
#[doc = "Field `SPI1RST` reader - SPI 1 reset"]
pub use TIM1RST_R as SPI1RST_R;
#[doc = "Field `SPI4RST` reader - SPI4 reset"]
pub use TIM1RST_R as SPI4RST_R;
#[doc = "Field `SYSCFGRST` reader - System configuration controller reset"]
pub use TIM1RST_R as SYSCFGRST_R;
#[doc = "Field `TIM9RST` reader - TIM9 reset"]
pub use TIM1RST_R as TIM9RST_R;
#[doc = "Field `TIM10RST` reader - TIM10 reset"]
pub use TIM1RST_R as TIM10RST_R;
#[doc = "Field `TIM11RST` reader - TIM11 reset"]
pub use TIM1RST_R as TIM11RST_R;
#[doc = "Field `SPI5RST` reader - SPI5RST"]
pub use TIM1RST_R as SPI5RST_R;
#[doc = "Field `SAI1RST` reader - SAI1 reset"]
pub use TIM1RST_R as SAI1RST_R;
#[doc = "Field `DFSDMRST` reader - DFSDMRST"]
pub use TIM1RST_R as DFSDMRST_R;
#[doc = "Field `DFSDM2RST` reader - DFSDM2 reset"]
pub use TIM1RST_R as DFSDM2RST_R;
#[doc = "Field `TIM8RST` writer - TIM8RST"]
pub use TIM1RST_W as TIM8RST_W;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub use TIM1RST_W as USART1RST_W;
#[doc = "Field `USART6RST` writer - USART6 reset"]
pub use TIM1RST_W as USART6RST_W;
#[doc = "Field `UART9RST` writer - UART9 reset"]
pub use TIM1RST_W as UART9RST_W;
#[doc = "Field `UART10RST` writer - UART10 reset"]
pub use TIM1RST_W as UART10RST_W;
#[doc = "Field `ADCRST` writer - ADC interface reset (common to all ADCs)"]
pub use TIM1RST_W as ADCRST_W;
#[doc = "Field `SDIORST` writer - SDIO reset"]
pub use TIM1RST_W as SDIORST_W;
#[doc = "Field `SPI1RST` writer - SPI 1 reset"]
pub use TIM1RST_W as SPI1RST_W;
#[doc = "Field `SPI4RST` writer - SPI4 reset"]
pub use TIM1RST_W as SPI4RST_W;
#[doc = "Field `SYSCFGRST` writer - System configuration controller reset"]
pub use TIM1RST_W as SYSCFGRST_W;
#[doc = "Field `TIM9RST` writer - TIM9 reset"]
pub use TIM1RST_W as TIM9RST_W;
#[doc = "Field `TIM10RST` writer - TIM10 reset"]
pub use TIM1RST_W as TIM10RST_W;
#[doc = "Field `TIM11RST` writer - TIM11 reset"]
pub use TIM1RST_W as TIM11RST_W;
#[doc = "Field `SPI5RST` writer - SPI5RST"]
pub use TIM1RST_W as SPI5RST_W;
#[doc = "Field `SAI1RST` writer - SAI1 reset"]
pub use TIM1RST_W as SAI1RST_W;
#[doc = "Field `DFSDMRST` writer - DFSDMRST"]
pub use TIM1RST_W as DFSDMRST_W;
#[doc = "Field `DFSDM2RST` writer - DFSDM2 reset"]
pub use TIM1RST_W as DFSDM2RST_W;
impl R {
    #[doc = "Bit 0 - TIM1 reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8RST"]
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
    #[doc = "Bit 6 - UART9 reset"]
    #[inline(always)]
    pub fn uart9rst(&self) -> UART9RST_R {
        UART9RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART10 reset"]
    #[inline(always)]
    pub fn uart10rst(&self) -> UART10RST_R {
        UART10RST_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 13) & 1) != 0)
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
    #[doc = "Bit 20 - SPI5RST"]
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 reset"]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DFSDMRST"]
    #[inline(always)]
    pub fn dfsdmrst(&self) -> DFSDMRST_R {
        DFSDMRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DFSDM2 reset"]
    #[inline(always)]
    pub fn dfsdm2rst(&self) -> DFSDM2RST_R {
        DFSDM2RST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APB2RSTRrs> {
        TIM1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8RST"]
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
    #[doc = "Bit 6 - UART9 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart9rst(&mut self) -> UART9RST_W<APB2RSTRrs> {
        UART9RST_W::new(self, 6)
    }
    #[doc = "Bit 7 - UART10 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart10rst(&mut self) -> UART10RST_W<APB2RSTRrs> {
        UART10RST_W::new(self, 7)
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
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi4rst(&mut self) -> SPI4RST_W<APB2RSTRrs> {
        SPI4RST_W::new(self, 13)
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
    #[doc = "Bit 20 - SPI5RST"]
    #[inline(always)]
    #[must_use]
    pub fn spi5rst(&mut self) -> SPI5RST_W<APB2RSTRrs> {
        SPI5RST_W::new(self, 20)
    }
    #[doc = "Bit 22 - SAI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<APB2RSTRrs> {
        SAI1RST_W::new(self, 22)
    }
    #[doc = "Bit 24 - DFSDMRST"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmrst(&mut self) -> DFSDMRST_W<APB2RSTRrs> {
        DFSDMRST_W::new(self, 24)
    }
    #[doc = "Bit 25 - DFSDM2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm2rst(&mut self) -> DFSDM2RST_W<APB2RSTRrs> {
        DFSDM2RST_W::new(self, 25)
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
