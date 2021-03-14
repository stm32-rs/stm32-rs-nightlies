#[doc = "Reader of register MACPOCR"]
pub type R = crate::R<u32, super::MACPOCR>;
#[doc = "Writer for register MACPOCR"]
pub type W = crate::W<u32, super::MACPOCR>;
#[doc = "Register MACPOCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACPOCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PTOEN`"]
pub type PTOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTOEN`"]
pub struct PTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTOEN_W<'a> {
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
#[doc = "Reader of field `ASYNCEN`"]
pub type ASYNCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASYNCEN`"]
pub struct ASYNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCEN_W<'a> {
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
#[doc = "Reader of field `APDREQEN`"]
pub type APDREQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APDREQEN`"]
pub struct APDREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APDREQEN_W<'a> {
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
#[doc = "Reader of field `ASYNCTRIG`"]
pub type ASYNCTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASYNCTRIG`"]
pub struct ASYNCTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCTRIG_W<'a> {
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
#[doc = "Reader of field `APDREQTRIG`"]
pub type APDREQTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APDREQTRIG`"]
pub struct APDREQTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> APDREQTRIG_W<'a> {
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
#[doc = "Reader of field `DRRDIS`"]
pub type DRRDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRRDIS`"]
pub struct DRRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRRDIS_W<'a> {
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
#[doc = "Reader of field `DN`"]
pub type DN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DN`"]
pub struct DN_W<'a> {
    w: &'a mut W,
}
impl<'a> DN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PTP Offload Enable"]
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Automatic PTP SYNC message Enable"]
    #[inline(always)]
    pub fn asyncen(&self) -> ASYNCEN_R {
        ASYNCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Automatic PTP Pdelay_Req message Enable"]
    #[inline(always)]
    pub fn apdreqen(&self) -> APDREQEN_R {
        APDREQEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Automatic PTP SYNC message Trigger"]
    #[inline(always)]
    pub fn asynctrig(&self) -> ASYNCTRIG_R {
        ASYNCTRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Automatic PTP Pdelay_Req message Trigger"]
    #[inline(always)]
    pub fn apdreqtrig(&self) -> APDREQTRIG_R {
        APDREQTRIG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable PTO Delay Request/Response response generation"]
    #[inline(always)]
    pub fn drrdis(&self) -> DRRDIS_R {
        DRRDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Domain Number"]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PTP Offload Enable"]
    #[inline(always)]
    pub fn ptoen(&mut self) -> PTOEN_W {
        PTOEN_W { w: self }
    }
    #[doc = "Bit 1 - Automatic PTP SYNC message Enable"]
    #[inline(always)]
    pub fn asyncen(&mut self) -> ASYNCEN_W {
        ASYNCEN_W { w: self }
    }
    #[doc = "Bit 2 - Automatic PTP Pdelay_Req message Enable"]
    #[inline(always)]
    pub fn apdreqen(&mut self) -> APDREQEN_W {
        APDREQEN_W { w: self }
    }
    #[doc = "Bit 4 - Automatic PTP SYNC message Trigger"]
    #[inline(always)]
    pub fn asynctrig(&mut self) -> ASYNCTRIG_W {
        ASYNCTRIG_W { w: self }
    }
    #[doc = "Bit 5 - Automatic PTP Pdelay_Req message Trigger"]
    #[inline(always)]
    pub fn apdreqtrig(&mut self) -> APDREQTRIG_W {
        APDREQTRIG_W { w: self }
    }
    #[doc = "Bit 6 - Disable PTO Delay Request/Response response generation"]
    #[inline(always)]
    pub fn drrdis(&mut self) -> DRRDIS_W {
        DRRDIS_W { w: self }
    }
    #[doc = "Bits 8:15 - Domain Number"]
    #[inline(always)]
    pub fn dn(&mut self) -> DN_W {
        DN_W { w: self }
    }
}
