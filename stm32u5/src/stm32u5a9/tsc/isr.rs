#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `EOAF` reader - End of acquisition flag"]
pub type EOAF_R = crate::BitReader;
#[doc = "Field `MCEF` reader - Max count error flag"]
pub type MCEF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of acquisition flag"]
    #[inline(always)]
    pub fn eoaf(&self) -> EOAF_R {
        EOAF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error flag"]
    #[inline(always)]
    pub fn mcef(&self) -> MCEF_R {
        MCEF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
