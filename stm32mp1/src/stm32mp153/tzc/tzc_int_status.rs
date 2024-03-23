#[doc = "Register `TZC_INT_STATUS` reader"]
pub type R = crate::R<TZC_INT_STATUSrs>;
#[doc = "Field `STATUS` reader - STATUS"]
pub type STATUS_R = crate::FieldReader;
#[doc = "Field `OVERRUN` reader - OVERRUN"]
pub type OVERRUN_R = crate::FieldReader;
#[doc = "Field `OVERLAP` reader - OVERLAP"]
pub type OVERLAP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - STATUS"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - OVERRUN"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - OVERLAP"]
    #[inline(always)]
    pub fn overlap(&self) -> OVERLAP_R {
        OVERLAP_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_int_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_INT_STATUSrs;
impl crate::RegisterSpec for TZC_INT_STATUSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_int_status::R`](R) reader structure"]
impl crate::Readable for TZC_INT_STATUSrs {}
#[doc = "`reset()` method sets TZC_INT_STATUS to value 0"]
impl crate::Resettable for TZC_INT_STATUSrs {
    const RESET_VALUE: u32 = 0;
}
