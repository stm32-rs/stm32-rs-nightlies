///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `LPMS` reader - Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode
pub type LPMS_R = crate::FieldReader;
///Field `LPMS` writer - Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FPD_STOP` reader - Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode.
pub type FPD_STOP_R = crate::BitReader;
///Field `FPD_STOP` writer - Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode.
pub type FPD_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPD_SLP` reader - Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode.
pub type FPD_SLP_R = crate::BitReader;
///Field `FPD_SLP` writer - Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode.
pub type FPD_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("CR1")
            .field("lpms", &self.lpms())
            .field("fpd_stop", &self.fpd_stop())
            .field("fpd_slp", &self.fpd_slp())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<CR1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bit 3 - Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode.
    #[inline(always)]
    pub fn fpd_stop(&mut self) -> FPD_STOP_W<CR1rs> {
        FPD_STOP_W::new(self, 3)
    }
    ///Bit 5 - Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode.
    #[inline(always)]
    pub fn fpd_slp(&mut self) -> FPD_SLP_W<CR1rs> {
        FPD_SLP_W::new(self, 5)
    }
}
/**PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#PWR:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR1 to value 0x0208
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x0208;
}
