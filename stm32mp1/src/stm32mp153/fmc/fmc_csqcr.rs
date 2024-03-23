#[doc = "Register `FMC_CSQCR` writer"]
pub type W = crate::W<FMC_CSQCRrs>;
#[doc = "Field `CSQSTART` writer - CSQSTART"]
pub type CSQSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CSQSTART"]
    #[inline(always)]
    #[must_use]
    pub fn csqstart(&mut self) -> CSQSTART_W<FMC_CSQCRrs> {
        CSQSTART_W::new(self, 0)
    }
}
#[doc = "FMC NAND Command Sequencer Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_CSQCRrs;
impl crate::RegisterSpec for FMC_CSQCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fmc_csqcr::W`](W) writer structure"]
impl crate::Writable for FMC_CSQCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_CSQCR to value 0"]
impl crate::Resettable for FMC_CSQCRrs {
    const RESET_VALUE: u32 = 0;
}
