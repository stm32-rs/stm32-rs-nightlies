#[doc = "Writer for register SECCFGR"]
pub type W = crate::W<u32, super::SECCFGR>;
#[doc = "Register SECCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SECCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SEC0`"]
pub struct SEC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC0_W<'a> {
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
#[doc = "Write proxy for field `SEC1`"]
pub struct SEC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC1_W<'a> {
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
#[doc = "Write proxy for field `SEC2`"]
pub struct SEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC2_W<'a> {
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
#[doc = "Write proxy for field `SEC3`"]
pub struct SEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC3_W<'a> {
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
#[doc = "Write proxy for field `SEC4`"]
pub struct SEC4_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC4_W<'a> {
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
#[doc = "Write proxy for field `SEC5`"]
pub struct SEC5_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC5_W<'a> {
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
#[doc = "Write proxy for field `SEC6`"]
pub struct SEC6_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC6_W<'a> {
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
#[doc = "Write proxy for field `SEC7`"]
pub struct SEC7_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC7_W<'a> {
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
#[doc = "Write proxy for field `SEC8`"]
pub struct SEC8_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC8_W<'a> {
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
#[doc = "Write proxy for field `SEC9`"]
pub struct SEC9_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC9_W<'a> {
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
#[doc = "Write proxy for field `SEC10`"]
pub struct SEC10_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC10_W<'a> {
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
#[doc = "Write proxy for field `SEC11`"]
pub struct SEC11_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC11_W<'a> {
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
#[doc = "Write proxy for field `SEC12`"]
pub struct SEC12_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC12_W<'a> {
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
#[doc = "Write proxy for field `SEC13`"]
pub struct SEC13_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC13_W<'a> {
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
#[doc = "Write proxy for field `SEC14`"]
pub struct SEC14_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC14_W<'a> {
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
#[doc = "Write proxy for field `SEC15`"]
pub struct SEC15_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC15_W<'a> {
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
impl W {
    #[doc = "Bit 0 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC0_W {
        SEC0_W { w: self }
    }
    #[doc = "Bit 1 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W {
        SEC1_W { w: self }
    }
    #[doc = "Bit 2 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC2_W {
        SEC2_W { w: self }
    }
    #[doc = "Bit 3 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W {
        SEC3_W { w: self }
    }
    #[doc = "Bit 4 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC4_W {
        SEC4_W { w: self }
    }
    #[doc = "Bit 5 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC5_W {
        SEC5_W { w: self }
    }
    #[doc = "Bit 6 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC6_W {
        SEC6_W { w: self }
    }
    #[doc = "Bit 7 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC7_W {
        SEC7_W { w: self }
    }
    #[doc = "Bit 8 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec8(&mut self) -> SEC8_W {
        SEC8_W { w: self }
    }
    #[doc = "Bit 9 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec9(&mut self) -> SEC9_W {
        SEC9_W { w: self }
    }
    #[doc = "Bit 10 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec10(&mut self) -> SEC10_W {
        SEC10_W { w: self }
    }
    #[doc = "Bit 11 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec11(&mut self) -> SEC11_W {
        SEC11_W { w: self }
    }
    #[doc = "Bit 12 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec12(&mut self) -> SEC12_W {
        SEC12_W { w: self }
    }
    #[doc = "Bit 13 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec13(&mut self) -> SEC13_W {
        SEC13_W { w: self }
    }
    #[doc = "Bit 14 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec14(&mut self) -> SEC14_W {
        SEC14_W { w: self }
    }
    #[doc = "Bit 15 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    pub fn sec15(&mut self) -> SEC15_W {
        SEC15_W { w: self }
    }
}
