#[doc = "Register `FMC_BCHDSR4` reader"]
pub type R = crate::R<FMC_BCHDSR4rs>;
#[doc = "Field `EBP7` reader - EBP7"]
pub type EBP7_R = crate::FieldReader<u16>;
#[doc = "Field `EBP8` reader - EBP8"]
pub type EBP8_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - EBP7"]
    #[inline(always)]
    pub fn ebp7(&self) -> EBP7_R {
        EBP7_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EBP8"]
    #[inline(always)]
    pub fn ebp8(&self) -> EBP8_R {
        EBP8_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchdsr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_BCHDSR4rs;
impl crate::RegisterSpec for FMC_BCHDSR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_bchdsr4::R`](R) reader structure"]
impl crate::Readable for FMC_BCHDSR4rs {}
#[doc = "`reset()` method sets FMC_BCHDSR4 to value 0"]
impl crate::Resettable for FMC_BCHDSR4rs {
    const RESET_VALUE: u32 = 0;
}
