#[doc = "Writer for register SCR"]
pub type W = crate::W<u32, super::SCR>;
#[doc = "Register SCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CC2HF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `C802AF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `CBLEAF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `CCRPEF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `C802WUF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `CBLEWUF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `CBORHF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `CSMPSFBF`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `CWUF5`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `CWUF4`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `CWUF3`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `CWUF2`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `CWUF1`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
}
