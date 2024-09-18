///Register `DAC_SIDR` reader
pub type R = crate::R<DAC_SIDRrs>;
///Field `SID` reader - SID
pub type SID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SID
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_SIDR")
            .field("sid", &self.sid())
            .finish()
    }
}
/**No

You can [`read`](crate::Reg::read) this register and get [`dac_sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:DAC_SIDR)*/
pub struct DAC_SIDRrs;
impl crate::RegisterSpec for DAC_SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`dac_sidr::R`](R) reader structure
impl crate::Readable for DAC_SIDRrs {}
///`reset()` method sets DAC_SIDR to value 0xa3c5_dd01
impl crate::Resettable for DAC_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
