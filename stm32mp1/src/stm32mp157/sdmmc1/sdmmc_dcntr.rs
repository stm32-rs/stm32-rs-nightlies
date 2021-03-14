#[doc = "Reader of register SDMMC_DCNTR"]
pub type R = crate::R<u32, super::SDMMC_DCNTR>;
#[doc = "Reader of field `DATACOUNT`"]
pub type DATACOUNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - DATACOUNT"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
