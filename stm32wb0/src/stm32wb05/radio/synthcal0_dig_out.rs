///Register `SYNTHCAL0_DIG_OUT` reader
pub type R = crate::R<SYNTHCAL0_DIG_OUTrs>;
///Field `VCO_CALAMP_OUT_6_0` reader - VCO CALAMP value
pub type VCO_CALAMP_OUT_6_0_R = crate::FieldReader;
impl R {
    ///Bits 0:6 - VCO CALAMP value
    #[inline(always)]
    pub fn vco_calamp_out_6_0(&self) -> VCO_CALAMP_OUT_6_0_R {
        VCO_CALAMP_OUT_6_0_R::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNTHCAL0_DIG_OUT")
            .field("vco_calamp_out_6_0", &self.vco_calamp_out_6_0())
            .finish()
    }
}
/**SYNTHCAL0_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal0_dig_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:SYNTHCAL0_DIG_OUT)*/
pub struct SYNTHCAL0_DIG_OUTrs;
impl crate::RegisterSpec for SYNTHCAL0_DIG_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`synthcal0_dig_out::R`](R) reader structure
impl crate::Readable for SYNTHCAL0_DIG_OUTrs {}
///`reset()` method sets SYNTHCAL0_DIG_OUT to value 0
impl crate::Resettable for SYNTHCAL0_DIG_OUTrs {}
