#[doc = "Register `FAR` reader"]
pub type R = crate::R<FARrs>;
#[doc = "Field `FAIL_ECC_ADDR` reader - Bank 1 ECC error address"]
pub type FAIL_ECC_ADDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    pub fn fail_ecc_addr(&self) -> FAIL_ECC_ADDR_R {
        FAIL_ECC_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "FLASH ECC fail address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`far::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FARrs;
impl crate::RegisterSpec for FARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`far::R`](R) reader structure"]
impl crate::Readable for FARrs {}
#[doc = "`reset()` method sets FAR to value 0"]
impl crate::Resettable for FARrs {
    const RESET_VALUE: u32 = 0;
}
