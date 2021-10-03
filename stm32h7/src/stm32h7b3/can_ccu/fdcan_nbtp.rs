#[doc = "Register `FDCAN_NBTP` reader"]
pub struct R(crate::R<FDCAN_NBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_NBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_NBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_NBTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_NBTP` writer"]
pub struct W(crate::W<FDCAN_NBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_NBTP_SPEC>;
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
impl From<crate::W<FDCAN_NBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_NBTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSJW` reader - NSJW: Nominal (Re)Synchronization Jump Width"]
pub struct NSJW_R(crate::FieldReader<u8, u8>);
impl NSJW_R {
    pub(crate) fn new(bits: u8) -> Self {
        NSJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSJW` writer - NSJW: Nominal (Re)Synchronization Jump Width"]
pub struct NSJW_W<'a> {
    w: &'a mut W,
}
impl<'a> NSJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
#[doc = "Field `NBRP` reader - Bit Rate Prescaler"]
pub struct NBRP_R(crate::FieldReader<u16, u16>);
impl NBRP_R {
    pub(crate) fn new(bits: u16) -> Self {
        NBRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBRP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBRP` writer - Bit Rate Prescaler"]
pub struct NBRP_W<'a> {
    w: &'a mut W,
}
impl<'a> NBRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `NTSEG1` reader - Nominal Time segment before sample point"]
pub struct NTSEG1_R(crate::FieldReader<u8, u8>);
impl NTSEG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        NTSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NTSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NTSEG1` writer - Nominal Time segment before sample point"]
pub struct NTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> NTSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TSEG2` reader - Nominal Time segment after sample point"]
pub struct TSEG2_R(crate::FieldReader<u8, u8>);
impl TSEG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEG2` writer - Nominal Time segment after sample point"]
pub struct TSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - NSJW: Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24 - Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - NSJW: Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&mut self) -> NSJW_W {
        NSJW_W { w: self }
    }
    #[doc = "Bits 16:24 - Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&mut self) -> NBRP_W {
        NBRP_W { w: self }
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    pub fn ntseg1(&mut self) -> NTSEG1_W {
        NTSEG1_W { w: self }
    }
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W {
        TSEG2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_nbtp](index.html) module"]
pub struct FDCAN_NBTP_SPEC;
impl crate::RegisterSpec for FDCAN_NBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_nbtp::R](R) reader structure"]
impl crate::Readable for FDCAN_NBTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_nbtp::W](W) writer structure"]
impl crate::Writable for FDCAN_NBTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_NBTP to value 0x0a33"]
impl crate::Resettable for FDCAN_NBTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a33
    }
}
