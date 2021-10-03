#[doc = "Register `TZC_SPECULATION_CTRL` reader"]
pub struct R(crate::R<TZC_SPECULATION_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_SPECULATION_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_SPECULATION_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_SPECULATION_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZC_SPECULATION_CTRL` writer"]
pub struct W(crate::W<TZC_SPECULATION_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_SPECULATION_CTRL_SPEC>;
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
impl From<crate::W<TZC_SPECULATION_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_SPECULATION_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READSPEC_DISABLE` reader - READSPEC_DISABLE"]
pub struct READSPEC_DISABLE_R(crate::FieldReader<bool, bool>);
impl READSPEC_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        READSPEC_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READSPEC_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READSPEC_DISABLE` writer - READSPEC_DISABLE"]
pub struct READSPEC_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> READSPEC_DISABLE_W<'a> {
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
#[doc = "Field `WRITESPEC_DISABLE` reader - WRITESPEC_DISABLE"]
pub struct WRITESPEC_DISABLE_R(crate::FieldReader<bool, bool>);
impl WRITESPEC_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRITESPEC_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITESPEC_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITESPEC_DISABLE` writer - WRITESPEC_DISABLE"]
pub struct WRITESPEC_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITESPEC_DISABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - READSPEC_DISABLE"]
    #[inline(always)]
    pub fn readspec_disable(&self) -> READSPEC_DISABLE_R {
        READSPEC_DISABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WRITESPEC_DISABLE"]
    #[inline(always)]
    pub fn writespec_disable(&self) -> WRITESPEC_DISABLE_R {
        WRITESPEC_DISABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - READSPEC_DISABLE"]
    #[inline(always)]
    pub fn readspec_disable(&mut self) -> READSPEC_DISABLE_W {
        READSPEC_DISABLE_W { w: self }
    }
    #[doc = "Bit 1 - WRITESPEC_DISABLE"]
    #[inline(always)]
    pub fn writespec_disable(&mut self) -> WRITESPEC_DISABLE_W {
        WRITESPEC_DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls read and write access speculation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_speculation_ctrl](index.html) module"]
pub struct TZC_SPECULATION_CTRL_SPEC;
impl crate::RegisterSpec for TZC_SPECULATION_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_speculation_ctrl::R](R) reader structure"]
impl crate::Readable for TZC_SPECULATION_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_speculation_ctrl::W](W) writer structure"]
impl crate::Writable for TZC_SPECULATION_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZC_SPECULATION_CTRL to value 0"]
impl crate::Resettable for TZC_SPECULATION_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
