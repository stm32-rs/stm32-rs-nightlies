///Register `AGC_DIG_OUT` reader
pub type R = crate::R<AGC_DIG_OUTrs>;
///Field `AGC_ATT_OUT` reader - AGC attenuation value
pub type AGC_ATT_OUT_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - AGC attenuation value
    #[inline(always)]
    pub fn agc_att_out(&self) -> AGC_ATT_OUT_R {
        AGC_ATT_OUT_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC_DIG_OUT")
            .field("agc_att_out", &self.agc_att_out())
            .finish()
    }
}
/**AGC_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`agc_dig_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:AGC_DIG_OUT)*/
pub struct AGC_DIG_OUTrs;
impl crate::RegisterSpec for AGC_DIG_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`agc_dig_out::R`](R) reader structure
impl crate::Readable for AGC_DIG_OUTrs {}
///`reset()` method sets AGC_DIG_OUT to value 0
impl crate::Resettable for AGC_DIG_OUTrs {}
