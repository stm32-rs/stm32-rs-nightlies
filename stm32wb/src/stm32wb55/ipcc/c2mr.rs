#[doc = "Reader of register C2MR"]
pub type R = crate::R<u32, super::C2MR>;
#[doc = "Writer for register C2MR"]
pub type W = crate::W<u32, super::C2MR>;
#[doc = "Register C2MR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::C2MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `CH6FM`"]
pub type CH6FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6FM`"]
pub struct CH6FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6FM_W<'a> {
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
#[doc = "Reader of field `CH5FM`"]
pub type CH5FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5FM`"]
pub struct CH5FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5FM_W<'a> {
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
#[doc = "Reader of field `CH4FM`"]
pub type CH4FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4FM`"]
pub struct CH4FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4FM_W<'a> {
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
#[doc = "Reader of field `CH3FM`"]
pub type CH3FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3FM`"]
pub struct CH3FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3FM_W<'a> {
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
#[doc = "Reader of field `CH2FM`"]
pub type CH2FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2FM`"]
pub struct CH2FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2FM_W<'a> {
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
#[doc = "Reader of field `CH1FM`"]
pub type CH1FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1FM`"]
pub struct CH1FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1FM_W<'a> {
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
#[doc = "Reader of field `CH6OM`"]
pub type CH6OM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6OM`"]
pub struct CH6OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6OM_W<'a> {
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
#[doc = "Reader of field `CH5OM`"]
pub type CH5OM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5OM`"]
pub struct CH5OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5OM_W<'a> {
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
#[doc = "Reader of field `CH4OM`"]
pub type CH4OM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4OM`"]
pub struct CH4OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4OM_W<'a> {
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
#[doc = "Reader of field `CH3OM`"]
pub type CH3OM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3OM`"]
pub struct CH3OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OM_W<'a> {
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
#[doc = "Reader of field `CH2OM`"]
pub type CH2OM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2OM`"]
pub struct CH2OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OM_W<'a> {
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
#[doc = "Reader of field `CH1OM`"]
pub type CH1OM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1OM`"]
pub struct CH1OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OM_W<'a> {
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
impl R {
    #[doc = "Bit 21 - processor 2 Transmit channel 6 free interrupt mask"]
    #[inline(always)]
    pub fn ch6fm(&self) -> CH6FM_R {
        CH6FM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - processor 2 Transmit channel 5 free interrupt mask"]
    #[inline(always)]
    pub fn ch5fm(&self) -> CH5FM_R {
        CH5FM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - processor 2 Transmit channel 4 free interrupt mask"]
    #[inline(always)]
    pub fn ch4fm(&self) -> CH4FM_R {
        CH4FM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - processor 2 Transmit channel 3 free interrupt mask"]
    #[inline(always)]
    pub fn ch3fm(&self) -> CH3FM_R {
        CH3FM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - processor 2 Transmit channel 2 free interrupt mask"]
    #[inline(always)]
    pub fn ch2fm(&self) -> CH2FM_R {
        CH2FM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - processor 2 Transmit channel 1 free interrupt mask"]
    #[inline(always)]
    pub fn ch1fm(&self) -> CH1FM_R {
        CH1FM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 5 - processor 2 Receive channel 6 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - processor 2 Receive channel 5 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - processor 2 Receive channel 4 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - processor 2 Receive channel 3 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - processor 2 Receive channel 2 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - processor 2 Receive channel 1 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - processor 2 Transmit channel 6 free interrupt mask"]
    #[inline(always)]
    pub fn ch6fm(&mut self) -> CH6FM_W {
        CH6FM_W { w: self }
    }
    #[doc = "Bit 20 - processor 2 Transmit channel 5 free interrupt mask"]
    #[inline(always)]
    pub fn ch5fm(&mut self) -> CH5FM_W {
        CH5FM_W { w: self }
    }
    #[doc = "Bit 19 - processor 2 Transmit channel 4 free interrupt mask"]
    #[inline(always)]
    pub fn ch4fm(&mut self) -> CH4FM_W {
        CH4FM_W { w: self }
    }
    #[doc = "Bit 18 - processor 2 Transmit channel 3 free interrupt mask"]
    #[inline(always)]
    pub fn ch3fm(&mut self) -> CH3FM_W {
        CH3FM_W { w: self }
    }
    #[doc = "Bit 17 - processor 2 Transmit channel 2 free interrupt mask"]
    #[inline(always)]
    pub fn ch2fm(&mut self) -> CH2FM_W {
        CH2FM_W { w: self }
    }
    #[doc = "Bit 16 - processor 2 Transmit channel 1 free interrupt mask"]
    #[inline(always)]
    pub fn ch1fm(&mut self) -> CH1FM_W {
        CH1FM_W { w: self }
    }
    #[doc = "Bit 5 - processor 2 Receive channel 6 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch6om(&mut self) -> CH6OM_W {
        CH6OM_W { w: self }
    }
    #[doc = "Bit 4 - processor 2 Receive channel 5 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch5om(&mut self) -> CH5OM_W {
        CH5OM_W { w: self }
    }
    #[doc = "Bit 3 - processor 2 Receive channel 4 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch4om(&mut self) -> CH4OM_W {
        CH4OM_W { w: self }
    }
    #[doc = "Bit 2 - processor 2 Receive channel 3 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch3om(&mut self) -> CH3OM_W {
        CH3OM_W { w: self }
    }
    #[doc = "Bit 1 - processor 2 Receive channel 2 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch2om(&mut self) -> CH2OM_W {
        CH2OM_W { w: self }
    }
    #[doc = "Bit 0 - processor 2 Receive channel 1 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch1om(&mut self) -> CH1OM_W {
        CH1OM_W { w: self }
    }
}
