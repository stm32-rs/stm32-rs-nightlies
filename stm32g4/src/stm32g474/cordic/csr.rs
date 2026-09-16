///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**FUNC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FUNC {
    ///0: Cosine function
    Cosine = 0,
    ///1: Sine function
    Sine = 1,
    ///2: Phase function
    Phase = 2,
    ///3: Modulus function
    Modulus = 3,
    ///4: Arctangent function
    Arctangent = 4,
    ///5: Hyperbolic Cosine function
    HyperbolicCosine = 5,
    ///6: Hyperbolic Sine function
    HyperbolicSine = 6,
    ///7: Arctanh function
    Arctanh = 7,
    ///8: Natural Logarithm function
    NaturalLogarithm = 8,
    ///9: Square Root function
    SquareRoot = 9,
}
impl From<FUNC> for u8 {
    #[inline(always)]
    fn from(variant: FUNC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FUNC {
    type Ux = u8;
}
impl crate::IsEnum for FUNC {}
///Field `FUNC` reader - FUNC
pub type FUNC_R = crate::FieldReader<FUNC>;
impl FUNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FUNC> {
        match self.bits {
            0 => Some(FUNC::Cosine),
            1 => Some(FUNC::Sine),
            2 => Some(FUNC::Phase),
            3 => Some(FUNC::Modulus),
            4 => Some(FUNC::Arctangent),
            5 => Some(FUNC::HyperbolicCosine),
            6 => Some(FUNC::HyperbolicSine),
            7 => Some(FUNC::Arctanh),
            8 => Some(FUNC::NaturalLogarithm),
            9 => Some(FUNC::SquareRoot),
            _ => None,
        }
    }
    ///Cosine function
    #[inline(always)]
    pub fn is_cosine(&self) -> bool {
        *self == FUNC::Cosine
    }
    ///Sine function
    #[inline(always)]
    pub fn is_sine(&self) -> bool {
        *self == FUNC::Sine
    }
    ///Phase function
    #[inline(always)]
    pub fn is_phase(&self) -> bool {
        *self == FUNC::Phase
    }
    ///Modulus function
    #[inline(always)]
    pub fn is_modulus(&self) -> bool {
        *self == FUNC::Modulus
    }
    ///Arctangent function
    #[inline(always)]
    pub fn is_arctangent(&self) -> bool {
        *self == FUNC::Arctangent
    }
    ///Hyperbolic Cosine function
    #[inline(always)]
    pub fn is_hyperbolic_cosine(&self) -> bool {
        *self == FUNC::HyperbolicCosine
    }
    ///Hyperbolic Sine function
    #[inline(always)]
    pub fn is_hyperbolic_sine(&self) -> bool {
        *self == FUNC::HyperbolicSine
    }
    ///Arctanh function
    #[inline(always)]
    pub fn is_arctanh(&self) -> bool {
        *self == FUNC::Arctanh
    }
    ///Natural Logarithm function
    #[inline(always)]
    pub fn is_natural_logarithm(&self) -> bool {
        *self == FUNC::NaturalLogarithm
    }
    ///Square Root function
    #[inline(always)]
    pub fn is_square_root(&self) -> bool {
        *self == FUNC::SquareRoot
    }
}
///Field `FUNC` writer - FUNC
pub type FUNC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, FUNC>;
impl<'a, REG> FUNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Cosine function
    #[inline(always)]
    pub fn cosine(self) -> &'a mut crate::W<REG> {
        self.variant(FUNC::Cosine)
    }
    ///Sine function
    #[inline(always)]
    pub fn sine(self) -> &'a mut crate::W<REG> {
        self.variant(FUNC::Sine)
    }
    ///Phase function
    #[inline(always)]
    pub fn phase(self) -> &'a mut crate::W<REG> {
        self.variant(FUNC::Phase)
    }
    ///Modulus function
    #[inline(always)]
    pub fn modulus(self) -> &'a mut crate::W<REG> {
        self.variant(FUNC::Modulus)
    }
    ///Arctangent function
    #[inline(always)]
    pub fn arctangent(self) -> &'a mut crate::W<REG> {
        self.variant(FUNC::Arctangent)
    }
    ///Hyperbolic Cosine function
    #[inline(always)]
    pub fn hyperbolic_cosine(self) -> &'a mut crate::W<REG> {
        self.variant(FUNC::HyperbolicCosine)
    }
    ///Hyperbolic Sine function
    #[inline(always)]
    pub fn hyperbolic_sine(self) -> &'a mut crate::W<REG> {
        self.variant(FUNC::HyperbolicSine)
    }
    ///Arctanh function
    #[inline(always)]
    pub fn arctanh(self) -> &'a mut crate::W<REG> {
        self.variant(FUNC::Arctanh)
    }
    ///Natural Logarithm function
    #[inline(always)]
    pub fn natural_logarithm(self) -> &'a mut crate::W<REG> {
        self.variant(FUNC::NaturalLogarithm)
    }
    ///Square Root function
    #[inline(always)]
    pub fn square_root(self) -> &'a mut crate::W<REG> {
        self.variant(FUNC::SquareRoot)
    }
}
/**Precision (number of iterations/cycles) required

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRECISION {
    ///1: 4 iterations
    Iters4 = 1,
    ///2: 8 iterations
    Iters8 = 2,
    ///3: 12 iterations
    Iters12 = 3,
    ///4: 16 iterations
    Iters16 = 4,
    ///5: 20 iterations
    Iters20 = 5,
    ///6: 24 iterations
    Iters24 = 6,
    ///7: 28 iterations
    Iters28 = 7,
    ///8: 32 iterations
    Iters32 = 8,
    ///9: 36 iterations
    Iters36 = 9,
    ///10: 40 iterations
    Iters40 = 10,
    ///11: 44 iterations
    Iters44 = 11,
    ///12: 48 iterations
    Iters48 = 12,
    ///13: 52 iterations
    Iters52 = 13,
    ///14: 56 iterations
    Iters56 = 14,
    ///15: 60 iterations
    Iters60 = 15,
}
impl From<PRECISION> for u8 {
    #[inline(always)]
    fn from(variant: PRECISION) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRECISION {
    type Ux = u8;
}
impl crate::IsEnum for PRECISION {}
///Field `PRECISION` reader - Precision (number of iterations/cycles) required
pub type PRECISION_R = crate::FieldReader<PRECISION>;
impl PRECISION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRECISION> {
        match self.bits {
            1 => Some(PRECISION::Iters4),
            2 => Some(PRECISION::Iters8),
            3 => Some(PRECISION::Iters12),
            4 => Some(PRECISION::Iters16),
            5 => Some(PRECISION::Iters20),
            6 => Some(PRECISION::Iters24),
            7 => Some(PRECISION::Iters28),
            8 => Some(PRECISION::Iters32),
            9 => Some(PRECISION::Iters36),
            10 => Some(PRECISION::Iters40),
            11 => Some(PRECISION::Iters44),
            12 => Some(PRECISION::Iters48),
            13 => Some(PRECISION::Iters52),
            14 => Some(PRECISION::Iters56),
            15 => Some(PRECISION::Iters60),
            _ => None,
        }
    }
    ///4 iterations
    #[inline(always)]
    pub fn is_iters4(&self) -> bool {
        *self == PRECISION::Iters4
    }
    ///8 iterations
    #[inline(always)]
    pub fn is_iters8(&self) -> bool {
        *self == PRECISION::Iters8
    }
    ///12 iterations
    #[inline(always)]
    pub fn is_iters12(&self) -> bool {
        *self == PRECISION::Iters12
    }
    ///16 iterations
    #[inline(always)]
    pub fn is_iters16(&self) -> bool {
        *self == PRECISION::Iters16
    }
    ///20 iterations
    #[inline(always)]
    pub fn is_iters20(&self) -> bool {
        *self == PRECISION::Iters20
    }
    ///24 iterations
    #[inline(always)]
    pub fn is_iters24(&self) -> bool {
        *self == PRECISION::Iters24
    }
    ///28 iterations
    #[inline(always)]
    pub fn is_iters28(&self) -> bool {
        *self == PRECISION::Iters28
    }
    ///32 iterations
    #[inline(always)]
    pub fn is_iters32(&self) -> bool {
        *self == PRECISION::Iters32
    }
    ///36 iterations
    #[inline(always)]
    pub fn is_iters36(&self) -> bool {
        *self == PRECISION::Iters36
    }
    ///40 iterations
    #[inline(always)]
    pub fn is_iters40(&self) -> bool {
        *self == PRECISION::Iters40
    }
    ///44 iterations
    #[inline(always)]
    pub fn is_iters44(&self) -> bool {
        *self == PRECISION::Iters44
    }
    ///48 iterations
    #[inline(always)]
    pub fn is_iters48(&self) -> bool {
        *self == PRECISION::Iters48
    }
    ///52 iterations
    #[inline(always)]
    pub fn is_iters52(&self) -> bool {
        *self == PRECISION::Iters52
    }
    ///56 iterations
    #[inline(always)]
    pub fn is_iters56(&self) -> bool {
        *self == PRECISION::Iters56
    }
    ///60 iterations
    #[inline(always)]
    pub fn is_iters60(&self) -> bool {
        *self == PRECISION::Iters60
    }
}
///Field `PRECISION` writer - Precision (number of iterations/cycles) required
pub type PRECISION_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRECISION>;
impl<'a, REG> PRECISION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///4 iterations
    #[inline(always)]
    pub fn iters4(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters4)
    }
    ///8 iterations
    #[inline(always)]
    pub fn iters8(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters8)
    }
    ///12 iterations
    #[inline(always)]
    pub fn iters12(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters12)
    }
    ///16 iterations
    #[inline(always)]
    pub fn iters16(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters16)
    }
    ///20 iterations
    #[inline(always)]
    pub fn iters20(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters20)
    }
    ///24 iterations
    #[inline(always)]
    pub fn iters24(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters24)
    }
    ///28 iterations
    #[inline(always)]
    pub fn iters28(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters28)
    }
    ///32 iterations
    #[inline(always)]
    pub fn iters32(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters32)
    }
    ///36 iterations
    #[inline(always)]
    pub fn iters36(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters36)
    }
    ///40 iterations
    #[inline(always)]
    pub fn iters40(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters40)
    }
    ///44 iterations
    #[inline(always)]
    pub fn iters44(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters44)
    }
    ///48 iterations
    #[inline(always)]
    pub fn iters48(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters48)
    }
    ///52 iterations
    #[inline(always)]
    pub fn iters52(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters52)
    }
    ///56 iterations
    #[inline(always)]
    pub fn iters56(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters56)
    }
    ///60 iterations
    #[inline(always)]
    pub fn iters60(self) -> &'a mut crate::W<REG> {
        self.variant(PRECISION::Iters60)
    }
}
///Field `SCALE` reader - Scaling factor (2^-n for arguments, 2^n for results)
pub type SCALE_R = crate::FieldReader;
///Field `SCALE` writer - Scaling factor (2^-n for arguments, 2^n for results)
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/**IEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN {
    ///0: Disable interrupt request generation
    Disabled = 0,
    ///1: Enable interrupt request generation
    Enabled = 1,
}
impl From<IEN> for bool {
    #[inline(always)]
    fn from(variant: IEN) -> Self {
        variant as u8 != 0
    }
}
///Field `IEN` reader - IEN
pub type IEN_R = crate::BitReader<IEN>;
impl IEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IEN {
        match self.bits {
            false => IEN::Disabled,
            true => IEN::Enabled,
        }
    }
    ///Disable interrupt request generation
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IEN::Disabled
    }
    ///Enable interrupt request generation
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IEN::Enabled
    }
}
///Field `IEN` writer - IEN
pub type IEN_W<'a, REG> = crate::BitWriter<'a, REG, IEN>;
impl<'a, REG> IEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable interrupt request generation
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IEN::Disabled)
    }
    ///Enable interrupt request generation
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IEN::Enabled)
    }
}
/**DMAREN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAREN {
    ///0: No DMA channel reads are generated
    Disabled = 0,
    ///1: Read requests are generated on the DMA channel when RRDY flag is set
    Enabled = 1,
}
impl From<DMAREN> for bool {
    #[inline(always)]
    fn from(variant: DMAREN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAREN` reader - DMAREN
pub type DMAREN_R = crate::BitReader<DMAREN>;
impl DMAREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAREN {
        match self.bits {
            false => DMAREN::Disabled,
            true => DMAREN::Enabled,
        }
    }
    ///No DMA channel reads are generated
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAREN::Disabled
    }
    ///Read requests are generated on the DMA channel when RRDY flag is set
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAREN::Enabled
    }
}
///Field `DMAREN` writer - DMAREN
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG, DMAREN>;
impl<'a, REG> DMAREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No DMA channel reads are generated
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREN::Disabled)
    }
    ///Read requests are generated on the DMA channel when RRDY flag is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREN::Enabled)
    }
}
/**DMAWEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAWEN {
    ///0: No DMA channel writes are generated
    Disabled = 0,
    ///1: Write requests are generated on the DMA channel when no operation is pending
    Enabled = 1,
}
impl From<DMAWEN> for bool {
    #[inline(always)]
    fn from(variant: DMAWEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAWEN` reader - DMAWEN
pub type DMAWEN_R = crate::BitReader<DMAWEN>;
impl DMAWEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAWEN {
        match self.bits {
            false => DMAWEN::Disabled,
            true => DMAWEN::Enabled,
        }
    }
    ///No DMA channel writes are generated
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAWEN::Disabled
    }
    ///Write requests are generated on the DMA channel when no operation is pending
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAWEN::Enabled
    }
}
///Field `DMAWEN` writer - DMAWEN
pub type DMAWEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAWEN>;
impl<'a, REG> DMAWEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No DMA channel writes are generated
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAWEN::Disabled)
    }
    ///Write requests are generated on the DMA channel when no operation is pending
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAWEN::Enabled)
    }
}
/**NRES

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRES {
    ///0: Only single result value will be returned. After a single read RRDY will be automatically cleared
    Num1 = 0,
    ///1: Two return reads need to be performed. After two reads RRDY will be automatically cleared
    Num2 = 1,
}
impl From<NRES> for bool {
    #[inline(always)]
    fn from(variant: NRES) -> Self {
        variant as u8 != 0
    }
}
///Field `NRES` reader - NRES
pub type NRES_R = crate::BitReader<NRES>;
impl NRES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NRES {
        match self.bits {
            false => NRES::Num1,
            true => NRES::Num2,
        }
    }
    ///Only single result value will be returned. After a single read RRDY will be automatically cleared
    #[inline(always)]
    pub fn is_num1(&self) -> bool {
        *self == NRES::Num1
    }
    ///Two return reads need to be performed. After two reads RRDY will be automatically cleared
    #[inline(always)]
    pub fn is_num2(&self) -> bool {
        *self == NRES::Num2
    }
}
///Field `NRES` writer - NRES
pub type NRES_W<'a, REG> = crate::BitWriter<'a, REG, NRES>;
impl<'a, REG> NRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Only single result value will be returned. After a single read RRDY will be automatically cleared
    #[inline(always)]
    pub fn num1(self) -> &'a mut crate::W<REG> {
        self.variant(NRES::Num1)
    }
    ///Two return reads need to be performed. After two reads RRDY will be automatically cleared
    #[inline(always)]
    pub fn num2(self) -> &'a mut crate::W<REG> {
        self.variant(NRES::Num2)
    }
}
/**NARGS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NARGS {
    ///0: Only single argument write is needed for next calculation
    Num1 = 0,
    ///1: Two argument writes need to be performed for next calculation
    Num2 = 1,
}
impl From<NARGS> for bool {
    #[inline(always)]
    fn from(variant: NARGS) -> Self {
        variant as u8 != 0
    }
}
///Field `NARGS` reader - NARGS
pub type NARGS_R = crate::BitReader<NARGS>;
impl NARGS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NARGS {
        match self.bits {
            false => NARGS::Num1,
            true => NARGS::Num2,
        }
    }
    ///Only single argument write is needed for next calculation
    #[inline(always)]
    pub fn is_num1(&self) -> bool {
        *self == NARGS::Num1
    }
    ///Two argument writes need to be performed for next calculation
    #[inline(always)]
    pub fn is_num2(&self) -> bool {
        *self == NARGS::Num2
    }
}
///Field `NARGS` writer - NARGS
pub type NARGS_W<'a, REG> = crate::BitWriter<'a, REG, NARGS>;
impl<'a, REG> NARGS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Only single argument write is needed for next calculation
    #[inline(always)]
    pub fn num1(self) -> &'a mut crate::W<REG> {
        self.variant(NARGS::Num1)
    }
    ///Two argument writes need to be performed for next calculation
    #[inline(always)]
    pub fn num2(self) -> &'a mut crate::W<REG> {
        self.variant(NARGS::Num2)
    }
}
/**RESSIZE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESSIZE {
    ///0: Use 32 bit output values
    Bits32 = 0,
    ///1: Use 16 bit output values
    Bits16 = 1,
}
impl From<RESSIZE> for bool {
    #[inline(always)]
    fn from(variant: RESSIZE) -> Self {
        variant as u8 != 0
    }
}
///Field `RESSIZE` reader - RESSIZE
pub type RESSIZE_R = crate::BitReader<RESSIZE>;
impl RESSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RESSIZE {
        match self.bits {
            false => RESSIZE::Bits32,
            true => RESSIZE::Bits16,
        }
    }
    ///Use 32 bit output values
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == RESSIZE::Bits32
    }
    ///Use 16 bit output values
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == RESSIZE::Bits16
    }
}
///Field `RESSIZE` writer - RESSIZE
pub type RESSIZE_W<'a, REG> = crate::BitWriter<'a, REG, RESSIZE>;
impl<'a, REG> RESSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Use 32 bit output values
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(RESSIZE::Bits32)
    }
    ///Use 16 bit output values
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(RESSIZE::Bits16)
    }
}
/**ARGSIZE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARGSIZE {
    ///0: Use 32 bit input values
    Bits32 = 0,
    ///1: Use 16 bit input values
    Bits16 = 1,
}
impl From<ARGSIZE> for bool {
    #[inline(always)]
    fn from(variant: ARGSIZE) -> Self {
        variant as u8 != 0
    }
}
///Field `ARGSIZE` reader - ARGSIZE
pub type ARGSIZE_R = crate::BitReader<ARGSIZE>;
impl ARGSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARGSIZE {
        match self.bits {
            false => ARGSIZE::Bits32,
            true => ARGSIZE::Bits16,
        }
    }
    ///Use 32 bit input values
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == ARGSIZE::Bits32
    }
    ///Use 16 bit input values
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == ARGSIZE::Bits16
    }
}
///Field `ARGSIZE` writer - ARGSIZE
pub type ARGSIZE_W<'a, REG> = crate::BitWriter<'a, REG, ARGSIZE>;
impl<'a, REG> ARGSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Use 32 bit input values
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(ARGSIZE::Bits32)
    }
    ///Use 16 bit input values
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(ARGSIZE::Bits16)
    }
}
/**RRDY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRDYR {
    ///0: Results from computation are not read
    NotReady = 0,
    ///1: Results are ready, this flag will be automatically cleared once value is read
    Ready = 1,
}
impl From<RRDYR> for bool {
    #[inline(always)]
    fn from(variant: RRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `RRDY` reader - RRDY
pub type RRDY_R = crate::BitReader<RRDYR>;
impl RRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRDYR {
        match self.bits {
            false => RRDYR::NotReady,
            true => RRDYR::Ready,
        }
    }
    ///Results from computation are not read
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RRDYR::NotReady
    }
    ///Results are ready, this flag will be automatically cleared once value is read
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RRDYR::Ready
    }
}
///Field `RRDY` writer - RRDY
pub type RRDY_W<'a, REG> = crate::BitWriter<'a, REG, RRDYR>;
impl<'a, REG> RRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Results from computation are not read
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(RRDYR::NotReady)
    }
    ///Results are ready, this flag will be automatically cleared once value is read
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(RRDYR::Ready)
    }
}
impl R {
    ///Bits 0:3 - FUNC
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Precision (number of iterations/cycles) required
    #[inline(always)]
    pub fn precision(&self) -> PRECISION_R {
        PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - Scaling factor (2^-n for arguments, 2^n for results)
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 16 - IEN
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMAREN
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DMAWEN
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - NRES
    #[inline(always)]
    pub fn nres(&self) -> NRES_R {
        NRES_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - NARGS
    #[inline(always)]
    pub fn nargs(&self) -> NARGS_R {
        NARGS_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RESSIZE
    #[inline(always)]
    pub fn ressize(&self) -> RESSIZE_R {
        RESSIZE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ARGSIZE
    #[inline(always)]
    pub fn argsize(&self) -> ARGSIZE_R {
        ARGSIZE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 31 - RRDY
    #[inline(always)]
    pub fn rrdy(&self) -> RRDY_R {
        RRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("func", &self.func())
            .field("precision", &self.precision())
            .field("scale", &self.scale())
            .field("ien", &self.ien())
            .field("dmaren", &self.dmaren())
            .field("dmawen", &self.dmawen())
            .field("nres", &self.nres())
            .field("nargs", &self.nargs())
            .field("ressize", &self.ressize())
            .field("argsize", &self.argsize())
            .field("rrdy", &self.rrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - FUNC
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W<'_, CSRrs> {
        FUNC_W::new(self, 0)
    }
    ///Bits 4:7 - Precision (number of iterations/cycles) required
    #[inline(always)]
    pub fn precision(&mut self) -> PRECISION_W<'_, CSRrs> {
        PRECISION_W::new(self, 4)
    }
    ///Bits 8:10 - Scaling factor (2^-n for arguments, 2^n for results)
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<'_, CSRrs> {
        SCALE_W::new(self, 8)
    }
    ///Bit 16 - IEN
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W<'_, CSRrs> {
        IEN_W::new(self, 16)
    }
    ///Bit 17 - DMAREN
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W<'_, CSRrs> {
        DMAREN_W::new(self, 17)
    }
    ///Bit 18 - DMAWEN
    #[inline(always)]
    pub fn dmawen(&mut self) -> DMAWEN_W<'_, CSRrs> {
        DMAWEN_W::new(self, 18)
    }
    ///Bit 19 - NRES
    #[inline(always)]
    pub fn nres(&mut self) -> NRES_W<'_, CSRrs> {
        NRES_W::new(self, 19)
    }
    ///Bit 20 - NARGS
    #[inline(always)]
    pub fn nargs(&mut self) -> NARGS_W<'_, CSRrs> {
        NARGS_W::new(self, 20)
    }
    ///Bit 21 - RESSIZE
    #[inline(always)]
    pub fn ressize(&mut self) -> RESSIZE_W<'_, CSRrs> {
        RESSIZE_W::new(self, 21)
    }
    ///Bit 22 - ARGSIZE
    #[inline(always)]
    pub fn argsize(&mut self) -> ARGSIZE_W<'_, CSRrs> {
        ARGSIZE_W::new(self, 22)
    }
    ///Bit 31 - RRDY
    #[inline(always)]
    pub fn rrdy(&mut self) -> RRDY_W<'_, CSRrs> {
        RRDY_W::new(self, 31)
    }
}
/**CORDIC Control Status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#CORDIC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
