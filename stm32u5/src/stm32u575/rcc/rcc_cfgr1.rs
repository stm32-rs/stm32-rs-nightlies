///Register `RCC_CFGR1` reader
pub type R = crate::R<RCC_CFGR1rs>;
///Register `RCC_CFGR1` writer
pub type W = crate::W<RCC_CFGR1rs>;
///Field `SW` reader - system clock switch This bitfield is set and cleared by software to select system clock source (SYSCLK). It is configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. This bitfield is configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK.
pub type SW_R = crate::FieldReader;
///Field `SW` writer - system clock switch This bitfield is set and cleared by software to select system clock source (SYSCLK). It is configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. This bitfield is configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK.
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SWS` reader - system clock switch status This bitfield is set and cleared by hardware to indicate which clock source is used as system clock.
pub type SWS_R = crate::FieldReader;
///Field `STOPWUCK` reader - wake-up from Stop and CSS backup clock selection This bit is set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on�HSE. STOPWUCK must not be modified when the CSS is enabled by HSECSSON in�RCC_CR, and the system clock is HSE (SWS = 10) or a switch on HSE is�requested (SW�=�10).
pub type STOPWUCK_R = crate::BitReader;
///Field `STOPWUCK` writer - wake-up from Stop and CSS backup clock selection This bit is set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on�HSE. STOPWUCK must not be modified when the CSS is enabled by HSECSSON in�RCC_CR, and the system clock is HSE (SWS = 10) or a switch on HSE is�requested (SW�=�10).
pub type STOPWUCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPKERWUCK` reader - wake-up from Stop kernel clock automatic enable selection This bit is set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
pub type STOPKERWUCK_R = crate::BitReader;
///Field `STOPKERWUCK` writer - wake-up from Stop kernel clock automatic enable selection This bit is set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
pub type STOPKERWUCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCOSEL` reader - microcontroller clock output This bitfield is set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
pub type MCOSEL_R = crate::FieldReader;
///Field `MCOSEL` writer - microcontroller clock output This bitfield is set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCOPRE` reader - microcontroller clock output prescaler This bitfield is set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
pub type MCOPRE_R = crate::FieldReader;
///Field `MCOPRE` writer - microcontroller clock output prescaler This bitfield is set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
pub type MCOPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:1 - system clock switch This bitfield is set and cleared by software to select system clock source (SYSCLK). It is configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. This bitfield is configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK.
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - system clock switch status This bitfield is set and cleared by hardware to indicate which clock source is used as system clock.
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - wake-up from Stop and CSS backup clock selection This bit is set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on�HSE. STOPWUCK must not be modified when the CSS is enabled by HSECSSON in�RCC_CR, and the system clock is HSE (SWS = 10) or a switch on HSE is�requested (SW�=�10).
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - wake-up from Stop kernel clock automatic enable selection This bit is set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
    #[inline(always)]
    pub fn stopkerwuck(&self) -> STOPKERWUCK_R {
        STOPKERWUCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 24:27 - microcontroller clock output This bitfield is set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:30 - microcontroller clock output prescaler This bitfield is set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CFGR1")
            .field("sw", &self.sw())
            .field("sws", &self.sws())
            .field("stopwuck", &self.stopwuck())
            .field("stopkerwuck", &self.stopkerwuck())
            .field("mcosel", &self.mcosel())
            .field("mcopre", &self.mcopre())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - system clock switch This bitfield is set and cleared by software to select system clock source (SYSCLK). It is configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. This bitfield is configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK.
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<RCC_CFGR1rs> {
        SW_W::new(self, 0)
    }
    ///Bit 4 - wake-up from Stop and CSS backup clock selection This bit is set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on�HSE. STOPWUCK must not be modified when the CSS is enabled by HSECSSON in�RCC_CR, and the system clock is HSE (SWS = 10) or a switch on HSE is�requested (SW�=�10).
    #[inline(always)]
    #[must_use]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<RCC_CFGR1rs> {
        STOPWUCK_W::new(self, 4)
    }
    ///Bit 5 - wake-up from Stop kernel clock automatic enable selection This bit is set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.
    #[inline(always)]
    #[must_use]
    pub fn stopkerwuck(&mut self) -> STOPKERWUCK_W<RCC_CFGR1rs> {
        STOPKERWUCK_W::new(self, 5)
    }
    ///Bits 24:27 - microcontroller clock output This bitfield is set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> MCOSEL_W<RCC_CFGR1rs> {
        MCOSEL_W::new(self, 24)
    }
    ///Bits 28:30 - microcontroller clock output prescaler This bitfield is set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed
    #[inline(always)]
    #[must_use]
    pub fn mcopre(&mut self) -> MCOPRE_W<RCC_CFGR1rs> {
        MCOPRE_W::new(self, 28)
    }
}
/**RCC clock configuration register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RCC:RCC_CFGR1)*/
pub struct RCC_CFGR1rs;
impl crate::RegisterSpec for RCC_CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_cfgr1::R`](R) reader structure
impl crate::Readable for RCC_CFGR1rs {}
///`write(|w| ..)` method takes [`rcc_cfgr1::W`](W) writer structure
impl crate::Writable for RCC_CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CFGR1 to value 0
impl crate::Resettable for RCC_CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
