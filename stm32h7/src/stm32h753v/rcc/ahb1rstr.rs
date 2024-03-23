#[doc = "Register `AHB1RSTR` reader"]
pub type R = crate::R<AHB1RSTRrs>;
#[doc = "Register `AHB1RSTR` writer"]
pub type W = crate::W<AHB1RSTRrs>;
#[doc = "DMA1 block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<DMA1RST> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1RST` reader - DMA1 block reset"]
pub type DMA1RST_R = crate::BitReader<DMA1RST>;
impl DMA1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMA1RST> {
        match self.bits {
            true => Some(DMA1RST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA1RST::Reset
    }
}
#[doc = "Field `DMA1RST` writer - DMA1 block reset"]
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, DMA1RST>;
impl<'a, REG> DMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RST::Reset)
    }
}
#[doc = "Field `DMA2RST` reader - DMA2 block reset"]
pub use DMA1RST_R as DMA2RST_R;
#[doc = "Field `ADC12RST` reader - ADC1&amp;2 block reset"]
pub use DMA1RST_R as ADC12RST_R;
#[doc = "Field `ETH1MACRST` reader - ETH1MAC block reset"]
pub use DMA1RST_R as ETH1MACRST_R;
#[doc = "Field `USB1OTGRST` reader - USB1OTG block reset"]
pub use DMA1RST_R as USB1OTGRST_R;
#[doc = "Field `USB2OTGRST` reader - USB2OTG block reset"]
pub use DMA1RST_R as USB2OTGRST_R;
#[doc = "Field `DMA2RST` writer - DMA2 block reset"]
pub use DMA1RST_W as DMA2RST_W;
#[doc = "Field `ADC12RST` writer - ADC1&amp;2 block reset"]
pub use DMA1RST_W as ADC12RST_W;
#[doc = "Field `ETH1MACRST` writer - ETH1MAC block reset"]
pub use DMA1RST_W as ETH1MACRST_W;
#[doc = "Field `USB1OTGRST` writer - USB1OTG block reset"]
pub use DMA1RST_W as USB1OTGRST_W;
#[doc = "Field `USB2OTGRST` writer - USB2OTG block reset"]
pub use DMA1RST_W as USB2OTGRST_W;
impl R {
    #[doc = "Bit 0 - DMA1 block reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 block reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1&amp;2 block reset"]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - ETH1MAC block reset"]
    #[inline(always)]
    pub fn eth1macrst(&self) -> ETH1MACRST_R {
        ETH1MACRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 25 - USB1OTG block reset"]
    #[inline(always)]
    pub fn usb1otgrst(&self) -> USB1OTGRST_R {
        USB1OTGRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - USB2OTG block reset"]
    #[inline(always)]
    pub fn usb2otgrst(&self) -> USB2OTGRST_R {
        USB2OTGRST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<AHB1RSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<AHB1RSTRrs> {
        DMA2RST_W::new(self, 1)
    }
    #[doc = "Bit 5 - ADC1&amp;2 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<AHB1RSTRrs> {
        ADC12RST_W::new(self, 5)
    }
    #[doc = "Bit 15 - ETH1MAC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn eth1macrst(&mut self) -> ETH1MACRST_W<AHB1RSTRrs> {
        ETH1MACRST_W::new(self, 15)
    }
    #[doc = "Bit 25 - USB1OTG block reset"]
    #[inline(always)]
    #[must_use]
    pub fn usb1otgrst(&mut self) -> USB1OTGRST_W<AHB1RSTRrs> {
        USB1OTGRST_W::new(self, 25)
    }
    #[doc = "Bit 27 - USB2OTG block reset"]
    #[inline(always)]
    #[must_use]
    pub fn usb2otgrst(&mut self) -> USB2OTGRST_W<AHB1RSTRrs> {
        USB2OTGRST_W::new(self, 27)
    }
}
#[doc = "RCC AHB1 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1RSTRrs;
impl crate::RegisterSpec for AHB1RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rstr::R`](R) reader structure"]
impl crate::Readable for AHB1RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure"]
impl crate::Writable for AHB1RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for AHB1RSTRrs {
    const RESET_VALUE: u32 = 0;
}
