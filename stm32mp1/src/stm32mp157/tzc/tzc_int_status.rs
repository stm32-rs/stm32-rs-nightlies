#[doc = "Reader of register TZC_INT_STATUS"]
pub type R = crate::R<u32, super::TZC_INT_STATUS>;
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<u8, u8>;
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<u8, u8>;
#[doc = "Reader of field `OVERLAP`"]
pub type OVERLAP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - STATUS"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - OVERRUN"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - OVERLAP"]
    #[inline(always)]
    pub fn overlap(&self) -> OVERLAP_R {
        OVERLAP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
