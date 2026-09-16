///Register `ECCSFADDR` reader
pub type R = crate::R<ECCSFADDRrs>;
///Field `SEC_FADD` reader - ECC single error correction fail address When a single ECC error correction occurs during a read operation, the SEC_FADD bitfield contains the system bus address that generated the error. This register is automatically cleared when SNECCERRF flag that generated the error is cleared. Note that only the first address that generated an ECC single error correction error is saved in this register.
pub type SEC_FADD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC single error correction fail address When a single ECC error correction occurs during a read operation, the SEC_FADD bitfield contains the system bus address that generated the error. This register is automatically cleared when SNECCERRF flag that generated the error is cleared. Note that only the first address that generated an ECC single error correction error is saved in this register.
    #[inline(always)]
    pub fn sec_fadd(&self) -> SEC_FADD_R {
        SEC_FADD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCSFADDR")
            .field("sec_fadd", &self.sec_fadd())
            .finish()
    }
}
/**FLASH ECC single error fail address

You can [`read`](crate::Reg::read) this register and get [`eccsfaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:ECCSFADDR)*/
pub struct ECCSFADDRrs;
impl crate::RegisterSpec for ECCSFADDRrs {
    type Ux = u32;
}
///`read()` method returns [`eccsfaddr::R`](R) reader structure
impl crate::Readable for ECCSFADDRrs {}
///`reset()` method sets ECCSFADDR to value 0
impl crate::Resettable for ECCSFADDRrs {}
