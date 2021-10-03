#[doc = "Register `RCC_PWRLPDLYCR` reader"]
pub struct R(crate::R<RCC_PWRLPDLYCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PWRLPDLYCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PWRLPDLYCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PWRLPDLYCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_PWRLPDLYCR` writer"]
pub struct W(crate::W<RCC_PWRLPDLYCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PWRLPDLYCR_SPEC>;
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
impl From<crate::W<RCC_PWRLPDLYCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PWRLPDLYCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRLP_DLY` reader - PWRLP_DLY"]
pub struct PWRLP_DLY_R(crate::FieldReader<u32, u32>);
impl PWRLP_DLY_R {
    pub(crate) fn new(bits: u32) -> Self {
        PWRLP_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRLP_DLY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRLP_DLY` writer - PWRLP_DLY"]
pub struct PWRLP_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRLP_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
#[doc = "Field `MCTMPSKP` reader - MCTMPSKP"]
pub struct MCTMPSKP_R(crate::FieldReader<bool, bool>);
impl MCTMPSKP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCTMPSKP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCTMPSKP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCTMPSKP` writer - MCTMPSKP"]
pub struct MCTMPSKP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCTMPSKP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - PWRLP_DLY"]
    #[inline(always)]
    pub fn pwrlp_dly(&self) -> PWRLP_DLY_R {
        PWRLP_DLY_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 24 - MCTMPSKP"]
    #[inline(always)]
    pub fn mctmpskp(&self) -> MCTMPSKP_R {
        MCTMPSKP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - PWRLP_DLY"]
    #[inline(always)]
    pub fn pwrlp_dly(&mut self) -> PWRLP_DLY_W {
        PWRLP_DLY_W { w: self }
    }
    #[doc = "Bit 24 - MCTMPSKP"]
    #[inline(always)]
    pub fn mctmpskp(&mut self) -> MCTMPSKP_W {
        MCTMPSKP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pwrlpdlycr](index.html) module"]
pub struct RCC_PWRLPDLYCR_SPEC;
impl crate::RegisterSpec for RCC_PWRLPDLYCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_pwrlpdlycr::R](R) reader structure"]
impl crate::Readable for RCC_PWRLPDLYCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_pwrlpdlycr::W](W) writer structure"]
impl crate::Writable for RCC_PWRLPDLYCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_PWRLPDLYCR to value 0"]
impl crate::Resettable for RCC_PWRLPDLYCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
