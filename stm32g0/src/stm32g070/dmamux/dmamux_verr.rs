#[doc = "Reader of register DMAMUX_VERR"]
pub type R = crate::R<u32, super::DMAMUX_VERR>;
#[doc = "Reader of field `MINREV`"]
pub type MINREV_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJREV`"]
pub type MAJREV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Minor IP revision"]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major IP revision"]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
