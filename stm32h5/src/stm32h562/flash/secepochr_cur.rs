///Register `SECEPOCHR_CUR` reader
pub type R = crate::R<SECEPOCHR_CURrs>;
///Field `SEC_EPOCH` reader - Non-volatile secure EPOCH counter
pub type SEC_EPOCH_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - Non-volatile secure EPOCH counter
    #[inline(always)]
    pub fn sec_epoch(&self) -> SEC_EPOCH_R {
        SEC_EPOCH_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECEPOCHR_CUR")
            .field("sec_epoch", &self.sec_epoch())
            .finish()
    }
}
/**FLASH secure EPOCH register

You can [`read`](crate::Reg::read) this register and get [`secepochr_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FLASH:SECEPOCHR_CUR)*/
pub struct SECEPOCHR_CURrs;
impl crate::RegisterSpec for SECEPOCHR_CURrs {
    type Ux = u32;
}
///`read()` method returns [`secepochr_cur::R`](R) reader structure
impl crate::Readable for SECEPOCHR_CURrs {}
///`reset()` method sets SECEPOCHR_CUR to value 0
impl crate::Resettable for SECEPOCHR_CURrs {}
