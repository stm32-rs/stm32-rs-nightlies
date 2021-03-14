#[doc = "Reader of register EEEFR3"]
pub type R = crate::R<u32, super::EEEFR3>;
#[doc = "Writer for register EEEFR3"]
pub type W = crate::W<u32, super::EEEFR3>;
#[doc = "Register EEEFR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EEEFR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EEVACNT`"]
pub type EEVACNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EEVACNT`"]
pub struct EEVACNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVACNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EEVASEL`"]
pub type EEVASEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EEVASEL`"]
pub struct EEVASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVASEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EEVARSTM`"]
pub type EEVARSTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EEVARSTM`"]
pub struct EEVARSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVARSTM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EEVACRES`"]
pub type EEVACRES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EEVACRES`"]
pub struct EEVACRES_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVACRES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EEVACE`"]
pub type EEVACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EEVACE`"]
pub struct EEVACE_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVACE_W<'a> {
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
impl R {
    #[doc = "Bits 8:13 - External Event A counter"]
    #[inline(always)]
    pub fn eevacnt(&self) -> EEVACNT_R {
        EEVACNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 4:7 - External Event A Selection"]
    #[inline(always)]
    pub fn eevasel(&self) -> EEVASEL_R {
        EEVASEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - External Event A Reset Mode"]
    #[inline(always)]
    pub fn eevarstm(&self) -> EEVARSTM_R {
        EEVARSTM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Event A Counter Reset"]
    #[inline(always)]
    pub fn eevacres(&self) -> EEVACRES_R {
        EEVACRES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - External Event A Counter Enable"]
    #[inline(always)]
    pub fn eevace(&self) -> EEVACE_R {
        EEVACE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13 - External Event A counter"]
    #[inline(always)]
    pub fn eevacnt(&mut self) -> EEVACNT_W {
        EEVACNT_W { w: self }
    }
    #[doc = "Bits 4:7 - External Event A Selection"]
    #[inline(always)]
    pub fn eevasel(&mut self) -> EEVASEL_W {
        EEVASEL_W { w: self }
    }
    #[doc = "Bit 2 - External Event A Reset Mode"]
    #[inline(always)]
    pub fn eevarstm(&mut self) -> EEVARSTM_W {
        EEVARSTM_W { w: self }
    }
    #[doc = "Bit 1 - External Event A Counter Reset"]
    #[inline(always)]
    pub fn eevacres(&mut self) -> EEVACRES_W {
        EEVACRES_W { w: self }
    }
    #[doc = "Bit 0 - External Event A Counter Enable"]
    #[inline(always)]
    pub fn eevace(&mut self) -> EEVACE_W {
        EEVACE_W { w: self }
    }
}
