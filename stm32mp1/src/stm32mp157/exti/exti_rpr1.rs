#[doc = "Reader of register EXTI_RPR1"]
pub type R = crate::R<u32, super::EXTI_RPR1>;
#[doc = "Writer for register EXTI_RPR1"]
pub type W = crate::W<u32, super::EXTI_RPR1>;
#[doc = "Register EXTI_RPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_RPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPIF0`"]
pub type RPIF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF0`"]
pub struct RPIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF0_W<'a> {
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
#[doc = "Reader of field `RPIF1`"]
pub type RPIF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF1`"]
pub struct RPIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF1_W<'a> {
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
#[doc = "Reader of field `RPIF2`"]
pub type RPIF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF2`"]
pub struct RPIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF2_W<'a> {
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
#[doc = "Reader of field `RPIF3`"]
pub type RPIF3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF3`"]
pub struct RPIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF3_W<'a> {
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
#[doc = "Reader of field `RPIF4`"]
pub type RPIF4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF4`"]
pub struct RPIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF4_W<'a> {
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
#[doc = "Reader of field `RPIF5`"]
pub type RPIF5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF5`"]
pub struct RPIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF5_W<'a> {
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
#[doc = "Reader of field `RPIF6`"]
pub type RPIF6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF6`"]
pub struct RPIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF6_W<'a> {
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
#[doc = "Reader of field `RPIF7`"]
pub type RPIF7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF7`"]
pub struct RPIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF7_W<'a> {
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
#[doc = "Reader of field `RPIF8`"]
pub type RPIF8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF8`"]
pub struct RPIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF8_W<'a> {
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
#[doc = "Reader of field `RPIF9`"]
pub type RPIF9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF9`"]
pub struct RPIF9_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF9_W<'a> {
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
#[doc = "Reader of field `RPIF10`"]
pub type RPIF10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF10`"]
pub struct RPIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF10_W<'a> {
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
#[doc = "Reader of field `RPIF11`"]
pub type RPIF11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF11`"]
pub struct RPIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF11_W<'a> {
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
#[doc = "Reader of field `RPIF12`"]
pub type RPIF12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF12`"]
pub struct RPIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF12_W<'a> {
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
#[doc = "Reader of field `RPIF13`"]
pub type RPIF13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF13`"]
pub struct RPIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF13_W<'a> {
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
#[doc = "Reader of field `RPIF14`"]
pub type RPIF14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF14`"]
pub struct RPIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF14_W<'a> {
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
#[doc = "Reader of field `RPIF15`"]
pub type RPIF15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF15`"]
pub struct RPIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF15_W<'a> {
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
#[doc = "Reader of field `RPIF16`"]
pub type RPIF16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF16`"]
pub struct RPIF16_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RPIF0"]
    #[inline(always)]
    pub fn rpif0(&self) -> RPIF0_R {
        RPIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RPIF1"]
    #[inline(always)]
    pub fn rpif1(&self) -> RPIF1_R {
        RPIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RPIF2"]
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RPIF3"]
    #[inline(always)]
    pub fn rpif3(&self) -> RPIF3_R {
        RPIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RPIF4"]
    #[inline(always)]
    pub fn rpif4(&self) -> RPIF4_R {
        RPIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RPIF5"]
    #[inline(always)]
    pub fn rpif5(&self) -> RPIF5_R {
        RPIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RPIF6"]
    #[inline(always)]
    pub fn rpif6(&self) -> RPIF6_R {
        RPIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RPIF7"]
    #[inline(always)]
    pub fn rpif7(&self) -> RPIF7_R {
        RPIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RPIF8"]
    #[inline(always)]
    pub fn rpif8(&self) -> RPIF8_R {
        RPIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RPIF9"]
    #[inline(always)]
    pub fn rpif9(&self) -> RPIF9_R {
        RPIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RPIF10"]
    #[inline(always)]
    pub fn rpif10(&self) -> RPIF10_R {
        RPIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RPIF11"]
    #[inline(always)]
    pub fn rpif11(&self) -> RPIF11_R {
        RPIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RPIF12"]
    #[inline(always)]
    pub fn rpif12(&self) -> RPIF12_R {
        RPIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RPIF13"]
    #[inline(always)]
    pub fn rpif13(&self) -> RPIF13_R {
        RPIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RPIF14"]
    #[inline(always)]
    pub fn rpif14(&self) -> RPIF14_R {
        RPIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RPIF15"]
    #[inline(always)]
    pub fn rpif15(&self) -> RPIF15_R {
        RPIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RPIF16"]
    #[inline(always)]
    pub fn rpif16(&self) -> RPIF16_R {
        RPIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RPIF0"]
    #[inline(always)]
    pub fn rpif0(&mut self) -> RPIF0_W {
        RPIF0_W { w: self }
    }
    #[doc = "Bit 1 - RPIF1"]
    #[inline(always)]
    pub fn rpif1(&mut self) -> RPIF1_W {
        RPIF1_W { w: self }
    }
    #[doc = "Bit 2 - RPIF2"]
    #[inline(always)]
    pub fn rpif2(&mut self) -> RPIF2_W {
        RPIF2_W { w: self }
    }
    #[doc = "Bit 3 - RPIF3"]
    #[inline(always)]
    pub fn rpif3(&mut self) -> RPIF3_W {
        RPIF3_W { w: self }
    }
    #[doc = "Bit 4 - RPIF4"]
    #[inline(always)]
    pub fn rpif4(&mut self) -> RPIF4_W {
        RPIF4_W { w: self }
    }
    #[doc = "Bit 5 - RPIF5"]
    #[inline(always)]
    pub fn rpif5(&mut self) -> RPIF5_W {
        RPIF5_W { w: self }
    }
    #[doc = "Bit 6 - RPIF6"]
    #[inline(always)]
    pub fn rpif6(&mut self) -> RPIF6_W {
        RPIF6_W { w: self }
    }
    #[doc = "Bit 7 - RPIF7"]
    #[inline(always)]
    pub fn rpif7(&mut self) -> RPIF7_W {
        RPIF7_W { w: self }
    }
    #[doc = "Bit 8 - RPIF8"]
    #[inline(always)]
    pub fn rpif8(&mut self) -> RPIF8_W {
        RPIF8_W { w: self }
    }
    #[doc = "Bit 9 - RPIF9"]
    #[inline(always)]
    pub fn rpif9(&mut self) -> RPIF9_W {
        RPIF9_W { w: self }
    }
    #[doc = "Bit 10 - RPIF10"]
    #[inline(always)]
    pub fn rpif10(&mut self) -> RPIF10_W {
        RPIF10_W { w: self }
    }
    #[doc = "Bit 11 - RPIF11"]
    #[inline(always)]
    pub fn rpif11(&mut self) -> RPIF11_W {
        RPIF11_W { w: self }
    }
    #[doc = "Bit 12 - RPIF12"]
    #[inline(always)]
    pub fn rpif12(&mut self) -> RPIF12_W {
        RPIF12_W { w: self }
    }
    #[doc = "Bit 13 - RPIF13"]
    #[inline(always)]
    pub fn rpif13(&mut self) -> RPIF13_W {
        RPIF13_W { w: self }
    }
    #[doc = "Bit 14 - RPIF14"]
    #[inline(always)]
    pub fn rpif14(&mut self) -> RPIF14_W {
        RPIF14_W { w: self }
    }
    #[doc = "Bit 15 - RPIF15"]
    #[inline(always)]
    pub fn rpif15(&mut self) -> RPIF15_W {
        RPIF15_W { w: self }
    }
    #[doc = "Bit 16 - RPIF16"]
    #[inline(always)]
    pub fn rpif16(&mut self) -> RPIF16_W {
        RPIF16_W { w: self }
    }
}
