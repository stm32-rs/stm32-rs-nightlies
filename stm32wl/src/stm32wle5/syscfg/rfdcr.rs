#[doc = "Register `RFDCR` reader"]
pub struct R(crate::R<RFDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFDCR` writer"]
pub struct W(crate::W<RFDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFDCR_SPEC>;
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
impl From<crate::W<RFDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "radio debug test bus selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFTBSEL_A {
    #[doc = "0: Digital test bus selected on RF_ADTB\\[3:0\\]"]
    DIGITAL = 0,
    #[doc = "1: Analog test bus selected on RF_ADTB\\[3:0\\]"]
    ANALOG = 1,
}
impl From<RFTBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RFTBSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFTBSEL` reader - radio debug test bus selection"]
pub struct RFTBSEL_R(crate::FieldReader<bool, RFTBSEL_A>);
impl RFTBSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFTBSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFTBSEL_A {
        match self.bits {
            false => RFTBSEL_A::DIGITAL,
            true => RFTBSEL_A::ANALOG,
        }
    }
    #[doc = "Checks if the value of the field is `DIGITAL`"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        **self == RFTBSEL_A::DIGITAL
    }
    #[doc = "Checks if the value of the field is `ANALOG`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        **self == RFTBSEL_A::ANALOG
    }
}
impl core::ops::Deref for RFTBSEL_R {
    type Target = crate::FieldReader<bool, RFTBSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFTBSEL` writer - radio debug test bus selection"]
pub struct RFTBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFTBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFTBSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Digital test bus selected on RF_ADTB\\[3:0\\]"]
    #[inline(always)]
    pub fn digital(self) -> &'a mut W {
        self.variant(RFTBSEL_A::DIGITAL)
    }
    #[doc = "Analog test bus selected on RF_ADTB\\[3:0\\]"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(RFTBSEL_A::ANALOG)
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
    #[doc = "Bit 0 - radio debug test bus selection"]
    #[inline(always)]
    pub fn rftbsel(&self) -> RFTBSEL_R {
        RFTBSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - radio debug test bus selection"]
    #[inline(always)]
    pub fn rftbsel(&mut self) -> RFTBSEL_W {
        RFTBSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "radio debug control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfdcr](index.html) module"]
pub struct RFDCR_SPEC;
impl crate::RegisterSpec for RFDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfdcr::R](R) reader structure"]
impl crate::Readable for RFDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfdcr::W](W) writer structure"]
impl crate::Writable for RFDCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFDCR to value 0"]
impl crate::Resettable for RFDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
