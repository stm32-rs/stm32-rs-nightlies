///Register `FMC_CSQEMSR` reader
pub type R = crate::R<FMC_CSQEMSRrs>;
///Field `SEM` reader - SEM
pub type SEM_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - SEM
    #[inline(always)]
    pub fn sem(&self) -> SEM_R {
        SEM_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC_CSQEMSR")
            .field("sem", &self.sem())
            .finish()
    }
}
/**This register holds a sector error mapping status when the whole transfer is complete.

You can [`read`](crate::Reg::read) this register and get [`fmc_csqemsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:FMC_CSQEMSR)*/
pub struct FMC_CSQEMSRrs;
impl crate::RegisterSpec for FMC_CSQEMSRrs {
    type Ux = u32;
}
///`read()` method returns [`fmc_csqemsr::R`](R) reader structure
impl crate::Readable for FMC_CSQEMSRrs {}
///`reset()` method sets FMC_CSQEMSR to value 0
impl crate::Resettable for FMC_CSQEMSRrs {
    const RESET_VALUE: u32 = 0;
}
