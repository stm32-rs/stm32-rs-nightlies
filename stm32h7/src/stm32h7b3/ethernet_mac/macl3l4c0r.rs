#[doc = "Reader of register MACL3L4C0R"]
pub type R = crate::R<u32, super::MACL3L4C0R>;
#[doc = "Writer for register MACL3L4C0R"]
pub type W = crate::W<u32, super::MACL3L4C0R>;
#[doc = "Register MACL3L4C0R `reset()`'s with value 0"]
impl crate::ResetValue for super::MACL3L4C0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `L3PEN0`"]
pub type L3PEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L3PEN0`"]
pub struct L3PEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3PEN0_W<'a> {
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
#[doc = "Reader of field `L3SAM0`"]
pub type L3SAM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L3SAM0`"]
pub struct L3SAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3SAM0_W<'a> {
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
#[doc = "Reader of field `L3SAIM0`"]
pub type L3SAIM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L3SAIM0`"]
pub struct L3SAIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3SAIM0_W<'a> {
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
#[doc = "Reader of field `L3DAM0`"]
pub type L3DAM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L3DAM0`"]
pub struct L3DAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3DAM0_W<'a> {
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
#[doc = "Reader of field `L3DAIM0`"]
pub type L3DAIM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L3DAIM0`"]
pub struct L3DAIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3DAIM0_W<'a> {
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
#[doc = "Reader of field `L3HSBM0`"]
pub type L3HSBM0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `L3HSBM0`"]
pub struct L3HSBM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3HSBM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `L3HDBM0`"]
pub type L3HDBM0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `L3HDBM0`"]
pub struct L3HDBM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L3HDBM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `L4PEN0`"]
pub type L4PEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L4PEN0`"]
pub struct L4PEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4PEN0_W<'a> {
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
#[doc = "Reader of field `L4SPM0`"]
pub type L4SPM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L4SPM0`"]
pub struct L4SPM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4SPM0_W<'a> {
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
#[doc = "Reader of field `L4SPIM0`"]
pub type L4SPIM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L4SPIM0`"]
pub struct L4SPIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4SPIM0_W<'a> {
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
#[doc = "Reader of field `L4DPM0`"]
pub type L4DPM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L4DPM0`"]
pub struct L4DPM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4DPM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `L4DPIM0`"]
pub type L4DPIM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L4DPIM0`"]
pub struct L4DPIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> L4DPIM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - L3PEN0"]
    #[inline(always)]
    pub fn l3pen0(&self) -> L3PEN0_R {
        L3PEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - L3SAM0"]
    #[inline(always)]
    pub fn l3sam0(&self) -> L3SAM0_R {
        L3SAM0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - L3SAIM0"]
    #[inline(always)]
    pub fn l3saim0(&self) -> L3SAIM0_R {
        L3SAIM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - L3DAM0"]
    #[inline(always)]
    pub fn l3dam0(&self) -> L3DAM0_R {
        L3DAM0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - L3DAIM0"]
    #[inline(always)]
    pub fn l3daim0(&self) -> L3DAIM0_R {
        L3DAIM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:10 - L3HSBM0"]
    #[inline(always)]
    pub fn l3hsbm0(&self) -> L3HSBM0_R {
        L3HSBM0_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - L3HDBM0"]
    #[inline(always)]
    pub fn l3hdbm0(&self) -> L3HDBM0_R {
        L3HDBM0_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - L4PEN0"]
    #[inline(always)]
    pub fn l4pen0(&self) -> L4PEN0_R {
        L4PEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - L4SPM0"]
    #[inline(always)]
    pub fn l4spm0(&self) -> L4SPM0_R {
        L4SPM0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - L4SPIM0"]
    #[inline(always)]
    pub fn l4spim0(&self) -> L4SPIM0_R {
        L4SPIM0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - L4DPM0"]
    #[inline(always)]
    pub fn l4dpm0(&self) -> L4DPM0_R {
        L4DPM0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - L4DPIM0"]
    #[inline(always)]
    pub fn l4dpim0(&self) -> L4DPIM0_R {
        L4DPIM0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L3PEN0"]
    #[inline(always)]
    pub fn l3pen0(&mut self) -> L3PEN0_W {
        L3PEN0_W { w: self }
    }
    #[doc = "Bit 2 - L3SAM0"]
    #[inline(always)]
    pub fn l3sam0(&mut self) -> L3SAM0_W {
        L3SAM0_W { w: self }
    }
    #[doc = "Bit 3 - L3SAIM0"]
    #[inline(always)]
    pub fn l3saim0(&mut self) -> L3SAIM0_W {
        L3SAIM0_W { w: self }
    }
    #[doc = "Bit 4 - L3DAM0"]
    #[inline(always)]
    pub fn l3dam0(&mut self) -> L3DAM0_W {
        L3DAM0_W { w: self }
    }
    #[doc = "Bit 5 - L3DAIM0"]
    #[inline(always)]
    pub fn l3daim0(&mut self) -> L3DAIM0_W {
        L3DAIM0_W { w: self }
    }
    #[doc = "Bits 6:10 - L3HSBM0"]
    #[inline(always)]
    pub fn l3hsbm0(&mut self) -> L3HSBM0_W {
        L3HSBM0_W { w: self }
    }
    #[doc = "Bits 11:15 - L3HDBM0"]
    #[inline(always)]
    pub fn l3hdbm0(&mut self) -> L3HDBM0_W {
        L3HDBM0_W { w: self }
    }
    #[doc = "Bit 16 - L4PEN0"]
    #[inline(always)]
    pub fn l4pen0(&mut self) -> L4PEN0_W {
        L4PEN0_W { w: self }
    }
    #[doc = "Bit 18 - L4SPM0"]
    #[inline(always)]
    pub fn l4spm0(&mut self) -> L4SPM0_W {
        L4SPM0_W { w: self }
    }
    #[doc = "Bit 19 - L4SPIM0"]
    #[inline(always)]
    pub fn l4spim0(&mut self) -> L4SPIM0_W {
        L4SPIM0_W { w: self }
    }
    #[doc = "Bit 20 - L4DPM0"]
    #[inline(always)]
    pub fn l4dpm0(&mut self) -> L4DPM0_W {
        L4DPM0_W { w: self }
    }
    #[doc = "Bit 21 - L4DPIM0"]
    #[inline(always)]
    pub fn l4dpim0(&mut self) -> L4DPIM0_W {
        L4DPIM0_W { w: self }
    }
}
