#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `START` reader - Start"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - Suspend"]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - Suspend"]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - Abort"]
pub type ABORT_R = crate::BitReader;
#[doc = "Field `ABORT` writer - Abort"]
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOM` reader - Line Offset Mode"]
pub type LOM_R = crate::BitReader;
#[doc = "Field `LOM` writer - Line Offset Mode"]
pub type LOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWIE` reader - Transfer watermark interrupt enable"]
pub type TWIE_R = crate::BitReader;
#[doc = "Field `TWIE` writer - Transfer watermark interrupt enable"]
pub type TWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAEIE` reader - CLUT access error interrupt enable"]
pub type CAEIE_R = crate::BitReader;
#[doc = "Field `CAEIE` writer - CLUT access error interrupt enable"]
pub type CAEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIE` reader - CLUT transfer complete interrupt enable"]
pub type CTCIE_R = crate::BitReader;
#[doc = "Field `CTCIE` writer - CLUT transfer complete interrupt enable"]
pub type CTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEIE` reader - Configuration Error Interrupt Enable"]
pub type CEIE_R = crate::BitReader;
#[doc = "Field `CEIE` writer - Configuration Error Interrupt Enable"]
pub type CEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - DMA2D mode"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - DMA2D mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Abort"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Line Offset Mode"]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable"]
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable"]
    #[inline(always)]
    pub fn caeie(&self) -> CAEIE_R {
        CAEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable"]
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18 - DMA2D mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<CRrs> {
        SUSP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Abort"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<CRrs> {
        ABORT_W::new(self, 2)
    }
    #[doc = "Bit 6 - Line Offset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lom(&mut self) -> LOM_W<CRrs> {
        LOM_W::new(self, 6)
    }
    #[doc = "Bit 8 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn twie(&mut self) -> TWIE_W<CRrs> {
        TWIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn caeie(&mut self) -> CAEIE_W<CRrs> {
        CAEIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctcie(&mut self) -> CTCIE_W<CRrs> {
        CTCIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ceie(&mut self) -> CEIE_W<CRrs> {
        CEIE_W::new(self, 13)
    }
    #[doc = "Bits 16:18 - DMA2D mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 16)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
