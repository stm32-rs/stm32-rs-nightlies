#[doc = "Register `RCR` reader"]
pub struct R(crate::R<RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR` writer"]
pub struct W(crate::W<RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR_SPEC>;
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
impl From<crate::W<RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CCM SRAM page write protection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAGE0_WP_A {
    #[doc = "0: Write protection of pagex is disabled"]
    DISABLED = 0,
    #[doc = "1: Write protection of pagex is enabled"]
    ENABLED = 1,
}
impl From<PAGE0_WP_A> for bool {
    #[inline(always)]
    fn from(variant: PAGE0_WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAGE0_WP` reader - CCM SRAM page write protection bit"]
pub struct PAGE0_WP_R(crate::FieldReader<bool, PAGE0_WP_A>);
impl PAGE0_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE0_WP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGE0_WP_A {
        match self.bits {
            false => PAGE0_WP_A::DISABLED,
            true => PAGE0_WP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PAGE0_WP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PAGE0_WP_A::ENABLED
    }
}
impl core::ops::Deref for PAGE0_WP_R {
    type Target = crate::FieldReader<bool, PAGE0_WP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAGE0_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE0_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE0_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE0_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE1_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE1_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE1_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE1_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE1_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE1_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE1_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE1_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE1_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE2_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE2_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE2_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE2_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE2_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE2_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE2_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE2_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE2_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE3_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE3_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE3_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE3_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE3_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE3_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE3_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE3_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE3_WP_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page0_wp(&self) -> PAGE0_WP_R {
        PAGE0_WP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page1_wp(&self) -> PAGE1_WP_R {
        PAGE1_WP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page2_wp(&self) -> PAGE2_WP_R {
        PAGE2_WP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page3_wp(&self) -> PAGE3_WP_R {
        PAGE3_WP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page0_wp(&mut self) -> PAGE0_WP_W {
        PAGE0_WP_W { w: self }
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page1_wp(&mut self) -> PAGE1_WP_W {
        PAGE1_WP_W { w: self }
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page2_wp(&mut self) -> PAGE2_WP_W {
        PAGE2_WP_W { w: self }
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page3_wp(&mut self) -> PAGE3_WP_W {
        PAGE3_WP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM SRAM protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr](index.html) module"]
pub struct RCR_SPEC;
impl crate::RegisterSpec for RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr::R](R) reader structure"]
impl crate::Readable for RCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr::W](W) writer structure"]
impl crate::Writable for RCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
