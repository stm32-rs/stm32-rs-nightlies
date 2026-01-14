///Register `QI_INFO` reader
pub type R = crate::R<QI_INFOrs>;
///Field `PQI_INFO` reader - Preamble Quality Indicator (PQI) value of the received packet.
pub type PQI_INFO_R = crate::FieldReader;
///Field `SQI_INFO` reader - SYNC Quality Indicator (SQI) value of the received packet.
pub type SQI_INFO_R = crate::FieldReader;
///Field `SQI_SEC` reader - Indicate if measured SQI refers to SYNC word or secondary SYNC word
pub type SQI_SEC_R = crate::BitReader;
///Field `AFC_CORRECTION` reader - AFC value frozen at sync reception.
pub type AFC_CORRECTION_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Preamble Quality Indicator (PQI) value of the received packet.
    #[inline(always)]
    pub fn pqi_info(&self) -> PQI_INFO_R {
        PQI_INFO_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:13 - SYNC Quality Indicator (SQI) value of the received packet.
    #[inline(always)]
    pub fn sqi_info(&self) -> SQI_INFO_R {
        SQI_INFO_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 14 - Indicate if measured SQI refers to SYNC word or secondary SYNC word
    #[inline(always)]
    pub fn sqi_sec(&self) -> SQI_SEC_R {
        SQI_SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:23 - AFC value frozen at sync reception.
    #[inline(always)]
    pub fn afc_correction(&self) -> AFC_CORRECTION_R {
        AFC_CORRECTION_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QI_INFO")
            .field("pqi_info", &self.pqi_info())
            .field("sqi_info", &self.sqi_info())
            .field("sqi_sec", &self.sqi_sec())
            .field("afc_correction", &self.afc_correction())
            .finish()
    }
}
/**QI_INFO register

You can [`read`](crate::Reg::read) this register and get [`qi_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:QI_INFO)*/
pub struct QI_INFOrs;
impl crate::RegisterSpec for QI_INFOrs {
    type Ux = u32;
}
///`read()` method returns [`qi_info::R`](R) reader structure
impl crate::Readable for QI_INFOrs {}
///`reset()` method sets QI_INFO to value 0
impl crate::Resettable for QI_INFOrs {}
