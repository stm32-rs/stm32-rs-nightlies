///Register `OBR` reader
pub type R = crate::R<OBRrs>;
/**Option byte error

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTERR {
    ///1: The loaded option byte and its complement do not match
    OptionByteError = 1,
}
impl From<OPTERR> for bool {
    #[inline(always)]
    fn from(variant: OPTERR) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTERR` reader - Option byte error
pub type OPTERR_R = crate::BitReader<OPTERR>;
impl OPTERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPTERR> {
        match self.bits {
            true => Some(OPTERR::OptionByteError),
            _ => None,
        }
    }
    ///The loaded option byte and its complement do not match
    #[inline(always)]
    pub fn is_option_byte_error(&self) -> bool {
        *self == OPTERR::OptionByteError
    }
}
/**Read protection Level status

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDPRT {
    ///0: Level 0
    Level0 = 0,
    ///1: Level 1
    Level1 = 1,
    ///3: Level 2
    Level2 = 3,
}
impl From<RDPRT> for u8 {
    #[inline(always)]
    fn from(variant: RDPRT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RDPRT {
    type Ux = u8;
}
impl crate::IsEnum for RDPRT {}
///Field `RDPRT` reader - Read protection Level status
pub type RDPRT_R = crate::FieldReader<RDPRT>;
impl RDPRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RDPRT> {
        match self.bits {
            0 => Some(RDPRT::Level0),
            1 => Some(RDPRT::Level1),
            3 => Some(RDPRT::Level2),
            _ => None,
        }
    }
    ///Level 0
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDPRT::Level0
    }
    ///Level 1
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDPRT::Level1
    }
    ///Level 2
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDPRT::Level2
    }
}
/**WDG_SW

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDG_SW {
    ///0: Hardware watchdog
    Hardware = 0,
    ///1: Software watchdog
    Software = 1,
}
impl From<WDG_SW> for bool {
    #[inline(always)]
    fn from(variant: WDG_SW) -> Self {
        variant as u8 != 0
    }
}
///Field `WDG_SW` reader - WDG_SW
pub type WDG_SW_R = crate::BitReader<WDG_SW>;
impl WDG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDG_SW {
        match self.bits {
            false => WDG_SW::Hardware,
            true => WDG_SW::Software,
        }
    }
    ///Hardware watchdog
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WDG_SW::Hardware
    }
    ///Software watchdog
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WDG_SW::Software
    }
}
/**nRST_STOP

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STOP {
    ///0: Reset generated when entering Stop mode
    Reset = 0,
    ///1: No reset generated
    NoReset = 1,
}
impl From<N_RST_STOP> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `nRST_STOP` reader - nRST_STOP
pub type N_RST_STOP_R = crate::BitReader<N_RST_STOP>;
impl N_RST_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_RST_STOP {
        match self.bits {
            false => N_RST_STOP::Reset,
            true => N_RST_STOP::NoReset,
        }
    }
    ///Reset generated when entering Stop mode
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STOP::Reset
    }
    ///No reset generated
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STOP::NoReset
    }
}
/**nRST_STDBY

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STDBY {
    ///0: Reset generated when entering Standby mode
    Reset = 0,
    ///1: No reset generated
    NoReset = 1,
}
impl From<N_RST_STDBY> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STDBY) -> Self {
        variant as u8 != 0
    }
}
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader<N_RST_STDBY>;
impl N_RST_STDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_RST_STDBY {
        match self.bits {
            false => N_RST_STDBY::Reset,
            true => N_RST_STDBY::NoReset,
        }
    }
    ///Reset generated when entering Standby mode
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STDBY::Reset
    }
    ///No reset generated
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STDBY::NoReset
    }
}
/**BOOT1

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_BOOT1 {
    ///0: Together with BOOT0, select the device boot mode
    Disabled = 0,
    ///1: Together with BOOT0, select the device boot mode
    Enabled = 1,
}
impl From<N_BOOT1> for bool {
    #[inline(always)]
    fn from(variant: N_BOOT1) -> Self {
        variant as u8 != 0
    }
}
///Field `nBOOT1` reader - BOOT1
pub type N_BOOT1_R = crate::BitReader<N_BOOT1>;
impl N_BOOT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> N_BOOT1 {
        match self.bits {
            false => N_BOOT1::Disabled,
            true => N_BOOT1::Enabled,
        }
    }
    ///Together with BOOT0, select the device boot mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_BOOT1::Disabled
    }
    ///Together with BOOT0, select the device boot mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_BOOT1::Enabled
    }
}
/**VDDA_MONITOR

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDA_MONITOR {
    ///0: VDDA power supply supervisor disabled
    Disabled = 0,
    ///1: VDDA power supply supervisor enabled
    Enabled = 1,
}
impl From<VDDA_MONITOR> for bool {
    #[inline(always)]
    fn from(variant: VDDA_MONITOR) -> Self {
        variant as u8 != 0
    }
}
///Field `VDDA_MONITOR` reader - VDDA_MONITOR
pub type VDDA_MONITOR_R = crate::BitReader<VDDA_MONITOR>;
impl VDDA_MONITOR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDDA_MONITOR {
        match self.bits {
            false => VDDA_MONITOR::Disabled,
            true => VDDA_MONITOR::Enabled,
        }
    }
    ///VDDA power supply supervisor disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VDDA_MONITOR::Disabled
    }
    ///VDDA power supply supervisor enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VDDA_MONITOR::Enabled
    }
}
///Field `SRAM_PARITY_CHECK` reader - SRAM_PARITY_CHECK
pub type SRAM_PARITY_CHECK_R = crate::BitReader;
///Field `Data0` reader - Data0
pub type DATA0_R = crate::FieldReader;
///Field `Data1` reader - Data1
pub type DATA1_R = crate::FieldReader;
impl R {
    ///Bit 0 - Option byte error
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Read protection Level status
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 8 - WDG_SW
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - BOOT1
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - VDDA_MONITOR
    #[inline(always)]
    pub fn vdda_monitor(&self) -> VDDA_MONITOR_R {
        VDDA_MONITOR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SRAM_PARITY_CHECK
    #[inline(always)]
    pub fn sram_parity_check(&self) -> SRAM_PARITY_CHECK_R {
        SRAM_PARITY_CHECK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:23 - Data0
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data1
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OBR")
            .field("opterr", &self.opterr())
            .field("wdg_sw", &self.wdg_sw())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("n_boot1", &self.n_boot1())
            .field("vdda_monitor", &self.vdda_monitor())
            .field("sram_parity_check", &self.sram_parity_check())
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .field("rdprt", &self.rdprt())
            .finish()
    }
}
/**Option byte register

You can [`read`](crate::Reg::read) this register and get [`obr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#FLASH:OBR)*/
pub struct OBRrs;
impl crate::RegisterSpec for OBRrs {
    type Ux = u32;
}
///`read()` method returns [`obr::R`](R) reader structure
impl crate::Readable for OBRrs {}
///`reset()` method sets OBR to value 0xffff_ff0f
impl crate::Resettable for OBRrs {
    const RESET_VALUE: u32 = 0xffff_ff0f;
}
