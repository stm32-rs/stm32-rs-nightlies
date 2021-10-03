#[doc = "Register `DCKCFGR` reader"]
pub struct R(crate::R<DCKCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCKCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCKCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCKCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCKCFGR` writer"]
pub struct W(crate::W<DCKCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCKCFGR_SPEC>;
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
impl From<crate::W<DCKCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCKCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DFSDM1 audio clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKDFSDM1ASEL_A {
    #[doc = "0: CK_I2S_APB1 selected as audio clock"]
    I2S1 = 0,
    #[doc = "1: CK_I2S_APB2 selected as audio clock"]
    I2S2 = 1,
}
impl From<CKDFSDM1ASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKDFSDM1ASEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKDFSDM1ASEL` reader - DFSDM1 audio clock selection"]
pub struct CKDFSDM1ASEL_R(crate::FieldReader<u8, CKDFSDM1ASEL_A>);
impl CKDFSDM1ASEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKDFSDM1ASEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKDFSDM1ASEL_A> {
        match self.bits {
            0 => Some(CKDFSDM1ASEL_A::I2S1),
            1 => Some(CKDFSDM1ASEL_A::I2S2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `I2S1`"]
    #[inline(always)]
    pub fn is_i2s1(&self) -> bool {
        **self == CKDFSDM1ASEL_A::I2S1
    }
    #[doc = "Checks if the value of the field is `I2S2`"]
    #[inline(always)]
    pub fn is_i2s2(&self) -> bool {
        **self == CKDFSDM1ASEL_A::I2S2
    }
}
impl core::ops::Deref for CKDFSDM1ASEL_R {
    type Target = crate::FieldReader<u8, CKDFSDM1ASEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKDFSDM1ASEL` writer - DFSDM1 audio clock selection"]
pub struct CKDFSDM1ASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDFSDM1ASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKDFSDM1ASEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CK_I2S_APB1 selected as audio clock"]
    #[inline(always)]
    pub fn i2s1(self) -> &'a mut W {
        self.variant(CKDFSDM1ASEL_A::I2S1)
    }
    #[doc = "CK_I2S_APB2 selected as audio clock"]
    #[inline(always)]
    pub fn i2s2(self) -> &'a mut W {
        self.variant(CKDFSDM1ASEL_A::I2S2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Timers clocks prescalers selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMPRE_A {
    #[doc = "0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    MUL2 = 0,
    #[doc = "1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    MUL4 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMPRE` reader - Timers clocks prescalers selection"]
pub struct TIMPRE_R(crate::FieldReader<bool, TIMPRE_A>);
impl TIMPRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::MUL2,
            true => TIMPRE_A::MUL4,
        }
    }
    #[doc = "Checks if the value of the field is `MUL2`"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        **self == TIMPRE_A::MUL2
    }
    #[doc = "Checks if the value of the field is `MUL4`"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        **self == TIMPRE_A::MUL4
    }
}
impl core::ops::Deref for TIMPRE_R {
    type Target = crate::FieldReader<bool, TIMPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMPRE` writer - Timers clocks prescalers selection"]
pub struct TIMPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMPRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(TIMPRE_A::MUL2)
    }
    #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(TIMPRE_A::MUL4)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "I2S APB1 clocks source selection (I2S2/3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2S1SRC_A {
    #[doc = "0: I2Sx clock frequency = f(PLLI2S_R)"]
    PLLI2SR = 0,
    #[doc = "1: I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    I2S_CKIN = 1,
    #[doc = "2: I2Sx clock frequency = f(PLL_R)"]
    PLLR = 2,
    #[doc = "3: I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    HSI_HSE = 3,
}
impl From<I2S1SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: I2S1SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2S1SRC` reader - I2S APB1 clocks source selection (I2S2/3)"]
pub struct I2S1SRC_R(crate::FieldReader<u8, I2S1SRC_A>);
impl I2S1SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2S1SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S1SRC_A {
        match self.bits {
            0 => I2S1SRC_A::PLLI2SR,
            1 => I2S1SRC_A::I2S_CKIN,
            2 => I2S1SRC_A::PLLR,
            3 => I2S1SRC_A::HSI_HSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PLLI2SR`"]
    #[inline(always)]
    pub fn is_plli2sr(&self) -> bool {
        **self == I2S1SRC_A::PLLI2SR
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        **self == I2S1SRC_A::I2S_CKIN
    }
    #[doc = "Checks if the value of the field is `PLLR`"]
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        **self == I2S1SRC_A::PLLR
    }
    #[doc = "Checks if the value of the field is `HSI_HSE`"]
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        **self == I2S1SRC_A::HSI_HSE
    }
}
impl core::ops::Deref for I2S1SRC_R {
    type Target = crate::FieldReader<u8, I2S1SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S1SRC` writer - I2S APB1 clocks source selection (I2S2/3)"]
pub struct I2S1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S1SRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "I2Sx clock frequency = f(PLLI2S_R)"]
    #[inline(always)]
    pub fn plli2sr(self) -> &'a mut W {
        self.variant(I2S1SRC_A::PLLI2SR)
    }
    #[doc = "I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(I2S1SRC_A::I2S_CKIN)
    }
    #[doc = "I2Sx clock frequency = f(PLL_R)"]
    #[inline(always)]
    pub fn pllr(self) -> &'a mut W {
        self.variant(I2S1SRC_A::PLLR)
    }
    #[doc = "I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut W {
        self.variant(I2S1SRC_A::HSI_HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "I2S APB2 clocks source selection (I2S1/4/5)"]
pub type I2S2SRC_A = I2S1SRC_A;
#[doc = "Field `I2S2SRC` reader - I2S APB2 clocks source selection (I2S1/4/5)"]
pub type I2S2SRC_R = I2S1SRC_R;
#[doc = "Field `I2S2SRC` writer - I2S APB2 clocks source selection (I2S1/4/5)"]
pub struct I2S2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S2SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S2SRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "I2Sx clock frequency = f(PLLI2S_R)"]
    #[inline(always)]
    pub fn plli2sr(self) -> &'a mut W {
        self.variant(I2S2SRC_A::PLLI2SR)
    }
    #[doc = "I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(I2S2SRC_A::I2S_CKIN)
    }
    #[doc = "I2Sx clock frequency = f(PLL_R)"]
    #[inline(always)]
    pub fn pllr(self) -> &'a mut W {
        self.variant(I2S2SRC_A::PLLR)
    }
    #[doc = "I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut W {
        self.variant(I2S2SRC_A::HSI_HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "DFSDM1 Kernel clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKDFSDM1SEL_A {
    #[doc = "0: APB2 clock used as Kernel clock"]
    APB2 = 0,
    #[doc = "1: System clock used as Kernel clock"]
    SYSCLK = 1,
}
impl From<CKDFSDM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CKDFSDM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKDFSDM1SEL` reader - DFSDM1 Kernel clock selection"]
pub struct CKDFSDM1SEL_R(crate::FieldReader<bool, CKDFSDM1SEL_A>);
impl CKDFSDM1SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKDFSDM1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKDFSDM1SEL_A {
        match self.bits {
            false => CKDFSDM1SEL_A::APB2,
            true => CKDFSDM1SEL_A::SYSCLK,
        }
    }
    #[doc = "Checks if the value of the field is `APB2`"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        **self == CKDFSDM1SEL_A::APB2
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == CKDFSDM1SEL_A::SYSCLK
    }
}
impl core::ops::Deref for CKDFSDM1SEL_R {
    type Target = crate::FieldReader<bool, CKDFSDM1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKDFSDM1SEL` writer - DFSDM1 Kernel clock selection"]
pub struct CKDFSDM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDFSDM1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKDFSDM1SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "APB2 clock used as Kernel clock"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut W {
        self.variant(CKDFSDM1SEL_A::APB2)
    }
    #[doc = "System clock used as Kernel clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(CKDFSDM1SEL_A::SYSCLK)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 15:19 - DFSDM1 audio clock selection"]
    #[inline(always)]
    pub fn ckdfsdm1asel(&self) -> CKDFSDM1ASEL_R {
        CKDFSDM1ASEL_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - I2S APB1 clocks source selection (I2S2/3)"]
    #[inline(always)]
    pub fn i2s1src(&self) -> I2S1SRC_R {
        I2S1SRC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 27:28 - I2S APB2 clocks source selection (I2S1/4/5)"]
    #[inline(always)]
    pub fn i2s2src(&self) -> I2S2SRC_R {
        I2S2SRC_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 31 - DFSDM1 Kernel clock selection"]
    #[inline(always)]
    pub fn ckdfsdm1sel(&self) -> CKDFSDM1SEL_R {
        CKDFSDM1SEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 15:19 - DFSDM1 audio clock selection"]
    #[inline(always)]
    pub fn ckdfsdm1asel(&mut self) -> CKDFSDM1ASEL_W {
        CKDFSDM1ASEL_W { w: self }
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W {
        TIMPRE_W { w: self }
    }
    #[doc = "Bits 25:26 - I2S APB1 clocks source selection (I2S2/3)"]
    #[inline(always)]
    pub fn i2s1src(&mut self) -> I2S1SRC_W {
        I2S1SRC_W { w: self }
    }
    #[doc = "Bits 27:28 - I2S APB2 clocks source selection (I2S1/4/5)"]
    #[inline(always)]
    pub fn i2s2src(&mut self) -> I2S2SRC_W {
        I2S2SRC_W { w: self }
    }
    #[doc = "Bit 31 - DFSDM1 Kernel clock selection"]
    #[inline(always)]
    pub fn ckdfsdm1sel(&mut self) -> CKDFSDM1SEL_W {
        CKDFSDM1SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dedicated Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dckcfgr](index.html) module"]
pub struct DCKCFGR_SPEC;
impl crate::RegisterSpec for DCKCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dckcfgr::R](R) reader structure"]
impl crate::Readable for DCKCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dckcfgr::W](W) writer structure"]
impl crate::Writable for DCKCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCKCFGR to value 0"]
impl crate::Resettable for DCKCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
