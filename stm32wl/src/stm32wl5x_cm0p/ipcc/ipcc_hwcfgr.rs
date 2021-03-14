#[doc = "Reader of register IPCC_HWCFGR"]
pub type R = crate::R<u32, super::IPCC_HWCFGR>;
#[doc = "Reader of field `CHANNELS`"]
pub type CHANNELS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CHANNELS"]
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new((self.bits & 0xff) as u8)
    }
}
