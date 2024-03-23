#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `AP_PRESENT` reader - AP_PRESENT"]
pub type AP_PRESENT_R = crate::FieldReader;
#[doc = "Field `AP_LOCKED` reader - AP_LOCKED"]
pub type AP_LOCKED_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - AP_PRESENT"]
    #[inline(always)]
    pub fn ap_present(&self) -> AP_PRESENT_R {
        AP_PRESENT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AP_LOCKED"]
    #[inline(always)]
    pub fn ap_locked(&self) -> AP_LOCKED_R {
        AP_LOCKED_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "DBGMCU status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
