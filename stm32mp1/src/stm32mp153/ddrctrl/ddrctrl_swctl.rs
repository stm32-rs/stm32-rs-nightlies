#[doc = "Register `DDRCTRL_SWCTL` reader"]
pub type R = crate::R<DDRCTRL_SWCTLrs>;
#[doc = "Register `DDRCTRL_SWCTL` writer"]
pub type W = crate::W<DDRCTRL_SWCTLrs>;
#[doc = "Field `SW_DONE` reader - SW_DONE"]
pub type SW_DONE_R = crate::BitReader;
#[doc = "Field `SW_DONE` writer - SW_DONE"]
pub type SW_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SW_DONE"]
    #[inline(always)]
    pub fn sw_done(&self) -> SW_DONE_R {
        SW_DONE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW_DONE"]
    #[inline(always)]
    #[must_use]
    pub fn sw_done(&mut self) -> SW_DONE_W<DDRCTRL_SWCTLrs> {
        SW_DONE_W::new(self, 0)
    }
}
#[doc = "DDRCTRL software register programming control enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_swctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_swctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_SWCTLrs;
impl crate::RegisterSpec for DDRCTRL_SWCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_swctl::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_SWCTLrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_swctl::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_SWCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_SWCTL to value 0x01"]
impl crate::Resettable for DDRCTRL_SWCTLrs {
    const RESET_VALUE: u32 = 0x01;
}
