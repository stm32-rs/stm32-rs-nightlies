#[doc = "Register `ECCDR` reader"]
pub type R = crate::R<ECCDRrs>;
#[doc = "Field `DATA_ECC` reader - ECC error data When an double detection ECC error occurs on special areas with 6-bit ECC on 16-bit data (data area, read-only/OTP area), the failing data is read to this register. By checking if it is possible to determine whether the failure was on a real data, or due to access to uninitialized memory."]
pub type DATA_ECC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - ECC error data When an double detection ECC error occurs on special areas with 6-bit ECC on 16-bit data (data area, read-only/OTP area), the failing data is read to this register. By checking if it is possible to determine whether the failure was on a real data, or due to access to uninitialized memory."]
    #[inline(always)]
    pub fn data_ecc(&self) -> DATA_ECC_R {
        DATA_ECC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "FLASH ECC data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCDRrs;
impl crate::RegisterSpec for ECCDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccdr::R`](R) reader structure"]
impl crate::Readable for ECCDRrs {}
#[doc = "`reset()` method sets ECCDR to value 0"]
impl crate::Resettable for ECCDRrs {
    const RESET_VALUE: u32 = 0;
}
