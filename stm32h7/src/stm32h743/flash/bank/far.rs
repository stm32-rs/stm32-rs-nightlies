#[doc = "Reader of register FAR"]
pub type R = crate::R<u32, super::FAR>;
#[doc = "Reader of field `FAIL_ECC_ADDR`"]
pub type FAIL_ECC_ADDR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    pub fn fail_ecc_addr(&self) -> FAIL_ECC_ADDR_R {
        FAIL_ECC_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
