#[doc = "Reader of register OTG_FS_GPWRDN"]
pub type R = crate::R<u32, super::OTG_FS_GPWRDN>;
#[doc = "Writer for register OTG_FS_GPWRDN"]
pub type W = crate::W<u32, super::OTG_FS_GPWRDN>;
#[doc = "Register OTG_FS_GPWRDN `reset()`'s with value 0x0200_0400"]
impl crate::ResetValue for super::OTG_FS_GPWRDN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0400
    }
}
#[doc = "Reader of field `ADPMEN`"]
pub type ADPMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADPMEN`"]
pub struct ADPMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ADPIF`"]
pub type ADPIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADPIF`"]
pub struct ADPIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPIF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    pub fn adpmen(&self) -> ADPMEN_R {
        ADPMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADP interrupt flag"]
    #[inline(always)]
    pub fn adpif(&self) -> ADPIF_R {
        ADPIF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    pub fn adpmen(&mut self) -> ADPMEN_W {
        ADPMEN_W { w: self }
    }
    #[doc = "Bit 23 - ADP interrupt flag"]
    #[inline(always)]
    pub fn adpif(&mut self) -> ADPIF_W {
        ADPIF_W { w: self }
    }
}
