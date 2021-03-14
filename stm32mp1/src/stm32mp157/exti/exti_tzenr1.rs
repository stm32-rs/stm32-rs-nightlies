#[doc = "Reader of register EXTI_TZENR1"]
pub type R = crate::R<u32, super::EXTI_TZENR1>;
#[doc = "Writer for register EXTI_TZENR1"]
pub type W = crate::W<u32, super::EXTI_TZENR1>;
#[doc = "Register EXTI_TZENR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_TZENR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TZEN0`"]
pub type TZEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN0`"]
pub struct TZEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN0_W<'a> {
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
#[doc = "Reader of field `TZEN1`"]
pub type TZEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN1`"]
pub struct TZEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN1_W<'a> {
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
#[doc = "Reader of field `TZEN2`"]
pub type TZEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN2`"]
pub struct TZEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN2_W<'a> {
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
#[doc = "Reader of field `TZEN3`"]
pub type TZEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN3`"]
pub struct TZEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TZEN4`"]
pub type TZEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN4`"]
pub struct TZEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TZEN5`"]
pub type TZEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN5`"]
pub struct TZEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TZEN6`"]
pub type TZEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN6`"]
pub struct TZEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TZEN7`"]
pub type TZEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN7`"]
pub struct TZEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TZEN8`"]
pub type TZEN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN8`"]
pub struct TZEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TZEN9`"]
pub type TZEN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN9`"]
pub struct TZEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TZEN10`"]
pub type TZEN10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN10`"]
pub struct TZEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TZEN11`"]
pub type TZEN11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN11`"]
pub struct TZEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TZEN12`"]
pub type TZEN12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN12`"]
pub struct TZEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TZEN13`"]
pub type TZEN13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN13`"]
pub struct TZEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TZEN14`"]
pub type TZEN14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN14`"]
pub struct TZEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TZEN15`"]
pub type TZEN15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN15`"]
pub struct TZEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TZEN17`"]
pub type TZEN17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN17`"]
pub struct TZEN17_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TZEN18`"]
pub type TZEN18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN18`"]
pub struct TZEN18_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TZEN19`"]
pub type TZEN19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN19`"]
pub struct TZEN19_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `TZEN24`"]
pub type TZEN24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN24`"]
pub struct TZEN24_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `TZEN26`"]
pub type TZEN26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZEN26`"]
pub struct TZEN26_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN26_W<'a> {
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
    #[doc = "Bit 0 - TZEN0"]
    #[inline(always)]
    pub fn tzen0(&self) -> TZEN0_R {
        TZEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZEN1"]
    #[inline(always)]
    pub fn tzen1(&self) -> TZEN1_R {
        TZEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TZEN2"]
    #[inline(always)]
    pub fn tzen2(&self) -> TZEN2_R {
        TZEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TZEN3"]
    #[inline(always)]
    pub fn tzen3(&self) -> TZEN3_R {
        TZEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TZEN4"]
    #[inline(always)]
    pub fn tzen4(&self) -> TZEN4_R {
        TZEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TZEN5"]
    #[inline(always)]
    pub fn tzen5(&self) -> TZEN5_R {
        TZEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TZEN6"]
    #[inline(always)]
    pub fn tzen6(&self) -> TZEN6_R {
        TZEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TZEN7"]
    #[inline(always)]
    pub fn tzen7(&self) -> TZEN7_R {
        TZEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TZEN8"]
    #[inline(always)]
    pub fn tzen8(&self) -> TZEN8_R {
        TZEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TZEN9"]
    #[inline(always)]
    pub fn tzen9(&self) -> TZEN9_R {
        TZEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TZEN10"]
    #[inline(always)]
    pub fn tzen10(&self) -> TZEN10_R {
        TZEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TZEN11"]
    #[inline(always)]
    pub fn tzen11(&self) -> TZEN11_R {
        TZEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TZEN12"]
    #[inline(always)]
    pub fn tzen12(&self) -> TZEN12_R {
        TZEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TZEN13"]
    #[inline(always)]
    pub fn tzen13(&self) -> TZEN13_R {
        TZEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TZEN14"]
    #[inline(always)]
    pub fn tzen14(&self) -> TZEN14_R {
        TZEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TZEN15"]
    #[inline(always)]
    pub fn tzen15(&self) -> TZEN15_R {
        TZEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TZEN17"]
    #[inline(always)]
    pub fn tzen17(&self) -> TZEN17_R {
        TZEN17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TZEN18"]
    #[inline(always)]
    pub fn tzen18(&self) -> TZEN18_R {
        TZEN18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TZEN19"]
    #[inline(always)]
    pub fn tzen19(&self) -> TZEN19_R {
        TZEN19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TZEN24"]
    #[inline(always)]
    pub fn tzen24(&self) -> TZEN24_R {
        TZEN24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TZEN26"]
    #[inline(always)]
    pub fn tzen26(&self) -> TZEN26_R {
        TZEN26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZEN0"]
    #[inline(always)]
    pub fn tzen0(&mut self) -> TZEN0_W {
        TZEN0_W { w: self }
    }
    #[doc = "Bit 1 - TZEN1"]
    #[inline(always)]
    pub fn tzen1(&mut self) -> TZEN1_W {
        TZEN1_W { w: self }
    }
    #[doc = "Bit 2 - TZEN2"]
    #[inline(always)]
    pub fn tzen2(&mut self) -> TZEN2_W {
        TZEN2_W { w: self }
    }
    #[doc = "Bit 3 - TZEN3"]
    #[inline(always)]
    pub fn tzen3(&mut self) -> TZEN3_W {
        TZEN3_W { w: self }
    }
    #[doc = "Bit 4 - TZEN4"]
    #[inline(always)]
    pub fn tzen4(&mut self) -> TZEN4_W {
        TZEN4_W { w: self }
    }
    #[doc = "Bit 5 - TZEN5"]
    #[inline(always)]
    pub fn tzen5(&mut self) -> TZEN5_W {
        TZEN5_W { w: self }
    }
    #[doc = "Bit 6 - TZEN6"]
    #[inline(always)]
    pub fn tzen6(&mut self) -> TZEN6_W {
        TZEN6_W { w: self }
    }
    #[doc = "Bit 7 - TZEN7"]
    #[inline(always)]
    pub fn tzen7(&mut self) -> TZEN7_W {
        TZEN7_W { w: self }
    }
    #[doc = "Bit 8 - TZEN8"]
    #[inline(always)]
    pub fn tzen8(&mut self) -> TZEN8_W {
        TZEN8_W { w: self }
    }
    #[doc = "Bit 9 - TZEN9"]
    #[inline(always)]
    pub fn tzen9(&mut self) -> TZEN9_W {
        TZEN9_W { w: self }
    }
    #[doc = "Bit 10 - TZEN10"]
    #[inline(always)]
    pub fn tzen10(&mut self) -> TZEN10_W {
        TZEN10_W { w: self }
    }
    #[doc = "Bit 11 - TZEN11"]
    #[inline(always)]
    pub fn tzen11(&mut self) -> TZEN11_W {
        TZEN11_W { w: self }
    }
    #[doc = "Bit 12 - TZEN12"]
    #[inline(always)]
    pub fn tzen12(&mut self) -> TZEN12_W {
        TZEN12_W { w: self }
    }
    #[doc = "Bit 13 - TZEN13"]
    #[inline(always)]
    pub fn tzen13(&mut self) -> TZEN13_W {
        TZEN13_W { w: self }
    }
    #[doc = "Bit 14 - TZEN14"]
    #[inline(always)]
    pub fn tzen14(&mut self) -> TZEN14_W {
        TZEN14_W { w: self }
    }
    #[doc = "Bit 15 - TZEN15"]
    #[inline(always)]
    pub fn tzen15(&mut self) -> TZEN15_W {
        TZEN15_W { w: self }
    }
    #[doc = "Bit 17 - TZEN17"]
    #[inline(always)]
    pub fn tzen17(&mut self) -> TZEN17_W {
        TZEN17_W { w: self }
    }
    #[doc = "Bit 18 - TZEN18"]
    #[inline(always)]
    pub fn tzen18(&mut self) -> TZEN18_W {
        TZEN18_W { w: self }
    }
    #[doc = "Bit 19 - TZEN19"]
    #[inline(always)]
    pub fn tzen19(&mut self) -> TZEN19_W {
        TZEN19_W { w: self }
    }
    #[doc = "Bit 24 - TZEN24"]
    #[inline(always)]
    pub fn tzen24(&mut self) -> TZEN24_W {
        TZEN24_W { w: self }
    }
    #[doc = "Bit 26 - TZEN26"]
    #[inline(always)]
    pub fn tzen26(&mut self) -> TZEN26_W {
        TZEN26_W { w: self }
    }
}
