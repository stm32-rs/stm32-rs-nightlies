#[doc = "Register `DDRCTRL_RFSHCTL0` reader"]
pub struct R(crate::R<DDRCTRL_RFSHCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_RFSHCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_RFSHCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_RFSHCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_RFSHCTL0` writer"]
pub struct W(crate::W<DDRCTRL_RFSHCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_RFSHCTL0_SPEC>;
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
impl From<crate::W<DDRCTRL_RFSHCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_RFSHCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER_BANK_REFRESH` reader - PER_BANK_REFRESH"]
pub struct PER_BANK_REFRESH_R(crate::FieldReader<bool, bool>);
impl PER_BANK_REFRESH_R {
    pub(crate) fn new(bits: bool) -> Self {
        PER_BANK_REFRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_BANK_REFRESH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER_BANK_REFRESH` writer - PER_BANK_REFRESH"]
pub struct PER_BANK_REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_BANK_REFRESH_W<'a> {
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
#[doc = "Field `REFRESH_BURST` reader - REFRESH_BURST"]
pub struct REFRESH_BURST_R(crate::FieldReader<u8, u8>);
impl REFRESH_BURST_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFRESH_BURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFRESH_BURST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFRESH_BURST` writer - REFRESH_BURST"]
pub struct REFRESH_BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_BURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | ((value as u32 & 0x1f) << 4);
        self.w
    }
}
#[doc = "Field `REFRESH_TO_X32` reader - REFRESH_TO_X32"]
pub struct REFRESH_TO_X32_R(crate::FieldReader<u8, u8>);
impl REFRESH_TO_X32_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFRESH_TO_X32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFRESH_TO_X32_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFRESH_TO_X32` writer - REFRESH_TO_X32"]
pub struct REFRESH_TO_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_TO_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | ((value as u32 & 0x1f) << 12);
        self.w
    }
}
#[doc = "Field `REFRESH_MARGIN` reader - REFRESH_MARGIN"]
pub struct REFRESH_MARGIN_R(crate::FieldReader<u8, u8>);
impl REFRESH_MARGIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFRESH_MARGIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFRESH_MARGIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFRESH_MARGIN` writer - REFRESH_MARGIN"]
pub struct REFRESH_MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_MARGIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - PER_BANK_REFRESH"]
    #[inline(always)]
    pub fn per_bank_refresh(&self) -> PER_BANK_REFRESH_R {
        PER_BANK_REFRESH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:8 - REFRESH_BURST"]
    #[inline(always)]
    pub fn refresh_burst(&self) -> REFRESH_BURST_R {
        REFRESH_BURST_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - REFRESH_TO_X32"]
    #[inline(always)]
    pub fn refresh_to_x32(&self) -> REFRESH_TO_X32_R {
        REFRESH_TO_X32_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - REFRESH_MARGIN"]
    #[inline(always)]
    pub fn refresh_margin(&self) -> REFRESH_MARGIN_R {
        REFRESH_MARGIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - PER_BANK_REFRESH"]
    #[inline(always)]
    pub fn per_bank_refresh(&mut self) -> PER_BANK_REFRESH_W {
        PER_BANK_REFRESH_W { w: self }
    }
    #[doc = "Bits 4:8 - REFRESH_BURST"]
    #[inline(always)]
    pub fn refresh_burst(&mut self) -> REFRESH_BURST_W {
        REFRESH_BURST_W { w: self }
    }
    #[doc = "Bits 12:16 - REFRESH_TO_X32"]
    #[inline(always)]
    pub fn refresh_to_x32(&mut self) -> REFRESH_TO_X32_W {
        REFRESH_TO_X32_W { w: self }
    }
    #[doc = "Bits 20:23 - REFRESH_MARGIN"]
    #[inline(always)]
    pub fn refresh_margin(&mut self) -> REFRESH_MARGIN_W {
        REFRESH_MARGIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL refresh control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_rfshctl0](index.html) module"]
pub struct DDRCTRL_RFSHCTL0_SPEC;
impl crate::RegisterSpec for DDRCTRL_RFSHCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_rfshctl0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_RFSHCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_rfshctl0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_RFSHCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_RFSHCTL0 to value 0x0021_0000"]
impl crate::Resettable for DDRCTRL_RFSHCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0021_0000
    }
}
