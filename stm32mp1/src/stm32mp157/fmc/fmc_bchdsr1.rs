#[doc = "Register `FMC_BCHDSR1` reader"]
pub type R = crate::R<FMC_BCHDSR1rs>;
#[doc = "Field `EBP1` reader - EBP1"]
pub type EBP1_R = crate::FieldReader<u16>;
#[doc = "Field `EBP2` reader - EBP2"]
pub type EBP2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - EBP1"]
    #[inline(always)]
    pub fn ebp1(&self) -> EBP1_R {
        EBP1_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EBP2"]
    #[inline(always)]
    pub fn ebp2(&self) -> EBP2_R {
        EBP2_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchdsr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_BCHDSR1rs;
impl crate::RegisterSpec for FMC_BCHDSR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_bchdsr1::R`](R) reader structure"]
impl crate::Readable for FMC_BCHDSR1rs {}
#[doc = "`reset()` method sets FMC_BCHDSR1 to value 0"]
impl crate::Resettable for FMC_BCHDSR1rs {
    const RESET_VALUE: u32 = 0;
}
