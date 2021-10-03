#[doc = "Register `SRDCFGR` reader"]
pub struct R(crate::R<SRDCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRDCFGR` writer"]
pub struct W(crate::W<SRDCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRDCFGR_SPEC>;
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
impl From<crate::W<SRDCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRDCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRDPPRE_A {
    #[doc = "4: rcc_pclk4 = rcc_hclk4 / 2"]
    B_0X4 = 4,
    #[doc = "5: rcc_pclk4 = rcc_hclk4 / 4"]
    B_0X5 = 5,
    #[doc = "6: rcc_pclk4 = rcc_hclk4 / 8"]
    B_0X6 = 6,
    #[doc = "7: rcc_pclk4 = rcc_hclk4 / 16"]
    B_0X7 = 7,
}
impl From<SRDPPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRDPPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRDPPRE` reader - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
pub struct SRDPPRE_R(crate::FieldReader<u8, SRDPPRE_A>);
impl SRDPPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRDPPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRDPPRE_A> {
        match self.bits {
            4 => Some(SRDPPRE_A::B_0X4),
            5 => Some(SRDPPRE_A::B_0X5),
            6 => Some(SRDPPRE_A::B_0X6),
            7 => Some(SRDPPRE_A::B_0X7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == SRDPPRE_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == SRDPPRE_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == SRDPPRE_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == SRDPPRE_A::B_0X7
    }
}
impl core::ops::Deref for SRDPPRE_R {
    type Target = crate::FieldReader<u8, SRDPPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRDPPRE` writer - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
pub struct SRDPPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRDPPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRDPPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "rcc_pclk4 = rcc_hclk4 / 2"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(SRDPPRE_A::B_0X4)
    }
    #[doc = "rcc_pclk4 = rcc_hclk4 / 4"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(SRDPPRE_A::B_0X5)
    }
    #[doc = "rcc_pclk4 = rcc_hclk4 / 8"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(SRDPPRE_A::B_0X6)
    }
    #[doc = "rcc_pclk4 = rcc_hclk4 / 16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(SRDPPRE_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
    #[inline(always)]
    pub fn srdppre(&self) -> SRDPPRE_R {
        SRDPPRE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - SmartRun domain APB4 prescaler Set and reset by software to control the SmartRun domain APB4 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk4 after SRDPPRE write. 0xx: rcc_pclk4 = rcc_hclk4 (default after reset)"]
    #[inline(always)]
    pub fn srdppre(&mut self) -> SRDPPRE_W {
        SRDPPRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdcfgr](index.html) module"]
pub struct SRDCFGR_SPEC;
impl crate::RegisterSpec for SRDCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdcfgr::R](R) reader structure"]
impl crate::Readable for SRDCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srdcfgr::W](W) writer structure"]
impl crate::Writable for SRDCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRDCFGR to value 0"]
impl crate::Resettable for SRDCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
