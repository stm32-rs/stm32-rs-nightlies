///Register `SYNTHCAL4_DIG_OUT` reader
pub type R = crate::R<SYNTHCAL4_DIG_OUTrs>;
///Field `MOD_REF_DAC_WORD_OUT` reader - Calibration word
pub type MOD_REF_DAC_WORD_OUT_R = crate::FieldReader;
impl R {
    ///Bits 0:5 - Calibration word
    #[inline(always)]
    pub fn mod_ref_dac_word_out(&self) -> MOD_REF_DAC_WORD_OUT_R {
        MOD_REF_DAC_WORD_OUT_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNTHCAL4_DIG_OUT")
            .field("mod_ref_dac_word_out", &self.mod_ref_dac_word_out())
            .finish()
    }
}
/**SYNTHCAL4_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal4_dig_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RADIO:SYNTHCAL4_DIG_OUT)*/
pub struct SYNTHCAL4_DIG_OUTrs;
impl crate::RegisterSpec for SYNTHCAL4_DIG_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`synthcal4_dig_out::R`](R) reader structure
impl crate::Readable for SYNTHCAL4_DIG_OUTrs {}
///`reset()` method sets SYNTHCAL4_DIG_OUT to value 0x18
impl crate::Resettable for SYNTHCAL4_DIG_OUTrs {
    const RESET_VALUE: u32 = 0x18;
}
