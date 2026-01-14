///Register `SYNTHCAL5_DIG_OUT` reader
pub type R = crate::R<SYNTHCAL5_DIG_OUTrs>;
///Field `CBP_CALIB_WORD` reader - CBP Calibration word
pub type CBP_CALIB_WORD_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - CBP Calibration word
    #[inline(always)]
    pub fn cbp_calib_word(&self) -> CBP_CALIB_WORD_R {
        CBP_CALIB_WORD_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNTHCAL5_DIG_OUT")
            .field("cbp_calib_word", &self.cbp_calib_word())
            .finish()
    }
}
/**SYNTHCAL5_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal5_dig_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#RADIO:SYNTHCAL5_DIG_OUT)*/
pub struct SYNTHCAL5_DIG_OUTrs;
impl crate::RegisterSpec for SYNTHCAL5_DIG_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`synthcal5_dig_out::R`](R) reader structure
impl crate::Readable for SYNTHCAL5_DIG_OUTrs {}
///`reset()` method sets SYNTHCAL5_DIG_OUT to value 0x07
impl crate::Resettable for SYNTHCAL5_DIG_OUTrs {
    const RESET_VALUE: u32 = 0x07;
}
