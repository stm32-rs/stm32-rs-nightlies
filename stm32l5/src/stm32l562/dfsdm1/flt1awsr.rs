#[doc = "Register `FLT1AWSR` reader"]
pub type R = crate::R<FLT1AWSRrs>;
#[doc = "Field `AWLTF` reader - Analog watchdog low threshold flag"]
pub type AWLTF_R = crate::FieldReader;
#[doc = "Field `AWHTF` reader - Analog watchdog high threshold flag"]
pub type AWHTF_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn awltf(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Analog watchdog high threshold flag"]
    #[inline(always)]
    pub fn awhtf(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "analog watchdog status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flt1awsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLT1AWSRrs;
impl crate::RegisterSpec for FLT1AWSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flt1awsr::R`](R) reader structure"]
impl crate::Readable for FLT1AWSRrs {}
#[doc = "`reset()` method sets FLT1AWSR to value 0"]
impl crate::Resettable for FLT1AWSRrs {
    const RESET_VALUE: u32 = 0;
}
