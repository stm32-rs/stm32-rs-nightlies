#[doc = "Register `AHB4ENR` reader"]
pub type R = crate::R<AHB4ENRrs>;
#[doc = "Register `AHB4ENR` writer"]
pub type W = crate::W<AHB4ENRrs>;
#[doc = "0GPIO peripheral clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOAEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
#[doc = "Field `GPIOAEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
#[doc = "Field `GPIOBEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOBEN_R;
#[doc = "Field `GPIOCEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOCEN_R;
#[doc = "Field `GPIODEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIODEN_R;
#[doc = "Field `GPIOEEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOEEN_R;
#[doc = "Field `GPIOFEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOFEN_R;
#[doc = "Field `GPIOGEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOGEN_R;
#[doc = "Field `GPIOHEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOHEN_R;
#[doc = "Field `GPIOIEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOIEN_R;
#[doc = "Field `GPIOJEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOJEN_R;
#[doc = "Field `GPIOKEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOKEN_R;
#[doc = "Field `CRCEN` reader - CRC peripheral clock enable"]
pub use GPIOAEN_R as CRCEN_R;
#[doc = "Field `BDMAEN` reader - BDMA and DMAMUX2 Clock Enable"]
pub use GPIOAEN_R as BDMAEN_R;
#[doc = "Field `ADC3EN` reader - ADC3 Peripheral Clocks Enable"]
pub use GPIOAEN_R as ADC3EN_R;
#[doc = "Field `HSEMEN` reader - HSEM peripheral clock enable"]
pub use GPIOAEN_R as HSEMEN_R;
#[doc = "Field `BKPRAMEN` reader - Backup RAM Clock Enable"]
pub use GPIOAEN_R as BKPRAMEN_R;
#[doc = "Field `GPIOBEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOBEN_W;
#[doc = "Field `GPIOCEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOCEN_W;
#[doc = "Field `GPIODEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIODEN_W;
#[doc = "Field `GPIOEEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOEEN_W;
#[doc = "Field `GPIOFEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOFEN_W;
#[doc = "Field `GPIOGEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOGEN_W;
#[doc = "Field `GPIOHEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOHEN_W;
#[doc = "Field `GPIOIEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOIEN_W;
#[doc = "Field `GPIOJEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOJEN_W;
#[doc = "Field `GPIOKEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOKEN_W;
#[doc = "Field `CRCEN` writer - CRC peripheral clock enable"]
pub use GPIOAEN_W as CRCEN_W;
#[doc = "Field `BDMAEN` writer - BDMA and DMAMUX2 Clock Enable"]
pub use GPIOAEN_W as BDMAEN_W;
#[doc = "Field `ADC3EN` writer - ADC3 Peripheral Clocks Enable"]
pub use GPIOAEN_W as ADC3EN_W;
#[doc = "Field `HSEMEN` writer - HSEM peripheral clock enable"]
pub use GPIOAEN_W as HSEMEN_W;
#[doc = "Field `BKPRAMEN` writer - Backup RAM Clock Enable"]
pub use GPIOAEN_W as BKPRAMEN_W;
impl R {
    #[doc = "Bit 0 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioken(&self) -> GPIOKEN_R {
        GPIOKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - BDMA and DMAMUX2 Clock Enable"]
    #[inline(always)]
    pub fn bdmaen(&self) -> BDMAEN_R {
        BDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HSEM peripheral clock enable"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable"]
    #[inline(always)]
    pub fn bkpramen(&self) -> BKPRAMEN_R {
        BKPRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<AHB4ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<AHB4ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<AHB4ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<AHB4ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<AHB4ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<AHB4ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<AHB4ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<AHB4ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<AHB4ENRrs> {
        GPIOIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiojen(&mut self) -> GPIOJEN_W<AHB4ENRrs> {
        GPIOJEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioken(&mut self) -> GPIOKEN_W<AHB4ENRrs> {
        GPIOKEN_W::new(self, 10)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHB4ENRrs> {
        CRCEN_W::new(self, 19)
    }
    #[doc = "Bit 21 - BDMA and DMAMUX2 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bdmaen(&mut self) -> BDMAEN_W<AHB4ENRrs> {
        BDMAEN_W::new(self, 21)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3en(&mut self) -> ADC3EN_W<AHB4ENRrs> {
        ADC3EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - HSEM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsemen(&mut self) -> HSEMEN_W<AHB4ENRrs> {
        HSEMEN_W::new(self, 25)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpramen(&mut self) -> BKPRAMEN_W<AHB4ENRrs> {
        BKPRAMEN_W::new(self, 28)
    }
}
#[doc = "RCC AHB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB4ENRrs;
impl crate::RegisterSpec for AHB4ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb4enr::R`](R) reader structure"]
impl crate::Readable for AHB4ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb4enr::W`](W) writer structure"]
impl crate::Writable for AHB4ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB4ENR to value 0"]
impl crate::Resettable for AHB4ENRrs {
    const RESET_VALUE: u32 = 0;
}
