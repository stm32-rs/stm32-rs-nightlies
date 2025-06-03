///Register `FAR` reader
pub type R = crate::R<FARrs>;
///Field `FAIL_ECC_ADDR` reader - Bank 1 ECC error address
pub type FAIL_ECC_ADDR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:14 - Bank 1 ECC error address
    #[inline(always)]
    pub fn fail_ecc_addr(&self) -> FAIL_ECC_ADDR_R {
        FAIL_ECC_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FAR")
            .field("fail_ecc_addr", &self.fail_ecc_addr())
            .finish()
    }
}
/**FLASH ECC fail address for bank 1

You can [`read`](crate::Reg::read) this register and get [`far::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FARrs;
impl crate::RegisterSpec for FARrs {
    type Ux = u32;
}
///`read()` method returns [`far::R`](R) reader structure
impl crate::Readable for FARrs {}
///`reset()` method sets FAR to value 0
impl crate::Resettable for FARrs {}
