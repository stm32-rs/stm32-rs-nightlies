#[doc = "Reader of register IPCC_C1MR"]
pub type R = crate::R<u32, super::IPCC_C1MR>;
#[doc = "Writer for register IPCC_C1MR"]
pub type W = crate::W<u32, super::IPCC_C1MR>;
#[doc = "Register IPCC_C1MR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::IPCC_C1MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `CHxOM`"]
pub type CHXOM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHxOM`"]
pub struct CHXOM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHXOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CHxFM`"]
pub type CHXFM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHxFM`"]
pub struct CHXFM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHXFM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - CHxOM"]
    #[inline(always)]
    pub fn chx_om(&self) -> CHXOM_R {
        CHXOM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - CHxFM"]
    #[inline(always)]
    pub fn chx_fm(&self) -> CHXFM_R {
        CHXFM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CHxOM"]
    #[inline(always)]
    pub fn chx_om(&mut self) -> CHXOM_W {
        CHXOM_W { w: self }
    }
    #[doc = "Bits 16:21 - CHxFM"]
    #[inline(always)]
    pub fn chx_fm(&mut self) -> CHXFM_W {
        CHXFM_W { w: self }
    }
}
