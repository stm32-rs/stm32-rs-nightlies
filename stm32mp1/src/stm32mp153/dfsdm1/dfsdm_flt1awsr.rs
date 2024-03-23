#[doc = "Register `DFSDM_FLT1AWSR` reader"]
pub type R = crate::R<DFSDM_FLT1AWSRrs>;
#[doc = "Field `AWLTF` reader - AWLTF"]
pub type AWLTF_R = crate::FieldReader;
#[doc = "Field `AWHTF` reader - AWHTF"]
pub type AWHTF_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - AWLTF"]
    #[inline(always)]
    pub fn awltf(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AWHTF"]
    #[inline(always)]
    pub fn awhtf(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "DFSDM filter 1 analog watchdog status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt1awsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT1AWSRrs;
impl crate::RegisterSpec for DFSDM_FLT1AWSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt1awsr::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT1AWSRrs {}
#[doc = "`reset()` method sets DFSDM_FLT1AWSR to value 0"]
impl crate::Resettable for DFSDM_FLT1AWSRrs {
    const RESET_VALUE: u32 = 0;
}
