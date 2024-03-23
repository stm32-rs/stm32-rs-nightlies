#[doc = "Register `IFCR` reader"]
pub type R = crate::R<IFCRrs>;
#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCRrs>;
#[doc = "Field `CTEIF` reader - Clear Transfer error interrupt flag"]
pub type CTEIF_R = crate::BitReader;
#[doc = "Field `CTEIF` writer - Clear Transfer error interrupt flag"]
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF` reader - Clear transfer complete interrupt flag"]
pub type CTCIF_R = crate::BitReader;
#[doc = "Field `CTCIF` writer - Clear transfer complete interrupt flag"]
pub type CTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTWIF` reader - Clear transfer watermark interrupt flag"]
pub type CTWIF_R = crate::BitReader;
#[doc = "Field `CTWIF` writer - Clear transfer watermark interrupt flag"]
pub type CTWIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAECIF` reader - Clear CLUT access error interrupt flag"]
pub type CAECIF_R = crate::BitReader;
#[doc = "Field `CAECIF` writer - Clear CLUT access error interrupt flag"]
pub type CAECIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF` reader - Clear CLUT transfer complete interrupt flag"]
pub type CCTCIF_R = crate::BitReader;
#[doc = "Field `CCTCIF` writer - Clear CLUT transfer complete interrupt flag"]
pub type CCTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCEIF` reader - Clear configuration error interrupt flag"]
pub type CCEIF_R = crate::BitReader;
#[doc = "Field `CCEIF` writer - Clear configuration error interrupt flag"]
pub type CCEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear Transfer error interrupt flag"]
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag"]
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag"]
    #[inline(always)]
    pub fn ctwif(&self) -> CTWIF_R {
        CTWIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag"]
    #[inline(always)]
    pub fn caecif(&self) -> CAECIF_R {
        CAECIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag"]
    #[inline(always)]
    pub fn cctcif(&self) -> CCTCIF_R {
        CCTCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear configuration error interrupt flag"]
    #[inline(always)]
    pub fn cceif(&self) -> CCEIF_R {
        CCEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Transfer error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif(&mut self) -> CTCIF_W<IFCRrs> {
        CTCIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctwif(&mut self) -> CTWIF_W<IFCRrs> {
        CTWIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn caecif(&mut self) -> CAECIF_W<IFCRrs> {
        CAECIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cctcif(&mut self) -> CCTCIF_W<IFCRrs> {
        CCTCIF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear configuration error interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cceif(&mut self) -> CCEIF_W<IFCRrs> {
        CCEIF_W::new(self, 5)
    }
}
#[doc = "interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifcr::R`](R) reader structure"]
impl crate::Readable for IFCRrs {}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}
