#[doc = "Register `RAMPVALR` reader"]
pub type R = crate::R<RAMPVALRrs>;
#[doc = "Field `TS1_RAMP_COEFF` reader - Engineering value of the ramp coefficient for the temperature sensor 1. This value is expressed in Hz/�C."]
pub type TS1_RAMP_COEFF_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Engineering value of the ramp coefficient for the temperature sensor 1. This value is expressed in Hz/�C."]
    #[inline(always)]
    pub fn ts1_ramp_coeff(&self) -> TS1_RAMP_COEFF_R {
        TS1_RAMP_COEFF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Temperature sensor ramp value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rampvalr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAMPVALRrs;
impl crate::RegisterSpec for RAMPVALRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rampvalr::R`](R) reader structure"]
impl crate::Readable for RAMPVALRrs {}
#[doc = "`reset()` method sets RAMPVALR to value 0"]
impl crate::Resettable for RAMPVALRrs {
    const RESET_VALUE: u32 = 0;
}
