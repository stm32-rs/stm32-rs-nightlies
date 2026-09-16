///Register `SYNTHCAL2_DIG_OUT` reader
pub type R = crate::R<SYNTHCAL2_DIG_OUTrs>;
///Field `VCO_CALFREQ_OUT` reader - VCO CALFREQ value
pub type VCO_CALFREQ_OUT_R = crate::FieldReader;
impl R {
    ///Bits 0:6 - VCO CALFREQ value
    #[inline(always)]
    pub fn vco_calfreq_out(&self) -> VCO_CALFREQ_OUT_R {
        VCO_CALFREQ_OUT_R::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNTHCAL2_DIG_OUT")
            .field("vco_calfreq_out", &self.vco_calfreq_out())
            .finish()
    }
}
/**SYNTHCAL2_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal2_dig_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:SYNTHCAL2_DIG_OUT)*/
pub struct SYNTHCAL2_DIG_OUTrs;
impl crate::RegisterSpec for SYNTHCAL2_DIG_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`synthcal2_dig_out::R`](R) reader structure
impl crate::Readable for SYNTHCAL2_DIG_OUTrs {}
///`reset()` method sets SYNTHCAL2_DIG_OUT to value 0x40
impl crate::Resettable for SYNTHCAL2_DIG_OUTrs {
    const RESET_VALUE: u32 = 0x40;
}
