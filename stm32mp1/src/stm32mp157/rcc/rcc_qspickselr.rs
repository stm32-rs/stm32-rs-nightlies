#[doc = "Register `RCC_QSPICKSELR` reader"]
pub type R = crate::R<RCC_QSPICKSELRrs>;
#[doc = "Register `RCC_QSPICKSELR` writer"]
pub type W = crate::W<RCC_QSPICKSELRrs>;
#[doc = "Field `QSPISRC` reader - QSPISRC"]
pub type QSPISRC_R = crate::FieldReader;
#[doc = "Field `QSPISRC` writer - QSPISRC"]
pub type QSPISRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - QSPISRC"]
    #[inline(always)]
    pub fn qspisrc(&self) -> QSPISRC_R {
        QSPISRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - QSPISRC"]
    #[inline(always)]
    #[must_use]
    pub fn qspisrc(&mut self) -> QSPISRC_W<RCC_QSPICKSELRrs> {
        QSPISRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_qspickselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_qspickselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_QSPICKSELRrs;
impl crate::RegisterSpec for RCC_QSPICKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_qspickselr::R`](R) reader structure"]
impl crate::Readable for RCC_QSPICKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_qspickselr::W`](W) writer structure"]
impl crate::Writable for RCC_QSPICKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_QSPICKSELR to value 0"]
impl crate::Resettable for RCC_QSPICKSELRrs {
    const RESET_VALUE: u32 = 0;
}
