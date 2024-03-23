#[doc = "Register `GICV_RPR` reader"]
pub type R = crate::R<GICV_RPRrs>;
#[doc = "Field `PRIORITY` reader - PRIORITY"]
pub type PRIORITY_R = crate::FieldReader;
impl R {
    #[doc = "Bits 3:7 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
#[doc = "GICV VM running priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicv_rpr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICV_RPRrs;
impl crate::RegisterSpec for GICV_RPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicv_rpr::R`](R) reader structure"]
impl crate::Readable for GICV_RPRrs {}
#[doc = "`reset()` method sets GICV_RPR to value 0xff"]
impl crate::Resettable for GICV_RPRrs {
    const RESET_VALUE: u32 = 0xff;
}
