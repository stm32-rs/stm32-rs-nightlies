#[doc = "Register `DFSDM_FLT0FCR` reader"]
pub type R = crate::R<DFSDM_FLT0FCRrs>;
#[doc = "Register `DFSDM_FLT0FCR` writer"]
pub type W = crate::W<DFSDM_FLT0FCRrs>;
#[doc = "Field `IOSR` reader - IOSR"]
pub type IOSR_R = crate::FieldReader;
#[doc = "Field `IOSR` writer - IOSR"]
pub type IOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FOSR` reader - FOSR"]
pub type FOSR_R = crate::FieldReader<u16>;
#[doc = "Field `FOSR` writer - FOSR"]
pub type FOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `FORD` reader - FORD"]
pub type FORD_R = crate::FieldReader;
#[doc = "Field `FORD` writer - FORD"]
pub type FORD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - IOSR"]
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - FOSR"]
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:31 - FORD"]
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IOSR"]
    #[inline(always)]
    #[must_use]
    pub fn iosr(&mut self) -> IOSR_W<DFSDM_FLT0FCRrs> {
        IOSR_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - FOSR"]
    #[inline(always)]
    #[must_use]
    pub fn fosr(&mut self) -> FOSR_W<DFSDM_FLT0FCRrs> {
        FOSR_W::new(self, 16)
    }
    #[doc = "Bits 29:31 - FORD"]
    #[inline(always)]
    #[must_use]
    pub fn ford(&mut self) -> FORD_W<DFSDM_FLT0FCRrs> {
        FORD_W::new(self, 29)
    }
}
#[doc = "DFSDM filter 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT0FCRrs;
impl crate::RegisterSpec for DFSDM_FLT0FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0fcr::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT0FCRrs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt0fcr::W`](W) writer structure"]
impl crate::Writable for DFSDM_FLT0FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT0FCR to value 0"]
impl crate::Resettable for DFSDM_FLT0FCRrs {
    const RESET_VALUE: u32 = 0;
}
