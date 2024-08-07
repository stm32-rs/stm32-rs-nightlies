///Register `FMC_BCHDSR2` reader
pub type R = crate::R<FMC_BCHDSR2rs>;
///Field `EBP3` reader - EBP3
pub type EBP3_R = crate::FieldReader<u16>;
///Field `EBP4` reader - EBP4
pub type EBP4_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:12 - EBP3
    #[inline(always)]
    pub fn ebp3(&self) -> EBP3_R {
        EBP3_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - EBP4
    #[inline(always)]
    pub fn ebp4(&self) -> EBP4_R {
        EBP4_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC_BCHDSR2")
            .field("ebp3", &self.ebp3())
            .field("ebp4", &self.ebp4())
            .finish()
    }
}
/**The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 3rd and 4th error bits in EBP3 and EPB4 fields, respectively.

You can [`read`](crate::Reg::read) this register and get [`fmc_bchdsr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:FMC_BCHDSR2)*/
pub struct FMC_BCHDSR2rs;
impl crate::RegisterSpec for FMC_BCHDSR2rs {
    type Ux = u32;
}
///`read()` method returns [`fmc_bchdsr2::R`](R) reader structure
impl crate::Readable for FMC_BCHDSR2rs {}
///`reset()` method sets FMC_BCHDSR2 to value 0
impl crate::Resettable for FMC_BCHDSR2rs {
    const RESET_VALUE: u32 = 0;
}
