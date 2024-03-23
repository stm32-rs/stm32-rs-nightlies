#[doc = "Register `LCVCIDR` reader"]
pub type R = crate::R<LCVCIDRrs>;
#[doc = "Field `VCID` reader - Virtual Channel ID"]
pub type VCID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Virtual Channel ID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
#[doc = "DSI Host LTDC Current VCID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcvcidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCVCIDRrs;
impl crate::RegisterSpec for LCVCIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcvcidr::R`](R) reader structure"]
impl crate::Readable for LCVCIDRrs {}
#[doc = "`reset()` method sets LCVCIDR to value 0"]
impl crate::Resettable for LCVCIDRrs {
    const RESET_VALUE: u32 = 0;
}
