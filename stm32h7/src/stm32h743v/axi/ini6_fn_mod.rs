#[doc = "Register `INI6_FN_MOD` reader"]
pub struct R(crate::R<INI6_FN_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INI6_FN_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INI6_FN_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INI6_FN_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INI6_FN_MOD` writer"]
pub struct W(crate::W<INI6_FN_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INI6_FN_MOD_SPEC>;
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
impl From<crate::W<INI6_FN_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INI6_FN_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Override ASIB read issuing capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_ISS_OVERRIDE_A {
    #[doc = "0: Normal ASIB read issuing capability"]
    NORMAL = 0,
    #[doc = "1: Force ASIB read issuing capability to 1"]
    FORCE1 = 1,
}
impl From<READ_ISS_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: READ_ISS_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_ISS_OVERRIDE` reader - Override ASIB read issuing capability"]
pub struct READ_ISS_OVERRIDE_R(crate::FieldReader<bool, READ_ISS_OVERRIDE_A>);
impl READ_ISS_OVERRIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        READ_ISS_OVERRIDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_ISS_OVERRIDE_A {
        match self.bits {
            false => READ_ISS_OVERRIDE_A::NORMAL,
            true => READ_ISS_OVERRIDE_A::FORCE1,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == READ_ISS_OVERRIDE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        **self == READ_ISS_OVERRIDE_A::FORCE1
    }
}
impl core::ops::Deref for READ_ISS_OVERRIDE_R {
    type Target = crate::FieldReader<bool, READ_ISS_OVERRIDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_ISS_OVERRIDE` writer - Override ASIB read issuing capability"]
pub struct READ_ISS_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_ISS_OVERRIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_ISS_OVERRIDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal ASIB read issuing capability"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(READ_ISS_OVERRIDE_A::NORMAL)
    }
    #[doc = "Force ASIB read issuing capability to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(READ_ISS_OVERRIDE_A::FORCE1)
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
#[doc = "Override ASIB write issuing capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_ISS_OVERRIDE_A {
    #[doc = "0: Normal ASIB write issuing capability"]
    NORMAL = 0,
    #[doc = "1: Force ASIB write issuing capability to 1"]
    FORCE1 = 1,
}
impl From<WRITE_ISS_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_ISS_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Override ASIB write issuing capability"]
pub struct WRITE_ISS_OVERRIDE_R(crate::FieldReader<bool, WRITE_ISS_OVERRIDE_A>);
impl WRITE_ISS_OVERRIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_ISS_OVERRIDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_ISS_OVERRIDE_A {
        match self.bits {
            false => WRITE_ISS_OVERRIDE_A::NORMAL,
            true => WRITE_ISS_OVERRIDE_A::FORCE1,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == WRITE_ISS_OVERRIDE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        **self == WRITE_ISS_OVERRIDE_A::FORCE1
    }
}
impl core::ops::Deref for WRITE_ISS_OVERRIDE_R {
    type Target = crate::FieldReader<bool, WRITE_ISS_OVERRIDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Override ASIB write issuing capability"]
pub struct WRITE_ISS_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_ISS_OVERRIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITE_ISS_OVERRIDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal ASIB write issuing capability"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(WRITE_ISS_OVERRIDE_A::NORMAL)
    }
    #[doc = "Force ASIB write issuing capability to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(WRITE_ISS_OVERRIDE_A::FORCE1)
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
impl R {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W {
        READ_ISS_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W {
        WRITE_ISS_OVERRIDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini6_fn_mod](index.html) module"]
pub struct INI6_FN_MOD_SPEC;
impl crate::RegisterSpec for INI6_FN_MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ini6_fn_mod::R](R) reader structure"]
impl crate::Readable for INI6_FN_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ini6_fn_mod::W](W) writer structure"]
impl crate::Writable for INI6_FN_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INI6_FN_MOD to value 0x04"]
impl crate::Resettable for INI6_FN_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
