#[doc = "Register `AHB2RSTR` reader"]
pub struct R(crate::R<AHB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2RSTR` writer"]
pub struct W(crate::W<AHB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2RSTR_SPEC>;
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
impl From<crate::W<AHB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMI_PSSIRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<DCMI_PSSIRST_A> for bool {
    #[inline(always)]
    fn from(variant: DCMI_PSSIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMI_PSSIRST` reader - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub struct DCMI_PSSIRST_R(crate::FieldReader<bool, DCMI_PSSIRST_A>);
impl DCMI_PSSIRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMI_PSSIRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCMI_PSSIRST_A> {
        match self.bits {
            true => Some(DCMI_PSSIRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == DCMI_PSSIRST_A::RESET
    }
}
impl core::ops::Deref for DCMI_PSSIRST_R {
    type Target = crate::FieldReader<bool, DCMI_PSSIRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMI_PSSIRST` writer - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
pub struct DCMI_PSSIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMI_PSSIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMI_PSSIRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DCMI_PSSIRST_A::RESET)
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
#[doc = "HSEM block reset Set and reset by software."]
pub type HSEMRST_A = DCMI_PSSIRST_A;
#[doc = "Field `HSEMRST` reader - HSEM block reset Set and reset by software."]
pub type HSEMRST_R = DCMI_PSSIRST_R;
#[doc = "Field `HSEMRST` writer - HSEM block reset Set and reset by software."]
pub struct HSEMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEMRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEMRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(HSEMRST_A::RESET)
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
#[doc = "cryptography block reset Set and reset by software."]
pub type CRYPTRST_A = DCMI_PSSIRST_A;
#[doc = "Field `CRYPTRST` reader - cryptography block reset Set and reset by software."]
pub type CRYPTRST_R = DCMI_PSSIRST_R;
#[doc = "Field `CRYPTRST` writer - cryptography block reset Set and reset by software."]
pub struct CRYPTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYPTRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRYPTRST_A::RESET)
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
#[doc = "hash block reset Set and reset by software."]
pub type HASHRST_A = DCMI_PSSIRST_A;
#[doc = "Field `HASHRST` reader - hash block reset Set and reset by software."]
pub type HASHRST_R = DCMI_PSSIRST_R;
#[doc = "Field `HASHRST` writer - hash block reset Set and reset by software."]
pub struct HASHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASHRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(HASHRST_A::RESET)
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
#[doc = "random number generator block reset Set and reset by software."]
pub type RNGRST_A = DCMI_PSSIRST_A;
#[doc = "Field `RNGRST` reader - random number generator block reset Set and reset by software."]
pub type RNGRST_R = DCMI_PSSIRST_R;
#[doc = "Field `RNGRST` writer - random number generator block reset Set and reset by software."]
pub struct RNGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RNGRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
pub type SDMMC2RST_A = DCMI_PSSIRST_A;
#[doc = "Field `SDMMC2RST` reader - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
pub type SDMMC2RST_R = DCMI_PSSIRST_R;
#[doc = "Field `SDMMC2RST` writer - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
pub struct SDMMC2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SDMMC2RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
pub type BDMA1RST_A = DCMI_PSSIRST_A;
#[doc = "Field `BDMA1RST` reader - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
pub type BDMA1RST_R = DCMI_PSSIRST_R;
#[doc = "Field `BDMA1RST` writer - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
pub struct BDMA1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMA1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BDMA1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BDMA1RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    pub fn dcmi_pssirst(&self) -> DCMI_PSSIRST_R {
        DCMI_PSSIRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSEM block reset Set and reset by software."]
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - cryptography block reset Set and reset by software."]
    #[inline(always)]
    pub fn cryptrst(&self) -> CRYPTRST_R {
        CRYPTRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - hash block reset Set and reset by software."]
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - random number generator block reset Set and reset by software."]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
    #[inline(always)]
    pub fn bdma1rst(&self) -> BDMA1RST_R {
        BDMA1RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - digital camera interface block reset (DCMI or PSSI depending which IP is active) Set and reset by software."]
    #[inline(always)]
    pub fn dcmi_pssirst(&mut self) -> DCMI_PSSIRST_W {
        DCMI_PSSIRST_W { w: self }
    }
    #[doc = "Bit 2 - HSEM block reset Set and reset by software."]
    #[inline(always)]
    pub fn hsemrst(&mut self) -> HSEMRST_W {
        HSEMRST_W { w: self }
    }
    #[doc = "Bit 4 - cryptography block reset Set and reset by software."]
    #[inline(always)]
    pub fn cryptrst(&mut self) -> CRYPTRST_W {
        CRYPTRST_W { w: self }
    }
    #[doc = "Bit 5 - hash block reset Set and reset by software."]
    #[inline(always)]
    pub fn hashrst(&mut self) -> HASHRST_W {
        HASHRST_W { w: self }
    }
    #[doc = "Bit 6 - random number generator block reset Set and reset by software."]
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W {
        RNGRST_W { w: self }
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W {
        SDMMC2RST_W { w: self }
    }
    #[doc = "Bit 11 - BDMA1 reset (DFSDM dedicated DMA) Set and reset by software."]
    #[inline(always)]
    pub fn bdma1rst(&mut self) -> BDMA1RST_W {
        BDMA1RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2rstr](index.html) module"]
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2rstr::R](R) reader structure"]
impl crate::Readable for AHB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2rstr::W](W) writer structure"]
impl crate::Writable for AHB2RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
