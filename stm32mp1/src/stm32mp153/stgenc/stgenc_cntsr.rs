///Register `STGENC_CNTSR` reader
pub type R = crate::R<STGENC_CNTSRrs>;
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
        f.debug_struct("STGENC_CNTSR")
            .field("en", &self.en())
            .field("hltdbg", &self.hltdbg())
            .finish()
    }
}
/**STGENC status register

You can [`read`](crate::Reg::read) this register and get [`stgenc_cntsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CNTSR)*/
pub struct STGENC_CNTSRrs;
impl crate::RegisterSpec for STGENC_CNTSRrs {
    type Ux = u32;
}
///`read()` method returns [`stgenc_cntsr::R`](R) reader structure
impl crate::Readable for STGENC_CNTSRrs {}
///`reset()` method sets STGENC_CNTSR to value 0
impl crate::Resettable for STGENC_CNTSRrs {
    const RESET_VALUE: u32 = 0;
}
