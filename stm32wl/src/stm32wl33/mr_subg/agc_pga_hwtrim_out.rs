///Register `AGC_PGA_HWTRIM_OUT` reader
pub type R = crate::R<AGC_PGA_HWTRIM_OUTrs>;
///Field `AGC_HW_PGA_TRIM` reader - AGC PGA calibration information loaded by HW from the SoC flash.
pub type AGC_HW_PGA_TRIM_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - AGC PGA calibration information loaded by HW from the SoC flash.
    #[inline(always)]
    pub fn agc_hw_pga_trim(&self) -> AGC_HW_PGA_TRIM_R {
        AGC_HW_PGA_TRIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGC_PGA_HWTRIM_OUT")
            .field("agc_hw_pga_trim", &self.agc_hw_pga_trim())
            .finish()
    }
}
/**AGC_PGA_HWTRIM_OUT register

You can [`read`](crate::Reg::read) this register and get [`agc_pga_hwtrim_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AGC_PGA_HWTRIM_OUT)*/
pub struct AGC_PGA_HWTRIM_OUTrs;
impl crate::RegisterSpec for AGC_PGA_HWTRIM_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`agc_pga_hwtrim_out::R`](R) reader structure
impl crate::Readable for AGC_PGA_HWTRIM_OUTrs {}
///`reset()` method sets AGC_PGA_HWTRIM_OUT to value 0x08
impl crate::Resettable for AGC_PGA_HWTRIM_OUTrs {
    const RESET_VALUE: u32 = 0x08;
}
