#[doc = "Reader of register DDRPHYC_DDR3_MR1"]
pub type R = crate::R<u16, super::DDRPHYC_DDR3_MR1>;
#[doc = "Writer for register DDRPHYC_DDR3_MR1"]
pub type W = crate::W<u16, super::DDRPHYC_DDR3_MR1>;
#[doc = "Register DDRPHYC_DDR3_MR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_DDR3_MR1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DE`"]
pub type DE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DE`"]
pub struct DE_W<'a> {
    w: &'a mut W,
}
impl<'a> DE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DIC0`"]
pub type DIC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIC0`"]
pub struct DIC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIC0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RTT0`"]
pub type RTT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTT0`"]
pub struct RTT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `AL`"]
pub type AL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AL`"]
pub struct AL_W<'a> {
    w: &'a mut W,
}
impl<'a> AL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u16) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `DIC1`"]
pub type DIC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIC1`"]
pub struct DIC1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTT1`"]
pub type RTT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTT1`"]
pub struct RTT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LEVEL`"]
pub type LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEVEL`"]
pub struct LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTT2`"]
pub type RTT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTT2`"]
pub struct RTT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TDQS`"]
pub type TDQS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDQS`"]
pub struct TDQS_W<'a> {
    w: &'a mut W,
}
impl<'a> TDQS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `QOFF`"]
pub type QOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QOFF`"]
pub struct QOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> QOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DE"]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIC0"]
    #[inline(always)]
    pub fn dic0(&self) -> DIC0_R {
        DIC0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTT0"]
    #[inline(always)]
    pub fn rtt0(&self) -> RTT0_R {
        RTT0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - AL"]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - DIC1"]
    #[inline(always)]
    pub fn dic1(&self) -> DIC1_R {
        DIC1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTT1"]
    #[inline(always)]
    pub fn rtt1(&self) -> RTT1_R {
        RTT1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LEVEL"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTT2"]
    #[inline(always)]
    pub fn rtt2(&self) -> RTT2_R {
        RTT2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TDQS"]
    #[inline(always)]
    pub fn tdqs(&self) -> TDQS_R {
        TDQS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - QOFF"]
    #[inline(always)]
    pub fn qoff(&self) -> QOFF_R {
        QOFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DE"]
    #[inline(always)]
    pub fn de(&mut self) -> DE_W {
        DE_W { w: self }
    }
    #[doc = "Bit 1 - DIC0"]
    #[inline(always)]
    pub fn dic0(&mut self) -> DIC0_W {
        DIC0_W { w: self }
    }
    #[doc = "Bit 2 - RTT0"]
    #[inline(always)]
    pub fn rtt0(&mut self) -> RTT0_W {
        RTT0_W { w: self }
    }
    #[doc = "Bits 3:4 - AL"]
    #[inline(always)]
    pub fn al(&mut self) -> AL_W {
        AL_W { w: self }
    }
    #[doc = "Bit 5 - DIC1"]
    #[inline(always)]
    pub fn dic1(&mut self) -> DIC1_W {
        DIC1_W { w: self }
    }
    #[doc = "Bit 6 - RTT1"]
    #[inline(always)]
    pub fn rtt1(&mut self) -> RTT1_W {
        RTT1_W { w: self }
    }
    #[doc = "Bit 7 - LEVEL"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W {
        LEVEL_W { w: self }
    }
    #[doc = "Bit 9 - RTT2"]
    #[inline(always)]
    pub fn rtt2(&mut self) -> RTT2_W {
        RTT2_W { w: self }
    }
    #[doc = "Bit 11 - TDQS"]
    #[inline(always)]
    pub fn tdqs(&mut self) -> TDQS_W {
        TDQS_W { w: self }
    }
    #[doc = "Bit 12 - QOFF"]
    #[inline(always)]
    pub fn qoff(&mut self) -> QOFF_W {
        QOFF_W { w: self }
    }
}
