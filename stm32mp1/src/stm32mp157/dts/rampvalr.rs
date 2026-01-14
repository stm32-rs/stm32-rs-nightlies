///Register `RAMPVALR` reader
pub type R = crate::R<RAMPVALRrs>;
///Field `TS1_RAMP_COEFF` reader - TS1_RAMP_COEFF
pub type TS1_RAMP_COEFF_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - TS1_RAMP_COEFF
    #[inline(always)]
    pub fn ts1_ramp_coeff(&self) -> TS1_RAMP_COEFF_R {
        TS1_RAMP_COEFF_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMPVALR")
            .field("ts1_ramp_coeff", &self.ts1_ramp_coeff())
            .finish()
    }
}
/**The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed.

You can [`read`](crate::Reg::read) this register and get [`rampvalr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DTS:RAMPVALR)*/
pub struct RAMPVALRrs;
impl crate::RegisterSpec for RAMPVALRrs {
    type Ux = u32;
}
///`read()` method returns [`rampvalr::R`](R) reader structure
impl crate::Readable for RAMPVALRrs {}
///`reset()` method sets RAMPVALR to value 0
impl crate::Resettable for RAMPVALRrs {}
