#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "DMAEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE {
    #[doc = "0: Regular Oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Regular Oversampling enabled"]
    Enabled = 1,
}
impl From<ROVSE> for bool {
    #[inline(always)]
    fn from(variant: ROVSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVSE` reader - DMAEN"]
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
    #[doc = "Regular Oversampling disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVSE::Disabled
    }
    #[doc = "Regular Oversampling enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVSE::Enabled
    }
}
#[doc = "Field `ROVSE` writer - DMAEN"]
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG, ROVSE>;
impl<'a, REG> ROVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regular Oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Disabled)
    }
    #[doc = "Regular Oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSE::Enabled)
    }
}
#[doc = "DMACFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE {
    #[doc = "0: Injected Oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Injected Oversampling enabled"]
    Enabled = 1,
}
impl From<JOVSE> for bool {
    #[inline(always)]
    fn from(variant: JOVSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JOVSE` reader - DMACFG"]
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
    #[doc = "Injected Oversampling disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVSE::Disabled
    }
    #[doc = "Injected Oversampling enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVSE::Enabled
    }
}
#[doc = "Field `JOVSE` writer - DMACFG"]
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG, JOVSE>;
impl<'a, REG> JOVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injected Oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Disabled)
    }
    #[doc = "Injected Oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JOVSE::Enabled)
    }
}
#[doc = "RES\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR {
    #[doc = "0: 2x"]
    Ratio2 = 0,
    #[doc = "1: 4x"]
    Ratio4 = 1,
    #[doc = "2: 8x"]
    Ratio8 = 2,
    #[doc = "3: 16x"]
    Ratio16 = 3,
    #[doc = "4: 32x"]
    Ratio32 = 4,
    #[doc = "5: 64x"]
    Ratio64 = 5,
    #[doc = "6: 128x"]
    Ratio128 = 6,
    #[doc = "7: 256x"]
    Ratio256 = 7,
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
#[doc = "Field `OVSR` reader - RES"]
pub type OVSR_R = crate::FieldReader<OVSR>;
impl OVSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVSR {
        match self.bits {
            0 => OVSR::Ratio2,
            1 => OVSR::Ratio4,
            2 => OVSR::Ratio8,
            3 => OVSR::Ratio16,
            4 => OVSR::Ratio32,
            5 => OVSR::Ratio64,
            6 => OVSR::Ratio128,
            7 => OVSR::Ratio256,
            _ => unreachable!(),
        }
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn is_ratio2(&self) -> bool {
        *self == OVSR::Ratio2
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn is_ratio4(&self) -> bool {
        *self == OVSR::Ratio4
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn is_ratio8(&self) -> bool {
        *self == OVSR::Ratio8
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn is_ratio16(&self) -> bool {
        *self == OVSR::Ratio16
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn is_ratio32(&self) -> bool {
        *self == OVSR::Ratio32
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn is_ratio64(&self) -> bool {
        *self == OVSR::Ratio64
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn is_ratio128(&self) -> bool {
        *self == OVSR::Ratio128
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn is_ratio256(&self) -> bool {
        *self == OVSR::Ratio256
    }
}
#[doc = "Field `OVSR` writer - RES"]
pub type OVSR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OVSR>;
impl<'a, REG> OVSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2x"]
    #[inline(always)]
    pub fn ratio2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio2)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn ratio4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio4)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn ratio8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio8)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn ratio16(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio16)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn ratio32(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio32)
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn ratio64(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio64)
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn ratio128(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio128)
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn ratio256(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR::Ratio256)
    }
}
#[doc = "ALIGN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS {
    #[doc = "0: No Shift"]
    NoShift = 0,
    #[doc = "1: Shift 1-bit"]
    Shift1bit = 1,
    #[doc = "2: Shift 2-bit"]
    Shift2bit = 2,
    #[doc = "3: Shift 3-bit"]
    Shift3bit = 3,
    #[doc = "4: Shift 4-bit"]
    Shift4bit = 4,
    #[doc = "5: Shift 5-bit"]
    Shift5bit = 5,
    #[doc = "6: Shift 6-bit"]
    Shift6bit = 6,
    #[doc = "7: Shift 7-bit"]
    Shift7bit = 7,
    #[doc = "8: Shift 8-bit"]
    Shift8bit = 8,
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
#[doc = "Field `OVSS` reader - ALIGN"]
pub type OVSS_R = crate::FieldReader<OVSS>;
impl OVSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OVSS> {
        match self.bits {
            0 => Some(OVSS::NoShift),
            1 => Some(OVSS::Shift1bit),
            2 => Some(OVSS::Shift2bit),
            3 => Some(OVSS::Shift3bit),
            4 => Some(OVSS::Shift4bit),
            5 => Some(OVSS::Shift5bit),
            6 => Some(OVSS::Shift6bit),
            7 => Some(OVSS::Shift7bit),
            8 => Some(OVSS::Shift8bit),
            _ => None,
        }
    }
    #[doc = "No Shift"]
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == OVSS::NoShift
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn is_shift1bit(&self) -> bool {
        *self == OVSS::Shift1bit
    }
    #[doc = "Shift 2-bit"]
    #[inline(always)]
    pub fn is_shift2bit(&self) -> bool {
        *self == OVSS::Shift2bit
    }
    #[doc = "Shift 3-bit"]
    #[inline(always)]
    pub fn is_shift3bit(&self) -> bool {
        *self == OVSS::Shift3bit
    }
    #[doc = "Shift 4-bit"]
    #[inline(always)]
    pub fn is_shift4bit(&self) -> bool {
        *self == OVSS::Shift4bit
    }
    #[doc = "Shift 5-bit"]
    #[inline(always)]
    pub fn is_shift5bit(&self) -> bool {
        *self == OVSS::Shift5bit
    }
    #[doc = "Shift 6-bit"]
    #[inline(always)]
    pub fn is_shift6bit(&self) -> bool {
        *self == OVSS::Shift6bit
    }
    #[doc = "Shift 7-bit"]
    #[inline(always)]
    pub fn is_shift7bit(&self) -> bool {
        *self == OVSS::Shift7bit
    }
    #[doc = "Shift 8-bit"]
    #[inline(always)]
    pub fn is_shift8bit(&self) -> bool {
        *self == OVSS::Shift8bit
    }
}
#[doc = "Field `OVSS` writer - ALIGN"]
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSS>;
impl<'a, REG> OVSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Shift"]
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::NoShift)
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn shift1bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift1bit)
    }
    #[doc = "Shift 2-bit"]
    #[inline(always)]
    pub fn shift2bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift2bit)
    }
    #[doc = "Shift 3-bit"]
    #[inline(always)]
    pub fn shift3bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift3bit)
    }
    #[doc = "Shift 4-bit"]
    #[inline(always)]
    pub fn shift4bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift4bit)
    }
    #[doc = "Shift 5-bit"]
    #[inline(always)]
    pub fn shift5bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift5bit)
    }
    #[doc = "Shift 6-bit"]
    #[inline(always)]
    pub fn shift6bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift6bit)
    }
    #[doc = "Shift 7-bit"]
    #[inline(always)]
    pub fn shift7bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift7bit)
    }
    #[doc = "Shift 8-bit"]
    #[inline(always)]
    pub fn shift8bit(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS::Shift8bit)
    }
}
#[doc = "Triggered Regular Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS {
    #[doc = "0: All oversampled conversions for a channel are done consecutively following a trigger"]
    All = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a new trigger"]
    Single = 1,
}
impl From<TROVS> for bool {
    #[inline(always)]
    fn from(variant: TROVS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TROVS` reader - Triggered Regular Oversampling"]
pub type TROVS_R = crate::BitReader<TROVS>;
impl TROVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TROVS {
        match self.bits {
            false => TROVS::All,
            true => TROVS::Single,
        }
    }
    #[doc = "All oversampled conversions for a channel are done consecutively following a trigger"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == TROVS::All
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TROVS::Single
    }
}
#[doc = "Field `TROVS` writer - Triggered Regular Oversampling"]
pub type TROVS_W<'a, REG> = crate::BitWriter<'a, REG, TROVS>;
impl<'a, REG> TROVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All oversampled conversions for a channel are done consecutively following a trigger"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::All)
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(TROVS::Single)
    }
}
#[doc = "Regular Oversampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM {
    #[doc = "0: When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)"]
    ContinuedMode = 0,
    #[doc = "1: When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)"]
    ResumedMode = 1,
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
            false => ROVSM::ContinuedMode,
            true => ROVSM::ResumedMode,
        }
    }
    #[doc = "When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)"]
    #[inline(always)]
    pub fn is_continued_mode(&self) -> bool {
        *self == ROVSM::ContinuedMode
    }
    #[doc = "When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)"]
    #[inline(always)]
    pub fn is_resumed_mode(&self) -> bool {
        *self == ROVSM::ResumedMode
    }
}
#[doc = "Field `ROVSM` writer - Regular Oversampling mode"]
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG, ROVSM>;
impl<'a, REG> ROVSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)"]
    #[inline(always)]
    pub fn continued_mode(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::ContinuedMode)
    }
    #[doc = "When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)"]
    #[inline(always)]
    pub fn resumed_mode(self) -> &'a mut crate::W<REG> {
        self.variant(ROVSM::ResumedMode)
    }
}
impl R {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - ALIGN"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<CFGR2rs> {
        JOVSE_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<CFGR2rs> {
        OVSR_W::new(self, 2)
    }
    #[doc = "Bits 5:8 - ALIGN"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
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
}
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
