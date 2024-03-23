#[doc = "Register `CFR` writer"]
pub type W = crate::W<CFRrs>;
#[doc = "Field `CSOF0` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF1` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF2` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF0_W<CFRrs> {
        CSOF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF1_W<CFRrs> {
        CSOF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF2_W<CFRrs> {
        CSOF2_W::new(self, 2)
    }
}
#[doc = "DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFR to value 0"]
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u32 = 0;
}
