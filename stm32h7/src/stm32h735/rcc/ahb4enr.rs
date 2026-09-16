///Register `AHB4ENR` reader
pub type R = crate::R<AHB4ENRrs>;
///Register `AHB4ENR` writer
pub type W = crate::W<AHB4ENRrs>;
/**0GPIO peripheral clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOAEN` reader - 0GPIO peripheral clock enable
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
///Field `GPIOAEN` writer - 0GPIO peripheral clock enable
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
///Field `GPIOBEN` reader - 0GPIO peripheral clock enable
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - 0GPIO peripheral clock enable
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - 0GPIO peripheral clock enable
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - 0GPIO peripheral clock enable
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - 0GPIO peripheral clock enable
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOGEN` reader - 0GPIO peripheral clock enable
pub use GPIOAEN_R as GPIOGEN_R;
///Field `GPIOHEN` reader - 0GPIO peripheral clock enable
pub use GPIOAEN_R as GPIOHEN_R;
///Field `GPIOJEN` reader - 0GPIO peripheral clock enable
pub use GPIOAEN_R as GPIOJEN_R;
///Field `GPIOKEN` reader - 0GPIO peripheral clock enable
pub use GPIOAEN_R as GPIOKEN_R;
///Field `CRCEN` reader - CRC peripheral clock enable
pub use GPIOAEN_R as CRCEN_R;
///Field `BDMAEN` reader - BDMA and DMAMUX2 Clock Enable
pub use GPIOAEN_R as BDMAEN_R;
///Field `ADC3EN` reader - ADC3 Peripheral Clocks Enable
pub use GPIOAEN_R as ADC3EN_R;
///Field `HSEMEN` reader - HSEM peripheral clock enable
pub use GPIOAEN_R as HSEMEN_R;
///Field `BKPRAMEN` reader - Backup RAM Clock Enable
pub use GPIOAEN_R as BKPRAMEN_R;
///Field `GPIOBEN` writer - 0GPIO peripheral clock enable
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - 0GPIO peripheral clock enable
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - 0GPIO peripheral clock enable
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - 0GPIO peripheral clock enable
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - 0GPIO peripheral clock enable
pub use GPIOAEN_W as GPIOFEN_W;
///Field `GPIOGEN` writer - 0GPIO peripheral clock enable
pub use GPIOAEN_W as GPIOGEN_W;
///Field `GPIOHEN` writer - 0GPIO peripheral clock enable
pub use GPIOAEN_W as GPIOHEN_W;
///Field `GPIOJEN` writer - 0GPIO peripheral clock enable
pub use GPIOAEN_W as GPIOJEN_W;
///Field `GPIOKEN` writer - 0GPIO peripheral clock enable
pub use GPIOAEN_W as GPIOKEN_W;
///Field `CRCEN` writer - CRC peripheral clock enable
pub use GPIOAEN_W as CRCEN_W;
///Field `BDMAEN` writer - BDMA and DMAMUX2 Clock Enable
pub use GPIOAEN_W as BDMAEN_W;
///Field `ADC3EN` writer - ADC3 Peripheral Clocks Enable
pub use GPIOAEN_W as ADC3EN_W;
///Field `HSEMEN` writer - HSEM peripheral clock enable
pub use GPIOAEN_W as HSEMEN_W;
///Field `BKPRAMEN` writer - Backup RAM Clock Enable
pub use GPIOAEN_W as BKPRAMEN_W;
impl R {
    ///Bit 0 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpioken(&self) -> GPIOKEN_R {
        GPIOKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 19 - CRC peripheral clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - BDMA and DMAMUX2 Clock Enable
    #[inline(always)]
    pub fn bdmaen(&self) -> BDMAEN_R {
        BDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - ADC3 Peripheral Clocks Enable
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - HSEM peripheral clock enable
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Backup RAM Clock Enable
    #[inline(always)]
    pub fn bkpramen(&self) -> BKPRAMEN_R {
        BKPRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4ENR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .field("gpiogen", &self.gpiogen())
            .field("gpiohen", &self.gpiohen())
            .field("gpiojen", &self.gpiojen())
            .field("gpioken", &self.gpioken())
            .field("crcen", &self.crcen())
            .field("bdmaen", &self.bdmaen())
            .field("adc3en", &self.adc3en())
            .field("hsemen", &self.hsemen())
            .field("bkpramen", &self.bkpramen())
            .finish()
    }
}
impl W {
    ///Bit 0 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, AHB4ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, AHB4ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, AHB4ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, AHB4ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, AHB4ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, AHB4ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    ///Bit 6 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<'_, AHB4ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    ///Bit 7 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<'_, AHB4ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 9 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpiojen(&mut self) -> GPIOJEN_W<'_, AHB4ENRrs> {
        GPIOJEN_W::new(self, 9)
    }
    ///Bit 10 - 0GPIO peripheral clock enable
    #[inline(always)]
    pub fn gpioken(&mut self) -> GPIOKEN_W<'_, AHB4ENRrs> {
        GPIOKEN_W::new(self, 10)
    }
    ///Bit 19 - CRC peripheral clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHB4ENRrs> {
        CRCEN_W::new(self, 19)
    }
    ///Bit 21 - BDMA and DMAMUX2 Clock Enable
    #[inline(always)]
    pub fn bdmaen(&mut self) -> BDMAEN_W<'_, AHB4ENRrs> {
        BDMAEN_W::new(self, 21)
    }
    ///Bit 24 - ADC3 Peripheral Clocks Enable
    #[inline(always)]
    pub fn adc3en(&mut self) -> ADC3EN_W<'_, AHB4ENRrs> {
        ADC3EN_W::new(self, 24)
    }
    ///Bit 25 - HSEM peripheral clock enable
    #[inline(always)]
    pub fn hsemen(&mut self) -> HSEMEN_W<'_, AHB4ENRrs> {
        HSEMEN_W::new(self, 25)
    }
    ///Bit 28 - Backup RAM Clock Enable
    #[inline(always)]
    pub fn bkpramen(&mut self) -> BKPRAMEN_W<'_, AHB4ENRrs> {
        BKPRAMEN_W::new(self, 28)
    }
}
/**RCC AHB4 Clock Register

You can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#RCC:AHB4ENR)*/
pub struct AHB4ENRrs;
impl crate::RegisterSpec for AHB4ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4enr::R`](R) reader structure
impl crate::Readable for AHB4ENRrs {}
///`write(|w| ..)` method takes [`ahb4enr::W`](W) writer structure
impl crate::Writable for AHB4ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4ENR to value 0
impl crate::Resettable for AHB4ENRrs {}
