#[doc = "Register `TZSC_CR` reader"]
pub type R = crate::R<TZSC_CRrs>;
#[doc = "Register `TZSC_CR` writer"]
pub type W = crate::W<TZSC_CRrs>;
#[doc = "Field `LCK` reader - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx registers until next reset"]
pub type LCK_R = crate::BitReader;
#[doc = "Field `LCK` writer - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx registers until next reset"]
pub type LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx registers until next reset"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock the configuration of GTZC1_TZSC_SECCFGRx and GTZC1_TZSC_PRIVCFGRx registers until next reset"]
    #[inline(always)]
    #[must_use]
    pub fn lck(&mut self) -> LCK_W<TZSC_CRrs> {
        LCK_W::new(self, 0)
    }
}
#[doc = "TZSC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_CRrs;
impl crate::RegisterSpec for TZSC_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_cr::R`](R) reader structure"]
impl crate::Readable for TZSC_CRrs {}
#[doc = "`write(|w| ..)` method takes [`tzsc_cr::W`](W) writer structure"]
impl crate::Writable for TZSC_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSC_CR to value 0"]
impl crate::Resettable for TZSC_CRrs {
    const RESET_VALUE: u32 = 0;
}
