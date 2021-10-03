#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FUNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: Cosine funciton"]
    COSINE = 0,
    #[doc = "1: Sine function"]
    SINE = 1,
    #[doc = "2: Phase function"]
    PHASE = 2,
    #[doc = "3: Modulus function"]
    MODULUS = 3,
    #[doc = "4: Arctangent function"]
    ARCTANGENT = 4,
    #[doc = "5: Hyperbolic Cosine function"]
    HYPERBOLICCOSINE = 5,
    #[doc = "6: Hyperbolic Sine function"]
    HYPERBOLICSINE = 6,
    #[doc = "7: Arctanh function"]
    ARCTANH = 7,
    #[doc = "8: Natural Logarithm function"]
    NATURALLOGARITHM = 8,
    #[doc = "9: Square Root function"]
    SQUAREROOT = 9,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNC` reader - FUNC"]
pub struct FUNC_R(crate::FieldReader<u8, FUNC_A>);
impl FUNC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::COSINE),
            1 => Some(FUNC_A::SINE),
            2 => Some(FUNC_A::PHASE),
            3 => Some(FUNC_A::MODULUS),
            4 => Some(FUNC_A::ARCTANGENT),
            5 => Some(FUNC_A::HYPERBOLICCOSINE),
            6 => Some(FUNC_A::HYPERBOLICSINE),
            7 => Some(FUNC_A::ARCTANH),
            8 => Some(FUNC_A::NATURALLOGARITHM),
            9 => Some(FUNC_A::SQUAREROOT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COSINE`"]
    #[inline(always)]
    pub fn is_cosine(&self) -> bool {
        **self == FUNC_A::COSINE
    }
    #[doc = "Checks if the value of the field is `SINE`"]
    #[inline(always)]
    pub fn is_sine(&self) -> bool {
        **self == FUNC_A::SINE
    }
    #[doc = "Checks if the value of the field is `PHASE`"]
    #[inline(always)]
    pub fn is_phase(&self) -> bool {
        **self == FUNC_A::PHASE
    }
    #[doc = "Checks if the value of the field is `MODULUS`"]
    #[inline(always)]
    pub fn is_modulus(&self) -> bool {
        **self == FUNC_A::MODULUS
    }
    #[doc = "Checks if the value of the field is `ARCTANGENT`"]
    #[inline(always)]
    pub fn is_arctangent(&self) -> bool {
        **self == FUNC_A::ARCTANGENT
    }
    #[doc = "Checks if the value of the field is `HYPERBOLICCOSINE`"]
    #[inline(always)]
    pub fn is_hyperbolic_cosine(&self) -> bool {
        **self == FUNC_A::HYPERBOLICCOSINE
    }
    #[doc = "Checks if the value of the field is `HYPERBOLICSINE`"]
    #[inline(always)]
    pub fn is_hyperbolic_sine(&self) -> bool {
        **self == FUNC_A::HYPERBOLICSINE
    }
    #[doc = "Checks if the value of the field is `ARCTANH`"]
    #[inline(always)]
    pub fn is_arctanh(&self) -> bool {
        **self == FUNC_A::ARCTANH
    }
    #[doc = "Checks if the value of the field is `NATURALLOGARITHM`"]
    #[inline(always)]
    pub fn is_natural_logarithm(&self) -> bool {
        **self == FUNC_A::NATURALLOGARITHM
    }
    #[doc = "Checks if the value of the field is `SQUAREROOT`"]
    #[inline(always)]
    pub fn is_square_root(&self) -> bool {
        **self == FUNC_A::SQUAREROOT
    }
}
impl core::ops::Deref for FUNC_R {
    type Target = crate::FieldReader<u8, FUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC` writer - FUNC"]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Cosine funciton"]
    #[inline(always)]
    pub fn cosine(self) -> &'a mut W {
        self.variant(FUNC_A::COSINE)
    }
    #[doc = "Sine function"]
    #[inline(always)]
    pub fn sine(self) -> &'a mut W {
        self.variant(FUNC_A::SINE)
    }
    #[doc = "Phase function"]
    #[inline(always)]
    pub fn phase(self) -> &'a mut W {
        self.variant(FUNC_A::PHASE)
    }
    #[doc = "Modulus function"]
    #[inline(always)]
    pub fn modulus(self) -> &'a mut W {
        self.variant(FUNC_A::MODULUS)
    }
    #[doc = "Arctangent function"]
    #[inline(always)]
    pub fn arctangent(self) -> &'a mut W {
        self.variant(FUNC_A::ARCTANGENT)
    }
    #[doc = "Hyperbolic Cosine function"]
    #[inline(always)]
    pub fn hyperbolic_cosine(self) -> &'a mut W {
        self.variant(FUNC_A::HYPERBOLICCOSINE)
    }
    #[doc = "Hyperbolic Sine function"]
    #[inline(always)]
    pub fn hyperbolic_sine(self) -> &'a mut W {
        self.variant(FUNC_A::HYPERBOLICSINE)
    }
    #[doc = "Arctanh function"]
    #[inline(always)]
    pub fn arctanh(self) -> &'a mut W {
        self.variant(FUNC_A::ARCTANH)
    }
    #[doc = "Natural Logarithm function"]
    #[inline(always)]
    pub fn natural_logarithm(self) -> &'a mut W {
        self.variant(FUNC_A::NATURALLOGARITHM)
    }
    #[doc = "Square Root function"]
    #[inline(always)]
    pub fn square_root(self) -> &'a mut W {
        self.variant(FUNC_A::SQUAREROOT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PRECISION` reader - Precision (number of iterations/cycles) required"]
pub struct PRECISION_R(crate::FieldReader<u8, u8>);
impl PRECISION_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRECISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRECISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRECISION` writer - Precision (number of iterations/cycles) required"]
pub struct PRECISION_W<'a> {
    w: &'a mut W,
}
impl<'a> PRECISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `SCALE` reader - Scaling factor (2^-n for arguments, 2^n for results)"]
pub struct SCALE_R(crate::FieldReader<u8, u8>);
impl SCALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCALE` writer - Scaling factor (2^-n for arguments, 2^n for results)"]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "IEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEN_A {
    #[doc = "0: Disable interrupt request generation"]
    DISABLED = 0,
    #[doc = "1: Enable intterrupt request generation"]
    ENABLED = 1,
}
impl From<IEN_A> for bool {
    #[inline(always)]
    fn from(variant: IEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEN` reader - IEN"]
pub struct IEN_R(crate::FieldReader<bool, IEN_A>);
impl IEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN_A {
        match self.bits {
            false => IEN_A::DISABLED,
            true => IEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IEN_A::ENABLED
    }
}
impl core::ops::Deref for IEN_R {
    type Target = crate::FieldReader<bool, IEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEN` writer - IEN"]
pub struct IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt request generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IEN_A::DISABLED)
    }
    #[doc = "Enable intterrupt request generation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "DMAREN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAREN_A {
    #[doc = "0: No DMA channel reads are generated"]
    DISABLED = 0,
    #[doc = "1: Read requests are generated on the DMA channel when RRDY flag is set"]
    ENABLED = 1,
}
impl From<DMAREN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAREN` reader - DMAREN"]
pub struct DMAREN_R(crate::FieldReader<bool, DMAREN_A>);
impl DMAREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAREN_A {
        match self.bits {
            false => DMAREN_A::DISABLED,
            true => DMAREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAREN_A::ENABLED
    }
}
impl core::ops::Deref for DMAREN_R {
    type Target = crate::FieldReader<bool, DMAREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAREN` writer - DMAREN"]
pub struct DMAREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DMA channel reads are generated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAREN_A::DISABLED)
    }
    #[doc = "Read requests are generated on the DMA channel when RRDY flag is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAREN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "DMAWEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAWEN_A {
    #[doc = "0: No DMA channel writes are generated"]
    DISABLED = 0,
    #[doc = "1: Write requests are generated on the DMA channel when no operation is pending"]
    ENABLED = 1,
}
impl From<DMAWEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAWEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAWEN` reader - DMAWEN"]
pub struct DMAWEN_R(crate::FieldReader<bool, DMAWEN_A>);
impl DMAWEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAWEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAWEN_A {
        match self.bits {
            false => DMAWEN_A::DISABLED,
            true => DMAWEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAWEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAWEN_A::ENABLED
    }
}
impl core::ops::Deref for DMAWEN_R {
    type Target = crate::FieldReader<bool, DMAWEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAWEN` writer - DMAWEN"]
pub struct DMAWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAWEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAWEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DMA channel writes are generated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAWEN_A::DISABLED)
    }
    #[doc = "Write requests are generated on the DMA channel when no operation is pending"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAWEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "NRES\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRES_A {
    #[doc = "0: Only single result value will be returned. After a single read RRDY will be automatically cleared"]
    NUM1 = 0,
    #[doc = "1: Two return reads need to be performed. After two reads RRDY will be automatically cleared"]
    NUM2 = 1,
}
impl From<NRES_A> for bool {
    #[inline(always)]
    fn from(variant: NRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRES` reader - NRES"]
pub struct NRES_R(crate::FieldReader<bool, NRES_A>);
impl NRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        NRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRES_A {
        match self.bits {
            false => NRES_A::NUM1,
            true => NRES_A::NUM2,
        }
    }
    #[doc = "Checks if the value of the field is `NUM1`"]
    #[inline(always)]
    pub fn is_num1(&self) -> bool {
        **self == NRES_A::NUM1
    }
    #[doc = "Checks if the value of the field is `NUM2`"]
    #[inline(always)]
    pub fn is_num2(&self) -> bool {
        **self == NRES_A::NUM2
    }
}
impl core::ops::Deref for NRES_R {
    type Target = crate::FieldReader<bool, NRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRES` writer - NRES"]
pub struct NRES_W<'a> {
    w: &'a mut W,
}
impl<'a> NRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NRES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only single result value will be returned. After a single read RRDY will be automatically cleared"]
    #[inline(always)]
    pub fn num1(self) -> &'a mut W {
        self.variant(NRES_A::NUM1)
    }
    #[doc = "Two return reads need to be performed. After two reads RRDY will be automatically cleared"]
    #[inline(always)]
    pub fn num2(self) -> &'a mut W {
        self.variant(NRES_A::NUM2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "NARGS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NARGS_A {
    #[doc = "0: Only single argument write is needed for next calculation"]
    NUM1 = 0,
    #[doc = "1: Two argument writes need to be performed for next calculation"]
    NUM2 = 1,
}
impl From<NARGS_A> for bool {
    #[inline(always)]
    fn from(variant: NARGS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NARGS` reader - NARGS"]
pub struct NARGS_R(crate::FieldReader<bool, NARGS_A>);
impl NARGS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NARGS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NARGS_A {
        match self.bits {
            false => NARGS_A::NUM1,
            true => NARGS_A::NUM2,
        }
    }
    #[doc = "Checks if the value of the field is `NUM1`"]
    #[inline(always)]
    pub fn is_num1(&self) -> bool {
        **self == NARGS_A::NUM1
    }
    #[doc = "Checks if the value of the field is `NUM2`"]
    #[inline(always)]
    pub fn is_num2(&self) -> bool {
        **self == NARGS_A::NUM2
    }
}
impl core::ops::Deref for NARGS_R {
    type Target = crate::FieldReader<bool, NARGS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NARGS` writer - NARGS"]
pub struct NARGS_W<'a> {
    w: &'a mut W,
}
impl<'a> NARGS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NARGS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only single argument write is needed for next calculation"]
    #[inline(always)]
    pub fn num1(self) -> &'a mut W {
        self.variant(NARGS_A::NUM1)
    }
    #[doc = "Two argument writes need to be performed for next calculation"]
    #[inline(always)]
    pub fn num2(self) -> &'a mut W {
        self.variant(NARGS_A::NUM2)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "RESSIZE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESSIZE_A {
    #[doc = "0: Use 32 bit output values"]
    BITS32 = 0,
    #[doc = "1: Use 16 bit output values"]
    BITS16 = 1,
}
impl From<RESSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: RESSIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESSIZE` reader - RESSIZE"]
pub struct RESSIZE_R(crate::FieldReader<bool, RESSIZE_A>);
impl RESSIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESSIZE_A {
        match self.bits {
            false => RESSIZE_A::BITS32,
            true => RESSIZE_A::BITS16,
        }
    }
    #[doc = "Checks if the value of the field is `BITS32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        **self == RESSIZE_A::BITS32
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        **self == RESSIZE_A::BITS16
    }
}
impl core::ops::Deref for RESSIZE_R {
    type Target = crate::FieldReader<bool, RESSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESSIZE` writer - RESSIZE"]
pub struct RESSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESSIZE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use 32 bit output values"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(RESSIZE_A::BITS32)
    }
    #[doc = "Use 16 bit output values"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(RESSIZE_A::BITS16)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "ARGSIZE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARGSIZE_A {
    #[doc = "0: Use 32 bit input values"]
    BITS32 = 0,
    #[doc = "1: Use 16 bit input values"]
    BITS16 = 1,
}
impl From<ARGSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: ARGSIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARGSIZE` reader - ARGSIZE"]
pub struct ARGSIZE_R(crate::FieldReader<bool, ARGSIZE_A>);
impl ARGSIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARGSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARGSIZE_A {
        match self.bits {
            false => ARGSIZE_A::BITS32,
            true => ARGSIZE_A::BITS16,
        }
    }
    #[doc = "Checks if the value of the field is `BITS32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        **self == ARGSIZE_A::BITS32
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        **self == ARGSIZE_A::BITS16
    }
}
impl core::ops::Deref for ARGSIZE_R {
    type Target = crate::FieldReader<bool, ARGSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARGSIZE` writer - ARGSIZE"]
pub struct ARGSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARGSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARGSIZE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use 32 bit input values"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(ARGSIZE_A::BITS32)
    }
    #[doc = "Use 16 bit input values"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(ARGSIZE_A::BITS16)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "RRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRDY_A {
    #[doc = "0: Results from computation are not read"]
    NOTREADY = 0,
    #[doc = "1: Results are ready, this flag will be automatically cleared once value is read"]
    READY = 1,
}
impl From<RRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRDY` reader - RRDY"]
pub struct RRDY_R(crate::FieldReader<bool, RRDY_A>);
impl RRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRDY_A {
        match self.bits {
            false => RRDY_A::NOTREADY,
            true => RRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == RRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == RRDY_A::READY
    }
}
impl core::ops::Deref for RRDY_R {
    type Target = crate::FieldReader<bool, RRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRDY` writer - RRDY"]
pub struct RRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Results from computation are not read"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(RRDY_A::NOTREADY)
    }
    #[doc = "Results are ready, this flag will be automatically cleared once value is read"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(RRDY_A::READY)
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
    #[doc = "Bits 0:3 - FUNC"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Precision (number of iterations/cycles) required"]
    #[inline(always)]
    pub fn precision(&self) -> PRECISION_R {
        PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Scaling factor (2^-n for arguments, 2^n for results)"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 16 - IEN"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DMAREN"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DMAWEN"]
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - NRES"]
    #[inline(always)]
    pub fn nres(&self) -> NRES_R {
        NRES_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - NARGS"]
    #[inline(always)]
    pub fn nargs(&self) -> NARGS_R {
        NARGS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - RESSIZE"]
    #[inline(always)]
    pub fn ressize(&self) -> RESSIZE_R {
        RESSIZE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ARGSIZE"]
    #[inline(always)]
    pub fn argsize(&self) -> ARGSIZE_R {
        ARGSIZE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RRDY"]
    #[inline(always)]
    pub fn rrdy(&self) -> RRDY_R {
        RRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FUNC"]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Bits 4:7 - Precision (number of iterations/cycles) required"]
    #[inline(always)]
    pub fn precision(&mut self) -> PRECISION_W {
        PRECISION_W { w: self }
    }
    #[doc = "Bits 8:10 - Scaling factor (2^-n for arguments, 2^n for results)"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Bit 16 - IEN"]
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W {
        IEN_W { w: self }
    }
    #[doc = "Bit 17 - DMAREN"]
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W {
        DMAREN_W { w: self }
    }
    #[doc = "Bit 18 - DMAWEN"]
    #[inline(always)]
    pub fn dmawen(&mut self) -> DMAWEN_W {
        DMAWEN_W { w: self }
    }
    #[doc = "Bit 19 - NRES"]
    #[inline(always)]
    pub fn nres(&mut self) -> NRES_W {
        NRES_W { w: self }
    }
    #[doc = "Bit 20 - NARGS"]
    #[inline(always)]
    pub fn nargs(&mut self) -> NARGS_W {
        NARGS_W { w: self }
    }
    #[doc = "Bit 21 - RESSIZE"]
    #[inline(always)]
    pub fn ressize(&mut self) -> RESSIZE_W {
        RESSIZE_W { w: self }
    }
    #[doc = "Bit 22 - ARGSIZE"]
    #[inline(always)]
    pub fn argsize(&mut self) -> ARGSIZE_W {
        ARGSIZE_W { w: self }
    }
    #[doc = "Bit 31 - RRDY"]
    #[inline(always)]
    pub fn rrdy(&mut self) -> RRDY_W {
        RRDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CORDIC Control Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
