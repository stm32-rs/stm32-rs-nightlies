#[doc = "Reader of register OTG_HCFG"]
pub type R = crate::R<u32, super::OTG_HCFG>;
#[doc = "Writer for register OTG_HCFG"]
pub type W = crate::W<u32, super::OTG_HCFG>;
#[doc = "Register OTG_HCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::OTG_HCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSLSPCS`"]
pub type FSLSPCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSLSPCS`"]
pub struct FSLSPCS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLSPCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FSLSS`"]
pub type FSLSS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DESCDMA`"]
pub type DESCDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DESCDMA`"]
pub struct DESCDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCDMA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `FRLSTEN`"]
pub type FRLSTEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRLSTEN`"]
pub struct FRLSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRLSTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `PERSSCHEDENA`"]
pub type PERSSCHEDENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERSSCHEDENA`"]
pub struct PERSSCHEDENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSSCHEDENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FSLSPCS"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - FSLSS"]
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DESCDMA"]
    #[inline(always)]
    pub fn descdma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - FRLSTEN"]
    #[inline(always)]
    pub fn frlsten(&self) -> FRLSTEN_R {
        FRLSTEN_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - PERSSCHEDENA"]
    #[inline(always)]
    pub fn persschedena(&self) -> PERSSCHEDENA_R {
        PERSSCHEDENA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FSLSPCS"]
    #[inline(always)]
    pub fn fslspcs(&mut self) -> FSLSPCS_W {
        FSLSPCS_W { w: self }
    }
    #[doc = "Bit 23 - DESCDMA"]
    #[inline(always)]
    pub fn descdma(&mut self) -> DESCDMA_W {
        DESCDMA_W { w: self }
    }
    #[doc = "Bits 24:25 - FRLSTEN"]
    #[inline(always)]
    pub fn frlsten(&mut self) -> FRLSTEN_W {
        FRLSTEN_W { w: self }
    }
    #[doc = "Bit 26 - PERSSCHEDENA"]
    #[inline(always)]
    pub fn persschedena(&mut self) -> PERSSCHEDENA_W {
        PERSSCHEDENA_W { w: self }
    }
}
