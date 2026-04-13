///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**Oversampler Enable

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
///Field `OVSE` reader - Oversampler Enable
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
///Field `OVSE` writer - Oversampler Enable
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
/**Oversampling ratio

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
///Field `OVSR` reader - Oversampling ratio
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
///Field `OVSR` writer - Oversampling ratio
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
///Field `OVSS` reader - Oversampling shift
pub type OVSS_R = crate::FieldReader;
///Field `OVSS` writer - Oversampling shift
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Triggered Oversampling

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
///Field `TOVS` reader - Triggered Oversampling
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
///Field `TOVS` writer - Triggered Oversampling
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
/**ADC clock mode

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
///Field `CKMODE` reader - ADC clock mode
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
///Field `CKMODE` writer - ADC clock mode
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
    ///Bit 0 - Oversampler Enable
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:4 - Oversampling ratio
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - Triggered Oversampling
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 30:31 - ADC clock mode
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
            .field("ckmode", &self.ckmode())
            .finish()
    }
}
impl W {
    ///Bit 0 - Oversampler Enable
    #[inline(always)]
    pub fn ovse(&mut self) -> OVSE_W<'_, CFGR2rs> {
        OVSE_W::new(self, 0)
    }
    ///Bits 2:4 - Oversampling ratio
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W<'_, CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W<'_, CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - Triggered Oversampling
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W<'_, CFGR2rs> {
        TOVS_W::new(self, 9)
    }
    ///Bits 30:31 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<'_, CFGR2rs> {
        CKMODE_W::new(self, 30)
    }
}
/**configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x2.html#ADC:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
