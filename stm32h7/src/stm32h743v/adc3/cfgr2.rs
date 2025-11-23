///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**ADC oversampler enable on scope ADC group regular

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE {
    ///0: Regular oversampling disabled
    Disabled = 0,
    ///1: Regular oversampling enabled
    Enabled = 1,
}
impl From<ROVSE> for bool {
    #[inline(always)]
    fn from(variant: ROVSE) -> Self {
        variant as u8 != 0
    }
}
///Field `ROVSE` reader - ADC oversampler enable on scope ADC group regular
pub type ROVSE_R = crate::BitReader<ROVSE>;
impl ROVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROVSE {
        match self.bits {
            false => ROVSE::Disabled,
            true => ROVSE::Enabled,
        }
    }
    ///Regular oversampling disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVSE::Disabled
    }
    ///Regular oversampling enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVSE::Enabled
    }
}
///Field `ROVSE` writer - ADC oversampler enable on scope ADC group regular
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG, ROVSE>;
impl<'a, REG> ROVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular oversampling disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Disabled)
    }
    ///Regular oversampling enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Enabled)
    }
}
/**ADC oversampler enable on scope ADC group injected

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE {
    ///0: Injected oversampling disabled
    Disabled = 0,
    ///1: Injected oversampling enabled
    Enabled = 1,
}
impl From<JOVSE> for bool {
    #[inline(always)]
    fn from(variant: JOVSE) -> Self {
        variant as u8 != 0
    }
}
///Field `JOVSE` reader - ADC oversampler enable on scope ADC group injected
pub type JOVSE_R = crate::BitReader<JOVSE>;
impl JOVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JOVSE {
        match self.bits {
            false => JOVSE::Disabled,
            true => JOVSE::Enabled,
        }
    }
    ///Injected oversampling disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVSE::Disabled
    }
    ///Injected oversampling enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVSE::Enabled
    }
}
///Field `JOVSE` writer - ADC oversampler enable on scope ADC group injected
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG, JOVSE>;
impl<'a, REG> JOVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Injected oversampling disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Disabled)
    }
    ///Injected oversampling enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Enabled)
    }
}
///Field `OVSS` reader - ADC oversampling shift
pub type OVSS_R = crate::FieldReader;
///Field `OVSS` writer - ADC oversampling shift
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**ADC oversampling discontinuous mode (triggered mode) for ADC group regular

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS {
    ///0: All oversampled conversions for a channel are run following a trigger
    Automatic = 0,
    ///1: Each oversampled conversion for a channel needs a new trigger
    Triggered = 1,
}
impl From<TROVS> for bool {
    #[inline(always)]
    fn from(variant: TROVS) -> Self {
        variant as u8 != 0
    }
}
///Field `TROVS` reader - ADC oversampling discontinuous mode (triggered mode) for ADC group regular
pub type TROVS_R = crate::BitReader<TROVS>;
impl TROVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TROVS {
        match self.bits {
            false => TROVS::Automatic,
            true => TROVS::Triggered,
        }
    }
    ///All oversampled conversions for a channel are run following a trigger
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == TROVS::Automatic
    }
    ///Each oversampled conversion for a channel needs a new trigger
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TROVS::Triggered
    }
}
///Field `TROVS` writer - ADC oversampling discontinuous mode (triggered mode) for ADC group regular
pub type TROVS_W<'a, REG> = crate::BitWriter<'a, REG, TROVS>;
impl<'a, REG> TROVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///All oversampled conversions for a channel are run following a trigger
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Automatic)
    }
    ///Each oversampled conversion for a channel needs a new trigger
    #[inline(always)]
    pub fn triggered(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Triggered)
    }
}
/**Regular Oversampling mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM {
    ///0: Oversampling is temporary stopped and continued after injection sequence
    Continued = 0,
    ///1: Oversampling is aborted and resumed from start after injection sequence
    Resumed = 1,
}
impl From<ROVSM> for bool {
    #[inline(always)]
    fn from(variant: ROVSM) -> Self {
        variant as u8 != 0
    }
}
///Field `ROVSM` reader - Regular Oversampling mode
pub type ROVSM_R = crate::BitReader<ROVSM>;
impl ROVSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROVSM {
        match self.bits {
            false => ROVSM::Continued,
            true => ROVSM::Resumed,
        }
    }
    ///Oversampling is temporary stopped and continued after injection sequence
    #[inline(always)]
    pub fn is_continued(&self) -> bool {
        *self == ROVSM::Continued
    }
    ///Oversampling is aborted and resumed from start after injection sequence
    #[inline(always)]
    pub fn is_resumed(&self) -> bool {
        *self == ROVSM::Resumed
    }
}
///Field `ROVSM` writer - Regular Oversampling mode
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG, ROVSM>;
impl<'a, REG> ROVSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Oversampling is temporary stopped and continued after injection sequence
    #[inline(always)]
    pub fn continued(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::Continued)
    }
    ///Oversampling is aborted and resumed from start after injection sequence
    #[inline(always)]
    pub fn resumed(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::Resumed)
    }
}
/**Right-shift data after Offset %s correction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSHIFT1 {
    ///0: Right-shifting disabled
    Disabled = 0,
    ///1: Data is right-shifted 1-bit
    Enabled = 1,
}
impl From<RSHIFT1> for bool {
    #[inline(always)]
    fn from(variant: RSHIFT1) -> Self {
        variant as u8 != 0
    }
}
///Field `RSHIFT(1-4)` reader - Right-shift data after Offset %s correction
pub type RSHIFT_R = crate::BitReader<RSHIFT1>;
impl RSHIFT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSHIFT1 {
        match self.bits {
            false => RSHIFT1::Disabled,
            true => RSHIFT1::Enabled,
        }
    }
    ///Right-shifting disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSHIFT1::Disabled
    }
    ///Data is right-shifted 1-bit
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSHIFT1::Enabled
    }
}
///Field `RSHIFT(1-4)` writer - Right-shift data after Offset %s correction
pub type RSHIFT_W<'a, REG> = crate::BitWriter<'a, REG, RSHIFT1>;
impl<'a, REG> RSHIFT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Right-shifting disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSHIFT1::Disabled)
    }
    ///Data is right-shifted 1-bit
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSHIFT1::Enabled)
    }
}
///Field `OSVR` reader - Oversampling ratio
pub type OSVR_R = crate::FieldReader<u16>;
///Field `OSVR` writer - Oversampling ratio
pub type OSVR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
///Field `LSHIFT` reader - Left shift factor
pub type LSHIFT_R = crate::FieldReader;
///Field `LSHIFT` writer - Left shift factor
pub type LSHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Bit 0 - ADC oversampler enable on scope ADC group regular
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC oversampler enable on scope ADC group injected
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 5:8 - ADC oversampling shift
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Regular Oversampling mode
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Right-shift data after Offset (1-4) correction
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `RSHIFT1` field.</div>
    #[inline(always)]
    pub fn rshift(&self, n: u8) -> RSHIFT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RSHIFT_R::new(((self.bits >> (n + 11)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Right-shift data after Offset (1-4) correction
    #[inline(always)]
    pub fn rshift_iter(&self) -> impl Iterator<Item = RSHIFT_R> + '_ {
        (0..4).map(move |n| RSHIFT_R::new(((self.bits >> (n + 11)) & 1) != 0))
    }
    ///Bit 11 - Right-shift data after Offset 1 correction
    #[inline(always)]
    pub fn rshift1(&self) -> RSHIFT_R {
        RSHIFT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Right-shift data after Offset 2 correction
    #[inline(always)]
    pub fn rshift2(&self) -> RSHIFT_R {
        RSHIFT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Right-shift data after Offset 3 correction
    #[inline(always)]
    pub fn rshift3(&self) -> RSHIFT_R {
        RSHIFT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Right-shift data after Offset 4 correction
    #[inline(always)]
    pub fn rshift4(&self) -> RSHIFT_R {
        RSHIFT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:25 - Oversampling ratio
    #[inline(always)]
    pub fn osvr(&self) -> OSVR_R {
        OSVR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 28:31 - Left shift factor
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("rovse", &self.rovse())
            .field("jovse", &self.jovse())
            .field("ovss", &self.ovss())
            .field("trovs", &self.trovs())
            .field("rovsm", &self.rovsm())
            .field("rshift1", &self.rshift1())
            .field("rshift2", &self.rshift2())
            .field("rshift3", &self.rshift3())
            .field("rshift4", &self.rshift4())
            .field("osvr", &self.osvr())
            .field("lshift", &self.lshift())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC oversampler enable on scope ADC group regular
    #[inline(always)]
    pub fn rovse(&mut self) -> ROVSE_W<'_, CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    ///Bit 1 - ADC oversampler enable on scope ADC group injected
    #[inline(always)]
    pub fn jovse(&mut self) -> JOVSE_W<'_, CFGR2rs> {
        JOVSE_W::new(self, 1)
    }
    ///Bits 5:8 - ADC oversampling shift
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W<'_, CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular
    #[inline(always)]
    pub fn trovs(&mut self) -> TROVS_W<'_, CFGR2rs> {
        TROVS_W::new(self, 9)
    }
    ///Bit 10 - Regular Oversampling mode
    #[inline(always)]
    pub fn rovsm(&mut self) -> ROVSM_W<'_, CFGR2rs> {
        ROVSM_W::new(self, 10)
    }
    ///Right-shift data after Offset (1-4) correction
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `RSHIFT1` field.</div>
    #[inline(always)]
    pub fn rshift(&mut self, n: u8) -> RSHIFT_W<'_, CFGR2rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RSHIFT_W::new(self, n + 11)
    }
    ///Bit 11 - Right-shift data after Offset 1 correction
    #[inline(always)]
    pub fn rshift1(&mut self) -> RSHIFT_W<'_, CFGR2rs> {
        RSHIFT_W::new(self, 11)
    }
    ///Bit 12 - Right-shift data after Offset 2 correction
    #[inline(always)]
    pub fn rshift2(&mut self) -> RSHIFT_W<'_, CFGR2rs> {
        RSHIFT_W::new(self, 12)
    }
    ///Bit 13 - Right-shift data after Offset 3 correction
    #[inline(always)]
    pub fn rshift3(&mut self) -> RSHIFT_W<'_, CFGR2rs> {
        RSHIFT_W::new(self, 13)
    }
    ///Bit 14 - Right-shift data after Offset 4 correction
    #[inline(always)]
    pub fn rshift4(&mut self) -> RSHIFT_W<'_, CFGR2rs> {
        RSHIFT_W::new(self, 14)
    }
    ///Bits 16:25 - Oversampling ratio
    #[inline(always)]
    pub fn osvr(&mut self) -> OSVR_W<'_, CFGR2rs> {
        OSVR_W::new(self, 16)
    }
    ///Bits 28:31 - Left shift factor
    #[inline(always)]
    pub fn lshift(&mut self) -> LSHIFT_W<'_, CFGR2rs> {
        LSHIFT_W::new(self, 28)
    }
}
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#ADC3:CFGR2)*/
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
