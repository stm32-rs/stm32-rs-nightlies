#[doc = "Reader of register OTG_DAINT"]
pub type R = crate::R<u32, super::OTG_DAINT>;
#[doc = "Reader of field `IEPINT`"]
pub type IEPINT_R = crate::R<u16, u16>;
#[doc = "Reader of field `OEPINT`"]
pub type OEPINT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IEPINT"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OEPINT"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
