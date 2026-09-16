///Register `CRCDATAR` reader
pub type R = crate::R<CRCDATARrs>;
///Field `CRC_DATA` reader - CRC result This bitfield contains the result of the last CRC calculation. The value is valid only when CRC calculation completed (CRCENDF is set in FLASH_ISR register). CRC_DATA is cleared when CRC_EN is cleared in FLASH_CR register (CRC disabled), or when CLEAN_CRC bit is set in FLASH_CRCCR register.
pub type CRC_DATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CRC result This bitfield contains the result of the last CRC calculation. The value is valid only when CRC calculation completed (CRCENDF is set in FLASH_ISR register). CRC_DATA is cleared when CRC_EN is cleared in FLASH_CR register (CRC disabled), or when CLEAN_CRC bit is set in FLASH_CRCCR register.
    #[inline(always)]
    pub fn crc_data(&self) -> CRC_DATA_R {
        CRC_DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCDATAR")
            .field("crc_data", &self.crc_data())
            .finish()
    }
}
/**FLASH CRC data register

You can [`read`](crate::Reg::read) this register and get [`crcdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:CRCDATAR)*/
pub struct CRCDATARrs;
impl crate::RegisterSpec for CRCDATARrs {
    type Ux = u32;
}
///`read()` method returns [`crcdatar::R`](R) reader structure
impl crate::Readable for CRCDATARrs {}
///`reset()` method sets CRCDATAR to value 0
impl crate::Resettable for CRCDATARrs {}
