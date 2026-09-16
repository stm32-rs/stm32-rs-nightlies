///Register `PWR_SR1` reader
pub type R = crate::R<PWR_SR1rs>;
///Field `WUF1` reader - Wakeup flag 1 This bit is set when a wakeup condition is detected on WKUP1 wakeup pin. It is cleared by setting the CWUF1 bit of the PWR_SCR register.
pub type WUF1_R = crate::BitReader;
///Field `WUF2` reader - Wakeup flag 2 This bit is set when a wakeup condition is detected on WKUP2 wakeup pin. It is cleared by setting the CWUF2 bit of the PWR_SCR register.
pub type WUF2_R = crate::BitReader;
///Field `WUF3` reader - Wakeup flag 3 This bit is set when a wakeup condition is detected on WKUP3 wakeup pin. It is cleared by setting the CWUF3 bit of the PWR_SCR register.
pub type WUF3_R = crate::BitReader;
///Field `WUF4` reader - Wakeup flag 4 This bit is set when a wakeup condition is detected on WKUP4 wakeup pin. It is cleared by setting the CWUF4 bit of the PWR_SCR register.
pub type WUF4_R = crate::BitReader;
///Field `WUF5` reader - Wakeup flag 5 This bit is set when a wakeup condition is detected on WKUP5 wakeup pin. It is cleared by setting the CWUF5 bit of the PWR_SCR register.
pub type WUF5_R = crate::BitReader;
///Field `WUF6` reader - Wakeup flag 6 This bit is set when a wakeup condition is detected on WKUP6 wakeup pin. It is cleared by setting the CWUF6 bit of the PWR_SCR register.
pub type WUF6_R = crate::BitReader;
/**Standby flag This bit is set by hardware when the device enters Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBF {
    ///0: The device did not enter Standby mode
    B0x0 = 0,
    ///1: The device entered Standby mode
    B0x1 = 1,
}
impl From<SBF> for bool {
    #[inline(always)]
    fn from(variant: SBF) -> Self {
        variant as u8 != 0
    }
}
///Field `SBF` reader - Standby flag This bit is set by hardware when the device enters Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset.
pub type SBF_R = crate::BitReader<SBF>;
impl SBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBF {
        match self.bits {
            false => SBF::B0x0,
            true => SBF::B0x1,
        }
    }
    ///The device did not enter Standby mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SBF::B0x0
    }
    ///The device entered Standby mode
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SBF::B0x1
    }
}
///Field `WUFI` reader - Wakeup flag internal This bit is set when a wakeup condition is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared.
pub type WUFI_R = crate::BitReader;
impl R {
    ///Bit 0 - Wakeup flag 1 This bit is set when a wakeup condition is detected on WKUP1 wakeup pin. It is cleared by setting the CWUF1 bit of the PWR_SCR register.
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup flag 2 This bit is set when a wakeup condition is detected on WKUP2 wakeup pin. It is cleared by setting the CWUF2 bit of the PWR_SCR register.
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup flag 3 This bit is set when a wakeup condition is detected on WKUP3 wakeup pin. It is cleared by setting the CWUF3 bit of the PWR_SCR register.
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup flag 4 This bit is set when a wakeup condition is detected on WKUP4 wakeup pin. It is cleared by setting the CWUF4 bit of the PWR_SCR register.
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup flag 5 This bit is set when a wakeup condition is detected on WKUP5 wakeup pin. It is cleared by setting the CWUF5 bit of the PWR_SCR register.
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wakeup flag 6 This bit is set when a wakeup condition is detected on WKUP6 wakeup pin. It is cleared by setting the CWUF6 bit of the PWR_SCR register.
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Standby flag This bit is set by hardware when the device enters Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset.
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - Wakeup flag internal This bit is set when a wakeup condition is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared.
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_SR1")
            .field("wuf1", &self.wuf1())
            .field("wuf2", &self.wuf2())
            .field("wuf3", &self.wuf3())
            .field("wuf4", &self.wuf4())
            .field("wuf5", &self.wuf5())
            .field("wuf6", &self.wuf6())
            .field("sbf", &self.sbf())
            .field("wufi", &self.wufi())
            .finish()
    }
}
/**PWR status register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_SR1)*/
pub struct PWR_SR1rs;
impl crate::RegisterSpec for PWR_SR1rs {
    type Ux = u32;
}
///`read()` method returns [`pwr_sr1::R`](R) reader structure
impl crate::Readable for PWR_SR1rs {}
///`reset()` method sets PWR_SR1 to value 0
impl crate::Resettable for PWR_SR1rs {}
