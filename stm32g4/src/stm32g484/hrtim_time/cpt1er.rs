#[doc = "Register `CPT1ER` reader"]
pub type R = crate::R<CPT1ERrs>;
#[doc = "Field `CPT1x` reader - Timerx Capture 1 value"]
pub type CPT1X_R = crate::FieldReader<u16>;
#[doc = "Field `DIR` reader - Timerx Capture 1 Direction status"]
pub type DIR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 1 value"]
    #[inline(always)]
    pub fn cpt1x(&self) -> CPT1X_R {
        CPT1X_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Timerx Capture 1 Direction status"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1er::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPT1ERrs;
impl crate::RegisterSpec for CPT1ERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt1er::R`](R) reader structure"]
impl crate::Readable for CPT1ERrs {}
#[doc = "`reset()` method sets CPT1ER to value 0"]
impl crate::Resettable for CPT1ERrs {
    const RESET_VALUE: u32 = 0;
}
