///Register `VCO_CALIB_OUT` reader
pub type R = crate::R<VCO_CALIB_OUTrs>;
///Field `VCO_CALFREQ_OUT` reader - VCO frequency calibration value currently output by the VCO calibration block (and applied on the VCO when ON)
pub type VCO_CALFREQ_OUT_R = crate::FieldReader;
///Field `VCO_CALAMP_OUT` reader - VCO amplitude calibration value currently output by the VCO calibration block (and applied on the VCO when ON)
pub type VCO_CALAMP_OUT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:6 - VCO frequency calibration value currently output by the VCO calibration block (and applied on the VCO when ON)
    #[inline(always)]
    pub fn vco_calfreq_out(&self) -> VCO_CALFREQ_OUT_R {
        VCO_CALFREQ_OUT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:21 - VCO amplitude calibration value currently output by the VCO calibration block (and applied on the VCO when ON)
    #[inline(always)]
    pub fn vco_calamp_out(&self) -> VCO_CALAMP_OUT_R {
        VCO_CALAMP_OUT_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VCO_CALIB_OUT")
            .field("vco_calfreq_out", &self.vco_calfreq_out())
            .field("vco_calamp_out", &self.vco_calamp_out())
            .finish()
    }
}
/**VCO_CALIB_OUT register

You can [`read`](crate::Reg::read) this register and get [`vco_calib_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:VCO_CALIB_OUT)*/
pub struct VCO_CALIB_OUTrs;
impl crate::RegisterSpec for VCO_CALIB_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`vco_calib_out::R`](R) reader structure
impl crate::Readable for VCO_CALIB_OUTrs {}
///`reset()` method sets VCO_CALIB_OUT to value 0xff40
impl crate::Resettable for VCO_CALIB_OUTrs {
    const RESET_VALUE: u32 = 0xff40;
}
