#[doc = "Register `IOPSMEN` reader"]
pub struct R(crate::R<IOPSMEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPSMEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPSMEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPSMEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOPSMEN` writer"]
pub struct W(crate::W<IOPSMEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPSMEN_SPEC>;
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
impl From<crate::W<IOPSMEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPSMEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IOPHSMEN"]
pub type IOPHSMEN_A = IOPASMEN_A;
#[doc = "Field `IOPHSMEN` reader - IOPHSMEN"]
pub type IOPHSMEN_R = IOPASMEN_R;
#[doc = "Field `IOPHSMEN` writer - IOPHSMEN"]
pub struct IOPHSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPHSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPHSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port x clock is disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPHSMEN_A::DISABLED)
    }
    #[doc = "Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPHSMEN_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "IOPDSMEN"]
pub type IOPDSMEN_A = IOPASMEN_A;
#[doc = "Field `IOPDSMEN` reader - IOPDSMEN"]
pub type IOPDSMEN_R = IOPASMEN_R;
#[doc = "Field `IOPDSMEN` writer - IOPDSMEN"]
pub struct IOPDSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPDSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPDSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port x clock is disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPDSMEN_A::DISABLED)
    }
    #[doc = "Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPDSMEN_A::ENABLED)
    }
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
#[doc = "IOPCSMEN"]
pub type IOPCSMEN_A = IOPASMEN_A;
#[doc = "Field `IOPCSMEN` reader - IOPCSMEN"]
pub type IOPCSMEN_R = IOPASMEN_R;
#[doc = "Field `IOPCSMEN` writer - IOPCSMEN"]
pub struct IOPCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPCSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPCSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port x clock is disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPCSMEN_A::DISABLED)
    }
    #[doc = "Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPCSMEN_A::ENABLED)
    }
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
#[doc = "IOPBSMEN"]
pub type IOPBSMEN_A = IOPASMEN_A;
#[doc = "Field `IOPBSMEN` reader - IOPBSMEN"]
pub type IOPBSMEN_R = IOPASMEN_R;
#[doc = "Field `IOPBSMEN` writer - IOPBSMEN"]
pub struct IOPBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPBSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPBSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port x clock is disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPBSMEN_A::DISABLED)
    }
    #[doc = "Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPBSMEN_A::ENABLED)
    }
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
#[doc = "IOPASMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOPASMEN_A {
    #[doc = "0: Port x clock is disabled in Sleep mode"]
    DISABLED = 0,
    #[doc = "1: Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    ENABLED = 1,
}
impl From<IOPASMEN_A> for bool {
    #[inline(always)]
    fn from(variant: IOPASMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOPASMEN` reader - IOPASMEN"]
pub struct IOPASMEN_R(crate::FieldReader<bool, IOPASMEN_A>);
impl IOPASMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOPASMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOPASMEN_A {
        match self.bits {
            false => IOPASMEN_A::DISABLED,
            true => IOPASMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IOPASMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IOPASMEN_A::ENABLED
    }
}
impl core::ops::Deref for IOPASMEN_R {
    type Target = crate::FieldReader<bool, IOPASMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOPASMEN` writer - IOPASMEN"]
pub struct IOPASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPASMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPASMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port x clock is disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPASMEN_A::DISABLED)
    }
    #[doc = "Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPASMEN_A::ENABLED)
    }
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
#[doc = "Port E clock enable during Sleep mode bit"]
pub type IOPESMEN_A = IOPASMEN_A;
#[doc = "Field `IOPESMEN` reader - Port E clock enable during Sleep mode bit"]
pub type IOPESMEN_R = IOPASMEN_R;
#[doc = "Field `IOPESMEN` writer - Port E clock enable during Sleep mode bit"]
pub struct IOPESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPESMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPESMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Port x clock is disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPESMEN_A::DISABLED)
    }
    #[doc = "Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPESMEN_A::ENABLED)
    }
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
impl R {
    #[doc = "Bit 7 - IOPHSMEN"]
    #[inline(always)]
    pub fn iophsmen(&self) -> IOPHSMEN_R {
        IOPHSMEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IOPDSMEN"]
    #[inline(always)]
    pub fn iopdsmen(&self) -> IOPDSMEN_R {
        IOPDSMEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IOPCSMEN"]
    #[inline(always)]
    pub fn iopcsmen(&self) -> IOPCSMEN_R {
        IOPCSMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - IOPBSMEN"]
    #[inline(always)]
    pub fn iopbsmen(&self) -> IOPBSMEN_R {
        IOPBSMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IOPASMEN"]
    #[inline(always)]
    pub fn iopasmen(&self) -> IOPASMEN_R {
        IOPASMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port E clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopesmen(&self) -> IOPESMEN_R {
        IOPESMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - IOPHSMEN"]
    #[inline(always)]
    pub fn iophsmen(&mut self) -> IOPHSMEN_W {
        IOPHSMEN_W { w: self }
    }
    #[doc = "Bit 3 - IOPDSMEN"]
    #[inline(always)]
    pub fn iopdsmen(&mut self) -> IOPDSMEN_W {
        IOPDSMEN_W { w: self }
    }
    #[doc = "Bit 2 - IOPCSMEN"]
    #[inline(always)]
    pub fn iopcsmen(&mut self) -> IOPCSMEN_W {
        IOPCSMEN_W { w: self }
    }
    #[doc = "Bit 1 - IOPBSMEN"]
    #[inline(always)]
    pub fn iopbsmen(&mut self) -> IOPBSMEN_W {
        IOPBSMEN_W { w: self }
    }
    #[doc = "Bit 0 - IOPASMEN"]
    #[inline(always)]
    pub fn iopasmen(&mut self) -> IOPASMEN_W {
        IOPASMEN_W { w: self }
    }
    #[doc = "Bit 4 - Port E clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopesmen(&mut self) -> IOPESMEN_W {
        IOPESMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO clock enable in sleep mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopsmen](index.html) module"]
pub struct IOPSMEN_SPEC;
impl crate::RegisterSpec for IOPSMEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iopsmen::R](R) reader structure"]
impl crate::Readable for IOPSMEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iopsmen::W](W) writer structure"]
impl crate::Writable for IOPSMEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOPSMEN to value 0x8f"]
impl crate::Resettable for IOPSMEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8f
    }
}
