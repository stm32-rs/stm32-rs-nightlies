#[doc = "Reader of register ETH_MACL3L4C1R"]
pub type R = crate::R<u32, super::ETH_MACL3L4C1R>;
#[doc = "Writer for register ETH_MACL3L4C1R"]
pub type W = crate::W<u32, super::ETH_MACL3L4C1R>;
#[doc = "Register ETH_MACL3L4C1R `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACL3L4C1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `L3PEN1`"]
pub type L3PEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L3PEN1`"]
pub struct L3PEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> L3PEN1_W<'a> {
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
#[doc = "Reader of field `L3SAM1`"]
pub type L3SAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L3SAM1`"]
pub struct L3SAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> L3SAM1_W<'a> {
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
#[doc = "Reader of field `L3SAIM1`"]
pub type L3SAIM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L3SAIM1`"]
pub struct L3SAIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> L3SAIM1_W<'a> {
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
#[doc = "Reader of field `L3DAM1`"]
pub type L3DAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L3DAM1`"]
pub struct L3DAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> L3DAM1_W<'a> {
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
#[doc = "Reader of field `L3DAIM1`"]
pub type L3DAIM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L3DAIM1`"]
pub struct L3DAIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> L3DAIM1_W<'a> {
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
#[doc = "Reader of field `L3HSBM1`"]
pub type L3HSBM1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `L3HSBM1`"]
pub struct L3HSBM1_W<'a> {
    w: &'a mut W,
}
impl<'a> L3HSBM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `L3HDBM1`"]
pub type L3HDBM1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `L3HDBM1`"]
pub struct L3HDBM1_W<'a> {
    w: &'a mut W,
}
impl<'a> L3HDBM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `L4PEN1`"]
pub type L4PEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L4PEN1`"]
pub struct L4PEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> L4PEN1_W<'a> {
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
#[doc = "Reader of field `L4SPM1`"]
pub type L4SPM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L4SPM1`"]
pub struct L4SPM1_W<'a> {
    w: &'a mut W,
}
impl<'a> L4SPM1_W<'a> {
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
#[doc = "Reader of field `L4SPIM1`"]
pub type L4SPIM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L4SPIM1`"]
pub struct L4SPIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> L4SPIM1_W<'a> {
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
#[doc = "Reader of field `L4DPM1`"]
pub type L4DPM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L4DPM1`"]
pub struct L4DPM1_W<'a> {
    w: &'a mut W,
}
impl<'a> L4DPM1_W<'a> {
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
#[doc = "Reader of field `L4DPIM1`"]
pub type L4DPIM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L4DPIM1`"]
pub struct L4DPIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> L4DPIM1_W<'a> {
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
    #[doc = "Bit 0 - L3PEN1"]
    #[inline(always)]
    pub fn l3pen1(&self) -> L3PEN1_R {
        L3PEN1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - L3SAM1"]
    #[inline(always)]
    pub fn l3sam1(&self) -> L3SAM1_R {
        L3SAM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - L3SAIM1"]
    #[inline(always)]
    pub fn l3saim1(&self) -> L3SAIM1_R {
        L3SAIM1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - L3DAM1"]
    #[inline(always)]
    pub fn l3dam1(&self) -> L3DAM1_R {
        L3DAM1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - L3DAIM1"]
    #[inline(always)]
    pub fn l3daim1(&self) -> L3DAIM1_R {
        L3DAIM1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:10 - L3HSBM1"]
    #[inline(always)]
    pub fn l3hsbm1(&self) -> L3HSBM1_R {
        L3HSBM1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - L3HDBM1"]
    #[inline(always)]
    pub fn l3hdbm1(&self) -> L3HDBM1_R {
        L3HDBM1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - L4PEN1"]
    #[inline(always)]
    pub fn l4pen1(&self) -> L4PEN1_R {
        L4PEN1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - L4SPM1"]
    #[inline(always)]
    pub fn l4spm1(&self) -> L4SPM1_R {
        L4SPM1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - L4SPIM1"]
    #[inline(always)]
    pub fn l4spim1(&self) -> L4SPIM1_R {
        L4SPIM1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - L4DPM1"]
    #[inline(always)]
    pub fn l4dpm1(&self) -> L4DPM1_R {
        L4DPM1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - L4DPIM1"]
    #[inline(always)]
    pub fn l4dpim1(&self) -> L4DPIM1_R {
        L4DPIM1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L3PEN1"]
    #[inline(always)]
    pub fn l3pen1(&mut self) -> L3PEN1_W {
        L3PEN1_W { w: self }
    }
    #[doc = "Bit 2 - L3SAM1"]
    #[inline(always)]
    pub fn l3sam1(&mut self) -> L3SAM1_W {
        L3SAM1_W { w: self }
    }
    #[doc = "Bit 3 - L3SAIM1"]
    #[inline(always)]
    pub fn l3saim1(&mut self) -> L3SAIM1_W {
        L3SAIM1_W { w: self }
    }
    #[doc = "Bit 4 - L3DAM1"]
    #[inline(always)]
    pub fn l3dam1(&mut self) -> L3DAM1_W {
        L3DAM1_W { w: self }
    }
    #[doc = "Bit 5 - L3DAIM1"]
    #[inline(always)]
    pub fn l3daim1(&mut self) -> L3DAIM1_W {
        L3DAIM1_W { w: self }
    }
    #[doc = "Bits 6:10 - L3HSBM1"]
    #[inline(always)]
    pub fn l3hsbm1(&mut self) -> L3HSBM1_W {
        L3HSBM1_W { w: self }
    }
    #[doc = "Bits 11:15 - L3HDBM1"]
    #[inline(always)]
    pub fn l3hdbm1(&mut self) -> L3HDBM1_W {
        L3HDBM1_W { w: self }
    }
    #[doc = "Bit 16 - L4PEN1"]
    #[inline(always)]
    pub fn l4pen1(&mut self) -> L4PEN1_W {
        L4PEN1_W { w: self }
    }
    #[doc = "Bit 18 - L4SPM1"]
    #[inline(always)]
    pub fn l4spm1(&mut self) -> L4SPM1_W {
        L4SPM1_W { w: self }
    }
    #[doc = "Bit 19 - L4SPIM1"]
    #[inline(always)]
    pub fn l4spim1(&mut self) -> L4SPIM1_W {
        L4SPIM1_W { w: self }
    }
    #[doc = "Bit 20 - L4DPM1"]
    #[inline(always)]
    pub fn l4dpm1(&mut self) -> L4DPM1_W {
        L4DPM1_W { w: self }
    }
    #[doc = "Bit 21 - L4DPIM1"]
    #[inline(always)]
    pub fn l4dpim1(&mut self) -> L4DPIM1_W {
        L4DPIM1_W { w: self }
    }
}
