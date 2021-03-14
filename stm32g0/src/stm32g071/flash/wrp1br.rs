#[doc = "Reader of register WRP1BR"]
pub type R = crate::R<u32, super::WRP1BR>;
#[doc = "Reader of field `WRP1B_STRT`"]
pub type WRP1B_STRT_R = crate::R<u8, u8>;
#[doc = "Reader of field `WRP1B_END`"]
pub type WRP1B_END_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - WRP area B start offset"]
    #[inline(always)]
    pub fn wrp1b_strt(&self) -> WRP1B_STRT_R {
        WRP1B_STRT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - WRP area B end offset"]
    #[inline(always)]
    pub fn wrp1b_end(&self) -> WRP1B_END_R {
        WRP1B_END_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
