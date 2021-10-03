#[doc = "Register `CPUPR2` reader"]
pub struct R(crate::R<CPUPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUPR2` writer"]
pub struct W(crate::W<CPUPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUPR2_SPEC>;
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
impl From<crate::W<CPUPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configurable event inputs x+32 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR49_A {
    #[doc = "0: No trigger request occurred"]
    NOTPENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<PR49_A> for bool {
    #[inline(always)]
    fn from(variant: PR49_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR49` reader - Configurable event inputs x+32 Pending bit"]
pub struct PR49_R(crate::FieldReader<bool, PR49_A>);
impl PR49_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR49_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR49_A {
        match self.bits {
            false => PR49_A::NOTPENDING,
            true => PR49_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == PR49_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == PR49_A::PENDING
    }
}
impl core::ops::Deref for PR49_R {
    type Target = crate::FieldReader<bool, PR49_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Configurable event inputs x+32 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR49_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<PR49_AW> for bool {
    #[inline(always)]
    fn from(variant: PR49_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR49` writer - Configurable event inputs x+32 Pending bit"]
pub struct PR49_W<'a> {
    w: &'a mut W,
}
impl<'a> PR49_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR49_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR49_AW::CLEAR)
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
#[doc = "Configurable event inputs x+32 Pending bit"]
pub type PR51_A = PR49_A;
#[doc = "Field `PR51` reader - Configurable event inputs x+32 Pending bit"]
pub type PR51_R = PR49_R;
#[doc = "Configurable event inputs x+32 Pending bit"]
pub type PR51_AW = PR49_AW;
#[doc = "Field `PR51` writer - Configurable event inputs x+32 Pending bit"]
pub struct PR51_W<'a> {
    w: &'a mut W,
}
impl<'a> PR51_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR51_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR51_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr49(&self) -> PR49_R {
        PR49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr51(&self) -> PR51_R {
        PR51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr49(&mut self) -> PR49_W {
        PR49_W { w: self }
    }
    #[doc = "Bit 19 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr51(&mut self) -> PR51_W {
        PR51_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpupr2](index.html) module"]
pub struct CPUPR2_SPEC;
impl crate::RegisterSpec for CPUPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpupr2::R](R) reader structure"]
impl crate::Readable for CPUPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpupr2::W](W) writer structure"]
impl crate::Writable for CPUPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPUPR2 to value 0"]
impl crate::Resettable for CPUPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
