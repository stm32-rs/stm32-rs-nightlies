///Register `AHB2ENR` reader
pub type R = crate::R<AHB2ENRrs>;
///Register `AHB2ENR` writer
pub type W = crate::W<AHB2ENRrs>;
///Field `GPIOAEN` reader - IO port A clock enable
pub type GPIOAEN_R = crate::BitReader;
///Field `GPIOAEN` writer - IO port A clock enable
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBEN` reader - IO port B clock enable
pub type GPIOBEN_R = crate::BitReader;
///Field `GPIOBEN` writer - IO port B clock enable
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCEN` reader - IO port C clock enable
pub type GPIOCEN_R = crate::BitReader;
///Field `GPIOCEN` writer - IO port C clock enable
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODEN` reader - IO port D clock enable
pub type GPIODEN_R = crate::BitReader;
///Field `GPIODEN` writer - IO port D clock enable
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOEEN` reader - IO port E clock enable
pub type GPIOEEN_R = crate::BitReader;
///Field `GPIOEEN` writer - IO port E clock enable
pub type GPIOEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFEN` reader - IO port F clock enable
pub type GPIOFEN_R = crate::BitReader;
///Field `GPIOFEN` writer - IO port F clock enable
pub type GPIOFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGEN` reader - IO port G clock enable
pub type GPIOGEN_R = crate::BitReader;
///Field `GPIOGEN` writer - IO port G clock enable
pub type GPIOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHEN` reader - IO port H clock enable
pub type GPIOHEN_R = crate::BitReader;
///Field `GPIOHEN` writer - IO port H clock enable
pub type GPIOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOIEN` reader - IO port I clock enable
pub type GPIOIEN_R = crate::BitReader;
///Field `GPIOIEN` writer - IO port I clock enable
pub type GPIOIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGFSEN` reader - OTG full speed clock enable
pub type OTGFSEN_R = crate::BitReader;
///Field `OTGFSEN` writer - OTG full speed clock enable
pub type OTGFSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**ADC clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCEN {
    ///0: ADC clock disabled
    Disabled = 0,
    ///1: ADC clock enabled
    Enabled = 1,
}
impl From<ADCEN> for bool {
    #[inline(always)]
    fn from(variant: ADCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ADCEN` reader - ADC clock enable
pub type ADCEN_R = crate::BitReader<ADCEN>;
impl ADCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADCEN {
        match self.bits {
            false => ADCEN::Disabled,
            true => ADCEN::Enabled,
        }
    }
    ///ADC clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCEN::Disabled
    }
    ///ADC clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCEN::Enabled
    }
}
///Field `ADCEN` writer - ADC clock enable
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCEN>;
impl<'a, REG> ADCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Disabled)
    }
    ///ADC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Enabled)
    }
}
///Field `DCMIEN` reader - DCMI clock enable
pub type DCMIEN_R = crate::BitReader;
///Field `DCMIEN` writer - DCMI clock enable
pub type DCMIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESEN` reader - AES accelerator clock enable
pub type AESEN_R = crate::BitReader;
///Field `AESEN` writer - AES accelerator clock enable
pub type AESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHEN` reader - HASH clock enable
pub type HASHEN_R = crate::BitReader;
///Field `HASHEN` writer - HASH clock enable
pub type HASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGEN` reader - Random Number Generator clock enable
pub type RNGEN_R = crate::BitReader;
///Field `RNGEN` writer - Random Number Generator clock enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSPIMEN` reader - OctoSPI IO manager clock enable
pub type OSPIMEN_R = crate::BitReader;
///Field `OSPIMEN` writer - OctoSPI IO manager clock enable
pub type OSPIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1EN` reader - SDMMC1 clock enable
pub type SDMMC1EN_R = crate::BitReader;
///Field `SDMMC1EN` writer - SDMMC1 clock enable
pub type SDMMC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I clock enable
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - OTG full speed clock enable
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DCMI clock enable
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - OctoSPI IO manager clock enable
    #[inline(always)]
    pub fn ospimen(&self) -> OSPIMEN_R {
        OSPIMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - SDMMC1 clock enable
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .field("gpiogen", &self.gpiogen())
            .field("gpiohen", &self.gpiohen())
            .field("gpioien", &self.gpioien())
            .field("otgfsen", &self.otgfsen())
            .field("adcen", &self.adcen())
            .field("dcmien", &self.dcmien())
            .field("aesen", &self.aesen())
            .field("hashen", &self.hashen())
            .field("rngen", &self.rngen())
            .field("ospimen", &self.ospimen())
            .field("sdmmc1en", &self.sdmmc1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, AHB2ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, AHB2ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, AHB2ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, AHB2ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, AHB2ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, AHB2ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<'_, AHB2ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<'_, AHB2ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 8 - IO port I clock enable
    #[inline(always)]
    pub fn gpioien(&mut self) -> GPIOIEN_W<'_, AHB2ENRrs> {
        GPIOIEN_W::new(self, 8)
    }
    ///Bit 12 - OTG full speed clock enable
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<'_, AHB2ENRrs> {
        OTGFSEN_W::new(self, 12)
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<'_, AHB2ENRrs> {
        ADCEN_W::new(self, 13)
    }
    ///Bit 14 - DCMI clock enable
    #[inline(always)]
    pub fn dcmien(&mut self) -> DCMIEN_W<'_, AHB2ENRrs> {
        DCMIEN_W::new(self, 14)
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W<'_, AHB2ENRrs> {
        AESEN_W::new(self, 16)
    }
    ///Bit 17 - HASH clock enable
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W<'_, AHB2ENRrs> {
        HASHEN_W::new(self, 17)
    }
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB2ENRrs> {
        RNGEN_W::new(self, 18)
    }
    ///Bit 20 - OctoSPI IO manager clock enable
    #[inline(always)]
    pub fn ospimen(&mut self) -> OSPIMEN_W<'_, AHB2ENRrs> {
        OSPIMEN_W::new(self, 20)
    }
    ///Bit 22 - SDMMC1 clock enable
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<'_, AHB2ENRrs> {
        SDMMC1EN_W::new(self, 22)
    }
}
/**AHB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#RCC:AHB2ENR)*/
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2enr::R`](R) reader structure
impl crate::Readable for AHB2ENRrs {}
///`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENR to value 0
impl crate::Resettable for AHB2ENRrs {}
