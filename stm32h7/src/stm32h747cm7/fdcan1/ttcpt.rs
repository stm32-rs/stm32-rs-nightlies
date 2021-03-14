#[doc = "Reader of register TTCPT"]
pub type R = crate::R<u32, super::TTCPT>;
#[doc = "Reader of field `CCV`"]
pub type CCV_R = crate::R<u8, u8>;
#[doc = "Reader of field `SWV`"]
pub type SWV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:5 - Cycle Count Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Stop Watch Value"]
    #[inline(always)]
    pub fn swv(&self) -> SWV_R {
        SWV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
