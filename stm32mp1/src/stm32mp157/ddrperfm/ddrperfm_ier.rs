#[doc = "Register `DDRPERFM_IER` reader"]
pub type R = crate::R<DDRPERFM_IERrs>;
#[doc = "Register `DDRPERFM_IER` writer"]
pub type W = crate::W<DDRPERFM_IERrs>;
#[doc = "Field `OVFIE` reader - OVFIE"]
pub type OVFIE_R = crate::BitReader;
#[doc = "Field `OVFIE` writer - OVFIE"]
pub type OVFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OVFIE"]
    #[inline(always)]
    pub fn ovfie(&self) -> OVFIE_R {
        OVFIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OVFIE"]
    #[inline(always)]
    #[must_use]
    pub fn ovfie(&mut self) -> OVFIE_W<DDRPERFM_IERrs> {
        OVFIE_W::new(self, 0)
    }
}
#[doc = "DDRPERFM interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrperfm_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPERFM_IERrs;
impl crate::RegisterSpec for DDRPERFM_IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrperfm_ier::R`](R) reader structure"]
impl crate::Readable for DDRPERFM_IERrs {}
#[doc = "`write(|w| ..)` method takes [`ddrperfm_ier::W`](W) writer structure"]
impl crate::Writable for DDRPERFM_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPERFM_IER to value 0"]
impl crate::Resettable for DDRPERFM_IERrs {
    const RESET_VALUE: u32 = 0;
}
