#[doc = "Reader of register CNSLCKR"]
pub type R = crate::R<u32, super::CNSLCKR>;
#[doc = "Writer for register CNSLCKR"]
pub type W = crate::W<u32, super::CNSLCKR>;
#[doc = "Register CNSLCKR `reset()`'s with value 0"]
impl crate::ResetValue for super::CNSLCKR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCKNSVTOR`"]
pub type LOCKNSVTOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKNSVTOR`"]
pub struct LOCKNSVTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKNSVTOR_W<'a> {
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
#[doc = "Reader of field `LOCKNSMPU`"]
pub type LOCKNSMPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKNSMPU`"]
pub struct LOCKNSMPU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKNSMPU_W<'a> {
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
impl R {
    #[doc = "Bit 0 - VTOR_NS register lock"]
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Non-secure MPU registers lock"]
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VTOR_NS register lock"]
    #[inline(always)]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W {
        LOCKNSVTOR_W { w: self }
    }
    #[doc = "Bit 1 - Non-secure MPU registers lock"]
    #[inline(always)]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W {
        LOCKNSMPU_W { w: self }
    }
}
