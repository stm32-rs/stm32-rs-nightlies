#[doc = "Register `FCR` reader"]
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FIFO error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: FE interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: FE interrupt enabled"]
    ENABLED = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIE` reader - FIFO error interrupt enable"]
pub struct FEIE_R(crate::FieldReader<bool, FEIE_A>);
impl FEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::DISABLED,
            true => FEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FEIE_A::ENABLED
    }
}
impl core::ops::Deref for FEIE_R {
    type Target = crate::FieldReader<bool, FEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIE` writer - FIFO error interrupt enable"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FEIE_A::DISABLED)
    }
    #[doc = "FE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FEIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "FIFO status\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FS_A {
    #[doc = "0: 0 < fifo_level < 1/4"]
    QUARTER1 = 0,
    #[doc = "1: 1/4 <= fifo_level < 1/2"]
    QUARTER2 = 1,
    #[doc = "2: 1/2 <= fifo_level < 3/4"]
    QUARTER3 = 2,
    #[doc = "3: 3/4 <= fifo_level < full"]
    QUARTER4 = 3,
    #[doc = "4: FIFO is empty"]
    EMPTY = 4,
    #[doc = "5: FIFO is full"]
    FULL = 5,
}
impl From<FS_A> for u8 {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FS` reader - FIFO status"]
pub struct FS_R(crate::FieldReader<u8, FS_A>);
impl FS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FS_A> {
        match self.bits {
            0 => Some(FS_A::QUARTER1),
            1 => Some(FS_A::QUARTER2),
            2 => Some(FS_A::QUARTER3),
            3 => Some(FS_A::QUARTER4),
            4 => Some(FS_A::EMPTY),
            5 => Some(FS_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `QUARTER1`"]
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        **self == FS_A::QUARTER1
    }
    #[doc = "Checks if the value of the field is `QUARTER2`"]
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        **self == FS_A::QUARTER2
    }
    #[doc = "Checks if the value of the field is `QUARTER3`"]
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        **self == FS_A::QUARTER3
    }
    #[doc = "Checks if the value of the field is `QUARTER4`"]
    #[inline(always)]
    pub fn is_quarter4(&self) -> bool {
        **self == FS_A::QUARTER4
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == FS_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == FS_A::FULL
    }
}
impl core::ops::Deref for FS_R {
    type Target = crate::FieldReader<u8, FS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Direct mode disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMDIS_A {
    #[doc = "0: Direct mode is enabled"]
    ENABLED = 0,
    #[doc = "1: Direct mode is disabled"]
    DISABLED = 1,
}
impl From<DMDIS_A> for bool {
    #[inline(always)]
    fn from(variant: DMDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMDIS` reader - Direct mode disable"]
pub struct DMDIS_R(crate::FieldReader<bool, DMDIS_A>);
impl DMDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMDIS_A {
        match self.bits {
            false => DMDIS_A::ENABLED,
            true => DMDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMDIS_A::DISABLED
    }
}
impl core::ops::Deref for DMDIS_R {
    type Target = crate::FieldReader<bool, DMDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMDIS` writer - Direct mode disable"]
pub struct DMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Direct mode is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMDIS_A::ENABLED)
    }
    #[doc = "Direct mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMDIS_A::DISABLED)
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
#[doc = "FIFO threshold selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTH_A {
    #[doc = "0: 1/4 full FIFO"]
    QUARTER = 0,
    #[doc = "1: 1/2 full FIFO"]
    HALF = 1,
    #[doc = "2: 3/4 full FIFO"]
    THREEQUARTERS = 2,
    #[doc = "3: Full FIFO"]
    FULL = 3,
}
impl From<FTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTH` reader - FIFO threshold selection"]
pub struct FTH_R(crate::FieldReader<u8, FTH_A>);
impl FTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        FTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTH_A {
        match self.bits {
            0 => FTH_A::QUARTER,
            1 => FTH_A::HALF,
            2 => FTH_A::THREEQUARTERS,
            3 => FTH_A::FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `QUARTER`"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        **self == FTH_A::QUARTER
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        **self == FTH_A::HALF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERS`"]
    #[inline(always)]
    pub fn is_three_quarters(&self) -> bool {
        **self == FTH_A::THREEQUARTERS
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == FTH_A::FULL
    }
}
impl core::ops::Deref for FTH_R {
    type Target = crate::FieldReader<u8, FTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTH` writer - FIFO threshold selection"]
pub struct FTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1/4 full FIFO"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER)
    }
    #[doc = "1/2 full FIFO"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FTH_A::HALF)
    }
    #[doc = "3/4 full FIFO"]
    #[inline(always)]
    pub fn three_quarters(self) -> &'a mut W {
        self.variant(FTH_A::THREEQUARTERS)
    }
    #[doc = "Full FIFO"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(FTH_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&mut self) -> DMDIS_W {
        DMDIS_W { w: self }
    }
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W {
        FTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr::R](R) reader structure"]
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR to value 0x21"]
impl crate::Resettable for FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x21
    }
}
