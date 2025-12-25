///Register `RF_INFO_OUT` reader
pub type R = crate::R<RF_INFO_OUTrs>;
///Field `FQCY_BAND_ID` reader - FQCY_BAND_ID\[3:0\]: Indicates the version of the RFSUBG IP embedded in the device
pub type FQCY_BAND_ID_R = crate::FieldReader;
///Field `RFSUBG_ID` reader - Indicate the version of the analog RFSUBG IP embedded in the device
pub type RFSUBG_ID_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - FQCY_BAND_ID\[3:0\]: Indicates the version of the RFSUBG IP embedded in the device
    #[inline(always)]
    pub fn fqcy_band_id(&self) -> FQCY_BAND_ID_R {
        FQCY_BAND_ID_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Indicate the version of the analog RFSUBG IP embedded in the device
    #[inline(always)]
    pub fn rfsubg_id(&self) -> RFSUBG_ID_R {
        RFSUBG_ID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_INFO_OUT")
            .field("fqcy_band_id", &self.fqcy_band_id())
            .field("rfsubg_id", &self.rfsubg_id())
            .finish()
    }
}
/**RF_INFO_OUT register

You can [`read`](crate::Reg::read) this register and get [`rf_info_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_INFO_OUT)*/
pub struct RF_INFO_OUTrs;
impl crate::RegisterSpec for RF_INFO_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`rf_info_out::R`](R) reader structure
impl crate::Readable for RF_INFO_OUTrs {}
///`reset()` method sets RF_INFO_OUT to value 0x40
impl crate::Resettable for RF_INFO_OUTrs {
    const RESET_VALUE: u32 = 0x40;
}
