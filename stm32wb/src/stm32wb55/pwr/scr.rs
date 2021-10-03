#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC2HF` writer - Clear CPU2 Hold interrupt flag"]
pub struct CC2HF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2HF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `C802AF` writer - Clear 802.15.4 end of activity interrupt flag"]
pub struct C802AF_W<'a> {
    w: &'a mut W,
}
impl<'a> C802AF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CBLEAF` writer - Clear BLE end of activity interrupt flag"]
pub struct CBLEAF_W<'a> {
    w: &'a mut W,
}
impl<'a> CBLEAF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CCRPEF` writer - Clear critical radio phase end of activity interrupt flag"]
pub struct CCRPEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRPEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `C802WUF` writer - Clear 802.15.4 wakeup interrupt flag"]
pub struct C802WUF_W<'a> {
    w: &'a mut W,
}
impl<'a> C802WUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CBLEWUF` writer - Clear BLE wakeup interrupt flag"]
pub struct CBLEWUF_W<'a> {
    w: &'a mut W,
}
impl<'a> CBLEWUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CBORHF` writer - Clear BORH interrupt flag"]
pub struct CBORHF_W<'a> {
    w: &'a mut W,
}
impl<'a> CBORHF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CSMPSFBF` writer - Clear SMPS Step Down converter forced in Bypass interrupt flag"]
pub struct CSMPSFBF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSMPSFBF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CWUF5` writer - Clear wakeup flag 5"]
pub struct CWUF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUF5_W<'a> {
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
#[doc = "Field `CWUF4` writer - Clear wakeup flag 4"]
pub struct CWUF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUF4_W<'a> {
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
#[doc = "Field `CWUF3` writer - Clear wakeup flag 3"]
pub struct CWUF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUF3_W<'a> {
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
#[doc = "Field `CWUF2` writer - Clear wakeup flag 2"]
pub struct CWUF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUF2_W<'a> {
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
#[doc = "Field `CWUF1` writer - Clear wakeup flag 1"]
pub struct CWUF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUF1_W<'a> {
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
    #[doc = "Bit 14 - Clear CPU2 Hold interrupt flag"]
    #[inline(always)]
    pub fn cc2hf(&mut self) -> CC2HF_W {
        CC2HF_W { w: self }
    }
    #[doc = "Bit 13 - Clear 802.15.4 end of activity interrupt flag"]
    #[inline(always)]
    pub fn c802af(&mut self) -> C802AF_W {
        C802AF_W { w: self }
    }
    #[doc = "Bit 12 - Clear BLE end of activity interrupt flag"]
    #[inline(always)]
    pub fn cbleaf(&mut self) -> CBLEAF_W {
        CBLEAF_W { w: self }
    }
    #[doc = "Bit 11 - Clear critical radio phase end of activity interrupt flag"]
    #[inline(always)]
    pub fn ccrpef(&mut self) -> CCRPEF_W {
        CCRPEF_W { w: self }
    }
    #[doc = "Bit 10 - Clear 802.15.4 wakeup interrupt flag"]
    #[inline(always)]
    pub fn c802wuf(&mut self) -> C802WUF_W {
        C802WUF_W { w: self }
    }
    #[doc = "Bit 9 - Clear BLE wakeup interrupt flag"]
    #[inline(always)]
    pub fn cblewuf(&mut self) -> CBLEWUF_W {
        CBLEWUF_W { w: self }
    }
    #[doc = "Bit 8 - Clear BORH interrupt flag"]
    #[inline(always)]
    pub fn cborhf(&mut self) -> CBORHF_W {
        CBORHF_W { w: self }
    }
    #[doc = "Bit 7 - Clear SMPS Step Down converter forced in Bypass interrupt flag"]
    #[inline(always)]
    pub fn csmpsfbf(&mut self) -> CSMPSFBF_W {
        CSMPSFBF_W { w: self }
    }
    #[doc = "Bit 4 - Clear wakeup flag 5"]
    #[inline(always)]
    pub fn cwuf5(&mut self) -> CWUF5_W {
        CWUF5_W { w: self }
    }
    #[doc = "Bit 3 - Clear wakeup flag 4"]
    #[inline(always)]
    pub fn cwuf4(&mut self) -> CWUF4_W {
        CWUF4_W { w: self }
    }
    #[doc = "Bit 2 - Clear wakeup flag 3"]
    #[inline(always)]
    pub fn cwuf3(&mut self) -> CWUF3_W {
        CWUF3_W { w: self }
    }
    #[doc = "Bit 1 - Clear wakeup flag 2"]
    #[inline(always)]
    pub fn cwuf2(&mut self) -> CWUF2_W {
        CWUF2_W { w: self }
    }
    #[doc = "Bit 0 - Clear wakeup flag 1"]
    #[inline(always)]
    pub fn cwuf1(&mut self) -> CWUF1_W {
        CWUF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power status clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
