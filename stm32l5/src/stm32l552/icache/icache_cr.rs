#[doc = "Register `ICACHE_CR` reader"]
pub struct R(crate::R<ICACHE_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_CR` writer"]
pub struct W(crate::W<ICACHE_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_CR_SPEC>;
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
impl From<crate::W<ICACHE_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - EN"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `CACHEINV` reader - CACHEINV"]
pub struct CACHEINV_R(crate::FieldReader<bool, bool>);
impl CACHEINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHEINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHEINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEINV` writer - CACHEINV"]
pub struct CACHEINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEINV_W<'a> {
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
#[doc = "Field `WAYSEL` reader - WAYSEL"]
pub struct WAYSEL_R(crate::FieldReader<bool, bool>);
impl WAYSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAYSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAYSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAYSEL` writer - WAYSEL"]
pub struct WAYSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAYSEL_W<'a> {
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
#[doc = "Field `HITMEN` reader - HITMEN"]
pub struct HITMEN_R(crate::FieldReader<bool, bool>);
impl HITMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HITMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HITMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HITMEN` writer - HITMEN"]
pub struct HITMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HITMEN_W<'a> {
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
#[doc = "Field `MISSMEN` reader - MISSMEN"]
pub struct MISSMEN_R(crate::FieldReader<bool, bool>);
impl MISSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MISSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISSMEN` writer - MISSMEN"]
pub struct MISSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MISSMEN_W<'a> {
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
#[doc = "Field `HITMRST` reader - HITMRST"]
pub struct HITMRST_R(crate::FieldReader<bool, bool>);
impl HITMRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        HITMRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HITMRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HITMRST` writer - HITMRST"]
pub struct HITMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HITMRST_W<'a> {
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
#[doc = "Field `MISSMRST` reader - MISSMRST"]
pub struct MISSMRST_R(crate::FieldReader<bool, bool>);
impl MISSMRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MISSMRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISSMRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISSMRST` writer - MISSMRST"]
pub struct MISSMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MISSMRST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CACHEINV"]
    #[inline(always)]
    pub fn cacheinv(&self) -> CACHEINV_R {
        CACHEINV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WAYSEL"]
    #[inline(always)]
    pub fn waysel(&self) -> WAYSEL_R {
        WAYSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HITMEN"]
    #[inline(always)]
    pub fn hitmen(&self) -> HITMEN_R {
        HITMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MISSMEN"]
    #[inline(always)]
    pub fn missmen(&self) -> MISSMEN_R {
        MISSMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HITMRST"]
    #[inline(always)]
    pub fn hitmrst(&self) -> HITMRST_R {
        HITMRST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MISSMRST"]
    #[inline(always)]
    pub fn missmrst(&self) -> MISSMRST_R {
        MISSMRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - CACHEINV"]
    #[inline(always)]
    pub fn cacheinv(&mut self) -> CACHEINV_W {
        CACHEINV_W { w: self }
    }
    #[doc = "Bit 2 - WAYSEL"]
    #[inline(always)]
    pub fn waysel(&mut self) -> WAYSEL_W {
        WAYSEL_W { w: self }
    }
    #[doc = "Bit 16 - HITMEN"]
    #[inline(always)]
    pub fn hitmen(&mut self) -> HITMEN_W {
        HITMEN_W { w: self }
    }
    #[doc = "Bit 17 - MISSMEN"]
    #[inline(always)]
    pub fn missmen(&mut self) -> MISSMEN_W {
        MISSMEN_W { w: self }
    }
    #[doc = "Bit 18 - HITMRST"]
    #[inline(always)]
    pub fn hitmrst(&mut self) -> HITMRST_W {
        HITMRST_W { w: self }
    }
    #[doc = "Bit 19 - MISSMRST"]
    #[inline(always)]
    pub fn missmrst(&mut self) -> MISSMRST_W {
        MISSMRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICACHE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_cr](index.html) module"]
pub struct ICACHE_CR_SPEC;
impl crate::RegisterSpec for ICACHE_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_cr::R](R) reader structure"]
impl crate::Readable for ICACHE_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_cr::W](W) writer structure"]
impl crate::Writable for ICACHE_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_CR to value 0x04"]
impl crate::Resettable for ICACHE_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
