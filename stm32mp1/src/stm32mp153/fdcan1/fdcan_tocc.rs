#[doc = "Register `FDCAN_TOCC` reader"]
pub struct R(crate::R<FDCAN_TOCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TOCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TOCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TOCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TOCC` writer"]
pub struct W(crate::W<FDCAN_TOCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TOCC_SPEC>;
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
impl From<crate::W<FDCAN_TOCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TOCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETOC` reader - ETOC"]
pub struct ETOC_R(crate::FieldReader<bool, bool>);
impl ETOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETOC` writer - ETOC"]
pub struct ETOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETOC_W<'a> {
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
#[doc = "Field `TOS` reader - TOS"]
pub struct TOS_R(crate::FieldReader<u8, u8>);
impl TOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOS` writer - TOS"]
pub struct TOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `TOP` reader - TOP"]
pub struct TOP_R(crate::FieldReader<u16, u16>);
impl TOP_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOP` writer - TOP"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ETOC"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - TOS"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 16:31 - TOP"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - ETOC"]
    #[inline(always)]
    pub fn etoc(&mut self) -> ETOC_W {
        ETOC_W { w: self }
    }
    #[doc = "Bits 1:2 - TOS"]
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W {
        TOS_W { w: self }
    }
    #[doc = "Bits 16:31 - TOP"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN timeout counter configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tocc](index.html) module"]
pub struct FDCAN_TOCC_SPEC;
impl crate::RegisterSpec for FDCAN_TOCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_tocc::R](R) reader structure"]
impl crate::Readable for FDCAN_TOCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_tocc::W](W) writer structure"]
impl crate::Writable for FDCAN_TOCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TOCC to value 0xffff_0000"]
impl crate::Resettable for FDCAN_TOCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}
