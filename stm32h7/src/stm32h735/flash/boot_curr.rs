#[doc = "Register `BOOT_CURR` reader"]
pub type R = crate::R<BOOT_CURRrs>;
#[doc = "Field `BOOT_ADD0` reader - Boot address 0"]
pub type BOOT_ADD0_R = crate::FieldReader<u16>;
#[doc = "Field `BOOT_ADD1` reader - Boot address 1"]
pub type BOOT_ADD1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Boot address 0"]
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Boot address 1"]
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "FLASH register with boot address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_curr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_CURRrs;
impl crate::RegisterSpec for BOOT_CURRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_curr::R`](R) reader structure"]
impl crate::Readable for BOOT_CURRrs {}
#[doc = "`reset()` method sets BOOT_CURR to value 0"]
impl crate::Resettable for BOOT_CURRrs {
    const RESET_VALUE: u32 = 0;
}
