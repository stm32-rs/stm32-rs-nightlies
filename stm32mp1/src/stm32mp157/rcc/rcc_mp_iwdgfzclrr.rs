#[doc = "Reader of register RCC_MP_IWDGFZCLRR"]
pub type R = crate::R<u32, super::RCC_MP_IWDGFZCLRR>;
#[doc = "Writer for register RCC_MP_IWDGFZCLRR"]
pub type W = crate::W<u32, super::RCC_MP_IWDGFZCLRR>;
#[doc = "Register RCC_MP_IWDGFZCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_IWDGFZCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FZ_IWDG1`"]
pub type FZ_IWDG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FZ_IWDG1`"]
pub struct FZ_IWDG1_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_IWDG1_W<'a> {
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
#[doc = "Reader of field `FZ_IWDG2`"]
pub type FZ_IWDG2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FZ_IWDG2`"]
pub struct FZ_IWDG2_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_IWDG2_W<'a> {
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
    #[doc = "Bit 0 - FZ_IWDG1"]
    #[inline(always)]
    pub fn fz_iwdg1(&self) -> FZ_IWDG1_R {
        FZ_IWDG1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FZ_IWDG2"]
    #[inline(always)]
    pub fn fz_iwdg2(&self) -> FZ_IWDG2_R {
        FZ_IWDG2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FZ_IWDG1"]
    #[inline(always)]
    pub fn fz_iwdg1(&mut self) -> FZ_IWDG1_W {
        FZ_IWDG1_W { w: self }
    }
    #[doc = "Bit 1 - FZ_IWDG2"]
    #[inline(always)]
    pub fn fz_iwdg2(&mut self) -> FZ_IWDG2_W {
        FZ_IWDG2_W { w: self }
    }
}
