#[doc = "Reader of register WRP1AR"]
pub type R = crate::R<u32, super::WRP1AR>;
#[doc = "Reader of field `WRP1A_STRT`"]
pub type WRP1A_STRT_R = crate::R<u8, u8>;
#[doc = "Reader of field `WRP1A_END`"]
pub type WRP1A_END_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - WRP area A start offset"]
    #[inline(always)]
    pub fn wrp1a_strt(&self) -> WRP1A_STRT_R {
        WRP1A_STRT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - WRP area A end offset"]
    #[inline(always)]
    pub fn wrp1a_end(&self) -> WRP1A_END_R {
        WRP1A_END_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
