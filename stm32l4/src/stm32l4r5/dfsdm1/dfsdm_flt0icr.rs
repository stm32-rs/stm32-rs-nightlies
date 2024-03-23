#[doc = "Register `DFSDM_FLT0ICR` reader"]
pub type R = crate::R<DFSDM_FLT0ICRrs>;
#[doc = "Register `DFSDM_FLT0ICR` writer"]
pub type W = crate::W<DFSDM_FLT0ICRrs>;
#[doc = "Field `CLRJOVRF` reader - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_R = crate::BitReader;
#[doc = "Field `CLRJOVRF` writer - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRROVRF` reader - Clear the regular conversion overrun flag"]
pub type CLRROVRF_R = crate::BitReader;
#[doc = "Field `CLRROVRF` writer - Clear the regular conversion overrun flag"]
pub type CLRROVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRCKABF` reader - Clear the clock absence flag"]
pub type CLRCKABF_R = crate::FieldReader;
#[doc = "Field `CLRCKABF` writer - Clear the clock absence flag"]
pub type CLRCKABF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLRSCDF` reader - Clear the short-circuit detector flag"]
pub type CLRSCDF_R = crate::FieldReader;
#[doc = "Field `CLRSCDF` writer - Clear the short-circuit detector flag"]
pub type CLRSCDF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag"]
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag"]
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<DFSDM_FLT0ICRrs> {
        CLRJOVRF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<DFSDM_FLT0ICRrs> {
        CLRROVRF_W::new(self, 3)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrckabf(&mut self) -> CLRCKABF_W<DFSDM_FLT0ICRrs> {
        CLRCKABF_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrscdf(&mut self) -> CLRSCDF_W<DFSDM_FLT0ICRrs> {
        CLRSCDF_W::new(self, 24)
    }
}
#[doc = "interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT0ICRrs;
impl crate::RegisterSpec for DFSDM_FLT0ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0icr::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT0ICRrs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt0icr::W`](W) writer structure"]
impl crate::Writable for DFSDM_FLT0ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT0ICR to value 0"]
impl crate::Resettable for DFSDM_FLT0ICRrs {
    const RESET_VALUE: u32 = 0;
}
