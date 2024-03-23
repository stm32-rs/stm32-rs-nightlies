#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "ADC oversampler enable on scope ADC group regular\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVSE {
    #[doc = "0: Oversampler disabled"]
    Disabled = 0,
    #[doc = "1: Oversampler enabled"]
    Enabled = 1,
}
impl From<OVSE> for bool {
    #[inline(always)]
    fn from(variant: OVSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVSE` reader - ADC oversampler enable on scope ADC group regular"]
pub type OVSE_R = crate::BitReader<OVSE>;
impl OVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVSE {
        match self.bits {
            false => OVSE::Disabled,
            true => OVSE::Enabled,
        }
    }
    #[doc = "Oversampler disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVSE::Disabled
    }
    #[doc = "Oversampler enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVSE::Enabled
    }
}
#[doc = "Field `OVSE` writer - ADC oversampler enable on scope ADC group regular"]
pub type OVSE_W<'a, REG> = crate::BitWriter<'a, REG, OVSE>;
impl<'a, REG> OVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oversampler disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVSE::Disabled)
    }
    #[doc = "Oversampler enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVSE::Enabled)
    }
}
#[doc = "ADC oversampling ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR {
    #[doc = "0: 2x"]
    Mul2 = 0,
    #[doc = "1: 4x"]
    Mul4 = 1,
    #[doc = "2: 8x"]
    Mul8 = 2,
    #[doc = "3: 16x"]
    Mul16 = 3,
    #[doc = "4: 32x"]
    Mul32 = 4,
    #[doc = "5: 64x"]
    Mul64 = 5,
    #[doc = "6: 128x"]
    Mul128 = 6,
    #[doc = "7: 256x"]
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
#[doc = "Field `OVSR` reader - ADC oversampling ratio"]
pub type OVSR_R = crate::FieldReader<OVSR>;
impl OVSR_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "2x"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == OVSR::Mul2
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == OVSR::Mul4
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == OVSR::Mul8
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == OVSR::Mul16
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn is_mul32(&self) -> bool {
        *self == OVSR::Mul32
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn is_mul64(&self) -> bool {
        *self == OVSR::Mul64
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn is_mul128(&self) -> bool {
        *self == OVSR::Mul128
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn is_mul256(&self) -> bool {
        *self == OVSR::Mul256
    }
}
#[doc = "Field `OVSR` writer - ADC oversampling ratio"]
pub type OVSR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OVSR>;
impl<'a, REG> OVSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2x"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul2)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul4)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul8)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn mul16(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul16)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn mul32(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul32)
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn mul64(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul64)
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn mul128(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul128)
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn mul256(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Mul256)
    }
}
#[doc = "ADC oversampling shift\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS {
    #[doc = "0: No shift"]
    NoShift = 0,
    #[doc = "1: Shift 1-bit"]
    Shift1 = 1,
    #[doc = "2: Shift 2-bits"]
    Shift2 = 2,
    #[doc = "3: Shift 3-bits"]
    Shift3 = 3,
    #[doc = "4: Shift 4-bits"]
    Shift4 = 4,
    #[doc = "5: Shift 5-bits"]
    Shift5 = 5,
    #[doc = "6: Shift 6-bits"]
    Shift6 = 6,
    #[doc = "7: Shift 7-bits"]
    Shift7 = 7,
    #[doc = "8: Shift 8-bits"]
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
#[doc = "Field `OVSS` reader - ADC oversampling shift"]
pub type OVSS_R = crate::FieldReader<OVSS>;
impl OVSS_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No shift"]
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == OVSS::NoShift
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn is_shift1(&self) -> bool {
        *self == OVSS::Shift1
    }
    #[doc = "Shift 2-bits"]
    #[inline(always)]
    pub fn is_shift2(&self) -> bool {
        *self == OVSS::Shift2
    }
    #[doc = "Shift 3-bits"]
    #[inline(always)]
    pub fn is_shift3(&self) -> bool {
        *self == OVSS::Shift3
    }
    #[doc = "Shift 4-bits"]
    #[inline(always)]
    pub fn is_shift4(&self) -> bool {
        *self == OVSS::Shift4
    }
    #[doc = "Shift 5-bits"]
    #[inline(always)]
    pub fn is_shift5(&self) -> bool {
        *self == OVSS::Shift5
    }
    #[doc = "Shift 6-bits"]
    #[inline(always)]
    pub fn is_shift6(&self) -> bool {
        *self == OVSS::Shift6
    }
    #[doc = "Shift 7-bits"]
    #[inline(always)]
    pub fn is_shift7(&self) -> bool {
        *self == OVSS::Shift7
    }
    #[doc = "Shift 8-bits"]
    #[inline(always)]
    pub fn is_shift8(&self) -> bool {
        *self == OVSS::Shift8
    }
}
#[doc = "Field `OVSS` writer - ADC oversampling shift"]
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSS>;
impl<'a, REG> OVSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No shift"]
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::NoShift)
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn shift1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift1)
    }
    #[doc = "Shift 2-bits"]
    #[inline(always)]
    pub fn shift2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift2)
    }
    #[doc = "Shift 3-bits"]
    #[inline(always)]
    pub fn shift3(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift3)
    }
    #[doc = "Shift 4-bits"]
    #[inline(always)]
    pub fn shift4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift4)
    }
    #[doc = "Shift 5-bits"]
    #[inline(always)]
    pub fn shift5(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift5)
    }
    #[doc = "Shift 6-bits"]
    #[inline(always)]
    pub fn shift6(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift6)
    }
    #[doc = "Shift 7-bits"]
    #[inline(always)]
    pub fn shift7(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift7)
    }
    #[doc = "Shift 8-bits"]
    #[inline(always)]
    pub fn shift8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift8)
    }
}
#[doc = "ADC oversampling discontinuous mode (triggered mode) for ADC group regular\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOVS {
    #[doc = "0: All oversampled conversions for a channel are done consecutively after a trigger"]
    TriggerAll = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a trigger"]
    TriggerEach = 1,
}
impl From<TOVS> for bool {
    #[inline(always)]
    fn from(variant: TOVS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOVS` reader - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
pub type TOVS_R = crate::BitReader<TOVS>;
impl TOVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOVS {
        match self.bits {
            false => TOVS::TriggerAll,
            true => TOVS::TriggerEach,
        }
    }
    #[doc = "All oversampled conversions for a channel are done consecutively after a trigger"]
    #[inline(always)]
    pub fn is_trigger_all(&self) -> bool {
        *self == TOVS::TriggerAll
    }
    #[doc = "Each oversampled conversion for a channel needs a trigger"]
    #[inline(always)]
    pub fn is_trigger_each(&self) -> bool {
        *self == TOVS::TriggerEach
    }
}
#[doc = "Field `TOVS` writer - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG, TOVS>;
impl<'a, REG> TOVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All oversampled conversions for a channel are done consecutively after a trigger"]
    #[inline(always)]
    pub fn trigger_all(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS::TriggerAll)
    }
    #[doc = "Each oversampled conversion for a channel needs a trigger"]
    #[inline(always)]
    pub fn trigger_each(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS::TriggerEach)
    }
}
#[doc = "Low frequency trigger mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFTRIG {
    #[doc = "0: Low Frequency Trigger Mode disabled"]
    Disabled = 0,
    #[doc = "1: Low Frequency Trigger Mode enabled"]
    Enabled = 1,
}
impl From<LFTRIG> for bool {
    #[inline(always)]
    fn from(variant: LFTRIG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFTRIG` reader - Low frequency trigger mode enable"]
pub type LFTRIG_R = crate::BitReader<LFTRIG>;
impl LFTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFTRIG {
        match self.bits {
            false => LFTRIG::Disabled,
            true => LFTRIG::Enabled,
        }
    }
    #[doc = "Low Frequency Trigger Mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFTRIG::Disabled
    }
    #[doc = "Low Frequency Trigger Mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFTRIG::Enabled
    }
}
#[doc = "Field `LFTRIG` writer - Low frequency trigger mode enable"]
pub type LFTRIG_W<'a, REG> = crate::BitWriter<'a, REG, LFTRIG>;
impl<'a, REG> LFTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low Frequency Trigger Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFTRIG::Disabled)
    }
    #[doc = "Low Frequency Trigger Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFTRIG::Enabled)
    }
}
#[doc = "ADC clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE {
    #[doc = "0: ADCCLK (Asynchronous clock mode)"]
    Adclk = 0,
    #[doc = "1: PCLK/2 (Synchronous clock mode)"]
    PclkDiv2 = 1,
    #[doc = "2: PCLK/4 (Synchronous clock mode)"]
    PclkDiv4 = 2,
    #[doc = "3: PCLK (Synchronous clock mode)"]
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
#[doc = "Field `CKMODE` reader - ADC clock mode"]
pub type CKMODE_R = crate::FieldReader<CKMODE>;
impl CKMODE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "ADCCLK (Asynchronous clock mode)"]
    #[inline(always)]
    pub fn is_adclk(&self) -> bool {
        *self == CKMODE::Adclk
    }
    #[doc = "PCLK/2 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn is_pclk_div2(&self) -> bool {
        *self == CKMODE::PclkDiv2
    }
    #[doc = "PCLK/4 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn is_pclk_div4(&self) -> bool {
        *self == CKMODE::PclkDiv4
    }
    #[doc = "PCLK (Synchronous clock mode)"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == CKMODE::Pclk
    }
}
#[doc = "Field `CKMODE` writer - ADC clock mode"]
pub type CKMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CKMODE>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADCCLK (Asynchronous clock mode)"]
    #[inline(always)]
    pub fn adclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Adclk)
    }
    #[doc = "PCLK/2 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn pclk_div2(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::PclkDiv2)
    }
    #[doc = "PCLK/4 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn pclk_div4(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::PclkDiv4)
    }
    #[doc = "PCLK (Synchronous clock mode)"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Pclk)
    }
}
impl R {
    #[doc = "Bit 0 - ADC oversampler enable on scope ADC group regular"]
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - ADC oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - ADC oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - Low frequency trigger mode enable"]
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC oversampler enable on scope ADC group regular"]
    #[inline(always)]
    #[must_use]
    pub fn ovse(&mut self) -> OVSE_W<CFGR2rs> {
        OVSE_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - ADC oversampling ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    #[doc = "Bits 5:8 - ADC oversampling shift"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    #[doc = "Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<CFGR2rs> {
        TOVS_W::new(self, 9)
    }
    #[doc = "Bit 29 - Low frequency trigger mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lftrig(&mut self) -> LFTRIG_W<CFGR2rs> {
        LFTRIG_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - ADC clock mode"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<CFGR2rs> {
        CKMODE_W::new(self, 30)
    }
}
#[doc = "ADC configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
