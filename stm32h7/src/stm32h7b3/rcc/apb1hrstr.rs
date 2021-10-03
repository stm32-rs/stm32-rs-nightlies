#[doc = "Register `APB1HRSTR` reader"]
pub struct R(crate::R<APB1HRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1HRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1HRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1HRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1HRSTR` writer"]
pub struct W(crate::W<APB1HRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1HRSTR_SPEC>;
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
impl From<crate::W<APB1HRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1HRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "clock recovery system reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<CRSRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRSRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSRST` reader - clock recovery system reset Set and reset by software."]
pub struct CRSRST_R(crate::FieldReader<bool, CRSRST_A>);
impl CRSRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRSRST_A> {
        match self.bits {
            true => Some(CRSRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == CRSRST_A::RESET
    }
}
impl core::ops::Deref for CRSRST_R {
    type Target = crate::FieldReader<bool, CRSRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSRST` writer - clock recovery system reset Set and reset by software."]
pub struct CRSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRSRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRSRST_A::RESET)
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
#[doc = "SWPMI block reset Set and reset by software."]
pub type SWPMIRST_A = CRSRST_A;
#[doc = "Field `SWPMIRST` reader - SWPMI block reset Set and reset by software."]
pub type SWPMIRST_R = CRSRST_R;
#[doc = "Field `SWPMIRST` writer - SWPMI block reset Set and reset by software."]
pub struct SWPMIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPMIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPMIRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWPMIRST_A::RESET)
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
#[doc = "OPAMP block reset Set and reset by software."]
pub type OPAMPRST_A = CRSRST_A;
#[doc = "Field `OPAMPRST` reader - OPAMP block reset Set and reset by software."]
pub type OPAMPRST_R = CRSRST_R;
#[doc = "Field `OPAMPRST` writer - OPAMP block reset Set and reset by software."]
pub struct OPAMPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMPRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAMPRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OPAMPRST_A::RESET)
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
#[doc = "MDIOS block reset Set and reset by software."]
pub type MDIOSRST_A = CRSRST_A;
#[doc = "Field `MDIOSRST` reader - MDIOS block reset Set and reset by software."]
pub type MDIOSRST_R = CRSRST_R;
#[doc = "Field `MDIOSRST` writer - MDIOS block reset Set and reset by software."]
pub struct MDIOSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIOSRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIOSRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MDIOSRST_A::RESET)
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
#[doc = "FDCAN block reset Set and reset by software."]
pub type FDCANRST_A = CRSRST_A;
#[doc = "Field `FDCANRST` reader - FDCAN block reset Set and reset by software."]
pub type FDCANRST_R = CRSRST_R;
#[doc = "Field `FDCANRST` writer - FDCAN block reset Set and reset by software."]
pub struct FDCANRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FDCANRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - clock recovery system reset Set and reset by software."]
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWPMI block reset Set and reset by software."]
    #[inline(always)]
    pub fn swpmirst(&self) -> SWPMIRST_R {
        SWPMIRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OPAMP block reset Set and reset by software."]
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MDIOS block reset Set and reset by software."]
    #[inline(always)]
    pub fn mdiosrst(&self) -> MDIOSRST_R {
        MDIOSRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FDCAN block reset Set and reset by software."]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - clock recovery system reset Set and reset by software."]
    #[inline(always)]
    pub fn crsrst(&mut self) -> CRSRST_W {
        CRSRST_W { w: self }
    }
    #[doc = "Bit 2 - SWPMI block reset Set and reset by software."]
    #[inline(always)]
    pub fn swpmirst(&mut self) -> SWPMIRST_W {
        SWPMIRST_W { w: self }
    }
    #[doc = "Bit 4 - OPAMP block reset Set and reset by software."]
    #[inline(always)]
    pub fn opamprst(&mut self) -> OPAMPRST_W {
        OPAMPRST_W { w: self }
    }
    #[doc = "Bit 5 - MDIOS block reset Set and reset by software."]
    #[inline(always)]
    pub fn mdiosrst(&mut self) -> MDIOSRST_W {
        MDIOSRST_W { w: self }
    }
    #[doc = "Bit 8 - FDCAN block reset Set and reset by software."]
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W {
        FDCANRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1hrstr](index.html) module"]
pub struct APB1HRSTR_SPEC;
impl crate::RegisterSpec for APB1HRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1hrstr::R](R) reader structure"]
impl crate::Readable for APB1HRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1hrstr::W](W) writer structure"]
impl crate::Writable for APB1HRSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1HRSTR to value 0"]
impl crate::Resettable for APB1HRSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
