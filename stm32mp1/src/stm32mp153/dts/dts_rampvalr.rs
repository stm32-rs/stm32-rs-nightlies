#[doc = "Register `DTS_RAMPVALR` reader"]
pub type R = crate::R<DTS_RAMPVALRrs>;
#[doc = "Field `TS1_RAMP_COEFF` reader - TS1_RAMP_COEFF"]
pub type TS1_RAMP_COEFF_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - TS1_RAMP_COEFF"]
    #[inline(always)]
    pub fn ts1_ramp_coeff(&self) -> TS1_RAMP_COEFF_R {
        TS1_RAMP_COEFF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_rampvalr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTS_RAMPVALRrs;
impl crate::RegisterSpec for DTS_RAMPVALRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dts_rampvalr::R`](R) reader structure"]
impl crate::Readable for DTS_RAMPVALRrs {}
#[doc = "`reset()` method sets DTS_RAMPVALR to value 0"]
impl crate::Resettable for DTS_RAMPVALRrs {
    const RESET_VALUE: u32 = 0;
}
