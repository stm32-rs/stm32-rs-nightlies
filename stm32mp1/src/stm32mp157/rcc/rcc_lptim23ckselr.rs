#[doc = "Register `RCC_LPTIM23CKSELR` reader"]
pub type R = crate::R<RCC_LPTIM23CKSELRrs>;
#[doc = "Register `RCC_LPTIM23CKSELR` writer"]
pub type W = crate::W<RCC_LPTIM23CKSELRrs>;
#[doc = "Field `LPTIM23SRC` reader - LPTIM23SRC"]
pub type LPTIM23SRC_R = crate::FieldReader;
#[doc = "Field `LPTIM23SRC` writer - LPTIM23SRC"]
pub type LPTIM23SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - LPTIM23SRC"]
    #[inline(always)]
    pub fn lptim23src(&self) -> LPTIM23SRC_R {
        LPTIM23SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPTIM23SRC"]
    #[inline(always)]
    #[must_use]
    pub fn lptim23src(&mut self) -> LPTIM23SRC_W<RCC_LPTIM23CKSELRrs> {
        LPTIM23SRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_lptim23ckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_lptim23ckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_LPTIM23CKSELRrs;
impl crate::RegisterSpec for RCC_LPTIM23CKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_lptim23ckselr::R`](R) reader structure"]
impl crate::Readable for RCC_LPTIM23CKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_lptim23ckselr::W`](W) writer structure"]
impl crate::Writable for RCC_LPTIM23CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_LPTIM23CKSELR to value 0"]
impl crate::Resettable for RCC_LPTIM23CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
