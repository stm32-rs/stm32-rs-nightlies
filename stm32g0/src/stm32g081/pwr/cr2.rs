#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Writer for register CR2"]
pub type W = crate::W<u32, super::CR2>;
#[doc = "Register CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PVDE`"]
pub type PVDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVDE`"]
pub struct PVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDE_W<'a> {
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
#[doc = "Reader of field `PVDFT`"]
pub type PVDFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PVDFT`"]
pub struct PVDFT_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `PVDRT`"]
pub type PVDRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PVDRT`"]
pub struct PVDRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector falling threshold selection"]
    #[inline(always)]
    pub fn pvdft(&self) -> PVDFT_R {
        PVDFT_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Power voltage detector rising threshold selection"]
    #[inline(always)]
    pub fn pvdrt(&self) -> PVDRT_R {
        PVDRT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W {
        PVDE_W { w: self }
    }
    #[doc = "Bits 1:3 - Power voltage detector falling threshold selection"]
    #[inline(always)]
    pub fn pvdft(&mut self) -> PVDFT_W {
        PVDFT_W { w: self }
    }
    #[doc = "Bits 4:6 - Power voltage detector rising threshold selection"]
    #[inline(always)]
    pub fn pvdrt(&mut self) -> PVDRT_W {
        PVDRT_W { w: self }
    }
}
