#[doc = "Register `MACIER` reader"]
pub struct R(crate::R<MACIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACIER` writer"]
pub struct W(crate::W<MACIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACIER_SPEC>;
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
impl From<crate::W<MACIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHYIE` reader - PHY Interrupt Enable"]
pub struct PHYIE_R(crate::FieldReader<bool, bool>);
impl PHYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYIE` writer - PHY Interrupt Enable"]
pub struct PHYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYIE_W<'a> {
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
#[doc = "Field `PMTIE` reader - PMT Interrupt Enable"]
pub struct PMTIE_R(crate::FieldReader<bool, bool>);
impl PMTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMTIE` writer - PMT Interrupt Enable"]
pub struct PMTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMTIE_W<'a> {
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
#[doc = "Field `LPIIE` reader - LPI Interrupt Enable"]
pub struct LPIIE_R(crate::FieldReader<bool, bool>);
impl LPIIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPIIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPIIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPIIE` writer - LPI Interrupt Enable"]
pub struct LPIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIIE_W<'a> {
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
#[doc = "Field `TSIE` reader - Timestamp Interrupt Enable"]
pub struct TSIE_R(crate::FieldReader<bool, bool>);
impl TSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIE` writer - Timestamp Interrupt Enable"]
pub struct TSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIE_W<'a> {
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
#[doc = "Field `TXSTSIE` reader - Transmit Status Interrupt Enable"]
pub struct TXSTSIE_R(crate::FieldReader<bool, bool>);
impl TXSTSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSTSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTSIE` writer - Transmit Status Interrupt Enable"]
pub struct TXSTSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTSIE_W<'a> {
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
#[doc = "Field `RXSTSIE` reader - Receive Status Interrupt Enable"]
pub struct RXSTSIE_R(crate::FieldReader<bool, bool>);
impl RXSTSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTSIE` writer - Receive Status Interrupt Enable"]
pub struct RXSTSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTSIE_W<'a> {
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
impl R {
    #[doc = "Bit 3 - PHY Interrupt Enable"]
    #[inline(always)]
    pub fn phyie(&self) -> PHYIE_R {
        PHYIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMT Interrupt Enable"]
    #[inline(always)]
    pub fn pmtie(&self) -> PMTIE_R {
        PMTIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPI Interrupt Enable"]
    #[inline(always)]
    pub fn lpiie(&self) -> LPIIE_R {
        LPIIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timestamp Interrupt Enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt Enable"]
    #[inline(always)]
    pub fn txstsie(&self) -> TXSTSIE_R {
        TXSTSIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Status Interrupt Enable"]
    #[inline(always)]
    pub fn rxstsie(&self) -> RXSTSIE_R {
        RXSTSIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PHY Interrupt Enable"]
    #[inline(always)]
    pub fn phyie(&mut self) -> PHYIE_W {
        PHYIE_W { w: self }
    }
    #[doc = "Bit 4 - PMT Interrupt Enable"]
    #[inline(always)]
    pub fn pmtie(&mut self) -> PMTIE_W {
        PMTIE_W { w: self }
    }
    #[doc = "Bit 5 - LPI Interrupt Enable"]
    #[inline(always)]
    pub fn lpiie(&mut self) -> LPIIE_W {
        LPIIE_W { w: self }
    }
    #[doc = "Bit 12 - Timestamp Interrupt Enable"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W {
        TSIE_W { w: self }
    }
    #[doc = "Bit 13 - Transmit Status Interrupt Enable"]
    #[inline(always)]
    pub fn txstsie(&mut self) -> TXSTSIE_W {
        TXSTSIE_W { w: self }
    }
    #[doc = "Bit 14 - Receive Status Interrupt Enable"]
    #[inline(always)]
    pub fn rxstsie(&mut self) -> RXSTSIE_W {
        RXSTSIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macier](index.html) module"]
pub struct MACIER_SPEC;
impl crate::RegisterSpec for MACIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macier::R](R) reader structure"]
impl crate::Readable for MACIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macier::W](W) writer structure"]
impl crate::Writable for MACIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACIER to value 0"]
impl crate::Resettable for MACIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
