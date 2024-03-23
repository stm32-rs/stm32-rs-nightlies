#[doc = "Register `RGCFR` writer"]
pub type W = crate::W<RGCFRrs>;
#[doc = "Field `COF` writer - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
pub type COF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - Clear trigger event overrun flag Upon setting, this bit clears the corresponding overrun flag OFx in the DMAMUX_RGCSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof(&mut self) -> COF_W<RGCFRrs> {
        COF_W::new(self, 0)
    }
}
#[doc = "DMAMux - DMA request generator clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGCFRrs;
impl crate::RegisterSpec for RGCFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rgcfr::W`](W) writer structure"]
impl crate::Writable for RGCFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RGCFR to value 0"]
impl crate::Resettable for RGCFRrs {
    const RESET_VALUE: u32 = 0;
}
