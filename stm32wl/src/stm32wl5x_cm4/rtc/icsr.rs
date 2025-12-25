///Register `ICSR` reader
pub type R = crate::R<ICSRrs>;
///Register `ICSR` writer
pub type W = crate::W<ICSRrs>;
/**Wakeup timer write flag

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTWFR {
    ///0: Wakeup timer configuration update not allowed
    UpdateNotAllowed = 0,
    ///1: Wakeup timer configuration update allowed
    UpdateAllowed = 1,
}
impl From<WUTWFR> for bool {
    #[inline(always)]
    fn from(variant: WUTWFR) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTWF` reader - Wakeup timer write flag
pub type WUTWF_R = crate::BitReader<WUTWFR>;
impl WUTWF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUTWFR {
        match self.bits {
            false => WUTWFR::UpdateNotAllowed,
            true => WUTWFR::UpdateAllowed,
        }
    }
    ///Wakeup timer configuration update not allowed
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == WUTWFR::UpdateNotAllowed
    }
    ///Wakeup timer configuration update allowed
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == WUTWFR::UpdateAllowed
    }
}
/**Shift operation pending

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHPFR {
    ///0: No shift operation is pending
    NoShiftPending = 0,
    ///1: A shift operation is pending
    ShiftPending = 1,
}
impl From<SHPFR> for bool {
    #[inline(always)]
    fn from(variant: SHPFR) -> Self {
        variant as u8 != 0
    }
}
///Field `SHPF` reader - Shift operation pending
pub type SHPF_R = crate::BitReader<SHPFR>;
impl SHPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHPFR {
        match self.bits {
            false => SHPFR::NoShiftPending,
            true => SHPFR::ShiftPending,
        }
    }
    ///No shift operation is pending
    #[inline(always)]
    pub fn is_no_shift_pending(&self) -> bool {
        *self == SHPFR::NoShiftPending
    }
    ///A shift operation is pending
    #[inline(always)]
    pub fn is_shift_pending(&self) -> bool {
        *self == SHPFR::ShiftPending
    }
}
/**Initialization status flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITSR {
    ///0: Calendar has not been initialized
    NotInitalized = 0,
    ///1: Calendar has been initialized
    Initalized = 1,
}
impl From<INITSR> for bool {
    #[inline(always)]
    fn from(variant: INITSR) -> Self {
        variant as u8 != 0
    }
}
///Field `INITS` reader - Initialization status flag
pub type INITS_R = crate::BitReader<INITSR>;
impl INITS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INITSR {
        match self.bits {
            false => INITSR::NotInitalized,
            true => INITSR::Initalized,
        }
    }
    ///Calendar has not been initialized
    #[inline(always)]
    pub fn is_not_initalized(&self) -> bool {
        *self == INITSR::NotInitalized
    }
    ///Calendar has been initialized
    #[inline(always)]
    pub fn is_initalized(&self) -> bool {
        *self == INITSR::Initalized
    }
}
/**Registers synchronization flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFR {
    ///0: Calendar shadow registers not yet synchronized
    NotSynced = 0,
    ///1: Calendar shadow registers synchronized
    Synced = 1,
}
impl From<RSFR> for bool {
    #[inline(always)]
    fn from(variant: RSFR) -> Self {
        variant as u8 != 0
    }
}
///Field `RSF` reader - Registers synchronization flag
pub type RSF_R = crate::BitReader<RSFR>;
impl RSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSFR {
        match self.bits {
            false => RSFR::NotSynced,
            true => RSFR::Synced,
        }
    }
    ///Calendar shadow registers not yet synchronized
    #[inline(always)]
    pub fn is_not_synced(&self) -> bool {
        *self == RSFR::NotSynced
    }
    ///Calendar shadow registers synchronized
    #[inline(always)]
    pub fn is_synced(&self) -> bool {
        *self == RSFR::Synced
    }
}
/**Registers synchronization flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFW {
    ///0: This flag is cleared by software by writing 0
    Clear = 0,
}
impl From<RSFW> for bool {
    #[inline(always)]
    fn from(variant: RSFW) -> Self {
        variant as u8 != 0
    }
}
///Field `RSF` writer - Registers synchronization flag
pub type RSF_W<'a, REG> = crate::BitWriter0C<'a, REG, RSFW>;
impl<'a, REG> RSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This flag is cleared by software by writing 0
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RSFW::Clear)
    }
}
/**Initialization flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITFR {
    ///0: Calendar registers update is not allowed
    NotAllowed = 0,
    ///1: Calendar registers update is allowed
    Allowed = 1,
}
impl From<INITFR> for bool {
    #[inline(always)]
    fn from(variant: INITFR) -> Self {
        variant as u8 != 0
    }
}
///Field `INITF` reader - Initialization flag
pub type INITF_R = crate::BitReader<INITFR>;
impl INITF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INITFR {
        match self.bits {
            false => INITFR::NotAllowed,
            true => INITFR::Allowed,
        }
    }
    ///Calendar registers update is not allowed
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == INITFR::NotAllowed
    }
    ///Calendar registers update is allowed
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == INITFR::Allowed
    }
}
/**Initialization mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT {
    ///0: Free running mode
    FreeRunningMode = 0,
    ///1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    InitMode = 1,
}
impl From<INIT> for bool {
    #[inline(always)]
    fn from(variant: INIT) -> Self {
        variant as u8 != 0
    }
}
///Field `INIT` reader - Initialization mode
pub type INIT_R = crate::BitReader<INIT>;
impl INIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INIT {
        match self.bits {
            false => INIT::FreeRunningMode,
            true => INIT::InitMode,
        }
    }
    ///Free running mode
    #[inline(always)]
    pub fn is_free_running_mode(&self) -> bool {
        *self == INIT::FreeRunningMode
    }
    ///Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    #[inline(always)]
    pub fn is_init_mode(&self) -> bool {
        *self == INIT::InitMode
    }
}
///Field `INIT` writer - Initialization mode
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG, INIT>;
impl<'a, REG> INIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Free running mode
    #[inline(always)]
    pub fn free_running_mode(self) -> &'a mut crate::W<REG> {
        self.variant(INIT::FreeRunningMode)
    }
    ///Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    #[inline(always)]
    pub fn init_mode(self) -> &'a mut crate::W<REG> {
        self.variant(INIT::InitMode)
    }
}
/**Binary mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BIN {
    ///0: Free running BCD calendar mode (Binary mode disabled)
    Bcd = 0,
    ///1: Free running Binary mode (BCD mode disabled)
    Binary = 1,
    ///2: Free running BCD calendar and Binary modes
    BinBcd = 2,
    ///3: Free running BCD calendar and Binary modes
    BinBcd2 = 3,
}
impl From<BIN> for u8 {
    #[inline(always)]
    fn from(variant: BIN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BIN {
    type Ux = u8;
}
impl crate::IsEnum for BIN {}
///Field `BIN` reader - Binary mode
pub type BIN_R = crate::FieldReader<BIN>;
impl BIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIN {
        match self.bits {
            0 => BIN::Bcd,
            1 => BIN::Binary,
            2 => BIN::BinBcd,
            3 => BIN::BinBcd2,
            _ => unreachable!(),
        }
    }
    ///Free running BCD calendar mode (Binary mode disabled)
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        *self == BIN::Bcd
    }
    ///Free running Binary mode (BCD mode disabled)
    #[inline(always)]
    pub fn is_binary(&self) -> bool {
        *self == BIN::Binary
    }
    ///Free running BCD calendar and Binary modes
    #[inline(always)]
    pub fn is_bin_bcd(&self) -> bool {
        *self == BIN::BinBcd
    }
    ///Free running BCD calendar and Binary modes
    #[inline(always)]
    pub fn is_bin_bcd2(&self) -> bool {
        *self == BIN::BinBcd2
    }
}
///Field `BIN` writer - Binary mode
pub type BIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BIN, crate::Safe>;
impl<'a, REG> BIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Free running BCD calendar mode (Binary mode disabled)
    #[inline(always)]
    pub fn bcd(self) -> &'a mut crate::W<REG> {
        self.variant(BIN::Bcd)
    }
    ///Free running Binary mode (BCD mode disabled)
    #[inline(always)]
    pub fn binary(self) -> &'a mut crate::W<REG> {
        self.variant(BIN::Binary)
    }
    ///Free running BCD calendar and Binary modes
    #[inline(always)]
    pub fn bin_bcd(self) -> &'a mut crate::W<REG> {
        self.variant(BIN::BinBcd)
    }
    ///Free running BCD calendar and Binary modes
    #[inline(always)]
    pub fn bin_bcd2(self) -> &'a mut crate::W<REG> {
        self.variant(BIN::BinBcd2)
    }
}
/**BCD update

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCDU {
    ///0: 1s increment each time SS\[7:0\]=0
    Bit7 = 0,
    ///1: 1s increment each time SS\[8:0\]=0
    Bit8 = 1,
    ///2: 1s increment each time SS\[9:0\]=0
    Bit9 = 2,
    ///3: 1s increment each time SS\[10:0\]=0
    Bit10 = 3,
    ///4: 1s increment each time SS\[11:0\]=0
    Bit11 = 4,
    ///5: 1s increment each time SS\[12:0\]=0
    Bit12 = 5,
    ///6: 1s increment each time SS\[13:0\]=0
    Bit13 = 6,
    ///7: 1s increment each time SS\[14:0\]=0
    Bit14 = 7,
}
impl From<BCDU> for u8 {
    #[inline(always)]
    fn from(variant: BCDU) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BCDU {
    type Ux = u8;
}
impl crate::IsEnum for BCDU {}
///Field `BCDU` reader - BCD update
pub type BCDU_R = crate::FieldReader<BCDU>;
impl BCDU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCDU {
        match self.bits {
            0 => BCDU::Bit7,
            1 => BCDU::Bit8,
            2 => BCDU::Bit9,
            3 => BCDU::Bit10,
            4 => BCDU::Bit11,
            5 => BCDU::Bit12,
            6 => BCDU::Bit13,
            7 => BCDU::Bit14,
            _ => unreachable!(),
        }
    }
    ///1s increment each time SS\[7:0\]=0
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == BCDU::Bit7
    }
    ///1s increment each time SS\[8:0\]=0
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == BCDU::Bit8
    }
    ///1s increment each time SS\[9:0\]=0
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == BCDU::Bit9
    }
    ///1s increment each time SS\[10:0\]=0
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == BCDU::Bit10
    }
    ///1s increment each time SS\[11:0\]=0
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == BCDU::Bit11
    }
    ///1s increment each time SS\[12:0\]=0
    #[inline(always)]
    pub fn is_bit12(&self) -> bool {
        *self == BCDU::Bit12
    }
    ///1s increment each time SS\[13:0\]=0
    #[inline(always)]
    pub fn is_bit13(&self) -> bool {
        *self == BCDU::Bit13
    }
    ///1s increment each time SS\[14:0\]=0
    #[inline(always)]
    pub fn is_bit14(&self) -> bool {
        *self == BCDU::Bit14
    }
}
///Field `BCDU` writer - BCD update
pub type BCDU_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BCDU, crate::Safe>;
impl<'a, REG> BCDU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1s increment each time SS\[7:0\]=0
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU::Bit7)
    }
    ///1s increment each time SS\[8:0\]=0
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU::Bit8)
    }
    ///1s increment each time SS\[9:0\]=0
    #[inline(always)]
    pub fn bit9(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU::Bit9)
    }
    ///1s increment each time SS\[10:0\]=0
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU::Bit10)
    }
    ///1s increment each time SS\[11:0\]=0
    #[inline(always)]
    pub fn bit11(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU::Bit11)
    }
    ///1s increment each time SS\[12:0\]=0
    #[inline(always)]
    pub fn bit12(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU::Bit12)
    }
    ///1s increment each time SS\[13:0\]=0
    #[inline(always)]
    pub fn bit13(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU::Bit13)
    }
    ///1s increment each time SS\[14:0\]=0
    #[inline(always)]
    pub fn bit14(self) -> &'a mut crate::W<REG> {
        self.variant(BCDU::Bit14)
    }
}
/**Recalibration pending Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RECALPFR {
    ///1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0
    Pending = 1,
}
impl From<RECALPFR> for bool {
    #[inline(always)]
    fn from(variant: RECALPFR) -> Self {
        variant as u8 != 0
    }
}
///Field `RECALPF` reader - Recalibration pending Flag
pub type RECALPF_R = crate::BitReader<RECALPFR>;
impl RECALPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RECALPFR> {
        match self.bits {
            true => Some(RECALPFR::Pending),
            _ => None,
        }
    }
    ///The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECALPFR::Pending
    }
}
impl R {
    ///Bit 2 - Wakeup timer write flag
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shift operation pending
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Initialization status flag
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Initialization flag
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Binary mode
    #[inline(always)]
    pub fn bin(&self) -> BIN_R {
        BIN_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:12 - BCD update
    #[inline(always)]
    pub fn bcdu(&self) -> BCDU_R {
        BCDU_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 16 - Recalibration pending Flag
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSR")
            .field("recalpf", &self.recalpf())
            .field("bcdu", &self.bcdu())
            .field("bin", &self.bin())
            .field("init", &self.init())
            .field("initf", &self.initf())
            .field("rsf", &self.rsf())
            .field("inits", &self.inits())
            .field("shpf", &self.shpf())
            .field("wutwf", &self.wutwf())
            .finish()
    }
}
impl W {
    ///Bit 5 - Registers synchronization flag
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, ICSRrs> {
        RSF_W::new(self, 5)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, ICSRrs> {
        INIT_W::new(self, 7)
    }
    ///Bits 8:9 - Binary mode
    #[inline(always)]
    pub fn bin(&mut self) -> BIN_W<'_, ICSRrs> {
        BIN_W::new(self, 8)
    }
    ///Bits 10:12 - BCD update
    #[inline(always)]
    pub fn bcdu(&mut self) -> BCDU_W<'_, ICSRrs> {
        BCDU_W::new(self, 10)
    }
}
/**Initialization control and status register

You can [`read`](crate::Reg::read) this register and get [`icsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#RTC:ICSR)*/
pub struct ICSRrs;
impl crate::RegisterSpec for ICSRrs {
    type Ux = u32;
}
///`read()` method returns [`icsr::R`](R) reader structure
impl crate::Readable for ICSRrs {}
///`write(|w| ..)` method takes [`icsr::W`](W) writer structure
impl crate::Writable for ICSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x20;
}
///`reset()` method sets ICSR to value 0x07
impl crate::Resettable for ICSRrs {
    const RESET_VALUE: u32 = 0x07;
}
