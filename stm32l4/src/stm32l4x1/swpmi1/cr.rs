#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDMA` reader - Reception DMA enable"]
pub struct RXDMA_R(crate::FieldReader<bool, bool>);
impl RXDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMA` writer - Reception DMA enable"]
pub struct RXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMA_W<'a> {
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
#[doc = "Field `TXDMA` reader - Transmission DMA enable"]
pub struct TXDMA_R(crate::FieldReader<bool, bool>);
impl TXDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMA` writer - Transmission DMA enable"]
pub struct TXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMA_W<'a> {
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
#[doc = "Field `RXMODE` reader - Reception buffering mode"]
pub struct RXMODE_R(crate::FieldReader<bool, bool>);
impl RXMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMODE` writer - Reception buffering mode"]
pub struct RXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMODE_W<'a> {
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
#[doc = "Field `TXMODE` reader - Transmission buffering mode"]
pub struct TXMODE_R(crate::FieldReader<bool, bool>);
impl TXMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMODE` writer - Transmission buffering mode"]
pub struct TXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMODE_W<'a> {
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
#[doc = "Field `LPBK` reader - Loopback mode enable"]
pub struct LPBK_R(crate::FieldReader<bool, bool>);
impl LPBK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPBK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPBK` writer - Loopback mode enable"]
pub struct LPBK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBK_W<'a> {
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
#[doc = "Field `SWPME` reader - Single wire protocol master interface enable"]
pub struct SWPME_R(crate::FieldReader<bool, bool>);
impl SWPME_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWPME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWPME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPME` writer - Single wire protocol master interface enable"]
pub struct SWPME_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPME_W<'a> {
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
#[doc = "Field `DEACT` reader - Single wire protocol master interface deactivate"]
pub struct DEACT_R(crate::FieldReader<bool, bool>);
impl DEACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEACT` writer - Single wire protocol master interface deactivate"]
pub struct DEACT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEACT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reception DMA enable"]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmission DMA enable"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reception buffering mode"]
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission buffering mode"]
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Loopback mode enable"]
    #[inline(always)]
    pub fn lpbk(&self) -> LPBK_R {
        LPBK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Single wire protocol master interface enable"]
    #[inline(always)]
    pub fn swpme(&self) -> SWPME_R {
        SWPME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Single wire protocol master interface deactivate"]
    #[inline(always)]
    pub fn deact(&self) -> DEACT_R {
        DEACT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reception DMA enable"]
    #[inline(always)]
    pub fn rxdma(&mut self) -> RXDMA_W {
        RXDMA_W { w: self }
    }
    #[doc = "Bit 1 - Transmission DMA enable"]
    #[inline(always)]
    pub fn txdma(&mut self) -> TXDMA_W {
        TXDMA_W { w: self }
    }
    #[doc = "Bit 2 - Reception buffering mode"]
    #[inline(always)]
    pub fn rxmode(&mut self) -> RXMODE_W {
        RXMODE_W { w: self }
    }
    #[doc = "Bit 3 - Transmission buffering mode"]
    #[inline(always)]
    pub fn txmode(&mut self) -> TXMODE_W {
        TXMODE_W { w: self }
    }
    #[doc = "Bit 4 - Loopback mode enable"]
    #[inline(always)]
    pub fn lpbk(&mut self) -> LPBK_W {
        LPBK_W { w: self }
    }
    #[doc = "Bit 5 - Single wire protocol master interface enable"]
    #[inline(always)]
    pub fn swpme(&mut self) -> SWPME_W {
        SWPME_W { w: self }
    }
    #[doc = "Bit 10 - Single wire protocol master interface deactivate"]
    #[inline(always)]
    pub fn deact(&mut self) -> DEACT_W {
        DEACT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SWPMI Configuration/Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
