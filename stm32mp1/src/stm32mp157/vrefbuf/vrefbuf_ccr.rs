#[doc = "Register `VREFBUF_CCR` reader"]
pub type R = crate::R<VREFBUF_CCRrs>;
#[doc = "Register `VREFBUF_CCR` writer"]
pub type W = crate::W<VREFBUF_CCRrs>;
#[doc = "Field `TRIM` reader - TRIM"]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `TRIM` writer - TRIM"]
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - TRIM"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - TRIM"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<VREFBUF_CCRrs> {
        TRIM_W::new(self, 0)
    }
}
#[doc = "VREFBUF calibration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vrefbuf_ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vrefbuf_ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREFBUF_CCRrs;
impl crate::RegisterSpec for VREFBUF_CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrefbuf_ccr::R`](R) reader structure"]
impl crate::Readable for VREFBUF_CCRrs {}
#[doc = "`write(|w| ..)` method takes [`vrefbuf_ccr::W`](W) writer structure"]
impl crate::Writable for VREFBUF_CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREFBUF_CCR to value 0"]
impl crate::Resettable for VREFBUF_CCRrs {
    const RESET_VALUE: u32 = 0;
}
