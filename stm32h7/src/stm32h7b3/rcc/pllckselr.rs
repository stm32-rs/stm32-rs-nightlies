#[doc = "Register `PLLCKSELR` reader"]
pub struct R(crate::R<PLLCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCKSELR` writer"]
pub struct W(crate::W<PLLCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCKSELR_SPEC>;
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
impl From<crate::W<PLLCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLLSRC must be set to '11â\u{80}\u{99}.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "0: HSI selected as PLL clock"]
    HSI = 0,
    #[doc = "1: CSI selected as PLL clock"]
    CSI = 1,
    #[doc = "2: HSE selected as PLL clock"]
    HSE = 2,
    #[doc = "3: No clock sent to DIVMx dividers and PLLs"]
    NONE = 3,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLSRC` reader - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLLSRC must be set to '11â\u{80}\u{99}."]
pub struct PLLSRC_R(crate::FieldReader<u8, PLLSRC_A>);
impl PLLSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            0 => PLLSRC_A::HSI,
            1 => PLLSRC_A::CSI,
            2 => PLLSRC_A::HSE,
            3 => PLLSRC_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == PLLSRC_A::HSI
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        **self == PLLSRC_A::CSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == PLLSRC_A::HSE
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == PLLSRC_A::NONE
    }
}
impl core::ops::Deref for PLLSRC_R {
    type Target = crate::FieldReader<u8, PLLSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSRC` writer - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLLSRC must be set to '11â\u{80}\u{99}."]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "HSI selected as PLL clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI)
    }
    #[doc = "CSI selected as PLL clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(PLLSRC_A::CSI)
    }
    #[doc = "HSE selected as PLL clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSE)
    }
    #[doc = "No clock sent to DIVMx dividers and PLLs"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PLLSRC_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DIVM1` reader - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ONÂ =Â 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
pub struct DIVM1_R(crate::FieldReader<u8, u8>);
impl DIVM1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVM1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVM1` writer - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ONÂ =Â 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
pub struct DIVM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | ((value as u32 & 0x3f) << 4);
        self.w
    }
}
#[doc = "Field `DIVM2` reader - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ONÂ =Â 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
pub struct DIVM2_R(crate::FieldReader<u8, u8>);
impl DIVM2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVM2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVM2` writer - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ONÂ =Â 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
pub struct DIVM2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `DIVM3` reader - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ONÂ =Â 1). In order to save power when PLL3 is not used, the value of DIVM3 must be set to 0. ... ..."]
pub struct DIVM3_R(crate::FieldReader<u8, u8>);
impl DIVM3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVM3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVM3` writer - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ONÂ =Â 1). In order to save power when PLL3 is not used, the value of DIVM3 must be set to 0. ... ..."]
pub struct DIVM3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | ((value as u32 & 0x3f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLLSRC must be set to '11â\u{80}\u{99}."]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:9 - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ONÂ =Â 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn divm1(&self) -> DIVM1_R {
        DIVM1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ONÂ =Â 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn divm2(&self) -> DIVM2_R {
        DIVM2_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ONÂ =Â 1). In order to save power when PLL3 is not used, the value of DIVM3 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn divm3(&self) -> DIVM3_R {
        DIVM3_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLLSRC must be set to '11â\u{80}\u{99}."]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Bits 4:9 - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ONÂ =Â 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn divm1(&mut self) -> DIVM1_W {
        DIVM1_W { w: self }
    }
    #[doc = "Bits 12:17 - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ONÂ =Â 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn divm2(&mut self) -> DIVM2_W {
        DIVM2_W { w: self }
    }
    #[doc = "Bits 20:25 - prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The hardware does not allow any modification of this prescaler when PLL3 is enabled (PLL3ONÂ =Â 1). In order to save power when PLL3 is not used, the value of DIVM3 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn divm3(&mut self) -> DIVM3_W {
        DIVM3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllckselr](index.html) module"]
pub struct PLLCKSELR_SPEC;
impl crate::RegisterSpec for PLLCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllckselr::R](R) reader structure"]
impl crate::Readable for PLLCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllckselr::W](W) writer structure"]
impl crate::Writable for PLLCKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCKSELR to value 0x0202_0200"]
impl crate::Resettable for PLLCKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0202_0200
    }
}
