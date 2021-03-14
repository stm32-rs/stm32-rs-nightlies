#[doc = "Reader of register LTDC_CPSR"]
pub type R = crate::R<u32, super::LTDC_CPSR>;
#[doc = "Reader of field `CYPOS`"]
pub type CYPOS_R = crate::R<u16, u16>;
#[doc = "Reader of field `CXPOS`"]
pub type CXPOS_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CYPOS"]
    #[inline(always)]
    pub fn cypos(&self) -> CYPOS_R {
        CYPOS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CXPOS"]
    #[inline(always)]
    pub fn cxpos(&self) -> CXPOS_R {
        CXPOS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
