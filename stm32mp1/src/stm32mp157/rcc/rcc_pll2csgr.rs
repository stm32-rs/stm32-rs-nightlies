#[doc = "Register `RCC_PLL2CSGR` reader"]
pub struct R(crate::R<RCC_PLL2CSGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL2CSGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL2CSGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL2CSGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_PLL2CSGR` writer"]
pub struct W(crate::W<RCC_PLL2CSGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL2CSGR_SPEC>;
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
impl From<crate::W<RCC_PLL2CSGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL2CSGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOD_PER` reader - MOD_PER"]
pub struct MOD_PER_R(crate::FieldReader<u16, u16>);
impl MOD_PER_R {
    pub(crate) fn new(bits: u16) -> Self {
        MOD_PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD_PER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD_PER` writer - MOD_PER"]
pub struct MOD_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
#[doc = "Field `TPDFN_DIS` reader - TPDFN_DIS"]
pub struct TPDFN_DIS_R(crate::FieldReader<bool, bool>);
impl TPDFN_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPDFN_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPDFN_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPDFN_DIS` writer - TPDFN_DIS"]
pub struct TPDFN_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TPDFN_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `RPDFN_DIS` reader - RPDFN_DIS"]
pub struct RPDFN_DIS_R(crate::FieldReader<bool, bool>);
impl RPDFN_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPDFN_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPDFN_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPDFN_DIS` writer - RPDFN_DIS"]
pub struct RPDFN_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RPDFN_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SSCG_MODE` reader - SSCG_MODE"]
pub struct SSCG_MODE_R(crate::FieldReader<bool, bool>);
impl SSCG_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSCG_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSCG_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSCG_MODE` writer - SSCG_MODE"]
pub struct SSCG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSCG_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `INC_STEP` reader - INC_STEP"]
pub struct INC_STEP_R(crate::FieldReader<u16, u16>);
impl INC_STEP_R {
    pub(crate) fn new(bits: u16) -> Self {
        INC_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_STEP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INC_STEP` writer - INC_STEP"]
pub struct INC_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | ((value as u32 & 0x7fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - MOD_PER"]
    #[inline(always)]
    pub fn mod_per(&self) -> MOD_PER_R {
        MOD_PER_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - TPDFN_DIS"]
    #[inline(always)]
    pub fn tpdfn_dis(&self) -> TPDFN_DIS_R {
        TPDFN_DIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RPDFN_DIS"]
    #[inline(always)]
    pub fn rpdfn_dis(&self) -> RPDFN_DIS_R {
        RPDFN_DIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SSCG_MODE"]
    #[inline(always)]
    pub fn sscg_mode(&self) -> SSCG_MODE_R {
        SSCG_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:30 - INC_STEP"]
    #[inline(always)]
    pub fn inc_step(&self) -> INC_STEP_R {
        INC_STEP_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - MOD_PER"]
    #[inline(always)]
    pub fn mod_per(&mut self) -> MOD_PER_W {
        MOD_PER_W { w: self }
    }
    #[doc = "Bit 13 - TPDFN_DIS"]
    #[inline(always)]
    pub fn tpdfn_dis(&mut self) -> TPDFN_DIS_W {
        TPDFN_DIS_W { w: self }
    }
    #[doc = "Bit 14 - RPDFN_DIS"]
    #[inline(always)]
    pub fn rpdfn_dis(&mut self) -> RPDFN_DIS_W {
        RPDFN_DIS_W { w: self }
    }
    #[doc = "Bit 15 - SSCG_MODE"]
    #[inline(always)]
    pub fn sscg_mode(&mut self) -> SSCG_MODE_W {
        SSCG_MODE_W { w: self }
    }
    #[doc = "Bits 16:30 - INC_STEP"]
    #[inline(always)]
    pub fn inc_step(&mut self) -> INC_STEP_W {
        INC_STEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll2csgr](index.html) module"]
pub struct RCC_PLL2CSGR_SPEC;
impl crate::RegisterSpec for RCC_PLL2CSGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_pll2csgr::R](R) reader structure"]
impl crate::Readable for RCC_PLL2CSGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_pll2csgr::W](W) writer structure"]
impl crate::Writable for RCC_PLL2CSGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_PLL2CSGR to value 0"]
impl crate::Resettable for RCC_PLL2CSGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
