#[doc = "Writer for register C2SCR"]
pub type W = crate::W<u32, super::C2SCR>;
#[doc = "Register C2SCR `reset()`'s with value 0"]
impl crate::ResetValue for super::C2SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CH6S`"]
pub struct CH6S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6S_W<'a> {
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
#[doc = "Write proxy for field `CH5S`"]
pub struct CH5S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5S_W<'a> {
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
#[doc = "Write proxy for field `CH4S`"]
pub struct CH4S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4S_W<'a> {
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
#[doc = "Write proxy for field `CH3S`"]
pub struct CH3S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3S_W<'a> {
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
#[doc = "Write proxy for field `CH2S`"]
pub struct CH2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2S_W<'a> {
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
#[doc = "Write proxy for field `CH1S`"]
pub struct CH1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1S_W<'a> {
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
#[doc = "Write proxy for field `CH6C`"]
pub struct CH6C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6C_W<'a> {
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
#[doc = "Write proxy for field `CH5C`"]
pub struct CH5C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5C_W<'a> {
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
#[doc = "Write proxy for field `CH4C`"]
pub struct CH4C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4C_W<'a> {
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
#[doc = "Write proxy for field `CH3C`"]
pub struct CH3C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3C_W<'a> {
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
#[doc = "Write proxy for field `CH2C`"]
pub struct CH2C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2C_W<'a> {
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
#[doc = "Write proxy for field `CH1C`"]
pub struct CH1C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1C_W<'a> {
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
impl W {
    #[doc = "Bit 21 - processor 2 Transmit channel 6 status set"]
    #[inline(always)]
    pub fn ch6s(&mut self) -> CH6S_W {
        CH6S_W { w: self }
    }
    #[doc = "Bit 20 - processor 2 Transmit channel 5 status set"]
    #[inline(always)]
    pub fn ch5s(&mut self) -> CH5S_W {
        CH5S_W { w: self }
    }
    #[doc = "Bit 19 - processor 2 Transmit channel 4 status set"]
    #[inline(always)]
    pub fn ch4s(&mut self) -> CH4S_W {
        CH4S_W { w: self }
    }
    #[doc = "Bit 18 - processor 2 Transmit channel 3 status set"]
    #[inline(always)]
    pub fn ch3s(&mut self) -> CH3S_W {
        CH3S_W { w: self }
    }
    #[doc = "Bit 17 - processor 2 Transmit channel 2 status set"]
    #[inline(always)]
    pub fn ch2s(&mut self) -> CH2S_W {
        CH2S_W { w: self }
    }
    #[doc = "Bit 16 - processor 2 Transmit channel 1 status set"]
    #[inline(always)]
    pub fn ch1s(&mut self) -> CH1S_W {
        CH1S_W { w: self }
    }
    #[doc = "Bit 5 - processor 2 Receive channel 6 status clear"]
    #[inline(always)]
    pub fn ch6c(&mut self) -> CH6C_W {
        CH6C_W { w: self }
    }
    #[doc = "Bit 4 - processor 2 Receive channel 5 status clear"]
    #[inline(always)]
    pub fn ch5c(&mut self) -> CH5C_W {
        CH5C_W { w: self }
    }
    #[doc = "Bit 3 - processor 2 Receive channel 4 status clear"]
    #[inline(always)]
    pub fn ch4c(&mut self) -> CH4C_W {
        CH4C_W { w: self }
    }
    #[doc = "Bit 2 - processor 2 Receive channel 3 status clear"]
    #[inline(always)]
    pub fn ch3c(&mut self) -> CH3C_W {
        CH3C_W { w: self }
    }
    #[doc = "Bit 1 - processor 2 Receive channel 2 status clear"]
    #[inline(always)]
    pub fn ch2c(&mut self) -> CH2C_W {
        CH2C_W { w: self }
    }
    #[doc = "Bit 0 - processor 2 Receive channel 1 status clear"]
    #[inline(always)]
    pub fn ch1c(&mut self) -> CH1C_W {
        CH1C_W { w: self }
    }
}
