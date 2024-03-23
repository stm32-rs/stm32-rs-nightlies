#[doc = "Register `SAI_PDMDLY` reader"]
pub type R = crate::R<SAI_PDMDLYrs>;
#[doc = "Register `SAI_PDMDLY` writer"]
pub type W = crate::W<SAI_PDMDLYrs>;
#[doc = "Field `DLYM1L` reader - DLYM1L"]
pub type DLYM1L_R = crate::FieldReader;
#[doc = "Field `DLYM1L` writer - DLYM1L"]
pub type DLYM1L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM1R` reader - DLYM1R"]
pub type DLYM1R_R = crate::FieldReader;
#[doc = "Field `DLYM1R` writer - DLYM1R"]
pub type DLYM1R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM2L` reader - DLYM2L"]
pub type DLYM2L_R = crate::FieldReader;
#[doc = "Field `DLYM2L` writer - DLYM2L"]
pub type DLYM2L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM2R` reader - DLYM2R"]
pub type DLYM2R_R = crate::FieldReader;
#[doc = "Field `DLYM2R` writer - DLYM2R"]
pub type DLYM2R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM3L` reader - DLYM3L"]
pub type DLYM3L_R = crate::FieldReader;
#[doc = "Field `DLYM3L` writer - DLYM3L"]
pub type DLYM3L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM3R` reader - DLYM3R"]
pub type DLYM3R_R = crate::FieldReader;
#[doc = "Field `DLYM3R` writer - DLYM3R"]
pub type DLYM3R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM4L` reader - DLYM4L"]
pub type DLYM4L_R = crate::FieldReader;
#[doc = "Field `DLYM4L` writer - DLYM4L"]
pub type DLYM4L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM4R` reader - DLYM4R"]
pub type DLYM4R_R = crate::FieldReader;
#[doc = "Field `DLYM4R` writer - DLYM4R"]
pub type DLYM4R_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - DLYM1L"]
    #[inline(always)]
    pub fn dlym1l(&self) -> DLYM1L_R {
        DLYM1L_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - DLYM1R"]
    #[inline(always)]
    pub fn dlym1r(&self) -> DLYM1R_R {
        DLYM1R_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - DLYM2L"]
    #[inline(always)]
    pub fn dlym2l(&self) -> DLYM2L_R {
        DLYM2L_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - DLYM2R"]
    #[inline(always)]
    pub fn dlym2r(&self) -> DLYM2R_R {
        DLYM2R_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - DLYM3L"]
    #[inline(always)]
    pub fn dlym3l(&self) -> DLYM3L_R {
        DLYM3L_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - DLYM3R"]
    #[inline(always)]
    pub fn dlym3r(&self) -> DLYM3R_R {
        DLYM3R_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - DLYM4L"]
    #[inline(always)]
    pub fn dlym4l(&self) -> DLYM4L_R {
        DLYM4L_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - DLYM4R"]
    #[inline(always)]
    pub fn dlym4r(&self) -> DLYM4R_R {
        DLYM4R_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DLYM1L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym1l(&mut self) -> DLYM1L_W<SAI_PDMDLYrs> {
        DLYM1L_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - DLYM1R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym1r(&mut self) -> DLYM1R_W<SAI_PDMDLYrs> {
        DLYM1R_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - DLYM2L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym2l(&mut self) -> DLYM2L_W<SAI_PDMDLYrs> {
        DLYM2L_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - DLYM2R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym2r(&mut self) -> DLYM2R_W<SAI_PDMDLYrs> {
        DLYM2R_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - DLYM3L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym3l(&mut self) -> DLYM3L_W<SAI_PDMDLYrs> {
        DLYM3L_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - DLYM3R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym3r(&mut self) -> DLYM3R_W<SAI_PDMDLYrs> {
        DLYM3R_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - DLYM4L"]
    #[inline(always)]
    #[must_use]
    pub fn dlym4l(&mut self) -> DLYM4L_W<SAI_PDMDLYrs> {
        DLYM4L_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - DLYM4R"]
    #[inline(always)]
    #[must_use]
    pub fn dlym4r(&mut self) -> DLYM4R_W<SAI_PDMDLYrs> {
        DLYM4R_W::new(self, 28)
    }
}
#[doc = "PDM delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_pdmdly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_pdmdly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_PDMDLYrs;
impl crate::RegisterSpec for SAI_PDMDLYrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_pdmdly::R`](R) reader structure"]
impl crate::Readable for SAI_PDMDLYrs {}
#[doc = "`write(|w| ..)` method takes [`sai_pdmdly::W`](W) writer structure"]
impl crate::Writable for SAI_PDMDLYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAI_PDMDLY to value 0"]
impl crate::Resettable for SAI_PDMDLYrs {
    const RESET_VALUE: u32 = 0;
}
