#[doc = "Reader of register FDCAN_TTCTC"]
pub type R = crate::R<u32, super::FDCAN_TTCTC>;
#[doc = "Reader of field `CT`"]
pub type CT_R = crate::R<u16, u16>;
#[doc = "Reader of field `CC`"]
pub type CC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - CT"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - CC"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
