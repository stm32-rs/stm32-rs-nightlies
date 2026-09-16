///Register `RTC_ICSR` reader
pub type R = crate::R<RTC_ICSRrs>;
///Register `RTC_ICSR` writer
pub type W = crate::W<RTC_ICSRrs>;
/**Alarm A write flag This bit is set by hardware when alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAWF {
    ///0: Alarm A update not allowed
    B0x0 = 0,
    ///1: Alarm A update allowed
    B0x1 = 1,
}
impl From<ALRAWF> for bool {
    #[inline(always)]
    fn from(variant: ALRAWF) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRAWF` reader - Alarm A write flag This bit is set by hardware when alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode.
pub type ALRAWF_R = crate::BitReader<ALRAWF>;
impl ALRAWF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALRAWF {
        match self.bits {
            false => ALRAWF::B0x0,
            true => ALRAWF::B0x1,
        }
    }
    ///Alarm A update not allowed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ALRAWF::B0x0
    }
    ///Alarm A update allowed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ALRAWF::B0x1
    }
}
/**Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHPF {
    ///0: No shift operation is pending
    B0x0 = 0,
    ///1: A shift operation is pending
    B0x1 = 1,
}
impl From<SHPF> for bool {
    #[inline(always)]
    fn from(variant: SHPF) -> Self {
        variant as u8 != 0
    }
}
///Field `SHPF` reader - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.
pub type SHPF_R = crate::BitReader<SHPF>;
impl SHPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHPF {
        match self.bits {
            false => SHPF::B0x0,
            true => SHPF::B0x1,
        }
    }
    ///No shift operation is pending
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SHPF::B0x0
    }
    ///A shift operation is pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SHPF::B0x1
    }
}
/**Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Power-on reset state).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITS {
    ///0: Calendar has not been initialized
    B0x0 = 0,
    ///1: Calendar has been initialized
    B0x1 = 1,
}
impl From<INITS> for bool {
    #[inline(always)]
    fn from(variant: INITS) -> Self {
        variant as u8 != 0
    }
}
///Field `INITS` reader - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Power-on reset state).
pub type INITS_R = crate::BitReader<INITS>;
impl INITS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INITS {
        match self.bits {
            false => INITS::B0x0,
            true => INITS::B0x1,
        }
    }
    ///Calendar has not been initialized
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == INITS::B0x0
    }
    ///Calendar has been initialized
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == INITS::B0x1
    }
}
/**Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSF {
    ///0: Calendar shadow registers not yet synchronized
    B0x0 = 0,
    ///1: Calendar shadow registers synchronized
    B0x1 = 1,
}
impl From<RSF> for bool {
    #[inline(always)]
    fn from(variant: RSF) -> Self {
        variant as u8 != 0
    }
}
///Field `RSF` reader - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.
pub type RSF_R = crate::BitReader<RSF>;
impl RSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSF {
        match self.bits {
            false => RSF::B0x0,
            true => RSF::B0x1,
        }
    }
    ///Calendar shadow registers not yet synchronized
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RSF::B0x0
    }
    ///Calendar shadow registers synchronized
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RSF::B0x1
    }
}
///Field `RSF` writer - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG, RSF>;
impl<'a, REG> RSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calendar shadow registers not yet synchronized
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RSF::B0x0)
    }
    ///Calendar shadow registers synchronized
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RSF::B0x1)
    }
}
/**Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITF {
    ///0: Calendar registers update is not allowed
    B0x0 = 0,
    ///1: Calendar registers update is allowed
    B0x1 = 1,
}
impl From<INITF> for bool {
    #[inline(always)]
    fn from(variant: INITF) -> Self {
        variant as u8 != 0
    }
}
///Field `INITF` reader - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated.
pub type INITF_R = crate::BitReader<INITF>;
impl INITF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INITF {
        match self.bits {
            false => INITF::B0x0,
            true => INITF::B0x1,
        }
    }
    ///Calendar registers update is not allowed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == INITF::B0x0
    }
    ///Calendar registers update is allowed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == INITF::B0x1
    }
}
/**Initialization mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT {
    ///0: Free running mode
    B0x0 = 0,
    ///1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    B0x1 = 1,
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
            false => INIT::B0x0,
            true => INIT::B0x1,
        }
    }
    ///Free running mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == INIT::B0x0
    }
    ///Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == INIT::B0x1
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
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(INIT::B0x0)
    }
    ///Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(INIT::B0x1)
    }
}
///Field `RECALPF` reader - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to Re-calibration on-the-fly.
pub type RECALPF_R = crate::BitReader;
impl R {
    ///Bit 0 - Alarm A write flag This bit is set by hardware when alarm A values can be changed, after the ALRAE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode.
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect.
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Power-on reset state).
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated.
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to Re-calibration on-the-fly.
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_ICSR")
            .field("alrawf", &self.alrawf())
            .field("shpf", &self.shpf())
            .field("inits", &self.inits())
            .field("rsf", &self.rsf())
            .field("initf", &self.initf())
            .field("init", &self.init())
            .field("recalpf", &self.recalpf())
            .finish()
    }
}
impl W {
    ///Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode.
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, RTC_ICSRrs> {
        RSF_W::new(self, 5)
    }
    ///Bit 7 - Initialization mode
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, RTC_ICSRrs> {
        INIT_W::new(self, 7)
    }
}
/**RTC initialization control and status register

You can [`read`](crate::Reg::read) this register and get [`rtc_icsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_icsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#RTC:RTC_ICSR)*/
pub struct RTC_ICSRrs;
impl crate::RegisterSpec for RTC_ICSRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_icsr::R`](R) reader structure
impl crate::Readable for RTC_ICSRrs {}
///`write(|w| ..)` method takes [`rtc_icsr::W`](W) writer structure
impl crate::Writable for RTC_ICSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTC_ICSR to value 0x07
impl crate::Resettable for RTC_ICSRrs {
    const RESET_VALUE: u32 = 0x07;
}
