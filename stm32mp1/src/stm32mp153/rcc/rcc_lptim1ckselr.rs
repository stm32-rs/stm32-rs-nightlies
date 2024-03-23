#[doc = "Register `RCC_LPTIM1CKSELR` reader"]
pub type R = crate::R<RCC_LPTIM1CKSELRrs>;
#[doc = "Register `RCC_LPTIM1CKSELR` writer"]
pub type W = crate::W<RCC_LPTIM1CKSELRrs>;
#[doc = "Field `LPTIM1SRC` reader - LPTIM1SRC"]
pub type LPTIM1SRC_R = crate::FieldReader;
#[doc = "Field `LPTIM1SRC` writer - LPTIM1SRC"]
pub type LPTIM1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - LPTIM1SRC"]
    #[inline(always)]
    pub fn lptim1src(&self) -> LPTIM1SRC_R {
        LPTIM1SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPTIM1SRC"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1src(&mut self) -> LPTIM1SRC_W<RCC_LPTIM1CKSELRrs> {
        LPTIM1SRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the LPTIM1 block.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_lptim1ckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_lptim1ckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_LPTIM1CKSELRrs;
impl crate::RegisterSpec for RCC_LPTIM1CKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_lptim1ckselr::R`](R) reader structure"]
impl crate::Readable for RCC_LPTIM1CKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_lptim1ckselr::W`](W) writer structure"]
impl crate::Writable for RCC_LPTIM1CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_LPTIM1CKSELR to value 0"]
impl crate::Resettable for RCC_LPTIM1CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
