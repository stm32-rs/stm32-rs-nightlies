#[doc = "Register `PLL1` reader"]
pub struct R(crate::R<PLL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1` writer"]
pub struct W(crate::W<PLL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1_SPEC>;
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
impl From<crate::W<PLL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL1EN` reader - Enable the PLL1 inside PHY"]
pub struct PLL1EN_R(crate::FieldReader<bool, bool>);
impl PLL1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL1EN` writer - Enable the PLL1 inside PHY"]
pub struct PLL1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1EN_W<'a> {
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
#[doc = "Field `PLL1SEL` reader - : Controls the PHY PLL1 input clock frequency selection"]
pub struct PLL1SEL_R(crate::FieldReader<u8, u8>);
impl PLL1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLL1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL1SEL` writer - : Controls the PHY PLL1 input clock frequency selection"]
pub struct PLL1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable the PLL1 inside PHY"]
    #[inline(always)]
    pub fn pll1en(&self) -> PLL1EN_R {
        PLL1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - : Controls the PHY PLL1 input clock frequency selection"]
    #[inline(always)]
    pub fn pll1sel(&self) -> PLL1SEL_R {
        PLL1SEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the PLL1 inside PHY"]
    #[inline(always)]
    pub fn pll1en(&mut self) -> PLL1EN_W {
        PLL1EN_W { w: self }
    }
    #[doc = "Bits 1:3 - : Controls the PHY PLL1 input clock frequency selection"]
    #[inline(always)]
    pub fn pll1sel(&mut self) -> PLL1SEL_W {
        PLL1SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBPHYC PLL1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1](index.html) module"]
pub struct PLL1_SPEC;
impl crate::RegisterSpec for PLL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1::R](R) reader structure"]
impl crate::Readable for PLL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1::W](W) writer structure"]
impl crate::Writable for PLL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL1 to value 0"]
impl crate::Resettable for PLL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
