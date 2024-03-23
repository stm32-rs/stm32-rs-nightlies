#[doc = "Register `FMC_BCHDSR2` reader"]
pub type R = crate::R<FMC_BCHDSR2rs>;
#[doc = "Field `EBP3` reader - EBP3"]
pub type EBP3_R = crate::FieldReader<u16>;
#[doc = "Field `EBP4` reader - EBP4"]
pub type EBP4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - EBP3"]
    #[inline(always)]
    pub fn ebp3(&self) -> EBP3_R {
        EBP3_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EBP4"]
    #[inline(always)]
    pub fn ebp4(&self) -> EBP4_R {
        EBP4_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchdsr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_BCHDSR2rs;
impl crate::RegisterSpec for FMC_BCHDSR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_bchdsr2::R`](R) reader structure"]
impl crate::Readable for FMC_BCHDSR2rs {}
#[doc = "`reset()` method sets FMC_BCHDSR2 to value 0"]
impl crate::Resettable for FMC_BCHDSR2rs {
    const RESET_VALUE: u32 = 0;
}
