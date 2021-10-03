#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Burst mode period Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMPERIE_A {
    #[doc = "0: Burst mode period interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Burst mode period interrupt enabled"]
    ENABLED = 1,
}
impl From<BMPERIE_A> for bool {
    #[inline(always)]
    fn from(variant: BMPERIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMPERIE` reader - Burst mode period Interrupt Enable"]
pub struct BMPERIE_R(crate::FieldReader<bool, BMPERIE_A>);
impl BMPERIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMPERIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMPERIE_A {
        match self.bits {
            false => BMPERIE_A::DISABLED,
            true => BMPERIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BMPERIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BMPERIE_A::ENABLED
    }
}
impl core::ops::Deref for BMPERIE_R {
    type Target = crate::FieldReader<bool, BMPERIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMPERIE` writer - Burst mode period Interrupt Enable"]
pub struct BMPERIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BMPERIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMPERIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Burst mode period interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BMPERIE_A::DISABLED)
    }
    #[doc = "Burst mode period interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BMPERIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "DLL Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLLRDYIE_A {
    #[doc = "0: DLL ready interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: DLL Ready interrupt enabled"]
    ENABLED = 1,
}
impl From<DLLRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: DLLRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLLRDYIE` reader - DLL Ready Interrupt Enable"]
pub struct DLLRDYIE_R(crate::FieldReader<bool, DLLRDYIE_A>);
impl DLLRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLLRDYIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLLRDYIE_A {
        match self.bits {
            false => DLLRDYIE_A::DISABLED,
            true => DLLRDYIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DLLRDYIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DLLRDYIE_A::ENABLED
    }
}
impl core::ops::Deref for DLLRDYIE_R {
    type Target = crate::FieldReader<bool, DLLRDYIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLLRDYIE` writer - DLL Ready Interrupt Enable"]
pub struct DLLRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLLRDYIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DLL ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DLLRDYIE_A::DISABLED)
    }
    #[doc = "DLL Ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DLLRDYIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "System Fault Interrupt Enable"]
pub type SYSFLTIE_A = FLT1IE_A;
#[doc = "Field `SYSFLTIE` reader - System Fault Interrupt Enable"]
pub type SYSFLTIE_R = FLT1IE_R;
#[doc = "Field `SYSFLTIE` writer - System Fault Interrupt Enable"]
pub struct SYSFLTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSFLTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSFLTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSFLTIE_A::DISABLED)
    }
    #[doc = "Fault interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSFLTIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Fault 5 Interrupt Enable"]
pub type FLT5IE_A = FLT1IE_A;
#[doc = "Field `FLT5IE` reader - Fault 5 Interrupt Enable"]
pub type FLT5IE_R = FLT1IE_R;
#[doc = "Field `FLT5IE` writer - Fault 5 Interrupt Enable"]
pub struct FLT5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT5IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT5IE_A::DISABLED)
    }
    #[doc = "Fault interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT5IE_A::ENABLED)
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
#[doc = "Fault 4 Interrupt Enable"]
pub type FLT4IE_A = FLT1IE_A;
#[doc = "Field `FLT4IE` reader - Fault 4 Interrupt Enable"]
pub type FLT4IE_R = FLT1IE_R;
#[doc = "Field `FLT4IE` writer - Fault 4 Interrupt Enable"]
pub struct FLT4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT4IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT4IE_A::DISABLED)
    }
    #[doc = "Fault interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT4IE_A::ENABLED)
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
#[doc = "Fault 3 Interrupt Enable"]
pub type FLT3IE_A = FLT1IE_A;
#[doc = "Field `FLT3IE` reader - Fault 3 Interrupt Enable"]
pub type FLT3IE_R = FLT1IE_R;
#[doc = "Field `FLT3IE` writer - Fault 3 Interrupt Enable"]
pub struct FLT3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT3IE_A::DISABLED)
    }
    #[doc = "Fault interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT3IE_A::ENABLED)
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
#[doc = "Fault 2 Interrupt Enable"]
pub type FLT2IE_A = FLT1IE_A;
#[doc = "Field `FLT2IE` reader - Fault 2 Interrupt Enable"]
pub type FLT2IE_R = FLT1IE_R;
#[doc = "Field `FLT2IE` writer - Fault 2 Interrupt Enable"]
pub struct FLT2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT2IE_A::DISABLED)
    }
    #[doc = "Fault interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT2IE_A::ENABLED)
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
#[doc = "Fault 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1IE_A {
    #[doc = "0: Fault interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Fault interrupt enabled"]
    ENABLED = 1,
}
impl From<FLT1IE_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1IE` reader - Fault 1 Interrupt Enable"]
pub struct FLT1IE_R(crate::FieldReader<bool, FLT1IE_A>);
impl FLT1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1IE_A {
        match self.bits {
            false => FLT1IE_A::DISABLED,
            true => FLT1IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FLT1IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FLT1IE_A::ENABLED
    }
}
impl core::ops::Deref for FLT1IE_R {
    type Target = crate::FieldReader<bool, FLT1IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1IE` writer - Fault 1 Interrupt Enable"]
pub struct FLT1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLT1IE_A::DISABLED)
    }
    #[doc = "Fault interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLT1IE_A::ENABLED)
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
impl R {
    #[doc = "Bit 17 - Burst mode period Interrupt Enable"]
    #[inline(always)]
    pub fn bmperie(&self) -> BMPERIE_R {
        BMPERIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dllrdyie(&self) -> DLLRDYIE_R {
        DLLRDYIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Enable"]
    #[inline(always)]
    pub fn sysfltie(&self) -> SYSFLTIE_R {
        SYSFLTIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Enable"]
    #[inline(always)]
    pub fn flt5ie(&self) -> FLT5IE_R {
        FLT5IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Enable"]
    #[inline(always)]
    pub fn flt4ie(&self) -> FLT4IE_R {
        FLT4IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Enable"]
    #[inline(always)]
    pub fn flt3ie(&self) -> FLT3IE_R {
        FLT3IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Enable"]
    #[inline(always)]
    pub fn flt2ie(&self) -> FLT2IE_R {
        FLT2IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Fault 1 Interrupt Enable"]
    #[inline(always)]
    pub fn flt1ie(&self) -> FLT1IE_R {
        FLT1IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Burst mode period Interrupt Enable"]
    #[inline(always)]
    pub fn bmperie(&mut self) -> BMPERIE_W {
        BMPERIE_W { w: self }
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dllrdyie(&mut self) -> DLLRDYIE_W {
        DLLRDYIE_W { w: self }
    }
    #[doc = "Bit 5 - System Fault Interrupt Enable"]
    #[inline(always)]
    pub fn sysfltie(&mut self) -> SYSFLTIE_W {
        SYSFLTIE_W { w: self }
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Enable"]
    #[inline(always)]
    pub fn flt5ie(&mut self) -> FLT5IE_W {
        FLT5IE_W { w: self }
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Enable"]
    #[inline(always)]
    pub fn flt4ie(&mut self) -> FLT4IE_W {
        FLT4IE_W { w: self }
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Enable"]
    #[inline(always)]
    pub fn flt3ie(&mut self) -> FLT3IE_W {
        FLT3IE_W { w: self }
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Enable"]
    #[inline(always)]
    pub fn flt2ie(&mut self) -> FLT2IE_W {
        FLT2IE_W { w: self }
    }
    #[doc = "Bit 0 - Fault 1 Interrupt Enable"]
    #[inline(always)]
    pub fn flt1ie(&mut self) -> FLT1IE_W {
        FLT1IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
