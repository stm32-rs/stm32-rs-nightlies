#[doc = "Register `CCVR` reader"]
pub type R = crate::R<CCVRrs>;
#[doc = "Field `NCV1` reader - NCV1"]
pub type NCV1_R = crate::FieldReader;
#[doc = "Field `PCV1` reader - PCV1"]
pub type PCV1_R = crate::FieldReader;
#[doc = "Field `NCV2` reader - NCV2"]
pub type NCV2_R = crate::FieldReader;
#[doc = "Field `PCV2` reader - PCV2"]
pub type PCV2_R = crate::FieldReader;
#[doc = "Field `NCV3` reader - NCV3"]
pub type NCV3_R = crate::FieldReader;
#[doc = "Field `PCV3` reader - PCV3"]
pub type PCV3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - NCV1"]
    #[inline(always)]
    pub fn ncv1(&self) -> NCV1_R {
        NCV1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PCV1"]
    #[inline(always)]
    pub fn pcv1(&self) -> PCV1_R {
        PCV1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - NCV2"]
    #[inline(always)]
    pub fn ncv2(&self) -> NCV2_R {
        NCV2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PCV2"]
    #[inline(always)]
    pub fn pcv2(&self) -> PCV2_R {
        PCV2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - NCV3"]
    #[inline(always)]
    pub fn ncv3(&self) -> NCV3_R {
        NCV3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PCV3"]
    #[inline(always)]
    pub fn pcv3(&self) -> PCV3_R {
        PCV3_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "compensation cell value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccvr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
