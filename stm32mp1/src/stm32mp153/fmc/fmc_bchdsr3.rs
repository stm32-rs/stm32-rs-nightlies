///Register `FMC_BCHDSR3` reader
pub type R = crate::R<FMC_BCHDSR3rs>;
///Field `EBP5` reader - EBP5
pub type EBP5_R = crate::FieldReader<u16>;
///Field `EBP6` reader - EBP6
pub type EBP6_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:12 - EBP5
    #[inline(always)]
    pub fn ebp5(&self) -> EBP5_R {
        EBP5_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - EBP6
    #[inline(always)]
    pub fn ebp6(&self) -> EBP6_R {
        EBP6_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC_BCHDSR3")
            .field("ebp5", &self.ebp5())
            .field("ebp6", &self.ebp6())
            .finish()
    }
}
/**The maximum error correction capability of the BCH block embedded in the FMC is 8 errors.

You can [`read`](crate::Reg::read) this register and get [`fmc_bchdsr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:FMC_BCHDSR3)*/
pub struct FMC_BCHDSR3rs;
impl crate::RegisterSpec for FMC_BCHDSR3rs {
    type Ux = u32;
}
///`read()` method returns [`fmc_bchdsr3::R`](R) reader structure
impl crate::Readable for FMC_BCHDSR3rs {}
///`reset()` method sets FMC_BCHDSR3 to value 0
impl crate::Resettable for FMC_BCHDSR3rs {
    const RESET_VALUE: u32 = 0;
}
