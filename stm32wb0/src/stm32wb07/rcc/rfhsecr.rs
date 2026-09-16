///Register `RFHSECR` reader
pub type R = crate::R<RFHSECRrs>;
///Field `XOTUNE` reader - RF-HSE capacitor bank tuning Set by option byte loading soon after Power On Reset.
pub type XOTUNE_R = crate::FieldReader;
impl R {
    ///Bits 0:5 - RF-HSE capacitor bank tuning Set by option byte loading soon after Power On Reset.
    #[inline(always)]
    pub fn xotune(&self) -> XOTUNE_R {
        XOTUNE_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFHSECR")
            .field("xotune", &self.xotune())
            .finish()
    }
}
/**RFHSECR register

You can [`read`](crate::Reg::read) this register and get [`rfhsecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:RFHSECR)*/
pub struct RFHSECRrs;
impl crate::RegisterSpec for RFHSECRrs {
    type Ux = u32;
}
///`read()` method returns [`rfhsecr::R`](R) reader structure
impl crate::Readable for RFHSECRrs {}
///`reset()` method sets RFHSECR to value 0
impl crate::Resettable for RFHSECRrs {}
