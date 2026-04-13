///Register `ADC_CFGR2` reader
pub type R = crate::R<ADC_CFGR2rs>;
///Register `ADC_CFGR2` writer
pub type W = crate::W<ADC_CFGR2rs>;
/**Oversampler Enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVSE {
    ///0: Oversampler disabled
    B0x0 = 0,
    ///1: Oversampler enabled
    B0x1 = 1,
}
impl From<OVSE> for bool {
    #[inline(always)]
    fn from(variant: OVSE) -> Self {
        variant as u8 != 0
    }
}
///Field `OVSE` reader - Oversampler Enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
pub type OVSE_R = crate::BitReader<OVSE>;
impl OVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVSE {
        match self.bits {
            false => OVSE::B0x0,
            true => OVSE::B0x1,
        }
    }
    ///Oversampler disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVSE::B0x0
    }
    ///Oversampler enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVSE::B0x1
    }
}
///Field `OVSE` writer - Oversampler Enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
pub type OVSE_W<'a, REG> = crate::BitWriter<'a, REG, OVSE>;
impl<'a, REG> OVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Oversampler disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVSE::B0x0)
    }
    ///Oversampler enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSE::B0x1)
    }
}
/**Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADEN bit is cleared.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR {
    ///0: 2x
    B0x0 = 0,
    ///1: 4x
    B0x1 = 1,
    ///2: 8x
    B0x2 = 2,
    ///3: 16x
    B0x3 = 3,
    ///4: 32x
    B0x4 = 4,
    ///5: 64x
    B0x5 = 5,
    ///6: 128x
    B0x6 = 6,
    ///7: 256x
    B0x7 = 7,
}
impl From<OVSR> for u8 {
    #[inline(always)]
    fn from(variant: OVSR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSR {
    type Ux = u8;
}
impl crate::IsEnum for OVSR {}
///Field `OVSR` reader - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADEN bit is cleared.
pub type OVSR_R = crate::FieldReader<OVSR>;
impl OVSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVSR {
        match self.bits {
            0 => OVSR::B0x0,
            1 => OVSR::B0x1,
            2 => OVSR::B0x2,
            3 => OVSR::B0x3,
            4 => OVSR::B0x4,
            5 => OVSR::B0x5,
            6 => OVSR::B0x6,
            7 => OVSR::B0x7,
            _ => unreachable!(),
        }
    }
    ///2x
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVSR::B0x0
    }
    ///4x
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVSR::B0x1
    }
    ///8x
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OVSR::B0x2
    }
    ///16x
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OVSR::B0x3
    }
    ///32x
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == OVSR::B0x4
    }
    ///64x
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == OVSR::B0x5
    }
    ///128x
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == OVSR::B0x6
    }
    ///256x
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == OVSR::B0x7
    }
}
///Field `OVSR` writer - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADEN bit is cleared.
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OVSR, crate::Safe>;
impl<'a, REG> OVSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2x
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::B0x0)
    }
    ///4x
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::B0x1)
    }
    ///8x
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::B0x2)
    }
    ///16x
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::B0x3)
    }
    ///32x
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::B0x4)
    }
    ///64x
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::B0x5)
    }
    ///128x
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::B0x6)
    }
    ///256x
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::B0x7)
    }
}
/**Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADEN bit is cleared.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS {
    ///0: No shift
    B0x0 = 0,
    ///1: Shift 1-bit
    B0x1 = 1,
    ///2: Shift 2-bits
    B0x2 = 2,
    ///3: Shift 3-bits
    B0x3 = 3,
    ///4: Shift 4-bits
    B0x4 = 4,
    ///5: Shift 5-bits
    B0x5 = 5,
    ///6: Shift 6-bits
    B0x6 = 6,
    ///7: Shift 7-bits
    B0x7 = 7,
    ///8: Shift 8-bits
    B0x8 = 8,
}
impl From<OVSS> for u8 {
    #[inline(always)]
    fn from(variant: OVSS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSS {
    type Ux = u8;
}
impl crate::IsEnum for OVSS {}
///Field `OVSS` reader - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADEN bit is cleared.
pub type OVSS_R = crate::FieldReader<OVSS>;
impl OVSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OVSS> {
        match self.bits {
            0 => Some(OVSS::B0x0),
            1 => Some(OVSS::B0x1),
            2 => Some(OVSS::B0x2),
            3 => Some(OVSS::B0x3),
            4 => Some(OVSS::B0x4),
            5 => Some(OVSS::B0x5),
            6 => Some(OVSS::B0x6),
            7 => Some(OVSS::B0x7),
            8 => Some(OVSS::B0x8),
            _ => None,
        }
    }
    ///No shift
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVSS::B0x0
    }
    ///Shift 1-bit
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVSS::B0x1
    }
    ///Shift 2-bits
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OVSS::B0x2
    }
    ///Shift 3-bits
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OVSS::B0x3
    }
    ///Shift 4-bits
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == OVSS::B0x4
    }
    ///Shift 5-bits
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == OVSS::B0x5
    }
    ///Shift 6-bits
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == OVSS::B0x6
    }
    ///Shift 7-bits
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == OVSS::B0x7
    }
    ///Shift 8-bits
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == OVSS::B0x8
    }
}
///Field `OVSS` writer - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADEN bit is cleared.
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSS>;
impl<'a, REG> OVSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No shift
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::B0x0)
    }
    ///Shift 1-bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::B0x1)
    }
    ///Shift 2-bits
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::B0x2)
    }
    ///Shift 3-bits
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::B0x3)
    }
    ///Shift 4-bits
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::B0x4)
    }
    ///Shift 5-bits
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::B0x5)
    }
    ///Shift 6-bits
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::B0x6)
    }
    ///Shift 7-bits
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::B0x7)
    }
    ///Shift 8-bits
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::B0x8)
    }
}
/**Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOVS {
    ///0: All oversampled conversions for a channel are done consecutively after a trigger
    B0x0 = 0,
    ///1: Each oversampled conversion for a channel needs a trigger
    B0x1 = 1,
}
impl From<TOVS> for bool {
    #[inline(always)]
    fn from(variant: TOVS) -> Self {
        variant as u8 != 0
    }
}
///Field `TOVS` reader - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
pub type TOVS_R = crate::BitReader<TOVS>;
impl TOVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOVS {
        match self.bits {
            false => TOVS::B0x0,
            true => TOVS::B0x1,
        }
    }
    ///All oversampled conversions for a channel are done consecutively after a trigger
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TOVS::B0x0
    }
    ///Each oversampled conversion for a channel needs a trigger
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TOVS::B0x1
    }
}
///Field `TOVS` writer - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG, TOVS>;
impl<'a, REG> TOVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All oversampled conversions for a channel are done consecutively after a trigger
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS::B0x0)
    }
    ///Each oversampled conversion for a channel needs a trigger
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS::B0x1)
    }
}
/**Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFTRIG {
    ///0: Low Frequency Trigger Mode disabled
    B0x0 = 0,
    ///1: Low Frequency Trigger Mode enabled
    B0x1 = 1,
}
impl From<LFTRIG> for bool {
    #[inline(always)]
    fn from(variant: LFTRIG) -> Self {
        variant as u8 != 0
    }
}
///Field `LFTRIG` reader - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
pub type LFTRIG_R = crate::BitReader<LFTRIG>;
impl LFTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LFTRIG {
        match self.bits {
            false => LFTRIG::B0x0,
            true => LFTRIG::B0x1,
        }
    }
    ///Low Frequency Trigger Mode disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LFTRIG::B0x0
    }
    ///Low Frequency Trigger Mode enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LFTRIG::B0x1
    }
}
///Field `LFTRIG` writer - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
pub type LFTRIG_W<'a, REG> = crate::BitWriter<'a, REG, LFTRIG>;
impl<'a, REG> LFTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low Frequency Trigger Mode disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LFTRIG::B0x0)
    }
    ///Low Frequency Trigger Mode enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LFTRIG::B0x1)
    }
}
/**ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE {
    ///0: ADCCLK (Asynchronous clock mode), generated at product level (refer to RCC section)
    B0x0 = 0,
    ///1: PCLK/2 (Synchronous clock mode)
    B0x1 = 1,
    ///2: PCLK/4 (Synchronous clock mode)
    B0x2 = 2,
    ///3: PCLK (Synchronous clock mode). This configuration must be enabled only if PCLK has a 50% duty clock cycle (APB prescaler configured inside the RCC must be bypassed and the system clock must by 50% duty cycle)
    B0x3 = 3,
}
impl From<CKMODE> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKMODE {
    type Ux = u8;
}
impl crate::IsEnum for CKMODE {}
///Field `CKMODE` reader - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type CKMODE_R = crate::FieldReader<CKMODE>;
impl CKMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKMODE {
        match self.bits {
            0 => CKMODE::B0x0,
            1 => CKMODE::B0x1,
            2 => CKMODE::B0x2,
            3 => CKMODE::B0x3,
            _ => unreachable!(),
        }
    }
    ///ADCCLK (Asynchronous clock mode), generated at product level (refer to RCC section)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CKMODE::B0x0
    }
    ///PCLK/2 (Synchronous clock mode)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CKMODE::B0x1
    }
    ///PCLK/4 (Synchronous clock mode)
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CKMODE::B0x2
    }
    ///PCLK (Synchronous clock mode). This configuration must be enabled only if PCLK has a 50% duty clock cycle (APB prescaler configured inside the RCC must be bypassed and the system clock must by 50% duty cycle)
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CKMODE::B0x3
    }
}
///Field `CKMODE` writer - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKMODE, crate::Safe>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ADCCLK (Asynchronous clock mode), generated at product level (refer to RCC section)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::B0x0)
    }
    ///PCLK/2 (Synchronous clock mode)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::B0x1)
    }
    ///PCLK/4 (Synchronous clock mode)
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::B0x2)
    }
    ///PCLK (Synchronous clock mode). This configuration must be enabled only if PCLK has a 50% duty clock cycle (APB prescaler configured inside the RCC must be bypassed and the system clock must by 50% duty cycle)
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::B0x3)
    }
}
impl R {
    ///Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADEN bit is cleared.
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADEN bit is cleared.
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CFGR2")
            .field("ovse", &self.ovse())
            .field("ovsr", &self.ovsr())
            .field("ovss", &self.ovss())
            .field("tovs", &self.tovs())
            .field("lftrig", &self.lftrig())
            .field("ckmode", &self.ckmode())
            .finish()
    }
}
impl W {
    ///Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
    #[inline(always)]
    pub fn ovse(&mut self) -> OVSE_W<'_, ADC_CFGR2rs> {
        OVSE_W::new(self, 0)
    }
    ///Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADEN bit is cleared.
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W<'_, ADC_CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    ///Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADEN bit is cleared.
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W<'_, ADC_CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W<'_, ADC_CFGR2rs> {
        TOVS_W::new(self, 9)
    }
    ///Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADEN bit is cleared.
    #[inline(always)]
    pub fn lftrig(&mut self) -> LFTRIG_W<'_, ADC_CFGR2rs> {
        LFTRIG_W::new(self, 29)
    }
    ///Bits 30:31 - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<'_, ADC_CFGR2rs> {
        CKMODE_W::new(self, 30)
    }
}
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`adc_cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#ADC:ADC_CFGR2)*/
pub struct ADC_CFGR2rs;
impl crate::RegisterSpec for ADC_CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`adc_cfgr2::R`](R) reader structure
impl crate::Readable for ADC_CFGR2rs {}
///`write(|w| ..)` method takes [`adc_cfgr2::W`](W) writer structure
impl crate::Writable for ADC_CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_CFGR2 to value 0
impl crate::Resettable for ADC_CFGR2rs {}
