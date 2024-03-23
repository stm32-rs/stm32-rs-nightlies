#[doc = "Register `RCC_SAI4CKSELR` reader"]
pub type R = crate::R<RCC_SAI4CKSELRrs>;
#[doc = "Register `RCC_SAI4CKSELR` writer"]
pub type W = crate::W<RCC_SAI4CKSELRrs>;
#[doc = "Field `SAI4SRC` reader - SAI4SRC"]
pub type SAI4SRC_R = crate::FieldReader;
#[doc = "Field `SAI4SRC` writer - SAI4SRC"]
pub type SAI4SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SAI4SRC"]
    #[inline(always)]
    pub fn sai4src(&self) -> SAI4SRC_R {
        SAI4SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI4SRC"]
    #[inline(always)]
    #[must_use]
    pub fn sai4src(&mut self) -> SAI4SRC_W<RCC_SAI4CKSELRrs> {
        SAI4SRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_sai4ckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_sai4ckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_SAI4CKSELRrs;
impl crate::RegisterSpec for RCC_SAI4CKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_sai4ckselr::R`](R) reader structure"]
impl crate::Readable for RCC_SAI4CKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_sai4ckselr::W`](W) writer structure"]
impl crate::Writable for RCC_SAI4CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_SAI4CKSELR to value 0"]
impl crate::Resettable for RCC_SAI4CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
