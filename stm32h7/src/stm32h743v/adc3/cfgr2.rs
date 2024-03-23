#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "ADC oversampler enable on scope ADC group regular\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE {
    #[doc = "0: Regular oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Regular oversampling enabled"]
    Enabled = 1,
}
impl From<ROVSE> for bool {
    #[inline(always)]
    fn from(variant: ROVSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVSE` reader - ADC oversampler enable on scope ADC group regular"]
pub type ROVSE_R = crate::BitReader<ROVSE>;
impl ROVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROVSE {
        match self.bits {
            false => ROVSE::Disabled,
            true => ROVSE::Enabled,
        }
    }
    #[doc = "Regular oversampling disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVSE::Disabled
    }
    #[doc = "Regular oversampling enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVSE::Enabled
    }
}
#[doc = "Field `ROVSE` writer - ADC oversampler enable on scope ADC group regular"]
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG, ROVSE>;
impl<'a, REG> ROVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Disabled)
    }
    #[doc = "Regular oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Enabled)
    }
}
#[doc = "ADC oversampler enable on scope ADC group injected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE {
    #[doc = "0: Injected oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Injected oversampling enabled"]
    Enabled = 1,
}
impl From<JOVSE> for bool {
    #[inline(always)]
    fn from(variant: JOVSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JOVSE` reader - ADC oversampler enable on scope ADC group injected"]
pub type JOVSE_R = crate::BitReader<JOVSE>;
impl JOVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JOVSE {
        match self.bits {
            false => JOVSE::Disabled,
            true => JOVSE::Enabled,
        }
    }
    #[doc = "Injected oversampling disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVSE::Disabled
    }
    #[doc = "Injected oversampling enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVSE::Enabled
    }
}
#[doc = "Field `JOVSE` writer - ADC oversampler enable on scope ADC group injected"]
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG, JOVSE>;
impl<'a, REG> JOVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Disabled)
    }
    #[doc = "Injected oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Enabled)
    }
}
#[doc = "Field `OVSS` reader - ADC oversampling shift"]
pub type OVSS_R = crate::FieldReader;
#[doc = "Field `OVSS` writer - ADC oversampling shift"]
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "ADC oversampling discontinuous mode (triggered mode) for ADC group regular\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS {
    #[doc = "0: All oversampled conversions for a channel are run following a trigger"]
    Automatic = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a new trigger"]
    Triggered = 1,
}
impl From<TROVS> for bool {
    #[inline(always)]
    fn from(variant: TROVS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TROVS` reader - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
pub type TROVS_R = crate::BitReader<TROVS>;
impl TROVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TROVS {
        match self.bits {
            false => TROVS::Automatic,
            true => TROVS::Triggered,
        }
    }
    #[doc = "All oversampled conversions for a channel are run following a trigger"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == TROVS::Automatic
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TROVS::Triggered
    }
}
#[doc = "Field `TROVS` writer - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
pub type TROVS_W<'a, REG> = crate::BitWriter<'a, REG, TROVS>;
impl<'a, REG> TROVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All oversampled conversions for a channel are run following a trigger"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Automatic)
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn triggered(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Triggered)
    }
}
#[doc = "Regular Oversampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM {
    #[doc = "0: Oversampling is temporary stopped and continued after injection sequence"]
    Continued = 0,
    #[doc = "1: Oversampling is aborted and resumed from start after injection sequence"]
    Resumed = 1,
}
impl From<ROVSM> for bool {
    #[inline(always)]
    fn from(variant: ROVSM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVSM` reader - Regular Oversampling mode"]
pub type ROVSM_R = crate::BitReader<ROVSM>;
impl ROVSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROVSM {
        match self.bits {
            false => ROVSM::Continued,
            true => ROVSM::Resumed,
        }
    }
    #[doc = "Oversampling is temporary stopped and continued after injection sequence"]
    #[inline(always)]
    pub fn is_continued(&self) -> bool {
        *self == ROVSM::Continued
    }
    #[doc = "Oversampling is aborted and resumed from start after injection sequence"]
    #[inline(always)]
    pub fn is_resumed(&self) -> bool {
        *self == ROVSM::Resumed
    }
}
#[doc = "Field `ROVSM` writer - Regular Oversampling mode"]
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG, ROVSM>;
impl<'a, REG> ROVSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oversampling is temporary stopped and continued after injection sequence"]
    #[inline(always)]
    pub fn continued(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::Continued)
    }
    #[doc = "Oversampling is aborted and resumed from start after injection sequence"]
    #[inline(always)]
    pub fn resumed(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::Resumed)
    }
}
#[doc = "Right-shift data after Offset 1 correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSHIFT1 {
    #[doc = "0: Right-shifting disabled"]
    Disabled = 0,
    #[doc = "1: Data is right-shifted 1-bit"]
    Enabled = 1,
}
impl From<RSHIFT1> for bool {
    #[inline(always)]
    fn from(variant: RSHIFT1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSHIFT1` reader - Right-shift data after Offset 1 correction"]
pub type RSHIFT1_R = crate::BitReader<RSHIFT1>;
impl RSHIFT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSHIFT1 {
        match self.bits {
            false => RSHIFT1::Disabled,
            true => RSHIFT1::Enabled,
        }
    }
    #[doc = "Right-shifting disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSHIFT1::Disabled
    }
    #[doc = "Data is right-shifted 1-bit"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSHIFT1::Enabled
    }
}
#[doc = "Field `RSHIFT1` writer - Right-shift data after Offset 1 correction"]
pub type RSHIFT1_W<'a, REG> = crate::BitWriter<'a, REG, RSHIFT1>;
impl<'a, REG> RSHIFT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right-shifting disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSHIFT1::Disabled)
    }
    #[doc = "Data is right-shifted 1-bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSHIFT1::Enabled)
    }
}
#[doc = "Field `RSHIFT2` reader - Right-shift data after Offset 2 correction"]
pub use RSHIFT1_R as RSHIFT2_R;
#[doc = "Field `RSHIFT3` reader - Right-shift data after Offset 3 correction"]
pub use RSHIFT1_R as RSHIFT3_R;
#[doc = "Field `RSHIFT4` reader - Right-shift data after Offset 4 correction"]
pub use RSHIFT1_R as RSHIFT4_R;
#[doc = "Field `RSHIFT2` writer - Right-shift data after Offset 2 correction"]
pub use RSHIFT1_W as RSHIFT2_W;
#[doc = "Field `RSHIFT3` writer - Right-shift data after Offset 3 correction"]
pub use RSHIFT1_W as RSHIFT3_W;
#[doc = "Field `RSHIFT4` writer - Right-shift data after Offset 4 correction"]
pub use RSHIFT1_W as RSHIFT4_W;
#[doc = "Field `OSVR` reader - Oversampling ratio"]
pub type OSVR_R = crate::FieldReader<u16>;
#[doc = "Field `OSVR` writer - Oversampling ratio"]
pub type OSVR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 10, u16>;
#[doc = "Field `LSHIFT` reader - Left shift factor"]
pub type LSHIFT_R = crate::FieldReader;
#[doc = "Field `LSHIFT` writer - Left shift factor"]
pub type LSHIFT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - ADC oversampler enable on scope ADC group regular"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC oversampler enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:8 - ADC oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Right-shift data after Offset 1 correction"]
    #[inline(always)]
    pub fn rshift1(&self) -> RSHIFT1_R {
        RSHIFT1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Right-shift data after Offset 2 correction"]
    #[inline(always)]
    pub fn rshift2(&self) -> RSHIFT2_R {
        RSHIFT2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Right-shift data after Offset 3 correction"]
    #[inline(always)]
    pub fn rshift3(&self) -> RSHIFT3_R {
        RSHIFT3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Right-shift data after Offset 4 correction"]
    #[inline(always)]
    pub fn rshift4(&self) -> RSHIFT4_R {
        RSHIFT4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Oversampling ratio"]
    #[inline(always)]
    pub fn osvr(&self) -> OSVR_R {
        OSVR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - Left shift factor"]
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC oversampler enable on scope ADC group regular"]
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC oversampler enable on scope ADC group injected"]
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<CFGR2rs> {
        JOVSE_W::new(self, 1)
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
    pub fn trovs(&mut self) -> TROVS_W<CFGR2rs> {
        TROVS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    #[must_use]
    pub fn rovsm(&mut self) -> ROVSM_W<CFGR2rs> {
        ROVSM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Right-shift data after Offset 1 correction"]
    #[inline(always)]
    #[must_use]
    pub fn rshift1(&mut self) -> RSHIFT1_W<CFGR2rs> {
        RSHIFT1_W::new(self, 11)
    }
    #[doc = "Bit 12 - Right-shift data after Offset 2 correction"]
    #[inline(always)]
    #[must_use]
    pub fn rshift2(&mut self) -> RSHIFT2_W<CFGR2rs> {
        RSHIFT2_W::new(self, 12)
    }
    #[doc = "Bit 13 - Right-shift data after Offset 3 correction"]
    #[inline(always)]
    #[must_use]
    pub fn rshift3(&mut self) -> RSHIFT3_W<CFGR2rs> {
        RSHIFT3_W::new(self, 13)
    }
    #[doc = "Bit 14 - Right-shift data after Offset 4 correction"]
    #[inline(always)]
    #[must_use]
    pub fn rshift4(&mut self) -> RSHIFT4_W<CFGR2rs> {
        RSHIFT4_W::new(self, 14)
    }
    #[doc = "Bits 16:25 - Oversampling ratio"]
    #[inline(always)]
    #[must_use]
    pub fn osvr(&mut self) -> OSVR_W<CFGR2rs> {
        OSVR_W::new(self, 16)
    }
    #[doc = "Bits 28:31 - Left shift factor"]
    #[inline(always)]
    #[must_use]
    pub fn lshift(&mut self) -> LSHIFT_W<CFGR2rs> {
        LSHIFT_W::new(self, 28)
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
