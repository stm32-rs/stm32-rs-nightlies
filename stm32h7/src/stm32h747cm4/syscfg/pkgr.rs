#[doc = "Register `PKGR` reader"]
pub type R = crate::R<PKGRrs>;
#[doc = "Field `PKG` reader - Package"]
pub type PKG_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Package"]
    #[inline(always)]
    pub fn pkg(&self) -> PKG_R {
        PKG_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "SYSCFG package register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pkgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PKGRrs;
impl crate::RegisterSpec for PKGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkgr::R`](R) reader structure"]
impl crate::Readable for PKGRrs {}
#[doc = "`reset()` method sets PKGR to value 0"]
impl crate::Resettable for PKGRrs {
    const RESET_VALUE: u32 = 0;
}
