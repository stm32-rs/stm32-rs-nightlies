#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `RIEN` reader - Enable read interrupt"]
pub type RIEN_R = crate::BitReader;
#[doc = "Field `RIEN` writer - Enable read interrupt"]
pub type RIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIEN` reader - Enable write interrupt"]
pub type WIEN_R = crate::BitReader;
#[doc = "Field `WIEN` writer - Enable write interrupt"]
pub type WIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFLIEN` reader - Enable overflow error interrupts"]
pub type OVFLIEN_R = crate::BitReader;
#[doc = "Field `OVFLIEN` writer - Enable overflow error interrupts"]
pub type OVFLIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNFLIEN` reader - Enable underflow error interrupts"]
pub type UNFLIEN_R = crate::BitReader;
#[doc = "Field `UNFLIEN` writer - Enable underflow error interrupts"]
pub type UNFLIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SATIEN` reader - Enable saturation error interrupts"]
pub type SATIEN_R = crate::BitReader;
#[doc = "Field `SATIEN` writer - Enable saturation error interrupts"]
pub type SATIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREN` reader - Enable DMA read channel requests"]
pub type DMAREN_R = crate::BitReader;
#[doc = "Field `DMAREN` writer - Enable DMA read channel requests"]
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAWEN` reader - Enable DMA write channel requests"]
pub type DMAWEN_R = crate::BitReader;
#[doc = "Field `DMAWEN` writer - Enable DMA write channel requests"]
pub type DMAWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLIPEN` reader - Enable clipping"]
pub type CLIPEN_R = crate::BitReader;
#[doc = "Field `CLIPEN` writer - Enable clipping"]
pub type CLIPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Reset FMAC unit"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - Reset FMAC unit"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable read interrupt"]
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable write interrupt"]
    #[inline(always)]
    pub fn wien(&self) -> WIEN_R {
        WIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable overflow error interrupts"]
    #[inline(always)]
    pub fn ovflien(&self) -> OVFLIEN_R {
        OVFLIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable underflow error interrupts"]
    #[inline(always)]
    pub fn unflien(&self) -> UNFLIEN_R {
        UNFLIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable saturation error interrupts"]
    #[inline(always)]
    pub fn satien(&self) -> SATIEN_R {
        SATIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable DMA read channel requests"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable DMA write channel requests"]
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable clipping"]
    #[inline(always)]
    pub fn clipen(&self) -> CLIPEN_R {
        CLIPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset FMAC unit"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable read interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RIEN_W<CRrs> {
        RIEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable write interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wien(&mut self) -> WIEN_W<CRrs> {
        WIEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable overflow error interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn ovflien(&mut self) -> OVFLIEN_W<CRrs> {
        OVFLIEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable underflow error interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn unflien(&mut self) -> UNFLIEN_W<CRrs> {
        UNFLIEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable saturation error interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn satien(&mut self) -> SATIEN_W<CRrs> {
        SATIEN_W::new(self, 4)
    }
    #[doc = "Bit 8 - Enable DMA read channel requests"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CRrs> {
        DMAREN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable DMA write channel requests"]
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DMAWEN_W<CRrs> {
        DMAWEN_W::new(self, 9)
    }
    #[doc = "Bit 15 - Enable clipping"]
    #[inline(always)]
    #[must_use]
    pub fn clipen(&mut self) -> CLIPEN_W<CRrs> {
        CLIPEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Reset FMAC unit"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CRrs> {
        RESET_W::new(self, 16)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
