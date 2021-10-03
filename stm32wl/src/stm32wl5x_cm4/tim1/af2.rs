#[doc = "Register `AF2` reader"]
pub struct R(crate::R<AF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AF2` writer"]
pub struct W(crate::W<AF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF2_SPEC>;
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
impl From<crate::W<AF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BRK2 COMP2 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2CMP2P_A {
    #[doc = "0: Input polarity not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Input polarity inverted"]
    INVERTED = 1,
}
impl From<BK2CMP2P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity"]
pub struct BK2CMP2P_R(crate::FieldReader<bool, BK2CMP2P_A>);
impl BK2CMP2P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP2P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP2P_A {
        match self.bits {
            false => BK2CMP2P_A::NOTINVERTED,
            true => BK2CMP2P_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == BK2CMP2P_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == BK2CMP2P_A::INVERTED
    }
}
impl core::ops::Deref for BK2CMP2P_R {
    type Target = crate::FieldReader<bool, BK2CMP2P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity"]
pub struct BK2CMP2P_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP2P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2CMP2P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(BK2CMP2P_A::NOTINVERTED)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(BK2CMP2P_A::INVERTED)
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
#[doc = "BRK2 COMP1 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2CMP1P_A {
    #[doc = "0: Input polarity not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Input polarity inverted"]
    INVERTED = 1,
}
impl From<BK2CMP1P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarity"]
pub struct BK2CMP1P_R(crate::FieldReader<bool, BK2CMP1P_A>);
impl BK2CMP1P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP1P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP1P_A {
        match self.bits {
            false => BK2CMP1P_A::NOTINVERTED,
            true => BK2CMP1P_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == BK2CMP1P_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == BK2CMP1P_A::INVERTED
    }
}
impl core::ops::Deref for BK2CMP1P_R {
    type Target = crate::FieldReader<bool, BK2CMP1P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarity"]
pub struct BK2CMP1P_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP1P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2CMP1P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(BK2CMP1P_A::NOTINVERTED)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(BK2CMP1P_A::INVERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "BRK2 BKIN2 input polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2INP_A {
    #[doc = "0: Input polarity not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Input polarity inverted"]
    INVERTED = 1,
}
impl From<BK2INP_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2INP` reader - BRK2 BKIN2 input polarity"]
pub struct BK2INP_R(crate::FieldReader<bool, BK2INP_A>);
impl BK2INP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2INP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2INP_A {
        match self.bits {
            false => BK2INP_A::NOTINVERTED,
            true => BK2INP_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == BK2INP_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == BK2INP_A::INVERTED
    }
}
impl core::ops::Deref for BK2INP_R {
    type Target = crate::FieldReader<bool, BK2INP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2INP` writer - BRK2 BKIN2 input polarity"]
pub struct BK2INP_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2INP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input polarity not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(BK2INP_A::NOTINVERTED)
    }
    #[doc = "Input polarity inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(BK2INP_A::INVERTED)
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
#[doc = "BRK2 COMP2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2CMP2E_A {
    #[doc = "0: COMP2 input disabled"]
    DISABLED = 0,
    #[doc = "1: COMP2 input enabled"]
    ENABLED = 1,
}
impl From<BK2CMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable"]
pub struct BK2CMP2E_R(crate::FieldReader<bool, BK2CMP2E_A>);
impl BK2CMP2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP2E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP2E_A {
        match self.bits {
            false => BK2CMP2E_A::DISABLED,
            true => BK2CMP2E_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BK2CMP2E_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BK2CMP2E_A::ENABLED
    }
}
impl core::ops::Deref for BK2CMP2E_R {
    type Target = crate::FieldReader<bool, BK2CMP2E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable"]
pub struct BK2CMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP2E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2CMP2E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BK2CMP2E_A::DISABLED)
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BK2CMP2E_A::ENABLED)
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
#[doc = "BRK2 COMP1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2CMP1E_A {
    #[doc = "0: COMP1 input disabled"]
    DISABLED = 0,
    #[doc = "1: COMP1 input enabled"]
    ENABLED = 1,
}
impl From<BK2CMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable"]
pub struct BK2CMP1E_R(crate::FieldReader<bool, BK2CMP1E_A>);
impl BK2CMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2CMP1E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2CMP1E_A {
        match self.bits {
            false => BK2CMP1E_A::DISABLED,
            true => BK2CMP1E_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BK2CMP1E_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BK2CMP1E_A::ENABLED
    }
}
impl core::ops::Deref for BK2CMP1E_R {
    type Target = crate::FieldReader<bool, BK2CMP1E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable"]
pub struct BK2CMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2CMP1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2CMP1E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BK2CMP1E_A::DISABLED)
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BK2CMP1E_A::ENABLED)
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
#[doc = "BRK2 BKIN input enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2INE_A {
    #[doc = "0: BKIN input disabled"]
    DISABLED = 0,
    #[doc = "1: BKIN input enabled"]
    ENABLED = 1,
}
impl From<BK2INE_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2INE` reader - BRK2 BKIN input enable"]
pub struct BK2INE_R(crate::FieldReader<bool, BK2INE_A>);
impl BK2INE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BK2INE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2INE_A {
        match self.bits {
            false => BK2INE_A::DISABLED,
            true => BK2INE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BK2INE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BK2INE_A::ENABLED
    }
}
impl core::ops::Deref for BK2INE_R {
    type Target = crate::FieldReader<bool, BK2INE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BK2INE` writer - BRK2 BKIN input enable"]
pub struct BK2INE_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2INE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BK2INE_A::DISABLED)
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BK2INE_A::ENABLED)
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
impl R {
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W {
        BK2CMP2P_W { w: self }
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W {
        BK2CMP1P_W { w: self }
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity"]
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W {
        BK2INP_W { w: self }
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W {
        BK2CMP2E_W { w: self }
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W {
        BK2CMP1E_W { w: self }
    }
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    pub fn bk2ine(&mut self) -> BK2INE_W {
        BK2INE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate function register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af2](index.html) module"]
pub struct AF2_SPEC;
impl crate::RegisterSpec for AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [af2::R](R) reader structure"]
impl crate::Readable for AF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [af2::W](W) writer structure"]
impl crate::Writable for AF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AF2 to value 0x01"]
impl crate::Resettable for AF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
