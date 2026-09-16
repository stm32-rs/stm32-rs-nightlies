///Register `PA_SAFEASK_OUT` reader
pub type R = crate::R<PA_SAFEASK_OUTrs>;
///Field `PA_CODEMAX` reader - Safe ASK level (provided after a CALIB_SAFEASK command), indicating the maximum PA Power to program before reaching ohmic saturation.
pub type PA_CODEMAX_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Safe ASK level (provided after a CALIB_SAFEASK command), indicating the maximum PA Power to program before reaching ohmic saturation.
    #[inline(always)]
    pub fn pa_codemax(&self) -> PA_CODEMAX_R {
        PA_CODEMAX_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PA_SAFEASK_OUT")
            .field("pa_codemax", &self.pa_codemax())
            .finish()
    }
}
/**PA_SAFEASK_OUT register

You can [`read`](crate::Reg::read) this register and get [`pa_safeask_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:PA_SAFEASK_OUT)*/
pub struct PA_SAFEASK_OUTrs;
impl crate::RegisterSpec for PA_SAFEASK_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`pa_safeask_out::R`](R) reader structure
impl crate::Readable for PA_SAFEASK_OUTrs {}
///`reset()` method sets PA_SAFEASK_OUT to value 0
impl crate::Resettable for PA_SAFEASK_OUTrs {}
