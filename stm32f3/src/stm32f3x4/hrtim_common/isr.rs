#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Burst mode Period Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMPER_A {
    #[doc = "0: No burst mode period interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Burst mode period interrupt occured"]
    EVENT = 1,
}
impl From<BMPER_A> for bool {
    #[inline(always)]
    fn from(variant: BMPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMPER` reader - Burst mode Period Interrupt Flag"]
pub struct BMPER_R(crate::FieldReader<bool, BMPER_A>);
impl BMPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMPER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMPER_A {
        match self.bits {
            false => BMPER_A::NOEVENT,
            true => BMPER_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == BMPER_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == BMPER_A::EVENT
    }
}
impl core::ops::Deref for BMPER_R {
    type Target = crate::FieldReader<bool, BMPER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Burst mode Period Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMPER_AW {
    #[doc = "1: Clear burst mode period interrupt"]
    CLEAR = 1,
}
impl From<BMPER_AW> for bool {
    #[inline(always)]
    fn from(variant: BMPER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMPER` writer - Burst mode Period Interrupt Flag"]
pub struct BMPER_W<'a> {
    w: &'a mut W,
}
impl<'a> BMPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMPER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear burst mode period interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BMPER_AW::CLEAR)
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
#[doc = "DLL Ready Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLLRDY_A {
    #[doc = "0: No DLL calibration ready interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: DLL calibration ready interrupt occurred"]
    EVENT = 1,
}
impl From<DLLRDY_A> for bool {
    #[inline(always)]
    fn from(variant: DLLRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLLRDY` reader - DLL Ready Interrupt Flag"]
pub struct DLLRDY_R(crate::FieldReader<bool, DLLRDY_A>);
impl DLLRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLLRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLLRDY_A {
        match self.bits {
            false => DLLRDY_A::NOEVENT,
            true => DLLRDY_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == DLLRDY_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == DLLRDY_A::EVENT
    }
}
impl core::ops::Deref for DLLRDY_R {
    type Target = crate::FieldReader<bool, DLLRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DLL Ready Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLLRDY_AW {
    #[doc = "1: Clear DLL calibration interrupt"]
    CLEAR = 1,
}
impl From<DLLRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: DLLRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLLRDY` writer - DLL Ready Interrupt Flag"]
pub struct DLLRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLLRDY_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear DLL calibration interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLLRDY_AW::CLEAR)
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
#[doc = "System Fault Interrupt Flag"]
pub type SYSFLT_A = FLT1_A;
#[doc = "Field `SYSFLT` reader - System Fault Interrupt Flag"]
pub type SYSFLT_R = FLT1_R;
#[doc = "System Fault Interrupt Flag"]
pub type SYSFLT_AW = FLT1_AW;
#[doc = "Field `SYSFLT` writer - System Fault Interrupt Flag"]
pub struct SYSFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSFLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSFLT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear fault interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SYSFLT_AW::CLEAR)
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
#[doc = "Fault 5 Interrupt Flag"]
pub type FLT5_A = FLT1_A;
#[doc = "Field `FLT5` reader - Fault 5 Interrupt Flag"]
pub type FLT5_R = FLT1_R;
#[doc = "Fault 5 Interrupt Flag"]
pub type FLT5_AW = FLT1_AW;
#[doc = "Field `FLT5` writer - Fault 5 Interrupt Flag"]
pub struct FLT5_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear fault interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLT5_AW::CLEAR)
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
#[doc = "Fault 4 Interrupt Flag"]
pub type FLT4_A = FLT1_A;
#[doc = "Field `FLT4` reader - Fault 4 Interrupt Flag"]
pub type FLT4_R = FLT1_R;
#[doc = "Fault 4 Interrupt Flag"]
pub type FLT4_AW = FLT1_AW;
#[doc = "Field `FLT4` writer - Fault 4 Interrupt Flag"]
pub struct FLT4_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear fault interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLT4_AW::CLEAR)
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
#[doc = "Fault 3 Interrupt Flag"]
pub type FLT3_A = FLT1_A;
#[doc = "Field `FLT3` reader - Fault 3 Interrupt Flag"]
pub type FLT3_R = FLT1_R;
#[doc = "Fault 3 Interrupt Flag"]
pub type FLT3_AW = FLT1_AW;
#[doc = "Field `FLT3` writer - Fault 3 Interrupt Flag"]
pub struct FLT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear fault interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLT3_AW::CLEAR)
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
#[doc = "Fault 2 Interrupt Flag"]
pub type FLT2_A = FLT1_A;
#[doc = "Field `FLT2` reader - Fault 2 Interrupt Flag"]
pub type FLT2_R = FLT1_R;
#[doc = "Fault 2 Interrupt Flag"]
pub type FLT2_AW = FLT1_AW;
#[doc = "Field `FLT2` writer - Fault 2 Interrupt Flag"]
pub struct FLT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear fault interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLT2_AW::CLEAR)
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
#[doc = "Fault 1 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1_A {
    #[doc = "0: No fault interrupt occurred"]
    NOEVENT = 0,
    #[doc = "1: Fault interrupt occurred"]
    EVENT = 1,
}
impl From<FLT1_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1` reader - Fault 1 Interrupt Flag"]
pub struct FLT1_R(crate::FieldReader<bool, FLT1_A>);
impl FLT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1_A {
        match self.bits {
            false => FLT1_A::NOEVENT,
            true => FLT1_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == FLT1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == FLT1_A::EVENT
    }
}
impl core::ops::Deref for FLT1_R {
    type Target = crate::FieldReader<bool, FLT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fault 1 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1_AW {
    #[doc = "1: Clear fault interrupt"]
    CLEAR = 1,
}
impl From<FLT1_AW> for bool {
    #[inline(always)]
    fn from(variant: FLT1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1` writer - Fault 1 Interrupt Flag"]
pub struct FLT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear fault interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLT1_AW::CLEAR)
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
    #[doc = "Bit 17 - Burst mode Period Interrupt Flag"]
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Flag"]
    #[inline(always)]
    pub fn dllrdy(&self) -> DLLRDY_R {
        DLLRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    pub fn sysflt(&self) -> SYSFLT_R {
        SYSFLT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag"]
    #[inline(always)]
    pub fn flt5(&self) -> FLT5_R {
        FLT5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag"]
    #[inline(always)]
    pub fn flt4(&self) -> FLT4_R {
        FLT4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag"]
    #[inline(always)]
    pub fn flt3(&self) -> FLT3_R {
        FLT3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag"]
    #[inline(always)]
    pub fn flt2(&self) -> FLT2_R {
        FLT2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Fault 1 Interrupt Flag"]
    #[inline(always)]
    pub fn flt1(&self) -> FLT1_R {
        FLT1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Burst mode Period Interrupt Flag"]
    #[inline(always)]
    pub fn bmper(&mut self) -> BMPER_W {
        BMPER_W { w: self }
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Flag"]
    #[inline(always)]
    pub fn dllrdy(&mut self) -> DLLRDY_W {
        DLLRDY_W { w: self }
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    pub fn sysflt(&mut self) -> SYSFLT_W {
        SYSFLT_W { w: self }
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag"]
    #[inline(always)]
    pub fn flt5(&mut self) -> FLT5_W {
        FLT5_W { w: self }
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag"]
    #[inline(always)]
    pub fn flt4(&mut self) -> FLT4_W {
        FLT4_W { w: self }
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag"]
    #[inline(always)]
    pub fn flt3(&mut self) -> FLT3_W {
        FLT3_W { w: self }
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag"]
    #[inline(always)]
    pub fn flt2(&mut self) -> FLT2_W {
        FLT2_W { w: self }
    }
    #[doc = "Bit 0 - Fault 1 Interrupt Flag"]
    #[inline(always)]
    pub fn flt1(&mut self) -> FLT1_W {
        FLT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
