#[doc = "Register `DDRPERFM_ISR` reader"]
pub type R = crate::R<DDRPERFM_ISRrs>;
#[doc = "Field `OVFF` reader - OVFF"]
pub type OVFF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - OVFF"]
    #[inline(always)]
    pub fn ovff(&self) -> OVFF_R {
        OVFF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DDRPERFM interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPERFM_ISRrs;
impl crate::RegisterSpec for DDRPERFM_ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrperfm_isr::R`](R) reader structure"]
impl crate::Readable for DDRPERFM_ISRrs {}
#[doc = "`reset()` method sets DDRPERFM_ISR to value 0"]
impl crate::Resettable for DDRPERFM_ISRrs {
    const RESET_VALUE: u32 = 0;
}
