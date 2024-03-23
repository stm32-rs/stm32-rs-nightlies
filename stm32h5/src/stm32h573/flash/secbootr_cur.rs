#[doc = "Register `SECBOOTR_CUR` reader"]
pub type R = crate::R<SECBOOTR_CURrs>;
#[doc = "Field `SECBOOT_LOCK` reader - A field locking the values of UBE, SWAP_BANK, and SECBOOTADD settings."]
pub type SECBOOT_LOCK_R = crate::FieldReader;
#[doc = "Field `SECBOOTADD` reader - Unique boot entry secure address These bits reflect the Secure UBE address"]
pub type SECBOOTADD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - A field locking the values of UBE, SWAP_BANK, and SECBOOTADD settings."]
    #[inline(always)]
    pub fn secboot_lock(&self) -> SECBOOT_LOCK_R {
        SECBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Unique boot entry secure address These bits reflect the Secure UBE address"]
    #[inline(always)]
    pub fn secbootadd(&self) -> SECBOOTADD_R {
        SECBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "FLASH secure boot register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secbootr_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECBOOTR_CURrs;
impl crate::RegisterSpec for SECBOOTR_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secbootr_cur::R`](R) reader structure"]
impl crate::Readable for SECBOOTR_CURrs {}
#[doc = "`reset()` method sets SECBOOTR_CUR to value 0"]
impl crate::Resettable for SECBOOTR_CURrs {
    const RESET_VALUE: u32 = 0;
}
