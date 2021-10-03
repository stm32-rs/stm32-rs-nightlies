#[doc = "Register `FLTDR` reader"]
pub struct R(crate::R<FLTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTDR` writer"]
pub struct W(crate::W<FLTDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTDR_SPEC>;
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
impl From<crate::W<FLTDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fault sources Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLTLCK_A {
    #[doc = "0: FLT1EN..FLT5EN bits are read/write"]
    UNLOCKED = 0,
    #[doc = "1: FLT1EN..FLT5EN bits are read only"]
    LOCKED = 1,
}
impl From<FLTLCK_A> for bool {
    #[inline(always)]
    fn from(variant: FLTLCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLTLCK` reader - Fault sources Lock"]
pub struct FLTLCK_R(crate::FieldReader<bool, FLTLCK_A>);
impl FLTLCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLTLCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTLCK_A {
        match self.bits {
            false => FLTLCK_A::UNLOCKED,
            true => FLTLCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == FLTLCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == FLTLCK_A::LOCKED
    }
}
impl core::ops::Deref for FLTLCK_R {
    type Target = crate::FieldReader<bool, FLTLCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTLCK` writer - Fault sources Lock"]
pub struct FLTLCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTLCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTLCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FLT1EN..FLT5EN bits are read/write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(FLTLCK_A::UNLOCKED)
    }
    #[doc = "FLT1EN..FLT5EN bits are read only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(FLTLCK_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Fault 5 enable"]
pub type FLT5EN_A = FLT1EN_A;
#[doc = "Field `FLT5EN` reader - Fault 5 enable"]
pub type FLT5EN_R = FLT1EN_R;
#[doc = "Field `FLT5EN` writer - Fault 5 enable"]
pub struct FLT5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT5EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(FLT5EN_A::IGNORED)
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(FLT5EN_A::ACTIVE)
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
#[doc = "Fault 4 enable"]
pub type FLT4EN_A = FLT1EN_A;
#[doc = "Field `FLT4EN` reader - Fault 4 enable"]
pub type FLT4EN_R = FLT1EN_R;
#[doc = "Field `FLT4EN` writer - Fault 4 enable"]
pub struct FLT4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT4EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(FLT4EN_A::IGNORED)
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(FLT4EN_A::ACTIVE)
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
#[doc = "Fault 3 enable"]
pub type FLT3EN_A = FLT1EN_A;
#[doc = "Field `FLT3EN` reader - Fault 3 enable"]
pub type FLT3EN_R = FLT1EN_R;
#[doc = "Field `FLT3EN` writer - Fault 3 enable"]
pub struct FLT3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(FLT3EN_A::IGNORED)
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(FLT3EN_A::ACTIVE)
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
#[doc = "Fault 2 enable"]
pub type FLT2EN_A = FLT1EN_A;
#[doc = "Field `FLT2EN` reader - Fault 2 enable"]
pub type FLT2EN_R = FLT1EN_R;
#[doc = "Field `FLT2EN` writer - Fault 2 enable"]
pub struct FLT2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(FLT2EN_A::IGNORED)
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(FLT2EN_A::ACTIVE)
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
#[doc = "Fault 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1EN_A {
    #[doc = "0: Fault input ignored"]
    IGNORED = 0,
    #[doc = "1: Fault input is active and can disable HRTIM outputs"]
    ACTIVE = 1,
}
impl From<FLT1EN_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1EN` reader - Fault 1 enable"]
pub struct FLT1EN_R(crate::FieldReader<bool, FLT1EN_A>);
impl FLT1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1EN_A {
        match self.bits {
            false => FLT1EN_A::IGNORED,
            true => FLT1EN_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORED`"]
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        **self == FLT1EN_A::IGNORED
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == FLT1EN_A::ACTIVE
    }
}
impl core::ops::Deref for FLT1EN_R {
    type Target = crate::FieldReader<bool, FLT1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1EN` writer - Fault 1 enable"]
pub struct FLT1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(FLT1EN_A::IGNORED)
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(FLT1EN_A::ACTIVE)
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
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&self) -> FLT5EN_R {
        FLT5EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&mut self) -> FLTLCK_W {
        FLTLCK_W { w: self }
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&mut self) -> FLT5EN_W {
        FLT5EN_W { w: self }
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&mut self) -> FLT4EN_W {
        FLT4EN_W { w: self }
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&mut self) -> FLT3EN_W {
        FLT3EN_W { w: self }
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&mut self) -> FLT2EN_W {
        FLT2EN_W { w: self }
    }
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&mut self) -> FLT1EN_W {
        FLT1EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Fault Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltdr](index.html) module"]
pub struct FLTDR_SPEC;
impl crate::RegisterSpec for FLTDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltdr::R](R) reader structure"]
impl crate::Readable for FLTDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltdr::W](W) writer structure"]
impl crate::Writable for FLTDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTDR to value 0"]
impl crate::Resettable for FLTDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
