///Register `AGC_HW_TRIM_OUT` reader
pub type R = crate::R<AGC_HW_TRIM_OUTrs>;
///Field `HW_AGC_ANTENNAE_TRIM` reader - AGC trim value (provided by the HW trimming, automatically loaded on POR).
pub type HW_AGC_ANTENNAE_TRIM_R = crate::FieldReader;
impl R {
    ///Bits 1:3 - AGC trim value (provided by the HW trimming, automatically loaded on POR).
    #[inline(always)]
    pub fn hw_agc_antennae_trim(&self) -> HW_AGC_ANTENNAE_TRIM_R {
        HW_AGC_ANTENNAE_TRIM_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC_HW_TRIM_OUT")
            .field("hw_agc_antennae_trim", &self.hw_agc_antennae_trim())
            .finish()
    }
}
/**AGC_HW_TRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`agc_hw_trim_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:AGC_HW_TRIM_OUT)*/
pub struct AGC_HW_TRIM_OUTrs;
impl crate::RegisterSpec for AGC_HW_TRIM_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`agc_hw_trim_out::R`](R) reader structure
impl crate::Readable for AGC_HW_TRIM_OUTrs {}
///`reset()` method sets AGC_HW_TRIM_OUT to value 0x06
impl crate::Resettable for AGC_HW_TRIM_OUTrs {
    const RESET_VALUE: u32 = 0x06;
}
