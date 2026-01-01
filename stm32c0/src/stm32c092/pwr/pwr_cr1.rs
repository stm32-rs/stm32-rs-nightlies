///Register `PWR_CR1` reader
pub type R = crate::R<PWR_CR1rs>;
///Register `PWR_CR1` writer
pub type W = crate::W<PWR_CR1rs>;
/**Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPMS {
    ///0: Stop mode
    B0x0 = 0,
    ///3: Standby mode
    B0x3 = 3,
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
///Field `LPMS` reader - Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode
pub type LPMS_R = crate::FieldReader<LPMS>;
impl LPMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPMS> {
        match self.bits {
            0 => Some(LPMS::B0x0),
            3 => Some(LPMS::B0x3),
            _ => None,
        }
    }
    ///Stop mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPMS::B0x0
    }
    ///Standby mode
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LPMS::B0x3
    }
}
///Field `LPMS` writer - Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPMS>;
impl<'a, REG> LPMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Stop mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::B0x0)
    }
    ///Standby mode
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::B0x3)
    }
}
/**Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPD_STOP {
    ///0: Flash memory idle
    B0x0 = 0,
    ///1: Flash memory powered down
    B0x1 = 1,
}
impl From<FPD_STOP> for bool {
    #[inline(always)]
    fn from(variant: FPD_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `FPD_STOP` reader - Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode.
pub type FPD_STOP_R = crate::BitReader<FPD_STOP>;
impl FPD_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPD_STOP {
        match self.bits {
            false => FPD_STOP::B0x0,
            true => FPD_STOP::B0x1,
        }
    }
    ///Flash memory idle
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FPD_STOP::B0x0
    }
    ///Flash memory powered down
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FPD_STOP::B0x1
    }
}
///Field `FPD_STOP` writer - Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode.
pub type FPD_STOP_W<'a, REG> = crate::BitWriter<'a, REG, FPD_STOP>;
impl<'a, REG> FPD_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash memory idle
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPD_STOP::B0x0)
    }
    ///Flash memory powered down
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPD_STOP::B0x1)
    }
}
/**Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPD_SLP {
    ///0: Flash memory idle
    B0x0 = 0,
    ///1: Flash memory powered down
    B0x1 = 1,
}
impl From<FPD_SLP> for bool {
    #[inline(always)]
    fn from(variant: FPD_SLP) -> Self {
        variant as u8 != 0
    }
}
///Field `FPD_SLP` reader - Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode.
pub type FPD_SLP_R = crate::BitReader<FPD_SLP>;
impl FPD_SLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPD_SLP {
        match self.bits {
            false => FPD_SLP::B0x0,
            true => FPD_SLP::B0x1,
        }
    }
    ///Flash memory idle
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FPD_SLP::B0x0
    }
    ///Flash memory powered down
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FPD_SLP::B0x1
    }
}
///Field `FPD_SLP` writer - Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode.
pub type FPD_SLP_W<'a, REG> = crate::BitWriter<'a, REG, FPD_SLP>;
impl<'a, REG> FPD_SLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash memory idle
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FPD_SLP::B0x0)
    }
    ///Flash memory powered down
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPD_SLP::B0x1)
    }
}
impl R {
    ///Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode.
    #[inline(always)]
    pub fn fpd_stop(&self) -> FPD_STOP_R {
        FPD_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode.
    #[inline(always)]
    pub fn fpd_slp(&self) -> FPD_SLP_R {
        FPD_SLP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR1")
            .field("lpms", &self.lpms())
            .field("fpd_stop", &self.fpd_stop())
            .field("fpd_slp", &self.fpd_slp())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<'_, PWR_CR1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bit 3 - Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode.
    #[inline(always)]
    pub fn fpd_stop(&mut self) -> FPD_STOP_W<'_, PWR_CR1rs> {
        FPD_STOP_W::new(self, 3)
    }
    ///Bit 5 - Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode.
    #[inline(always)]
    pub fn fpd_slp(&mut self) -> FPD_SLP_W<'_, PWR_CR1rs> {
        FPD_SLP_W::new(self, 5)
    }
}
/**PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#PWR:PWR_CR1)*/
pub struct PWR_CR1rs;
impl crate::RegisterSpec for PWR_CR1rs {
    type Ux = u32;
}
///`read()` method returns [`pwr_cr1::R`](R) reader structure
impl crate::Readable for PWR_CR1rs {}
///`write(|w| ..)` method takes [`pwr_cr1::W`](W) writer structure
impl crate::Writable for PWR_CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWR_CR1 to value 0x0208
impl crate::Resettable for PWR_CR1rs {
    const RESET_VALUE: u32 = 0x0208;
}
