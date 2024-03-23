#[doc = "Register `DDRPERFM_CTL` writer"]
pub type W = crate::W<DDRPERFM_CTLrs>;
#[doc = "Field `START` writer - START"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - STOP"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - START"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<DDRPERFM_CTLrs> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - STOP"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<DDRPERFM_CTLrs> {
        STOP_W::new(self, 1)
    }
}
#[doc = "Write-only register. A read request returns all zeros.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrperfm_ctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPERFM_CTLrs;
impl crate::RegisterSpec for DDRPERFM_CTLrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ddrperfm_ctl::W`](W) writer structure"]
impl crate::Writable for DDRPERFM_CTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPERFM_CTL to value 0"]
impl crate::Resettable for DDRPERFM_CTLrs {
    const RESET_VALUE: u32 = 0;
}
