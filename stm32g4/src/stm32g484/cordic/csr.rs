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
    ///0: Cosine funciton
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
    ///Cosine funciton
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
    ///Cosine funciton
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
///Field `PRECISION` reader - Precision (number of iterations/cycles) required
pub type PRECISION_R = crate::FieldReader;
///Field `PRECISION` writer - Precision (number of iterations/cycles) required
pub type PRECISION_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    ///1: Enable intterrupt request generation
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
    ///Enable intterrupt request generation
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
    ///Enable intterrupt request generation
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
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<CSRrs> {
        FUNC_W::new(self, 0)
    }
    ///Bits 4:7 - Precision (number of iterations/cycles) required
    #[inline(always)]
    #[must_use]
    pub fn precision(&mut self) -> PRECISION_W<CSRrs> {
        PRECISION_W::new(self, 4)
    }
    ///Bits 8:10 - Scaling factor (2^-n for arguments, 2^n for results)
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<CSRrs> {
        SCALE_W::new(self, 8)
    }
    ///Bit 16 - IEN
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<CSRrs> {
        IEN_W::new(self, 16)
    }
    ///Bit 17 - DMAREN
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CSRrs> {
        DMAREN_W::new(self, 17)
    }
    ///Bit 18 - DMAWEN
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DMAWEN_W<CSRrs> {
        DMAWEN_W::new(self, 18)
    }
    ///Bit 19 - NRES
    #[inline(always)]
    #[must_use]
    pub fn nres(&mut self) -> NRES_W<CSRrs> {
        NRES_W::new(self, 19)
    }
    ///Bit 20 - NARGS
    #[inline(always)]
    #[must_use]
    pub fn nargs(&mut self) -> NARGS_W<CSRrs> {
        NARGS_W::new(self, 20)
    }
    ///Bit 21 - RESSIZE
    #[inline(always)]
    #[must_use]
    pub fn ressize(&mut self) -> RESSIZE_W<CSRrs> {
        RESSIZE_W::new(self, 21)
    }
    ///Bit 22 - ARGSIZE
    #[inline(always)]
    #[must_use]
    pub fn argsize(&mut self) -> ARGSIZE_W<CSRrs> {
        ARGSIZE_W::new(self, 22)
    }
    ///Bit 31 - RRDY
    #[inline(always)]
    #[must_use]
    pub fn rrdy(&mut self) -> RRDY_W<CSRrs> {
        RRDY_W::new(self, 31)
    }
}
/**CORDIC Control Status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484xx.html#CORDIC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
