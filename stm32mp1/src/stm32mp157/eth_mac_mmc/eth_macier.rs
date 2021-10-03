#[doc = "Register `ETH_MACIER` reader"]
pub struct R(crate::R<ETH_MACIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACIER` writer"]
pub struct W(crate::W<ETH_MACIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACIER_SPEC>;
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
impl From<crate::W<ETH_MACIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RGSMIIIE` reader - RGSMIIIE"]
pub struct RGSMIIIE_R(crate::FieldReader<bool, bool>);
impl RGSMIIIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RGSMIIIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RGSMIIIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGSMIIIE` writer - RGSMIIIE"]
pub struct RGSMIIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RGSMIIIE_W<'a> {
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
#[doc = "Field `PHYIE` reader - PHYIE"]
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
#[doc = "Field `PHYIE` writer - PHYIE"]
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
#[doc = "Field `PMTIE` reader - PMTIE"]
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
#[doc = "Field `PMTIE` writer - PMTIE"]
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
#[doc = "Field `LPIIE` reader - LPIIE"]
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
#[doc = "Field `LPIIE` writer - LPIIE"]
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
#[doc = "Field `TSIE` reader - TSIE"]
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
#[doc = "Field `TSIE` writer - TSIE"]
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
#[doc = "Field `TXSTSIE` reader - TXSTSIE"]
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
#[doc = "Field `TXSTSIE` writer - TXSTSIE"]
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
#[doc = "Field `RXSTSIE` reader - RXSTSIE"]
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
#[doc = "Field `RXSTSIE` writer - RXSTSIE"]
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
    #[doc = "Bit 0 - RGSMIIIE"]
    #[inline(always)]
    pub fn rgsmiiie(&self) -> RGSMIIIE_R {
        RGSMIIIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - PHYIE"]
    #[inline(always)]
    pub fn phyie(&self) -> PHYIE_R {
        PHYIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMTIE"]
    #[inline(always)]
    pub fn pmtie(&self) -> PMTIE_R {
        PMTIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPIIE"]
    #[inline(always)]
    pub fn lpiie(&self) -> LPIIE_R {
        LPIIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TSIE"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TXSTSIE"]
    #[inline(always)]
    pub fn txstsie(&self) -> TXSTSIE_R {
        TXSTSIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RXSTSIE"]
    #[inline(always)]
    pub fn rxstsie(&self) -> RXSTSIE_R {
        RXSTSIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RGSMIIIE"]
    #[inline(always)]
    pub fn rgsmiiie(&mut self) -> RGSMIIIE_W {
        RGSMIIIE_W { w: self }
    }
    #[doc = "Bit 3 - PHYIE"]
    #[inline(always)]
    pub fn phyie(&mut self) -> PHYIE_W {
        PHYIE_W { w: self }
    }
    #[doc = "Bit 4 - PMTIE"]
    #[inline(always)]
    pub fn pmtie(&mut self) -> PMTIE_W {
        PMTIE_W { w: self }
    }
    #[doc = "Bit 5 - LPIIE"]
    #[inline(always)]
    pub fn lpiie(&mut self) -> LPIIE_W {
        LPIIE_W { w: self }
    }
    #[doc = "Bit 12 - TSIE"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W {
        TSIE_W { w: self }
    }
    #[doc = "Bit 13 - TXSTSIE"]
    #[inline(always)]
    pub fn txstsie(&mut self) -> TXSTSIE_W {
        TXSTSIE_W { w: self }
    }
    #[doc = "Bit 14 - RXSTSIE"]
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
#[doc = "The Interrupt Enable register contains the masks for generating the interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macier](index.html) module"]
pub struct ETH_MACIER_SPEC;
impl crate::RegisterSpec for ETH_MACIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macier::R](R) reader structure"]
impl crate::Readable for ETH_MACIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macier::W](W) writer structure"]
impl crate::Writable for ETH_MACIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACIER to value 0"]
impl crate::Resettable for ETH_MACIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
