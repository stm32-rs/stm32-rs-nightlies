#[doc = "Reader of register OTG_HFNUM"]
pub type R = crate::R<u32, super::OTG_HFNUM>;
#[doc = "Reader of field `FRNUM`"]
pub type FRNUM_R = crate::R<u16, u16>;
#[doc = "Reader of field `FTREM`"]
pub type FTREM_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - FRNUM"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - FTREM"]
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
