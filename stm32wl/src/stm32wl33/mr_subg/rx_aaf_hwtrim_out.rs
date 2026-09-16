///Register `RX_AAF_HWTRIM_OUT` reader
pub type R = crate::R<RX_AAF_HWTRIM_OUTrs>;
///Field `AAF_HW_FCTRIM` reader - AAF calibration information loaded by HW.
pub type AAF_HW_FCTRIM_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - AAF calibration information loaded by HW.
    #[inline(always)]
    pub fn aaf_hw_fctrim(&self) -> AAF_HW_FCTRIM_R {
        AAF_HW_FCTRIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_AAF_HWTRIM_OUT")
            .field("aaf_hw_fctrim", &self.aaf_hw_fctrim())
            .finish()
    }
}
/**RX_AAF_HWTRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`rx_aaf_hwtrim_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RX_AAF_HWTRIM_OUT)*/
pub struct RX_AAF_HWTRIM_OUTrs;
impl crate::RegisterSpec for RX_AAF_HWTRIM_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`rx_aaf_hwtrim_out::R`](R) reader structure
impl crate::Readable for RX_AAF_HWTRIM_OUTrs {}
///`reset()` method sets RX_AAF_HWTRIM_OUT to value 0x06
impl crate::Resettable for RX_AAF_HWTRIM_OUTrs {
    const RESET_VALUE: u32 = 0x06;
}
