#[doc = "Register `DFSDM_FLT3ICR` reader"]
pub type R = crate::R<DFSDM_FLT3ICRrs>;
#[doc = "Register `DFSDM_FLT3ICR` writer"]
pub type W = crate::W<DFSDM_FLT3ICRrs>;
#[doc = "Field `CLRJOVRF` reader - CLRJOVRF"]
pub type CLRJOVRF_R = crate::BitReader;
#[doc = "Field `CLRJOVRF` writer - CLRJOVRF"]
pub type CLRJOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRROVRF` reader - CLRROVRF"]
pub type CLRROVRF_R = crate::BitReader;
#[doc = "Field `CLRROVRF` writer - CLRROVRF"]
pub type CLRROVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRCKABF` reader - CLRCKABF"]
pub type CLRCKABF_R = crate::FieldReader;
#[doc = "Field `CLRCKABF` writer - CLRCKABF"]
pub type CLRCKABF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLRSCDF` reader - CLRSCDF"]
pub type CLRSCDF_R = crate::FieldReader;
#[doc = "Field `CLRSCDF` writer - CLRSCDF"]
pub type CLRSCDF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 2 - CLRJOVRF"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CLRROVRF"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CLRCKABF"]
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CLRSCDF"]
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - CLRJOVRF"]
    #[inline(always)]
    #[must_use]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<DFSDM_FLT3ICRrs> {
        CLRJOVRF_W::new(self, 2)
    }
    #[doc = "Bit 3 - CLRROVRF"]
    #[inline(always)]
    #[must_use]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<DFSDM_FLT3ICRrs> {
        CLRROVRF_W::new(self, 3)
    }
    #[doc = "Bits 16:23 - CLRCKABF"]
    #[inline(always)]
    #[must_use]
    pub fn clrckabf(&mut self) -> CLRCKABF_W<DFSDM_FLT3ICRrs> {
        CLRCKABF_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CLRSCDF"]
    #[inline(always)]
    #[must_use]
    pub fn clrscdf(&mut self) -> CLRSCDF_W<DFSDM_FLT3ICRrs> {
        CLRSCDF_W::new(self, 24)
    }
}
#[doc = "DFSDM filter 3 interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt3icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt3icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT3ICRrs;
impl crate::RegisterSpec for DFSDM_FLT3ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt3icr::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT3ICRrs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt3icr::W`](W) writer structure"]
impl crate::Writable for DFSDM_FLT3ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT3ICR to value 0"]
impl crate::Resettable for DFSDM_FLT3ICRrs {
    const RESET_VALUE: u32 = 0;
}
