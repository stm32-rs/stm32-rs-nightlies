#[doc = "Register `PWR_VER` reader"]
pub type R = crate::R<PWR_VERrs>;
#[doc = "Field `MINREV` reader - MINREV"]
pub type MINREV_R = crate::FieldReader;
#[doc = "Field `MAJREV` reader - MAJREV"]
pub type MAJREV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - MINREV"]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MAJREV"]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "PWR IP version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_ver::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_VERrs;
impl crate::RegisterSpec for PWR_VERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_ver::R`](R) reader structure"]
impl crate::Readable for PWR_VERrs {}
#[doc = "`reset()` method sets PWR_VER to value 0x11"]
impl crate::Resettable for PWR_VERrs {
    const RESET_VALUE: u32 = 0x11;
}
