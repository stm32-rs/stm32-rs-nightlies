#[doc = "Register `GVCIDR` reader"]
pub type R = crate::R<GVCIDRrs>;
#[doc = "Field `VCID` reader - VCID"]
pub type VCID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - VCID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
#[doc = "DSI Host generic VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gvcidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GVCIDRrs;
impl crate::RegisterSpec for GVCIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gvcidr::R`](R) reader structure"]
impl crate::Readable for GVCIDRrs {}
#[doc = "`reset()` method sets GVCIDR to value 0"]
impl crate::Resettable for GVCIDRrs {
    const RESET_VALUE: u32 = 0;
}
