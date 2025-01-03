///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVSE {
    ///0: Oversampler disabled
    Disabled = 0,
    ///1: Oversampler enabled
    Enabled = 1,
}
impl From<OVSE> for bool {
    #[inline(always)]
    fn from(variant: OVSE) -> Self {
        variant as u8 != 0
    }
}
///Field `OVSE` reader - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type OVSE_R = crate::BitReader<OVSE>;
impl OVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVSE {
        match self.bits {
            false => OVSE::Disabled,
            true => OVSE::Enabled,
        }
    }
    ///Oversampler disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVSE::Disabled
    }
    ///Oversampler enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVSE::Enabled
    }
}
///Field `OVSE` writer - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type OVSE_W<'a, REG> = crate::BitWriter<'a, REG, OVSE>;
impl<'a, REG> OVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Oversampler disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVSE::Disabled)
    }
    ///Oversampler enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVSE::Enabled)
    }
}
/**Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR {
    ///0: 2x
    Mul2 = 0,
    ///1: 4x
    Mul4 = 1,
    ///2: 8x
    Mul8 = 2,
    ///3: 16x
    Mul16 = 3,
    ///4: 32x
    Mul32 = 4,
    ///5: 64x
    Mul64 = 5,
    ///6: 128x
    Mul128 = 6,
    ///7: 256x
    Mul256 = 7,
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
///Field `OVSR` reader - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type OVSR_R = crate::FieldReader<OVSR>;
impl OVSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVSR {
        match self.bits {
            0 => OVSR::Mul2,
            1 => OVSR::Mul4,
            2 => OVSR::Mul8,
            3 => OVSR::Mul16,
            4 => OVSR::Mul32,
            5 => OVSR::Mul64,
            6 => OVSR::Mul128,
            7 => OVSR::Mul256,
            _ => unreachable!(),
        }
    }
    ///2x
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == OVSR::Mul2
    }
    ///4x
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == OVSR::Mul4
    }
    ///8x
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == OVSR::Mul8
    }
    ///16x
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == OVSR::Mul16
    }
    ///32x
    #[inline(always)]
    pub fn is_mul32(&self) -> bool {
        *self == OVSR::Mul32
    }
    ///64x
    #[inline(always)]
    pub fn is_mul64(&self) -> bool {
        *self == OVSR::Mul64
    }
    ///128x
    #[inline(always)]
    pub fn is_mul128(&self) -> bool {
        *self == OVSR::Mul128
    }
    ///256x
    #[inline(always)]
    pub fn is_mul256(&self) -> bool {
        *self == OVSR::Mul256
    }
}
///Field `OVSR` writer - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OVSR, crate::Safe>;
impl<'a, REG> OVSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2x
    #[inline(always)]
    pub fn mul2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul2)
    }
    ///4x
    #[inline(always)]
    pub fn mul4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul4)
    }
    ///8x
    #[inline(always)]
    pub fn mul8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul8)
    }
    ///16x
    #[inline(always)]
    pub fn mul16(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul16)
    }
    ///32x
    #[inline(always)]
    pub fn mul32(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul32)
    }
    ///64x
    #[inline(always)]
    pub fn mul64(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul64)
    }
    ///128x
    #[inline(always)]
    pub fn mul128(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul128)
    }
    ///256x
    #[inline(always)]
    pub fn mul256(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul256)
    }
}
/**Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS {
    ///0: No shift
    NoShift = 0,
    ///1: Shift 1-bit
    Shift1 = 1,
    ///2: Shift 2-bits
    Shift2 = 2,
    ///3: Shift 3-bits
    Shift3 = 3,
    ///4: Shift 4-bits
    Shift4 = 4,
    ///5: Shift 5-bits
    Shift5 = 5,
    ///6: Shift 6-bits
    Shift6 = 6,
    ///7: Shift 7-bits
    Shift7 = 7,
    ///8: Shift 8-bits
    Shift8 = 8,
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
///Field `OVSS` reader - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type OVSS_R = crate::FieldReader<OVSS>;
impl OVSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OVSS> {
        match self.bits {
            0 => Some(OVSS::NoShift),
            1 => Some(OVSS::Shift1),
            2 => Some(OVSS::Shift2),
            3 => Some(OVSS::Shift3),
            4 => Some(OVSS::Shift4),
            5 => Some(OVSS::Shift5),
            6 => Some(OVSS::Shift6),
            7 => Some(OVSS::Shift7),
            8 => Some(OVSS::Shift8),
            _ => None,
        }
    }
    ///No shift
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == OVSS::NoShift
    }
    ///Shift 1-bit
    #[inline(always)]
    pub fn is_shift1(&self) -> bool {
        *self == OVSS::Shift1
    }
    ///Shift 2-bits
    #[inline(always)]
    pub fn is_shift2(&self) -> bool {
        *self == OVSS::Shift2
    }
    ///Shift 3-bits
    #[inline(always)]
    pub fn is_shift3(&self) -> bool {
        *self == OVSS::Shift3
    }
    ///Shift 4-bits
    #[inline(always)]
    pub fn is_shift4(&self) -> bool {
        *self == OVSS::Shift4
    }
    ///Shift 5-bits
    #[inline(always)]
    pub fn is_shift5(&self) -> bool {
        *self == OVSS::Shift5
    }
    ///Shift 6-bits
    #[inline(always)]
    pub fn is_shift6(&self) -> bool {
        *self == OVSS::Shift6
    }
    ///Shift 7-bits
    #[inline(always)]
    pub fn is_shift7(&self) -> bool {
        *self == OVSS::Shift7
    }
    ///Shift 8-bits
    #[inline(always)]
    pub fn is_shift8(&self) -> bool {
        *self == OVSS::Shift8
    }
}
///Field `OVSS` writer - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSS>;
impl<'a, REG> OVSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No shift
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::NoShift)
    }
    ///Shift 1-bit
    #[inline(always)]
    pub fn shift1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift1)
    }
    ///Shift 2-bits
    #[inline(always)]
    pub fn shift2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift2)
    }
    ///Shift 3-bits
    #[inline(always)]
    pub fn shift3(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift3)
    }
    ///Shift 4-bits
    #[inline(always)]
    pub fn shift4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift4)
    }
    ///Shift 5-bits
    #[inline(always)]
    pub fn shift5(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift5)
    }
    ///Shift 6-bits
    #[inline(always)]
    pub fn shift6(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift6)
    }
    ///Shift 7-bits
    #[inline(always)]
    pub fn shift7(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift7)
    }
    ///Shift 8-bits
    #[inline(always)]
    pub fn shift8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift8)
    }
}
/**Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOVS {
    ///0: All oversampled conversions for a channel are done consecutively after a trigger
    TriggerAll = 0,
    ///1: Each oversampled conversion for a channel needs a trigger
    TriggerEach = 1,
}
impl From<TOVS> for bool {
    #[inline(always)]
    fn from(variant: TOVS) -> Self {
        variant as u8 != 0
    }
}
///Field `TOVS` reader - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type TOVS_R = crate::BitReader<TOVS>;
impl TOVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOVS {
        match self.bits {
            false => TOVS::TriggerAll,
            true => TOVS::TriggerEach,
        }
    }
    ///All oversampled conversions for a channel are done consecutively after a trigger
    #[inline(always)]
    pub fn is_trigger_all(&self) -> bool {
        *self == TOVS::TriggerAll
    }
    ///Each oversampled conversion for a channel needs a trigger
    #[inline(always)]
    pub fn is_trigger_each(&self) -> bool {
        *self == TOVS::TriggerEach
    }
}
///Field `TOVS` writer - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG, TOVS>;
impl<'a, REG> TOVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All oversampled conversions for a channel are done consecutively after a trigger
    #[inline(always)]
    pub fn trigger_all(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS::TriggerAll)
    }
    ///Each oversampled conversion for a channel needs a trigger
    #[inline(always)]
    pub fn trigger_each(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS::TriggerEach)
    }
}
/**Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFTRIG {
    ///0: Low Frequency Trigger Mode disabled
    Disabled = 0,
    ///1: Low Frequency Trigger Mode enabled
    Enabled = 1,
}
impl From<LFTRIG> for bool {
    #[inline(always)]
    fn from(variant: LFTRIG) -> Self {
        variant as u8 != 0
    }
}
///Field `LFTRIG` reader - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).
pub type LFTRIG_R = crate::BitReader<LFTRIG>;
impl LFTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LFTRIG {
        match self.bits {
            false => LFTRIG::Disabled,
            true => LFTRIG::Enabled,
        }
    }
    ///Low Frequency Trigger Mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFTRIG::Disabled
    }
    ///Low Frequency Trigger Mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFTRIG::Enabled
    }
}
///Field `LFTRIG` writer - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).
pub type LFTRIG_W<'a, REG> = crate::BitWriter<'a, REG, LFTRIG>;
impl<'a, REG> LFTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Low Frequency Trigger Mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFTRIG::Disabled)
    }
    ///Low Frequency Trigger Mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFTRIG::Enabled)
    }
}
/**ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE {
    ///0: ADCCLK (Asynchronous clock mode)
    Adclk = 0,
    ///1: PCLK/2 (Synchronous clock mode)
    PclkDiv2 = 1,
    ///2: PCLK/4 (Synchronous clock mode)
    PclkDiv4 = 2,
    ///3: PCLK (Synchronous clock mode)
    Pclk = 3,
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
            0 => CKMODE::Adclk,
            1 => CKMODE::PclkDiv2,
            2 => CKMODE::PclkDiv4,
            3 => CKMODE::Pclk,
            _ => unreachable!(),
        }
    }
    ///ADCCLK (Asynchronous clock mode)
    #[inline(always)]
    pub fn is_adclk(&self) -> bool {
        *self == CKMODE::Adclk
    }
    ///PCLK/2 (Synchronous clock mode)
    #[inline(always)]
    pub fn is_pclk_div2(&self) -> bool {
        *self == CKMODE::PclkDiv2
    }
    ///PCLK/4 (Synchronous clock mode)
    #[inline(always)]
    pub fn is_pclk_div4(&self) -> bool {
        *self == CKMODE::PclkDiv4
    }
    ///PCLK (Synchronous clock mode)
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == CKMODE::Pclk
    }
}
///Field `CKMODE` writer - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKMODE, crate::Safe>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ADCCLK (Asynchronous clock mode)
    #[inline(always)]
    pub fn adclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Adclk)
    }
    ///PCLK/2 (Synchronous clock mode)
    #[inline(always)]
    pub fn pclk_div2(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::PclkDiv2)
    }
    ///PCLK/4 (Synchronous clock mode)
    #[inline(always)]
    pub fn pclk_div4(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::PclkDiv4)
    }
    ///PCLK (Synchronous clock mode)
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Pclk)
    }
}
impl R {
    ///Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).
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
        f.debug_struct("CFGR2")
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
    ///Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovse(&mut self) -> OVSE_W<CFGR2rs> {
        OVSE_W::new(self, 0)
    }
    ///Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W<CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    ///Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W<CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W<CFGR2rs> {
        TOVS_W::new(self, 9)
    }
    ///Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn lftrig(&mut self) -> LFTRIG_W<CFGR2rs> {
        LFTRIG_W::new(self, 29)
    }
    ///Bits 30:31 - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<CFGR2rs> {
        CKMODE_W::new(self, 30)
    }
}
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#ADC:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
