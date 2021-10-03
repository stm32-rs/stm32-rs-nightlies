#[doc = "Register `WKUPCR` reader"]
pub struct R(crate::R<WKUPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUPCR` writer"]
pub struct W(crate::W<WKUPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUPCR_SPEC>;
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
impl From<crate::W<WKUPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPC1` reader - Clear Wakeup pin flag for WKUPC1"]
pub struct WKUPC1_R(crate::FieldReader<bool, bool>);
impl WKUPC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPC1` writer - Clear Wakeup pin flag for WKUPC1"]
pub struct WKUPC1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC1_W<'a> {
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
#[doc = "Field `WKUPC2` reader - Clear Wakeup pin flag for WKUPC2"]
pub struct WKUPC2_R(crate::FieldReader<bool, bool>);
impl WKUPC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPC2` writer - Clear Wakeup pin flag for WKUPC2"]
pub struct WKUPC2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC2_W<'a> {
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
#[doc = "Field `WKUPC4` reader - Clear Wakeup pin flag for WKUPC4"]
pub struct WKUPC4_R(crate::FieldReader<bool, bool>);
impl WKUPC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPC4` writer - Clear Wakeup pin flag for WKUPC4"]
pub struct WKUPC4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC4_W<'a> {
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
#[doc = "Field `WKUPC6` reader - Clear Wakeup pin flag for WKUPC6"]
pub struct WKUPC6_R(crate::FieldReader<bool, bool>);
impl WKUPC6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPC6` writer - Clear Wakeup pin flag for WKUPC6"]
pub struct WKUPC6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC6_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Clear Wakeup pin flag for WKUPC1"]
    #[inline(always)]
    pub fn wkupc1(&self) -> WKUPC1_R {
        WKUPC1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear Wakeup pin flag for WKUPC2"]
    #[inline(always)]
    pub fn wkupc2(&self) -> WKUPC2_R {
        WKUPC2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear Wakeup pin flag for WKUPC4"]
    #[inline(always)]
    pub fn wkupc4(&self) -> WKUPC4_R {
        WKUPC4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear Wakeup pin flag for WKUPC6"]
    #[inline(always)]
    pub fn wkupc6(&self) -> WKUPC6_R {
        WKUPC6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Wakeup pin flag for WKUPC1"]
    #[inline(always)]
    pub fn wkupc1(&mut self) -> WKUPC1_W {
        WKUPC1_W { w: self }
    }
    #[doc = "Bit 1 - Clear Wakeup pin flag for WKUPC2"]
    #[inline(always)]
    pub fn wkupc2(&mut self) -> WKUPC2_W {
        WKUPC2_W { w: self }
    }
    #[doc = "Bit 3 - Clear Wakeup pin flag for WKUPC4"]
    #[inline(always)]
    pub fn wkupc4(&mut self) -> WKUPC4_W {
        WKUPC4_W { w: self }
    }
    #[doc = "Bit 5 - Clear Wakeup pin flag for WKUPC6"]
    #[inline(always)]
    pub fn wkupc6(&mut self) -> WKUPC6_W {
        WKUPC6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkupcr](index.html) module"]
pub struct WKUPCR_SPEC;
impl crate::RegisterSpec for WKUPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkupcr::R](R) reader structure"]
impl crate::Readable for WKUPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkupcr::W](W) writer structure"]
impl crate::Writable for WKUPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WKUPCR to value 0"]
impl crate::Resettable for WKUPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
