#[doc = "Register `STGENC_CNTSR` reader"]
pub type R = crate::R<STGENC_CNTSRrs>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `HLTDBG` reader - HLTDBG"]
pub type HLTDBG_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HLTDBG"]
    #[inline(always)]
    pub fn hltdbg(&self) -> HLTDBG_R {
        HLTDBG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "STGENC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenc_cntsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENC_CNTSRrs;
impl crate::RegisterSpec for STGENC_CNTSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenc_cntsr::R`](R) reader structure"]
impl crate::Readable for STGENC_CNTSRrs {}
#[doc = "`reset()` method sets STGENC_CNTSR to value 0"]
impl crate::Resettable for STGENC_CNTSRrs {
    const RESET_VALUE: u32 = 0;
}
