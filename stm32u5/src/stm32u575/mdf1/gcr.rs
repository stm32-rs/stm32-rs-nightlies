#[doc = "Register `GCR` reader"]
pub type R = crate::R<GCRrs>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GCRrs>;
#[doc = "Field `TRGO` reader - TRGO"]
pub type TRGO_R = crate::BitReader;
#[doc = "Field `TRGO` writer - TRGO"]
pub type TRGO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILVNB` reader - ILVNB"]
pub type ILVNB_R = crate::FieldReader;
#[doc = "Field `ILVNB` writer - ILVNB"]
pub type ILVNB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - TRGO"]
    #[inline(always)]
    pub fn trgo(&self) -> TRGO_R {
        TRGO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - ILVNB"]
    #[inline(always)]
    pub fn ilvnb(&self) -> ILVNB_R {
        ILVNB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TRGO"]
    #[inline(always)]
    #[must_use]
    pub fn trgo(&mut self) -> TRGO_W<GCRrs> {
        TRGO_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - ILVNB"]
    #[inline(always)]
    #[must_use]
    pub fn ilvnb(&mut self) -> ILVNB_W<GCRrs> {
        ILVNB_W::new(self, 4)
    }
}
#[doc = "MDF global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCRrs;
impl crate::RegisterSpec for GCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GCRrs {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GCRrs {
    const RESET_VALUE: u32 = 0;
}
