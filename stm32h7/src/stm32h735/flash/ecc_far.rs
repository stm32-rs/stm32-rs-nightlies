///Register `ECC_FAR` reader
pub type R = crate::R<ECC_FARrs>;
///Field `FAIL_ECC_ADDR` reader - ECC error address
pub type FAIL_ECC_ADDR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:14 - ECC error address
    #[inline(always)]
    pub fn fail_ecc_addr(&self) -> FAIL_ECC_ADDR_R {
        FAIL_ECC_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_FAR")
            .field("fail_ecc_addr", &self.fail_ecc_addr())
            .finish()
    }
}
/**FLASH ECC fail address for bank 1

You can [`read`](crate::Reg::read) this register and get [`ecc_far::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FLASH:ECC_FAR)*/
pub struct ECC_FARrs;
impl crate::RegisterSpec for ECC_FARrs {
    type Ux = u32;
}
///`read()` method returns [`ecc_far::R`](R) reader structure
impl crate::Readable for ECC_FARrs {}
///`reset()` method sets ECC_FAR to value 0
impl crate::Resettable for ECC_FARrs {}
