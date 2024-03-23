#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `EN` reader - Channel enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Channel enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIE` reader - Half transfer interrupt enable"]
pub type HTIE_R = crate::BitReader;
#[doc = "Field `HTIE` writer - Half transfer interrupt enable"]
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Data transfer direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Data transfer direction"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIRC` reader - Circular mode"]
pub type CIRC_R = crate::BitReader;
#[doc = "Field `CIRC` writer - Circular mode"]
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINC` reader - Peripheral increment mode"]
pub type PINC_R = crate::BitReader;
#[doc = "Field `PINC` writer - Peripheral increment mode"]
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINC` reader - Memory increment mode"]
pub type MINC_R = crate::BitReader;
#[doc = "Field `MINC` writer - Memory increment mode"]
pub type MINC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIZE` reader - Peripheral size"]
pub type PSIZE_R = crate::FieldReader;
#[doc = "Field `PSIZE` writer - Peripheral size"]
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MSIZE` reader - Memory size"]
pub type MSIZE_R = crate::FieldReader;
#[doc = "Field `MSIZE` writer - Memory size"]
pub type MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PL` reader - Channel priority level"]
pub type PL_R = crate::FieldReader;
#[doc = "Field `PL` writer - Channel priority level"]
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM2MEM` reader - Memory to memory mode"]
pub type MEM2MEM_R = crate::BitReader;
#[doc = "Field `MEM2MEM` writer - Memory to memory mode"]
pub type MEM2MEM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Peripheral size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Memory size"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel priority level"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<CRrs> {
        HTIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CRrs> {
        DIR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Circular mode"]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<CRrs> {
        CIRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<CRrs> {
        PINC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<CRrs> {
        MINC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Peripheral size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<CRrs> {
        PSIZE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Memory size"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<CRrs> {
        MSIZE_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Channel priority level"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<CRrs> {
        PL_W::new(self, 12)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<CRrs> {
        MEM2MEM_W::new(self, 14)
    }
}
#[doc = "DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
