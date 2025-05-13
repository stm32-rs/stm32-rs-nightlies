///Register `CNTSR` reader
pub type R = crate::R<CNTSRrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `HLTDBG` reader - HLTDBG
pub type HLTDBG_R = crate::BitReader;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HLTDBG
    #[inline(always)]
    pub fn hltdbg(&self) -> HLTDBG_R {
        HLTDBG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTSR")
            .field("en", &self.en())
            .field("hltdbg", &self.hltdbg())
            .finish()
    }
}
/**STGENC status register

You can [`read`](crate::Reg::read) this register and get [`cntsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:CNTSR)*/
pub struct CNTSRrs;
impl crate::RegisterSpec for CNTSRrs {
    type Ux = u32;
}
///`read()` method returns [`cntsr::R`](R) reader structure
impl crate::Readable for CNTSRrs {}
///`reset()` method sets CNTSR to value 0
impl crate::Resettable for CNTSRrs {}
