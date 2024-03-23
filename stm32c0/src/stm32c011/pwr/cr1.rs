#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Field `LPMS` reader - Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode"]
pub type LPMS_R = crate::FieldReader;
#[doc = "Field `LPMS` writer - Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode"]
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FPD_STOP` reader - Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode."]
pub type FPD_STOP_R = crate::BitReader;
#[doc = "Field `FPD_STOP` writer - Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode."]
pub type FPD_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPD_SLP` reader - Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode."]
pub type FPD_SLP_R = crate::BitReader;
#[doc = "Field `FPD_SLP` writer - Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode."]
pub type FPD_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode."]
    #[inline(always)]
    pub fn fpd_stop(&self) -> FPD_STOP_R {
        FPD_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode."]
    #[inline(always)]
    pub fn fpd_slp(&self) -> FPD_SLP_R {
        FPD_SLP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<CR1rs> {
        LPMS_W::new(self, 0)
    }
    #[doc = "Bit 3 - Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode."]
    #[inline(always)]
    #[must_use]
    pub fn fpd_stop(&mut self) -> FPD_STOP_W<CR1rs> {
        FPD_STOP_W::new(self, 3)
    }
    #[doc = "Bit 5 - Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn fpd_slp(&mut self) -> FPD_SLP_W<CR1rs> {
        FPD_SLP_W::new(self, 5)
    }
}
#[doc = "PWR control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR1 to value 0x0208"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x0208;
}
