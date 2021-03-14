#[doc = "Reader of register ASCR"]
pub type R = crate::R<u32, super::ASCR>;
#[doc = "Writer for register ASCR"]
pub type W = crate::W<u32, super::ASCR>;
#[doc = "Register ASCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ASCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASC0`"]
pub type ASC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC0`"]
pub struct ASC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC0_W<'a> {
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
#[doc = "Reader of field `ASC1`"]
pub type ASC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC1`"]
pub struct ASC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC1_W<'a> {
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
#[doc = "Reader of field `ASC2`"]
pub type ASC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC2`"]
pub struct ASC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC2_W<'a> {
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
#[doc = "Reader of field `ASC3`"]
pub type ASC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC3`"]
pub struct ASC3_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC3_W<'a> {
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
#[doc = "Reader of field `ASC4`"]
pub type ASC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC4`"]
pub struct ASC4_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC4_W<'a> {
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
#[doc = "Reader of field `ASC5`"]
pub type ASC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC5`"]
pub struct ASC5_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC5_W<'a> {
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
#[doc = "Reader of field `ASC6`"]
pub type ASC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC6`"]
pub struct ASC6_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC6_W<'a> {
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
#[doc = "Reader of field `ASC7`"]
pub type ASC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC7`"]
pub struct ASC7_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC7_W<'a> {
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
#[doc = "Reader of field `ASC8`"]
pub type ASC8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC8`"]
pub struct ASC8_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC8_W<'a> {
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
#[doc = "Reader of field `ASC9`"]
pub type ASC9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC9`"]
pub struct ASC9_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC9_W<'a> {
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
#[doc = "Reader of field `ASC10`"]
pub type ASC10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC10`"]
pub struct ASC10_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC10_W<'a> {
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
#[doc = "Reader of field `ASC11`"]
pub type ASC11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC11`"]
pub struct ASC11_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC11_W<'a> {
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
#[doc = "Reader of field `ASC12`"]
pub type ASC12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC12`"]
pub struct ASC12_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC12_W<'a> {
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
#[doc = "Reader of field `ASC13`"]
pub type ASC13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC13`"]
pub struct ASC13_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC13_W<'a> {
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
#[doc = "Reader of field `ASC14`"]
pub type ASC14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC14`"]
pub struct ASC14_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC14_W<'a> {
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
#[doc = "Reader of field `ASC15`"]
pub type ASC15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASC15`"]
pub struct ASC15_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC15_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Port analog switch control"]
    #[inline(always)]
    pub fn asc0(&self) -> ASC0_R {
        ASC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port analog switch control"]
    #[inline(always)]
    pub fn asc1(&self) -> ASC1_R {
        ASC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port analog switch control"]
    #[inline(always)]
    pub fn asc2(&self) -> ASC2_R {
        ASC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port analog switch control"]
    #[inline(always)]
    pub fn asc3(&self) -> ASC3_R {
        ASC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port analog switch control"]
    #[inline(always)]
    pub fn asc4(&self) -> ASC4_R {
        ASC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port analog switch control"]
    #[inline(always)]
    pub fn asc5(&self) -> ASC5_R {
        ASC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port analog switch control"]
    #[inline(always)]
    pub fn asc6(&self) -> ASC6_R {
        ASC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port analog switch control"]
    #[inline(always)]
    pub fn asc7(&self) -> ASC7_R {
        ASC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port analog switch control"]
    #[inline(always)]
    pub fn asc8(&self) -> ASC8_R {
        ASC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port analog switch control"]
    #[inline(always)]
    pub fn asc9(&self) -> ASC9_R {
        ASC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port analog switch control"]
    #[inline(always)]
    pub fn asc10(&self) -> ASC10_R {
        ASC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port analog switch control"]
    #[inline(always)]
    pub fn asc11(&self) -> ASC11_R {
        ASC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port analog switch control"]
    #[inline(always)]
    pub fn asc12(&self) -> ASC12_R {
        ASC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port analog switch control"]
    #[inline(always)]
    pub fn asc13(&self) -> ASC13_R {
        ASC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port analog switch control"]
    #[inline(always)]
    pub fn asc14(&self) -> ASC14_R {
        ASC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port analog switch control"]
    #[inline(always)]
    pub fn asc15(&self) -> ASC15_R {
        ASC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port analog switch control"]
    #[inline(always)]
    pub fn asc0(&mut self) -> ASC0_W {
        ASC0_W { w: self }
    }
    #[doc = "Bit 1 - Port analog switch control"]
    #[inline(always)]
    pub fn asc1(&mut self) -> ASC1_W {
        ASC1_W { w: self }
    }
    #[doc = "Bit 2 - Port analog switch control"]
    #[inline(always)]
    pub fn asc2(&mut self) -> ASC2_W {
        ASC2_W { w: self }
    }
    #[doc = "Bit 3 - Port analog switch control"]
    #[inline(always)]
    pub fn asc3(&mut self) -> ASC3_W {
        ASC3_W { w: self }
    }
    #[doc = "Bit 4 - Port analog switch control"]
    #[inline(always)]
    pub fn asc4(&mut self) -> ASC4_W {
        ASC4_W { w: self }
    }
    #[doc = "Bit 5 - Port analog switch control"]
    #[inline(always)]
    pub fn asc5(&mut self) -> ASC5_W {
        ASC5_W { w: self }
    }
    #[doc = "Bit 6 - Port analog switch control"]
    #[inline(always)]
    pub fn asc6(&mut self) -> ASC6_W {
        ASC6_W { w: self }
    }
    #[doc = "Bit 7 - Port analog switch control"]
    #[inline(always)]
    pub fn asc7(&mut self) -> ASC7_W {
        ASC7_W { w: self }
    }
    #[doc = "Bit 8 - Port analog switch control"]
    #[inline(always)]
    pub fn asc8(&mut self) -> ASC8_W {
        ASC8_W { w: self }
    }
    #[doc = "Bit 9 - Port analog switch control"]
    #[inline(always)]
    pub fn asc9(&mut self) -> ASC9_W {
        ASC9_W { w: self }
    }
    #[doc = "Bit 10 - Port analog switch control"]
    #[inline(always)]
    pub fn asc10(&mut self) -> ASC10_W {
        ASC10_W { w: self }
    }
    #[doc = "Bit 11 - Port analog switch control"]
    #[inline(always)]
    pub fn asc11(&mut self) -> ASC11_W {
        ASC11_W { w: self }
    }
    #[doc = "Bit 12 - Port analog switch control"]
    #[inline(always)]
    pub fn asc12(&mut self) -> ASC12_W {
        ASC12_W { w: self }
    }
    #[doc = "Bit 13 - Port analog switch control"]
    #[inline(always)]
    pub fn asc13(&mut self) -> ASC13_W {
        ASC13_W { w: self }
    }
    #[doc = "Bit 14 - Port analog switch control"]
    #[inline(always)]
    pub fn asc14(&mut self) -> ASC14_W {
        ASC14_W { w: self }
    }
    #[doc = "Bit 15 - Port analog switch control"]
    #[inline(always)]
    pub fn asc15(&mut self) -> ASC15_W {
        ASC15_W { w: self }
    }
}
