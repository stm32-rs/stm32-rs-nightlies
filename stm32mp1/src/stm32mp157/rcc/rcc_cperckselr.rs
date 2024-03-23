#[doc = "Register `RCC_CPERCKSELR` reader"]
pub type R = crate::R<RCC_CPERCKSELRrs>;
#[doc = "Register `RCC_CPERCKSELR` writer"]
pub type W = crate::W<RCC_CPERCKSELRrs>;
#[doc = "Field `CKPERSRC` reader - CKPERSRC"]
pub type CKPERSRC_R = crate::FieldReader;
#[doc = "Field `CKPERSRC` writer - CKPERSRC"]
pub type CKPERSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - CKPERSRC"]
    #[inline(always)]
    pub fn ckpersrc(&self) -> CKPERSRC_R {
        CKPERSRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CKPERSRC"]
    #[inline(always)]
    #[must_use]
    pub fn ckpersrc(&mut self) -> CKPERSRC_W<RCC_CPERCKSELRrs> {
        CKPERSRC_W::new(self, 0)
    }
}
#[doc = "This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_cperckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_cperckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_CPERCKSELRrs;
impl crate::RegisterSpec for RCC_CPERCKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_cperckselr::R`](R) reader structure"]
impl crate::Readable for RCC_CPERCKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_cperckselr::W`](W) writer structure"]
impl crate::Writable for RCC_CPERCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_CPERCKSELR to value 0"]
impl crate::Resettable for RCC_CPERCKSELRrs {
    const RESET_VALUE: u32 = 0;
}
