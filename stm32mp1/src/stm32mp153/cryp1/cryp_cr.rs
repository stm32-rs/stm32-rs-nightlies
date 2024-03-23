#[doc = "Register `CRYP_CR` reader"]
pub type R = crate::R<CRYP_CRrs>;
#[doc = "Register `CRYP_CR` writer"]
pub type W = crate::W<CRYP_CRrs>;
#[doc = "Field `ALGODIR` reader - ALGODIR"]
pub type ALGODIR_R = crate::BitReader;
#[doc = "Field `ALGODIR` writer - ALGODIR"]
pub type ALGODIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGOMODE` reader - ALGOMODE"]
pub type ALGOMODE_R = crate::FieldReader;
#[doc = "Field `ALGOMODE` writer - ALGOMODE"]
pub type ALGOMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DATATYPE` reader - DATATYPE"]
pub type DATATYPE_R = crate::FieldReader;
#[doc = "Field `DATATYPE` writer - DATATYPE"]
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYSIZE` reader - KEYSIZE"]
pub type KEYSIZE_R = crate::FieldReader;
#[doc = "Field `KEYSIZE` writer - KEYSIZE"]
pub type KEYSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FFLUSH` writer - FFLUSH"]
pub type FFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPEN` reader - CRYPEN"]
pub type CRYPEN_R = crate::BitReader;
#[doc = "Field `CRYPEN` writer - CRYPEN"]
pub type CRYPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCM_CCMPH` reader - GCM_CCMPH"]
pub type GCM_CCMPH_R = crate::FieldReader;
#[doc = "Field `GCM_CCMPH` writer - GCM_CCMPH"]
pub type GCM_CCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ALGOMODE3` reader - ALGOMODE3"]
pub type ALGOMODE3_R = crate::BitReader;
#[doc = "Field `ALGOMODE3` writer - ALGOMODE3"]
pub type ALGOMODE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPBLB` reader - NPBLB"]
pub type NPBLB_R = crate::FieldReader;
#[doc = "Field `NPBLB` writer - NPBLB"]
pub type NPBLB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 2 - ALGODIR"]
    #[inline(always)]
    pub fn algodir(&self) -> ALGODIR_R {
        ALGODIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - ALGOMODE"]
    #[inline(always)]
    pub fn algomode(&self) -> ALGOMODE_R {
        ALGOMODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DATATYPE"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - KEYSIZE"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - CRYPEN"]
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - GCM_CCMPH"]
    #[inline(always)]
    pub fn gcm_ccmph(&self) -> GCM_CCMPH_R {
        GCM_CCMPH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - ALGOMODE3"]
    #[inline(always)]
    pub fn algomode3(&self) -> ALGOMODE3_R {
        ALGOMODE3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - NPBLB"]
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - ALGODIR"]
    #[inline(always)]
    #[must_use]
    pub fn algodir(&mut self) -> ALGODIR_W<CRYP_CRrs> {
        ALGODIR_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - ALGOMODE"]
    #[inline(always)]
    #[must_use]
    pub fn algomode(&mut self) -> ALGOMODE_W<CRYP_CRrs> {
        ALGOMODE_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - DATATYPE"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<CRYP_CRrs> {
        DATATYPE_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - KEYSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<CRYP_CRrs> {
        KEYSIZE_W::new(self, 8)
    }
    #[doc = "Bit 14 - FFLUSH"]
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<CRYP_CRrs> {
        FFLUSH_W::new(self, 14)
    }
    #[doc = "Bit 15 - CRYPEN"]
    #[inline(always)]
    #[must_use]
    pub fn crypen(&mut self) -> CRYPEN_W<CRYP_CRrs> {
        CRYPEN_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - GCM_CCMPH"]
    #[inline(always)]
    #[must_use]
    pub fn gcm_ccmph(&mut self) -> GCM_CCMPH_W<CRYP_CRrs> {
        GCM_CCMPH_W::new(self, 16)
    }
    #[doc = "Bit 19 - ALGOMODE3"]
    #[inline(always)]
    #[must_use]
    pub fn algomode3(&mut self) -> ALGOMODE3_W<CRYP_CRrs> {
        ALGOMODE3_W::new(self, 19)
    }
    #[doc = "Bits 20:23 - NPBLB"]
    #[inline(always)]
    #[must_use]
    pub fn npblb(&mut self) -> NPBLB_W<CRYP_CRrs> {
        NPBLB_W::new(self, 20)
    }
}
#[doc = "CRYP control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_CRrs;
impl crate::RegisterSpec for CRYP_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_cr::R`](R) reader structure"]
impl crate::Readable for CRYP_CRrs {}
#[doc = "`write(|w| ..)` method takes [`cryp_cr::W`](W) writer structure"]
impl crate::Writable for CRYP_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYP_CR to value 0"]
impl crate::Resettable for CRYP_CRrs {
    const RESET_VALUE: u32 = 0;
}
