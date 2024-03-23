#[doc = "Register `NSBOOTR_CUR` reader"]
pub type R = crate::R<NSBOOTR_CURrs>;
#[doc = "Field `NSBOOT_LOCK` reader - A field locking the values of SWAP_BANK, and NSBOOTADD settings."]
pub type NSBOOT_LOCK_R = crate::FieldReader;
#[doc = "Field `NSBOOTADD` reader - Non secure unique boot entry address These bits reflect the Non secure UBE address"]
pub type NSBOOTADD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - A field locking the values of SWAP_BANK, and NSBOOTADD settings."]
    #[inline(always)]
    pub fn nsboot_lock(&self) -> NSBOOT_LOCK_R {
        NSBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Non secure unique boot entry address These bits reflect the Non secure UBE address"]
    #[inline(always)]
    pub fn nsbootadd(&self) -> NSBOOTADD_R {
        NSBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "FLASH non-secure boot register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsbootr_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSBOOTR_CURrs;
impl crate::RegisterSpec for NSBOOTR_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsbootr_cur::R`](R) reader structure"]
impl crate::Readable for NSBOOTR_CURrs {}
#[doc = "`reset()` method sets NSBOOTR_CUR to value 0"]
impl crate::Resettable for NSBOOTR_CURrs {
    const RESET_VALUE: u32 = 0;
}
