#[doc = "Register `RGSR` reader"]
pub type R = crate::R<RGSRrs>;
#[doc = "Field `OF` reader - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
pub type OF_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "DMAMux - DMA request generator status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGSRrs;
impl crate::RegisterSpec for RGSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgsr::R`](R) reader structure"]
impl crate::Readable for RGSRrs {}
#[doc = "`reset()` method sets RGSR to value 0"]
impl crate::Resettable for RGSRrs {
    const RESET_VALUE: u32 = 0;
}
