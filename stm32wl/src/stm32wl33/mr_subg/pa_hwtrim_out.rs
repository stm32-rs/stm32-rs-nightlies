///Register `PA_HWTRIM_OUT` reader
pub type R = crate::R<PA_HWTRIM_OUTrs>;
///Field `PA_HW_DEGEN_TRIM` reader - MSB part meaning:
pub type PA_HW_DEGEN_TRIM_R = crate::FieldReader;
impl R {
    ///Bits 4:7 - MSB part meaning:
    #[inline(always)]
    pub fn pa_hw_degen_trim(&self) -> PA_HW_DEGEN_TRIM_R {
        PA_HW_DEGEN_TRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PA_HWTRIM_OUT")
            .field("pa_hw_degen_trim", &self.pa_hw_degen_trim())
            .finish()
    }
}
/**PA_HWTRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`pa_hwtrim_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:PA_HWTRIM_OUT)*/
pub struct PA_HWTRIM_OUTrs;
impl crate::RegisterSpec for PA_HWTRIM_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`pa_hwtrim_out::R`](R) reader structure
impl crate::Readable for PA_HWTRIM_OUTrs {}
///`reset()` method sets PA_HWTRIM_OUT to value 0x88
impl crate::Resettable for PA_HWTRIM_OUTrs {
    const RESET_VALUE: u32 = 0x88;
}
