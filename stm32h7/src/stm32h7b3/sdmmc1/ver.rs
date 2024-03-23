#[doc = "Register `VER` reader"]
pub type R = crate::R<VERrs>;
#[doc = "Field `MINREV` reader - IP minor revision number."]
pub type MINREV_R = crate::FieldReader;
#[doc = "Field `MAJREV` reader - IP major revision number."]
pub type MAJREV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - IP minor revision number."]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - IP major revision number."]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SDMMC IP version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
