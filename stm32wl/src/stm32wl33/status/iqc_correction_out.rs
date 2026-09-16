///Register `IQC_CORRECTION_OUT` reader
pub type R = crate::R<IQC_CORRECTION_OUTrs>;
///Field `IQC_CORRECT_OUT` reader - Final correction value output from IQC (compensation engine).
pub type IQC_CORRECT_OUT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - Final correction value output from IQC (compensation engine).
    #[inline(always)]
    pub fn iqc_correct_out(&self) -> IQC_CORRECT_OUT_R {
        IQC_CORRECT_OUT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IQC_CORRECTION_OUT")
            .field("iqc_correct_out", &self.iqc_correct_out())
            .finish()
    }
}
/**IQC_CORRECTION_OUT register

You can [`read`](crate::Reg::read) this register and get [`iqc_correction_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:IQC_CORRECTION_OUT)*/
pub struct IQC_CORRECTION_OUTrs;
impl crate::RegisterSpec for IQC_CORRECTION_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`iqc_correction_out::R`](R) reader structure
impl crate::Readable for IQC_CORRECTION_OUTrs {}
///`reset()` method sets IQC_CORRECTION_OUT to value 0
impl crate::Resettable for IQC_CORRECTION_OUTrs {}
