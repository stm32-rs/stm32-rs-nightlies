#[doc = "Reader of register HWCFGR"]
pub type R = crate::R<u32, super::HWCFGR>;
#[doc = "Reader of field `CHANNELS`"]
pub type CHANNELS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of channels per CPU supported by the IP, range 1 to 16"]
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new((self.bits & 0xff) as u8)
    }
}
