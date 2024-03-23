#[doc = "Register `DFSDM_FLT5EXMIN` reader"]
pub type R = crate::R<DFSDM_FLT5EXMINrs>;
#[doc = "Register `DFSDM_FLT5EXMIN` writer"]
pub type W = crate::W<DFSDM_FLT5EXMINrs>;
#[doc = "Field `EXMINCH` reader - EXMINCH"]
pub type EXMINCH_R = crate::FieldReader;
#[doc = "Field `EXMIN` reader - EXMIN"]
pub type EXMIN_R = crate::FieldReader<u32>;
#[doc = "Field `EXMIN` writer - EXMIN"]
pub type EXMIN_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:2 - EXMINCH"]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    #[must_use]
    pub fn exmin(&mut self) -> EXMIN_W<DFSDM_FLT5EXMINrs> {
        EXMIN_W::new(self, 8)
    }
}
#[doc = "DFSDM filter 5 extremes detector minimum register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt5exmin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt5exmin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT5EXMINrs;
impl crate::RegisterSpec for DFSDM_FLT5EXMINrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt5exmin::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT5EXMINrs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt5exmin::W`](W) writer structure"]
impl crate::Writable for DFSDM_FLT5EXMINrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT5EXMIN to value 0x7fff_ff00"]
impl crate::Resettable for DFSDM_FLT5EXMINrs {
    const RESET_VALUE: u32 = 0x7fff_ff00;
}
