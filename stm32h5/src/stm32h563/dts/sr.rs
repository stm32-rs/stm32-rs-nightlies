#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `TS1_ITEF` reader - Interrupt flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS2_CITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITEFEN bit is set"]
pub type TS1_ITEF_R = crate::BitReader;
#[doc = "Field `TS1_ITLF` reader - Interrupt flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when the low threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITLFEN bit is set"]
pub type TS1_ITLF_R = crate::BitReader;
#[doc = "Field `TS1_ITHF` reader - Interrupt flag for high threshold on temperature sensor 1, synchronized on PCLK This bit is set by hardware when the high threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITHFEN bit is set"]
pub type TS1_ITHF_R = crate::BitReader;
#[doc = "Field `TS1_AITEF` reader - Asynchronous interrupt flag for end of measure on temperature sensor 1 This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS1_CAITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITEFEN bit is set"]
pub type TS1_AITEF_R = crate::BitReader;
#[doc = "Field `TS1_AITLF` reader - Asynchronous interrupt flag for low threshold on temperature sensor 1 This bit is set by hardware when the low threshold is reached. It is cleared by software by writing 1 to the TS1_CAITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITLFEN bit is set"]
pub type TS1_AITLF_R = crate::BitReader;
#[doc = "Field `TS1_AITHF` reader - Asynchronous interrupt flag for high threshold on temperature sensor 1 This bit is set by hardware when the high threshold is reached. It is cleared by software by writing 1 to the TS1_CAITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITHFEN bit is set"]
pub type TS1_AITHF_R = crate::BitReader;
#[doc = "Field `TS1_RDY` reader - Temperature sensor 1 ready flag This bit is set and reset by hardware. It indicates that a measurement is ongoing."]
pub type TS1_RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt flag for end of measurement on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS2_CITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITEFEN bit is set"]
    #[inline(always)]
    pub fn ts1_itef(&self) -> TS1_ITEF_R {
        TS1_ITEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for low threshold on temperature sensor 1, synchronized on PCLK. This bit is set by hardware when the low threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITLFEN bit is set"]
    #[inline(always)]
    pub fn ts1_itlf(&self) -> TS1_ITLF_R {
        TS1_ITLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for high threshold on temperature sensor 1, synchronized on PCLK This bit is set by hardware when the high threshold is set and reached. It is cleared by software by writing 1 to the TS1_CITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_ITHFEN bit is set"]
    #[inline(always)]
    pub fn ts1_ithf(&self) -> TS1_ITHF_R {
        TS1_ITHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous interrupt flag for end of measure on temperature sensor 1 This bit is set by hardware when a temperature measure is done. It is cleared by software by writing 1 to the TS1_CAITEF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITEFEN bit is set"]
    #[inline(always)]
    pub fn ts1_aitef(&self) -> TS1_AITEF_R {
        TS1_AITEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous interrupt flag for low threshold on temperature sensor 1 This bit is set by hardware when the low threshold is reached. It is cleared by software by writing 1 to the TS1_CAITLF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITLFEN bit is set"]
    #[inline(always)]
    pub fn ts1_aitlf(&self) -> TS1_AITLF_R {
        TS1_AITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Asynchronous interrupt flag for high threshold on temperature sensor 1 This bit is set by hardware when the high threshold is reached. It is cleared by software by writing 1 to the TS1_CAITHF bit in the DTS_ICIFR register. Note: This bit is active only when the TS1_AITHFEN bit is set"]
    #[inline(always)]
    pub fn ts1_aithf(&self) -> TS1_AITHF_R {
        TS1_AITHF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 15 - Temperature sensor 1 ready flag This bit is set and reset by hardware. It indicates that a measurement is ongoing."]
    #[inline(always)]
    pub fn ts1_rdy(&self) -> TS1_RDY_R {
        TS1_RDY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Temperature sensor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
