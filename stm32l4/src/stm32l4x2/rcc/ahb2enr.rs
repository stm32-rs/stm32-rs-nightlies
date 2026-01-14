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
///Field `GPIOHEN` reader - IO port H clock enable
pub type GPIOHEN_R = crate::BitReader;
///Field `GPIOHEN` writer - IO port H clock enable
pub type GPIOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `AESEN` reader - AES accelerator clock enable
pub type AESEN_R = crate::BitReader;
///Field `AESEN` writer - AES accelerator clock enable
pub type AESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGEN` reader - Random Number Generator clock enable
pub type RNGEN_R = crate::BitReader;
///Field `RNGEN` writer - Random Number Generator clock enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR")
            .field("rngen", &self.rngen())
            .field("aesen", &self.aesen())
            .field("adcen", &self.adcen())
            .field("gpiohen", &self.gpiohen())
            .field("gpioeen", &self.gpioeen())
            .field("gpioden", &self.gpioden())
            .field("gpiocen", &self.gpiocen())
            .field("gpioben", &self.gpioben())
            .field("gpioaen", &self.gpioaen())
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
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<'_, AHB2ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<'_, AHB2ENRrs> {
        ADCEN_W::new(self, 13)
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W<'_, AHB2ENRrs> {
        AESEN_W::new(self, 16)
    }
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB2ENRrs> {
        RNGEN_W::new(self, 18)
    }
}
/**AHB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#RCC:AHB2ENR)*/
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
