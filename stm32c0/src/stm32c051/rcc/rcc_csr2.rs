///Register `RCC_CSR2` reader
pub type R = crate::R<RCC_CSR2rs>;
///Register `RCC_CSR2` writer
pub type W = crate::W<RCC_CSR2rs>;
/**LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<LSION> for bool {
    #[inline(always)]
    fn from(variant: LSION) -> Self {
        variant as u8 != 0
    }
}
///Field `LSION` reader - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
pub type LSION_R = crate::BitReader<LSION>;
impl LSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSION {
        match self.bits {
            false => LSION::B0x0,
            true => LSION::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSION::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSION::B0x1
    }
}
///Field `LSION` writer - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::B0x1)
    }
}
/**LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is ready (stable): After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDY {
    ///0: Not ready
    B0x0 = 0,
    ///1: Ready
    B0x1 = 1,
}
impl From<LSIRDY> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDY` reader - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is ready (stable): After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC.
pub type LSIRDY_R = crate::BitReader<LSIRDY>;
impl LSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDY {
        match self.bits {
            false => LSIRDY::B0x0,
            true => LSIRDY::B0x1,
        }
    }
    ///Not ready
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSIRDY::B0x0
    }
    ///Ready
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSIRDY::B0x1
    }
}
/**Remove reset flags Set by software to clear the reset flags.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF {
    ///0: No effect
    B0x0 = 0,
    ///1: Clear reset flags
    B0x1 = 1,
}
impl From<RMVF> for bool {
    #[inline(always)]
    fn from(variant: RMVF) -> Self {
        variant as u8 != 0
    }
}
///Field `RMVF` reader - Remove reset flags Set by software to clear the reset flags.
pub type RMVF_R = crate::BitReader<RMVF>;
impl RMVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RMVF {
        match self.bits {
            false => RMVF::B0x0,
            true => RMVF::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RMVF::B0x0
    }
    ///Clear reset flags
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RMVF::B0x1
    }
}
///Field `RMVF` writer - Remove reset flags Set by software to clear the reset flags.
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVF>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::B0x0)
    }
    ///Clear reset flags
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::B0x1)
    }
}
/**Option byte loader reset flag Set by hardware when a reset from the Option byte loading occurs. Cleared by setting the RMVF bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTF {
    ///0: No reset from Option byte loading occurred
    B0x0 = 0,
    ///1: Reset from Option byte loading occurred
    B0x1 = 1,
}
impl From<OBLRSTF> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `OBLRSTF` reader - Option byte loader reset flag Set by hardware when a reset from the Option byte loading occurs. Cleared by setting the RMVF bit.
pub type OBLRSTF_R = crate::BitReader<OBLRSTF>;
impl OBLRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBLRSTF {
        match self.bits {
            false => OBLRSTF::B0x0,
            true => OBLRSTF::B0x1,
        }
    }
    ///No reset from Option byte loading occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OBLRSTF::B0x0
    }
    ///Reset from Option byte loading occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OBLRSTF::B0x1
    }
}
/**Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by setting the RMVF bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINRSTF {
    ///0: No reset from NRST pin occurred
    B0x0 = 0,
    ///1: Reset from NRST pin occurred
    B0x1 = 1,
}
impl From<PINRSTF> for bool {
    #[inline(always)]
    fn from(variant: PINRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `PINRSTF` reader - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by setting the RMVF bit.
pub type PINRSTF_R = crate::BitReader<PINRSTF>;
impl PINRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PINRSTF {
        match self.bits {
            false => PINRSTF::B0x0,
            true => PINRSTF::B0x1,
        }
    }
    ///No reset from NRST pin occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PINRSTF::B0x0
    }
    ///Reset from NRST pin occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PINRSTF::B0x1
    }
}
/**BOR or POR/PDR flag Set by hardware when a BOR or POR/PDR occurs. Cleared by setting the RMVF bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRRSTF {
    ///0: No BOR or POR occurred
    B0x0 = 0,
    ///1: BOR or POR occurred
    B0x1 = 1,
}
impl From<PWRRSTF> for bool {
    #[inline(always)]
    fn from(variant: PWRRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `PWRRSTF` reader - BOR or POR/PDR flag Set by hardware when a BOR or POR/PDR occurs. Cleared by setting the RMVF bit.
pub type PWRRSTF_R = crate::BitReader<PWRRSTF>;
impl PWRRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PWRRSTF {
        match self.bits {
            false => PWRRSTF::B0x0,
            true => PWRRSTF::B0x1,
        }
    }
    ///No BOR or POR occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PWRRSTF::B0x0
    }
    ///BOR or POR occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PWRRSTF::B0x1
    }
}
/**Software reset flag Set by hardware when a software reset occurs. Cleared by setting the RMVF bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFTRSTF {
    ///0: No software reset occurred
    B0x0 = 0,
    ///1: Software reset occurred
    B0x1 = 1,
}
impl From<SFTRSTF> for bool {
    #[inline(always)]
    fn from(variant: SFTRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `SFTRSTF` reader - Software reset flag Set by hardware when a software reset occurs. Cleared by setting the RMVF bit.
pub type SFTRSTF_R = crate::BitReader<SFTRSTF>;
impl SFTRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SFTRSTF {
        match self.bits {
            false => SFTRSTF::B0x0,
            true => SFTRSTF::B0x1,
        }
    }
    ///No software reset occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SFTRSTF::B0x0
    }
    ///Software reset occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SFTRSTF::B0x1
    }
}
/**Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by setting the RMVF bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDGRSTF {
    ///0: No independent watchdog reset occurred
    B0x0 = 0,
    ///1: Independent watchdog reset occurred
    B0x1 = 1,
}
impl From<IWDGRSTF> for bool {
    #[inline(always)]
    fn from(variant: IWDGRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDGRSTF` reader - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by setting the RMVF bit.
pub type IWDGRSTF_R = crate::BitReader<IWDGRSTF>;
impl IWDGRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDGRSTF {
        match self.bits {
            false => IWDGRSTF::B0x0,
            true => IWDGRSTF::B0x1,
        }
    }
    ///No independent watchdog reset occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IWDGRSTF::B0x0
    }
    ///Independent watchdog reset occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IWDGRSTF::B0x1
    }
}
/**Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by setting the RMVF bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGRSTF {
    ///0: No window watchdog reset occurred
    B0x0 = 0,
    ///1: Window watchdog reset occurred
    B0x1 = 1,
}
impl From<WWDGRSTF> for bool {
    #[inline(always)]
    fn from(variant: WWDGRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `WWDGRSTF` reader - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by setting the RMVF bit.
pub type WWDGRSTF_R = crate::BitReader<WWDGRSTF>;
impl WWDGRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WWDGRSTF {
        match self.bits {
            false => WWDGRSTF::B0x0,
            true => WWDGRSTF::B0x1,
        }
    }
    ///No window watchdog reset occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WWDGRSTF::B0x0
    }
    ///Window watchdog reset occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WWDGRSTF::B0x1
    }
}
/**Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, or Standby, or Shutdown mode entry. Cleared by setting the RMVF bit. This operates only if nRST_STOP, or nRST_STDBY or nRST_SHDW option bits are cleared.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPWRRSTF {
    ///0: No illegal mode reset occurred
    B0x0 = 0,
    ///1: Illegal mode reset occurred
    B0x1 = 1,
}
impl From<LPWRRSTF> for bool {
    #[inline(always)]
    fn from(variant: LPWRRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `LPWRRSTF` reader - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, or Standby, or Shutdown mode entry. Cleared by setting the RMVF bit. This operates only if nRST_STOP, or nRST_STDBY or nRST_SHDW option bits are cleared.
pub type LPWRRSTF_R = crate::BitReader<LPWRRSTF>;
impl LPWRRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPWRRSTF {
        match self.bits {
            false => LPWRRSTF::B0x0,
            true => LPWRRSTF::B0x1,
        }
    }
    ///No illegal mode reset occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPWRRSTF::B0x0
    }
    ///Illegal mode reset occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPWRRSTF::B0x1
    }
}
impl R {
    ///Bit 0 - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is ready (stable): After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC.
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 23 - Remove reset flags Set by software to clear the reset flags.
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Option byte loader reset flag Set by hardware when a reset from the Option byte loading occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BOR or POR/PDR flag Set by hardware when a BOR or POR/PDR occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn pwrrstf(&self) -> PWRRSTF_R {
        PWRRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag Set by hardware when a software reset occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, or Standby, or Shutdown mode entry. Cleared by setting the RMVF bit. This operates only if nRST_STOP, or nRST_STDBY or nRST_SHDW option bits are cleared.
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CSR2")
            .field("lsion", &self.lsion())
            .field("lsirdy", &self.lsirdy())
            .field("rmvf", &self.rmvf())
            .field("oblrstf", &self.oblrstf())
            .field("pinrstf", &self.pinrstf())
            .field("pwrrstf", &self.pwrrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<'_, RCC_CSR2rs> {
        LSION_W::new(self, 0)
    }
    ///Bit 23 - Remove reset flags Set by software to clear the reset flags.
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, RCC_CSR2rs> {
        RMVF_W::new(self, 23)
    }
}
/**RCC control/status register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_csr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_csr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RCC:RCC_CSR2)*/
pub struct RCC_CSR2rs;
impl crate::RegisterSpec for RCC_CSR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_csr2::R`](R) reader structure
impl crate::Readable for RCC_CSR2rs {}
///`write(|w| ..)` method takes [`rcc_csr2::W`](W) writer structure
impl crate::Writable for RCC_CSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_CSR2 to value 0
impl crate::Resettable for RCC_CSR2rs {}
