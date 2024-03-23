#[doc = "Register `RCC_STGENCKSELR` reader"]
pub type R = crate::R<RCC_STGENCKSELRrs>;
#[doc = "Register `RCC_STGENCKSELR` writer"]
pub type W = crate::W<RCC_STGENCKSELRrs>;
#[doc = "Field `STGENSRC` reader - STGENSRC"]
pub type STGENSRC_R = crate::FieldReader;
#[doc = "Field `STGENSRC` writer - STGENSRC"]
pub type STGENSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - STGENSRC"]
    #[inline(always)]
    pub fn stgensrc(&self) -> STGENSRC_R {
        STGENSRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - STGENSRC"]
    #[inline(always)]
    #[must_use]
    pub fn stgensrc(&mut self) -> STGENSRC_W<RCC_STGENCKSELRrs> {
        STGENSRC_W::new(self, 0)
    }
}
#[doc = "This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_stgenckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_stgenckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_STGENCKSELRrs;
impl crate::RegisterSpec for RCC_STGENCKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_stgenckselr::R`](R) reader structure"]
impl crate::Readable for RCC_STGENCKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_stgenckselr::W`](W) writer structure"]
impl crate::Writable for RCC_STGENCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_STGENCKSELR to value 0"]
impl crate::Resettable for RCC_STGENCKSELRrs {
    const RESET_VALUE: u32 = 0;
}
