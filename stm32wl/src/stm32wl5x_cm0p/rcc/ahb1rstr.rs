#[doc = "Register `AHB1RSTR` reader"]
pub type R = crate::R<AHB1RSTRrs>;
#[doc = "Register `AHB1RSTR` writer"]
pub type W = crate::W<AHB1RSTRrs>;
#[doc = "DMA1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RST {
    #[doc = "0: No effect"]
    NoReset = 0,
    #[doc = "1: Reset peripheral"]
    Reset = 1,
}
impl From<DMA1RST> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub type DMA1RST_R = crate::BitReader<DMA1RST>;
impl DMA1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1RST {
        match self.bits {
            false => DMA1RST::NoReset,
            true => DMA1RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == DMA1RST::NoReset
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA1RST::Reset
    }
}
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG, DMA1RST>;
impl<'a, REG> DMA1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RST::NoReset)
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RST::Reset)
    }
}
#[doc = "Field `DMA2RST` reader - DMA2 reset"]
pub use DMA1RST_R as DMA2RST_R;
#[doc = "Field `DMAMUX1RST` reader - DMAMUX1 reset"]
pub use DMA1RST_R as DMAMUX1RST_R;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub use DMA1RST_R as CRCRST_R;
#[doc = "Field `DMA2RST` writer - DMA2 reset"]
pub use DMA1RST_W as DMA2RST_W;
#[doc = "Field `DMAMUX1RST` writer - DMAMUX1 reset"]
pub use DMA1RST_W as DMAMUX1RST_W;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub use DMA1RST_W as CRCRST_W;
impl R {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1 reset"]
    #[inline(always)]
    pub fn dmamux1rst(&self) -> DMAMUX1RST_R {
        DMAMUX1RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<AHB1RSTRrs> {
        DMA1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<AHB1RSTRrs> {
        DMA2RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAMUX1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1rst(&mut self) -> DMAMUX1RST_W<AHB1RSTRrs> {
        DMAMUX1RST_W::new(self, 2)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<AHB1RSTRrs> {
        CRCRST_W::new(self, 12)
    }
}
#[doc = "AHB1 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
