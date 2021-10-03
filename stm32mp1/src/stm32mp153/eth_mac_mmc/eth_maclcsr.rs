#[doc = "Register `ETH_MACLCSR` reader"]
pub struct R(crate::R<ETH_MACLCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACLCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACLCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACLCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACLCSR` writer"]
pub struct W(crate::W<ETH_MACLCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACLCSR_SPEC>;
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
impl From<crate::W<ETH_MACLCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACLCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLPIEN` reader - TLPIEN"]
pub struct TLPIEN_R(crate::FieldReader<bool, bool>);
impl TLPIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TLPIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLPIEX` reader - TLPIEX"]
pub struct TLPIEX_R(crate::FieldReader<bool, bool>);
impl TLPIEX_R {
    pub(crate) fn new(bits: bool) -> Self {
        TLPIEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPIEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLPIEN` reader - RLPIEN"]
pub struct RLPIEN_R(crate::FieldReader<bool, bool>);
impl RLPIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RLPIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLPIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLPIEX` reader - RLPIEX"]
pub struct RLPIEX_R(crate::FieldReader<bool, bool>);
impl RLPIEX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RLPIEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLPIEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLPIST` reader - TLPIST"]
pub struct TLPIST_R(crate::FieldReader<bool, bool>);
impl TLPIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TLPIST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPIST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLPIST` reader - RLPIST"]
pub struct RLPIST_R(crate::FieldReader<bool, bool>);
impl RLPIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RLPIST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLPIST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPIEN` reader - LPIEN"]
pub struct LPIEN_R(crate::FieldReader<bool, bool>);
impl LPIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPIEN` writer - LPIEN"]
pub struct LPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PLS` reader - PLS"]
pub struct PLS_R(crate::FieldReader<bool, bool>);
impl PLS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLS` writer - PLS"]
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PLSEN` reader - PLSEN"]
pub struct PLSEN_R(crate::FieldReader<bool, bool>);
impl PLSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLSEN` writer - PLSEN"]
pub struct PLSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `LPITXA` reader - LPITXA"]
pub struct LPITXA_R(crate::FieldReader<bool, bool>);
impl LPITXA_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPITXA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPITXA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPITXA` writer - LPITXA"]
pub struct LPITXA_W<'a> {
    w: &'a mut W,
}
impl<'a> LPITXA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `LPITE` reader - LPITE"]
pub struct LPITE_R(crate::FieldReader<bool, bool>);
impl LPITE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPITE` writer - LPITE"]
pub struct LPITE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPITE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TLPIEN"]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TLPIEX"]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RLPIEN"]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RLPIEX"]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TLPIST"]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RLPIST"]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LPIEN"]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PLS"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PLSEN"]
    #[inline(always)]
    pub fn plsen(&self) -> PLSEN_R {
        PLSEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LPITXA"]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPITE"]
    #[inline(always)]
    pub fn lpite(&self) -> LPITE_R {
        LPITE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - LPIEN"]
    #[inline(always)]
    pub fn lpien(&mut self) -> LPIEN_W {
        LPIEN_W { w: self }
    }
    #[doc = "Bit 17 - PLS"]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    #[doc = "Bit 18 - PLSEN"]
    #[inline(always)]
    pub fn plsen(&mut self) -> PLSEN_W {
        PLSEN_W { w: self }
    }
    #[doc = "Bit 19 - LPITXA"]
    #[inline(always)]
    pub fn lpitxa(&mut self) -> LPITXA_W {
        LPITXA_W { w: self }
    }
    #[doc = "Bit 20 - LPITE"]
    #[inline(always)]
    pub fn lpite(&mut self) -> LPITE_W {
        LPITE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The LPI Control and Status Register controls the LPI functions and provides the LPI interrupt status. The status bits are cleared when this register is read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maclcsr](index.html) module"]
pub struct ETH_MACLCSR_SPEC;
impl crate::RegisterSpec for ETH_MACLCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_maclcsr::R](R) reader structure"]
impl crate::Readable for ETH_MACLCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_maclcsr::W](W) writer structure"]
impl crate::Writable for ETH_MACLCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACLCSR to value 0"]
impl crate::Resettable for ETH_MACLCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
