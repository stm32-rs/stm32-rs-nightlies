#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Low-power mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPMS {
    #[doc = "0: Stop 0 mode"]
    Stop0 = 0,
    #[doc = "1: Stop 1 mode"]
    Stop1 = 1,
    #[doc = "2: Stop 2 mode"]
    Stop2 = 2,
    #[doc = "3: Standby mode"]
    Standby = 3,
    #[doc = "4: Shutdown mode"]
    Shutdown = 4,
}
impl From<LPMS> for u8 {
    #[inline(always)]
    fn from(variant: LPMS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPMS {
    type Ux = u8;
}
#[doc = "Field `LPMS` reader - Low-power mode selection"]
pub type LPMS_R = crate::FieldReader<LPMS>;
impl LPMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPMS> {
        match self.bits {
            0 => Some(LPMS::Stop0),
            1 => Some(LPMS::Stop1),
            2 => Some(LPMS::Stop2),
            3 => Some(LPMS::Standby),
            4 => Some(LPMS::Shutdown),
            _ => None,
        }
    }
    #[doc = "Stop 0 mode"]
    #[inline(always)]
    pub fn is_stop0(&self) -> bool {
        *self == LPMS::Stop0
    }
    #[doc = "Stop 1 mode"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == LPMS::Stop1
    }
    #[doc = "Stop 2 mode"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == LPMS::Stop2
    }
    #[doc = "Standby mode"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMS::Standby
    }
    #[doc = "Shutdown mode"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == LPMS::Shutdown
    }
}
#[doc = "Field `LPMS` writer - Low-power mode selection"]
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPMS>;
impl<'a, REG> LPMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop 0 mode"]
    #[inline(always)]
    pub fn stop0(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop0)
    }
    #[doc = "Stop 1 mode"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop1)
    }
    #[doc = "Stop 2 mode"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop2)
    }
    #[doc = "Standby mode"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Standby)
    }
    #[doc = "Shutdown mode"]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Shutdown)
    }
}
#[doc = "SRAM3 retention in Stop 2 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRSTP {
    #[doc = "0: SRAM3 is powered off in Stop 2 mode (SRAM3 content is lost)"]
    Disabled = 0,
    #[doc = "1: SRAM3 is powered in Stop 2 mode (RAM3 content is kept)"]
    Enabled = 1,
}
impl From<RRSTP> for bool {
    #[inline(always)]
    fn from(variant: RRSTP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRSTP` reader - SRAM3 retention in Stop 2 mode"]
pub type RRSTP_R = crate::BitReader<RRSTP>;
impl RRSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RRSTP {
        match self.bits {
            false => RRSTP::Disabled,
            true => RRSTP::Enabled,
        }
    }
    #[doc = "SRAM3 is powered off in Stop 2 mode (SRAM3 content is lost)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RRSTP::Disabled
    }
    #[doc = "SRAM3 is powered in Stop 2 mode (RAM3 content is kept)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RRSTP::Enabled
    }
}
#[doc = "Field `RRSTP` writer - SRAM3 retention in Stop 2 mode"]
pub type RRSTP_W<'a, REG> = crate::BitWriter<'a, REG, RRSTP>;
impl<'a, REG> RRSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM3 is powered off in Stop 2 mode (SRAM3 content is lost)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRSTP::Disabled)
    }
    #[doc = "SRAM3 is powered in Stop 2 mode (RAM3 content is kept)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRSTP::Enabled)
    }
}
#[doc = "Disable backup domain write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP {
    #[doc = "0: Access to RTC and Backup registers disabled"]
    Disabled = 0,
    #[doc = "1: Access to RTC and Backup registers enabled"]
    Enabled = 1,
}
impl From<DBP> for bool {
    #[inline(always)]
    fn from(variant: DBP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DBP_R = crate::BitReader<DBP>;
impl DBP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBP {
        match self.bits {
            false => DBP::Disabled,
            true => DBP::Enabled,
        }
    }
    #[doc = "Access to RTC and Backup registers disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBP::Disabled
    }
    #[doc = "Access to RTC and Backup registers enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBP::Enabled
    }
}
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG, DBP>;
impl<'a, REG> DBP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access to RTC and Backup registers disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Disabled)
    }
    #[doc = "Access to RTC and Backup registers enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Enabled)
    }
}
#[doc = "Voltage scaling range selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS {
    #[doc = "1: Range 1"]
    Range1 = 1,
    #[doc = "2: Range 1"]
    Range2 = 2,
}
impl From<VOS> for u8 {
    #[inline(always)]
    fn from(variant: VOS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VOS {
    type Ux = u8;
}
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VOS_R = crate::FieldReader<VOS>;
impl VOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VOS> {
        match self.bits {
            1 => Some(VOS::Range1),
            2 => Some(VOS::Range2),
            _ => None,
        }
    }
    #[doc = "Range 1"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == VOS::Range1
    }
    #[doc = "Range 1"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == VOS::Range2
    }
}
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VOS>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Range 1"]
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Range1)
    }
    #[doc = "Range 1"]
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Range2)
    }
}
#[doc = "Low-power run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPR {
    #[doc = "0: Main Mode"]
    MainMode = 0,
    #[doc = "1: Low Power Mode"]
    LowPowerMode = 1,
}
impl From<LPR> for bool {
    #[inline(always)]
    fn from(variant: LPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPR` reader - Low-power run"]
pub type LPR_R = crate::BitReader<LPR>;
impl LPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPR {
        match self.bits {
            false => LPR::MainMode,
            true => LPR::LowPowerMode,
        }
    }
    #[doc = "Main Mode"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPR::MainMode
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPR::LowPowerMode
    }
}
#[doc = "Field `LPR` writer - Low-power run"]
pub type LPR_W<'a, REG> = crate::BitWriter<'a, REG, LPR>;
impl<'a, REG> LPR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Main Mode"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPR::MainMode)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPR::LowPowerMode)
    }
}
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - SRAM3 retention in Stop 2 mode"]
    #[inline(always)]
    pub fn rrstp(&self) -> RRSTP_R {
        RRSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<CR1rs> {
        LPMS_W::new(self, 0)
    }
    #[doc = "Bit 4 - SRAM3 retention in Stop 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn rrstp(&mut self) -> RRSTP_W<CR1rs> {
        RRSTP_W::new(self, 4)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<CR1rs> {
        DBP_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<CR1rs> {
        VOS_W::new(self, 9)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    #[must_use]
    pub fn lpr(&mut self) -> LPR_W<CR1rs> {
        LPR_W::new(self, 14)
    }
}
#[doc = "Power control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x0200"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x0200;
}
