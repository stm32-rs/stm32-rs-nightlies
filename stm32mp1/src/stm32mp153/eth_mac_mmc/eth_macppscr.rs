#[doc = "Register `ETH_MACPPSCR` reader"]
pub struct R(crate::R<ETH_MACPPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPPSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPPSCR` writer"]
pub struct W(crate::W<ETH_MACPPSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPPSCR_SPEC>;
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
impl From<crate::W<ETH_MACPPSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPPSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSCTRL` reader - PPSCTRL"]
pub struct PPSCTRL_R(crate::FieldReader<u8, u8>);
impl PPSCTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPSCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPSCTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPSCTRL` writer - PPSCTRL"]
pub struct PPSCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PPSEN0` reader - PPSEN0"]
pub struct PPSEN0_R(crate::FieldReader<bool, bool>);
impl PPSEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPSEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPSEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPSEN0` writer - PPSEN0"]
pub struct PPSEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSEN0_W<'a> {
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
#[doc = "Field `TRGTMODSEL0` reader - TRGTMODSEL0"]
pub struct TRGTMODSEL0_R(crate::FieldReader<u8, u8>);
impl TRGTMODSEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGTMODSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGTMODSEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGTMODSEL0` writer - TRGTMODSEL0"]
pub struct TRGTMODSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGTMODSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PPSCTRL"]
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - PPSEN0"]
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - TRGTMODSEL0"]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PPSCTRL"]
    #[inline(always)]
    pub fn ppsctrl(&mut self) -> PPSCTRL_W {
        PPSCTRL_W { w: self }
    }
    #[doc = "Bit 4 - PPSEN0"]
    #[inline(always)]
    pub fn ppsen0(&mut self) -> PPSEN0_W {
        PPSEN0_W { w: self }
    }
    #[doc = "Bits 5:6 - TRGTMODSEL0"]
    #[inline(always)]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W {
        TRGTMODSEL0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\\[30:24\\]
of this register are valid only when four Flexible PPS outputs are selected. Bits\\[22:16\\]
are valid only when three or more Flexible PPS outputs are selected. Bits\\[14:8\\]
are valid only when two or more Flexible PPS outputs are selected. Bits\\[6:4\\]
are valid only when Flexible PPS feature is selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppscr](index.html) module"]
pub struct ETH_MACPPSCR_SPEC;
impl crate::RegisterSpec for ETH_MACPPSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macppscr::R](R) reader structure"]
impl crate::Readable for ETH_MACPPSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macppscr::W](W) writer structure"]
impl crate::Writable for ETH_MACPPSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACPPSCR to value 0"]
impl crate::Resettable for ETH_MACPPSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
