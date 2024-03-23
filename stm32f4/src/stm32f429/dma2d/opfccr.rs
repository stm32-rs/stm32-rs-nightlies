#[doc = "Register `OPFCCR` reader"]
pub type R = crate::R<OPFCCRrs>;
#[doc = "Register `OPFCCR` writer"]
pub type W = crate::W<OPFCCRrs>;
#[doc = "Field `CM` reader - Color mode"]
pub type CM_R = crate::FieldReader;
#[doc = "Field `CM` writer - Color mode"]
pub type CM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Color mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Color mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<OPFCCRrs> {
        CM_W::new(self, 0)
    }
}
#[doc = "output PFC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opfccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opfccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPFCCRrs;
impl crate::RegisterSpec for OPFCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opfccr::R`](R) reader structure"]
impl crate::Readable for OPFCCRrs {}
#[doc = "`write(|w| ..)` method takes [`opfccr::W`](W) writer structure"]
impl crate::Writable for OPFCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPFCCR to value 0"]
impl crate::Resettable for OPFCCRrs {
    const RESET_VALUE: u32 = 0;
}
