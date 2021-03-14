#[doc = "Reader of register SDMMC_RESPCMDR"]
pub type R = crate::R<u32, super::SDMMC_RESPCMDR>;
#[doc = "Reader of field `RESPCMD`"]
pub type RESPCMD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - RESPCMD"]
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
