#[doc = "Reader of register LTDC_IDR"]
pub type R = crate::R<u32, super::LTDC_IDR>;
#[doc = "Reader of field `REV`"]
pub type REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `MINVER`"]
pub type MINVER_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJVER`"]
pub type MAJVER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - REV"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MINVER"]
    #[inline(always)]
    pub fn minver(&self) -> MINVER_R {
        MINVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MAJVER"]
    #[inline(always)]
    pub fn majver(&self) -> MAJVER_R {
        MAJVER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
