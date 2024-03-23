#[doc = "Register `BSEC_SCRATCH` reader"]
pub type R = crate::R<BSEC_SCRATCHrs>;
#[doc = "Register `BSEC_SCRATCH` writer"]
pub type W = crate::W<BSEC_SCRATCHrs>;
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - DATA"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<BSEC_SCRATCHrs> {
        DATA_W::new(self, 0)
    }
}
#[doc = "BSEC scratch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_scratch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsec_scratch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_SCRATCHrs;
impl crate::RegisterSpec for BSEC_SCRATCHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_scratch::R`](R) reader structure"]
impl crate::Readable for BSEC_SCRATCHrs {}
#[doc = "`write(|w| ..)` method takes [`bsec_scratch::W`](W) writer structure"]
impl crate::Writable for BSEC_SCRATCHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSEC_SCRATCH to value 0"]
impl crate::Resettable for BSEC_SCRATCHrs {
    const RESET_VALUE: u32 = 0;
}
