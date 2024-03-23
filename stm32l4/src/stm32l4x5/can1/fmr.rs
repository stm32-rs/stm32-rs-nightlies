#[doc = "Register `FMR` reader"]
pub type R = crate::R<FMRrs>;
#[doc = "Register `FMR` writer"]
pub type W = crate::W<FMRrs>;
#[doc = "Field `FINIT` reader - Filter initialization mode"]
pub type FINIT_R = crate::BitReader;
#[doc = "Field `FINIT` writer - Filter initialization mode"]
pub type FINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filter initialization mode"]
    #[inline(always)]
    pub fn finit(&self) -> FINIT_R {
        FINIT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter initialization mode"]
    #[inline(always)]
    #[must_use]
    pub fn finit(&mut self) -> FINIT_W<FMRrs> {
        FINIT_W::new(self, 0)
    }
}
#[doc = "filter master register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMRrs;
impl crate::RegisterSpec for FMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmr::R`](R) reader structure"]
impl crate::Readable for FMRrs {}
#[doc = "`write(|w| ..)` method takes [`fmr::W`](W) writer structure"]
impl crate::Writable for FMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMR to value 0x2a1c_0e01"]
impl crate::Resettable for FMRrs {
    const RESET_VALUE: u32 = 0x2a1c_0e01;
}
