#[doc = "Register `VER` reader"]
pub type R = crate::R<VERrs>;
#[doc = "Field `VER` reader - Version"]
pub type VER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Version"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERrs;
impl crate::RegisterSpec for VERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver::R`](R) reader structure"]
impl crate::Readable for VERrs {}
#[doc = "`reset()` method sets VER to value 0x10"]
impl crate::Resettable for VERrs {
    const RESET_VALUE: u32 = 0x10;
}
