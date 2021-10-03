#[doc = "Register `C1SCR` writer"]
pub struct W(crate::W<C1SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<C1SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH6S` writer - processor 1 Transmit channel 6 status set"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CH5S` writer - processor 1 Transmit channel 5 status set"]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CH4S` writer - processor 1 Transmit channel 4 status set"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CH3S` writer - processor 1 Transmit channel 3 status set"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CH2S` writer - processor 1 Transmit channel 2 status set"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CH1S` writer - processor 1 Transmit channel 1 status set"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CH6C` writer - processor 1 Receive channel 6 status clear"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CH5C` writer - processor 1 Receive channel 5 status clear"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CH4C` writer - processor 1 Receive channel 4 status clear"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CH3C` writer - processor 1 Receive channel 3 status clear"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CH2C` writer - processor 1 Receive channel 2 status clear"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CH1C` writer - processor 1 Receive channel 1 status clear"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 21 - processor 1 Transmit channel 6 status set"]
    #[inline(always)]
    pub fn ch6s(&mut self) -> CH6S_W {
        CH6S_W { w: self }
    }
    #[doc = "Bit 20 - processor 1 Transmit channel 5 status set"]
    #[inline(always)]
    pub fn ch5s(&mut self) -> CH5S_W {
        CH5S_W { w: self }
    }
    #[doc = "Bit 19 - processor 1 Transmit channel 4 status set"]
    #[inline(always)]
    pub fn ch4s(&mut self) -> CH4S_W {
        CH4S_W { w: self }
    }
    #[doc = "Bit 18 - processor 1 Transmit channel 3 status set"]
    #[inline(always)]
    pub fn ch3s(&mut self) -> CH3S_W {
        CH3S_W { w: self }
    }
    #[doc = "Bit 17 - processor 1 Transmit channel 2 status set"]
    #[inline(always)]
    pub fn ch2s(&mut self) -> CH2S_W {
        CH2S_W { w: self }
    }
    #[doc = "Bit 16 - processor 1 Transmit channel 1 status set"]
    #[inline(always)]
    pub fn ch1s(&mut self) -> CH1S_W {
        CH1S_W { w: self }
    }
    #[doc = "Bit 5 - processor 1 Receive channel 6 status clear"]
    #[inline(always)]
    pub fn ch6c(&mut self) -> CH6C_W {
        CH6C_W { w: self }
    }
    #[doc = "Bit 4 - processor 1 Receive channel 5 status clear"]
    #[inline(always)]
    pub fn ch5c(&mut self) -> CH5C_W {
        CH5C_W { w: self }
    }
    #[doc = "Bit 3 - processor 1 Receive channel 4 status clear"]
    #[inline(always)]
    pub fn ch4c(&mut self) -> CH4C_W {
        CH4C_W { w: self }
    }
    #[doc = "Bit 2 - processor 1 Receive channel 3 status clear"]
    #[inline(always)]
    pub fn ch3c(&mut self) -> CH3C_W {
        CH3C_W { w: self }
    }
    #[doc = "Bit 1 - processor 1 Receive channel 2 status clear"]
    #[inline(always)]
    pub fn ch2c(&mut self) -> CH2C_W {
        CH2C_W { w: self }
    }
    #[doc = "Bit 0 - processor 1 Receive channel 1 status clear"]
    #[inline(always)]
    pub fn ch1c(&mut self) -> CH1C_W {
        CH1C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Set or Clear register CPU1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1scr](index.html) module"]
pub struct C1SCR_SPEC;
impl crate::RegisterSpec for C1SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [c1scr::W](W) writer structure"]
impl crate::Writable for C1SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1SCR to value 0"]
impl crate::Resettable for C1SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
