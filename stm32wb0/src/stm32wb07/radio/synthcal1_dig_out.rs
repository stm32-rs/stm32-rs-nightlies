///Register `SYNTHCAL1_DIG_OUT` reader
pub type R = crate::R<SYNTHCAL1_DIG_OUTrs>;
///Field `VCO_CALAMP_OUT_10_7` reader - VCO CALAMP value
pub type VCO_CALAMP_OUT_10_7_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - VCO CALAMP value
    #[inline(always)]
    pub fn vco_calamp_out_10_7(&self) -> VCO_CALAMP_OUT_10_7_R {
        VCO_CALAMP_OUT_10_7_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNTHCAL1_DIG_OUT")
            .field("vco_calamp_out_10_7", &self.vco_calamp_out_10_7())
            .finish()
    }
}
/**SYNTHCAL1_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal1_dig_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:SYNTHCAL1_DIG_OUT)*/
pub struct SYNTHCAL1_DIG_OUTrs;
impl crate::RegisterSpec for SYNTHCAL1_DIG_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`synthcal1_dig_out::R`](R) reader structure
impl crate::Readable for SYNTHCAL1_DIG_OUTrs {}
///`reset()` method sets SYNTHCAL1_DIG_OUT to value 0x01
impl crate::Resettable for SYNTHCAL1_DIG_OUTrs {
    const RESET_VALUE: u32 = 0x01;
}
