#[doc = "Register `CCVR` reader"]
pub type R = crate::R<CCVRrs>;
#[doc = "Field `NCV` reader - NMOS compensation value"]
pub type NCV_R = crate::FieldReader;
#[doc = "Field `PCV` reader - PMOS compensation value"]
pub type PCV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - NMOS compensation value"]
    #[inline(always)]
    pub fn ncv(&self) -> NCV_R {
        NCV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PMOS compensation value"]
    #[inline(always)]
    pub fn pcv(&self) -> PCV_R {
        PCV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SYSCFG compensation cell value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccvr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCVRrs;
impl crate::RegisterSpec for CCVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccvr::R`](R) reader structure"]
impl crate::Readable for CCVRrs {}
#[doc = "`reset()` method sets CCVR to value 0"]
impl crate::Resettable for CCVRrs {
    const RESET_VALUE: u32 = 0;
}
