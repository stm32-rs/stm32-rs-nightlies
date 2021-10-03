#[doc = "Register `FDCAN_TTTMK` reader"]
pub struct R(crate::R<FDCAN_TTTMK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTTMK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTTMK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTTMK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTTMK` writer"]
pub struct W(crate::W<FDCAN_TTTMK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTTMK_SPEC>;
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
impl From<crate::W<FDCAN_TTTMK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTTMK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TM` reader - Time Mark"]
pub struct TM_R(crate::FieldReader<u16, u16>);
impl TM_R {
    pub(crate) fn new(bits: u16) -> Self {
        TM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TM` writer - Time Mark"]
pub struct TM_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `TICC` reader - Time Mark Cycle Code"]
pub struct TICC_R(crate::FieldReader<u8, u8>);
impl TICC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TICC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TICC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICC` writer - Time Mark Cycle Code"]
pub struct TICC_W<'a> {
    w: &'a mut W,
}
impl<'a> TICC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `LCKM` reader - TT Time Mark Register Locked"]
pub struct LCKM_R(crate::FieldReader<bool, bool>);
impl LCKM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKM` writer - TT Time Mark Register Locked"]
pub struct LCKM_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W {
        TM_W { w: self }
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    pub fn ticc(&mut self) -> TICC_W {
        TICC_W { w: self }
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    pub fn lckm(&mut self) -> LCKM_W {
        LCKM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Time Mark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tttmk](index.html) module"]
pub struct FDCAN_TTTMK_SPEC;
impl crate::RegisterSpec for FDCAN_TTTMK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_tttmk::R](R) reader structure"]
impl crate::Readable for FDCAN_TTTMK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_tttmk::W](W) writer structure"]
impl crate::Writable for FDCAN_TTTMK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TTTMK to value 0"]
impl crate::Resettable for FDCAN_TTTMK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
