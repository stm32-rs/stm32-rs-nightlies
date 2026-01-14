///Register `ECCDFADDR` reader
pub type R = crate::R<ECCDFADDRrs>;
///Field `DED_FADD` reader - ECC double error detection fail address When a double ECC detection occurs during a read operation, the DED_FADD bitfield contains the system bus address that generated the error. This register is automatically cleared when the DBECCERRF flag that generated the error is cleared. Note that only the first address that generated an ECC double error detection error is saved in this register.
pub type DED_FADD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC double error detection fail address When a double ECC detection occurs during a read operation, the DED_FADD bitfield contains the system bus address that generated the error. This register is automatically cleared when the DBECCERRF flag that generated the error is cleared. Note that only the first address that generated an ECC double error detection error is saved in this register.
    #[inline(always)]
    pub fn ded_fadd(&self) -> DED_FADD_R {
        DED_FADD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCDFADDR")
            .field("ded_fadd", &self.ded_fadd())
            .finish()
    }
}
/**FLASH ECC double error fail address

You can [`read`](crate::Reg::read) this register and get [`eccdfaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:ECCDFADDR)*/
pub struct ECCDFADDRrs;
impl crate::RegisterSpec for ECCDFADDRrs {
    type Ux = u32;
}
///`read()` method returns [`eccdfaddr::R`](R) reader structure
impl crate::Readable for ECCDFADDRrs {}
///`reset()` method sets ECCDFADDR to value 0
impl crate::Resettable for ECCDFADDRrs {}
