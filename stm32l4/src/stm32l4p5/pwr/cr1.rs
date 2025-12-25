///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
/**Low-power mode selection These bits select the low-power mode entered when CPU enters the Deepsleep mode. 1xx: Shutdown mode Note: If LPR bit is set, Stop 2 mode cannot be selected and Stop 1 mode shall be entered instead of Stop 2. Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPMS {
    ///0: Stop 0 mode
    Stop0 = 0,
    ///1: Stop 1 mode
    Stop1 = 1,
    ///2: Stop 2 mode
    Stop2 = 2,
    ///3: Standby mode
    Standby = 3,
    ///4: Shutdown mode
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
impl crate::IsEnum for LPMS {}
///Field `LPMS` reader - Low-power mode selection These bits select the low-power mode entered when CPU enters the Deepsleep mode. 1xx: Shutdown mode Note: If LPR bit is set, Stop 2 mode cannot be selected and Stop 1 mode shall be entered instead of Stop 2. Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3.
pub type LPMS_R = crate::FieldReader<LPMS>;
impl LPMS_R {
    ///Get enumerated values variant
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
    ///Stop 0 mode
    #[inline(always)]
    pub fn is_stop0(&self) -> bool {
        *self == LPMS::Stop0
    }
    ///Stop 1 mode
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == LPMS::Stop1
    }
    ///Stop 2 mode
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == LPMS::Stop2
    }
    ///Standby mode
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMS::Standby
    }
    ///Shutdown mode
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == LPMS::Shutdown
    }
}
///Field `LPMS` writer - Low-power mode selection These bits select the low-power mode entered when CPU enters the Deepsleep mode. 1xx: Shutdown mode Note: If LPR bit is set, Stop 2 mode cannot be selected and Stop 1 mode shall be entered instead of Stop 2. Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3.
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPMS>;
impl<'a, REG> LPMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Stop 0 mode
    #[inline(always)]
    pub fn stop0(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop0)
    }
    ///Stop 1 mode
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop1)
    }
    ///Stop 2 mode
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop2)
    }
    ///Standby mode
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Standby)
    }
    ///Shutdown mode
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Shutdown)
    }
}
/**SRAM3 retention in Stop 2 mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRSTP {
    ///0: SRAM3 is powered off in Stop 2 mode (SRAM3 content is lost)
    Disabled = 0,
    ///1: SRAM3 is powered in Stop 2 mode (RAM3 content is kept)
    Enabled = 1,
}
impl From<RRSTP> for bool {
    #[inline(always)]
    fn from(variant: RRSTP) -> Self {
        variant as u8 != 0
    }
}
///Field `RRSTP` reader - SRAM3 retention in Stop 2 mode
pub type RRSTP_R = crate::BitReader<RRSTP>;
impl RRSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RRSTP {
        match self.bits {
            false => RRSTP::Disabled,
            true => RRSTP::Enabled,
        }
    }
    ///SRAM3 is powered off in Stop 2 mode (SRAM3 content is lost)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RRSTP::Disabled
    }
    ///SRAM3 is powered in Stop 2 mode (RAM3 content is kept)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RRSTP::Enabled
    }
}
///Field `RRSTP` writer - SRAM3 retention in Stop 2 mode
pub type RRSTP_W<'a, REG> = crate::BitWriter<'a, REG, RRSTP>;
impl<'a, REG> RRSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM3 is powered off in Stop 2 mode (SRAM3 content is lost)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRSTP::Disabled)
    }
    ///SRAM3 is powered in Stop 2 mode (RAM3 content is kept)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RRSTP::Enabled)
    }
}
/**Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP {
    ///0: Access to RTC and Backup registers disabled
    Disabled = 0,
    ///1: Access to RTC and Backup registers enabled
    Enabled = 1,
}
impl From<DBP> for bool {
    #[inline(always)]
    fn from(variant: DBP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBP` reader - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers.
pub type DBP_R = crate::BitReader<DBP>;
impl DBP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBP {
        match self.bits {
            false => DBP::Disabled,
            true => DBP::Enabled,
        }
    }
    ///Access to RTC and Backup registers disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBP::Disabled
    }
    ///Access to RTC and Backup registers enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBP::Enabled
    }
}
///Field `DBP` writer - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers.
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG, DBP>;
impl<'a, REG> DBP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Access to RTC and Backup registers disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Disabled)
    }
    ///Access to RTC and Backup registers enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Enabled)
    }
}
/**Voltage scaling range selection

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS {
    ///1: Range 1
    Range1 = 1,
    ///2: Range 1
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
impl crate::IsEnum for VOS {}
///Field `VOS` reader - Voltage scaling range selection
pub type VOS_R = crate::FieldReader<VOS>;
impl VOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VOS> {
        match self.bits {
            1 => Some(VOS::Range1),
            2 => Some(VOS::Range2),
            _ => None,
        }
    }
    ///Range 1
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == VOS::Range1
    }
    ///Range 1
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == VOS::Range2
    }
}
///Field `VOS` writer - Voltage scaling range selection
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VOS>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Range 1
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Range1)
    }
    ///Range 1
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Range2)
    }
}
/**Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR). Note: Stop 2 mode cannot be entered when LPR bit is set. Stop 1 is entered instead.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPR {
    ///0: Main Mode
    MainMode = 0,
    ///1: Low Power Mode
    LowPowerMode = 1,
}
impl From<LPR> for bool {
    #[inline(always)]
    fn from(variant: LPR) -> Self {
        variant as u8 != 0
    }
}
///Field `LPR` reader - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR). Note: Stop 2 mode cannot be entered when LPR bit is set. Stop 1 is entered instead.
pub type LPR_R = crate::BitReader<LPR>;
impl LPR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPR {
        match self.bits {
            false => LPR::MainMode,
            true => LPR::LowPowerMode,
        }
    }
    ///Main Mode
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPR::MainMode
    }
    ///Low Power Mode
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPR::LowPowerMode
    }
}
///Field `LPR` writer - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR). Note: Stop 2 mode cannot be entered when LPR bit is set. Stop 1 is entered instead.
pub type LPR_W<'a, REG> = crate::BitWriter<'a, REG, LPR>;
impl<'a, REG> LPR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main Mode
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPR::MainMode)
    }
    ///Low Power Mode
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut crate::W<REG> {
        self.variant(LPR::LowPowerMode)
    }
}
impl R {
    ///Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when CPU enters the Deepsleep mode. 1xx: Shutdown mode Note: If LPR bit is set, Stop 2 mode cannot be selected and Stop 1 mode shall be entered instead of Stop 2. Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3.
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - SRAM3 retention in Stop 2 mode
    #[inline(always)]
    pub fn rrstp(&self) -> RRSTP_R {
        RRSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers.
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 14 - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR). Note: Stop 2 mode cannot be entered when LPR bit is set. Stop 1 is entered instead.
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lpms", &self.lpms())
            .field("rrstp", &self.rrstp())
            .field("dbp", &self.dbp())
            .field("vos", &self.vos())
            .field("lpr", &self.lpr())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when CPU enters the Deepsleep mode. 1xx: Shutdown mode Note: If LPR bit is set, Stop 2 mode cannot be selected and Stop 1 mode shall be entered instead of Stop 2. Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3.
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<'_, CR1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bit 4 - SRAM3 retention in Stop 2 mode
    #[inline(always)]
    pub fn rrstp(&mut self) -> RRSTP_W<'_, CR1rs> {
        RRSTP_W::new(self, 4)
    }
    ///Bit 8 - Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers.
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, CR1rs> {
        DBP_W::new(self, 8)
    }
    ///Bits 9:10 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, CR1rs> {
        VOS_W::new(self, 9)
    }
    ///Bit 14 - Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR). Note: Stop 2 mode cannot be entered when LPR bit is set. Stop 1 is entered instead.
    #[inline(always)]
    pub fn lpr(&mut self) -> LPR_W<'_, CR1rs> {
        LPR_W::new(self, 14)
    }
}
/**Power control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#PWR:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0x0200
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x0200;
}
