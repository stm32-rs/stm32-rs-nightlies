#[doc = "Register `MPCBB2_CR` reader"]
pub struct R(crate::R<MPCBB2_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_CR` writer"]
pub struct W(crate::W<MPCBB2_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_CR_SPEC>;
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
impl From<crate::W<MPCBB2_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCK` reader - LCK"]
pub struct LCK_R(crate::FieldReader<bool, bool>);
impl LCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK` writer - LCK"]
pub struct LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK_W<'a> {
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
#[doc = "Field `INVSECSTATE` reader - INVSECSTATE"]
pub struct INVSECSTATE_R(crate::FieldReader<bool, bool>);
impl INVSECSTATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVSECSTATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVSECSTATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVSECSTATE` writer - INVSECSTATE"]
pub struct INVSECSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVSECSTATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SRWILADIS` reader - SRWILADIS"]
pub struct SRWILADIS_R(crate::FieldReader<bool, bool>);
impl SRWILADIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRWILADIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRWILADIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRWILADIS` writer - SRWILADIS"]
pub struct SRWILADIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SRWILADIS_W<'a> {
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
    #[doc = "Bit 0 - LCK"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 30 - INVSECSTATE"]
    #[inline(always)]
    pub fn invsecstate(&self) -> INVSECSTATE_R {
        INVSECSTATE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SRWILADIS"]
    #[inline(always)]
    pub fn srwiladis(&self) -> SRWILADIS_R {
        SRWILADIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCK"]
    #[inline(always)]
    pub fn lck(&mut self) -> LCK_W {
        LCK_W { w: self }
    }
    #[doc = "Bit 30 - INVSECSTATE"]
    #[inline(always)]
    pub fn invsecstate(&mut self) -> INVSECSTATE_W {
        INVSECSTATE_W { w: self }
    }
    #[doc = "Bit 31 - SRWILADIS"]
    #[inline(always)]
    pub fn srwiladis(&mut self) -> SRWILADIS_W {
        SRWILADIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_cr](index.html) module"]
pub struct MPCBB2_CR_SPEC;
impl crate::RegisterSpec for MPCBB2_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_cr::R](R) reader structure"]
impl crate::Readable for MPCBB2_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_cr::W](W) writer structure"]
impl crate::Writable for MPCBB2_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_CR to value 0"]
impl crate::Resettable for MPCBB2_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
