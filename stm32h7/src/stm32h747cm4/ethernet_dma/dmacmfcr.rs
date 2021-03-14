#[doc = "Reader of register DMACMFCR"]
pub type R = crate::R<u32, super::DMACMFCR>;
#[doc = "Writer for register DMACMFCR"]
pub type W = crate::W<u32, super::DMACMFCR>;
#[doc = "Register DMACMFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACMFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MFCO`"]
pub type MFCO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MFCO`"]
pub struct MFCO_W<'a> {
    w: &'a mut W,
}
impl<'a> MFCO_W<'a> {
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
#[doc = "Reader of field `MFC`"]
pub type MFC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MFC`"]
pub struct MFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Overflow status of the MFC Counter"]
    #[inline(always)]
    pub fn mfco(&self) -> MFCO_R {
        MFCO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:10 - Dropped Packet Counters"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Overflow status of the MFC Counter"]
    #[inline(always)]
    pub fn mfco(&mut self) -> MFCO_W {
        MFCO_W { w: self }
    }
    #[doc = "Bits 0:10 - Dropped Packet Counters"]
    #[inline(always)]
    pub fn mfc(&mut self) -> MFC_W {
        MFC_W { w: self }
    }
}
