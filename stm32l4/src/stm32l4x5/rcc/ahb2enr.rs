#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<AHB2ENRrs>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<AHB2ENRrs>;
#[doc = "Field `GPIOAEN` reader - IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - IO port A clock enable"]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - IO port B clock enable"]
pub type GPIOBEN_R = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - IO port B clock enable"]
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - IO port C clock enable"]
pub type GPIOCEN_R = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - IO port C clock enable"]
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODEN` reader - IO port D clock enable"]
pub type GPIODEN_R = crate::BitReader;
#[doc = "Field `GPIODEN` writer - IO port D clock enable"]
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOEEN` reader - IO port E clock enable"]
pub type GPIOEEN_R = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - IO port E clock enable"]
pub type GPIOEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFEN` reader - IO port F clock enable"]
pub type GPIOFEN_R = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - IO port F clock enable"]
pub type GPIOFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGEN` reader - IO port G clock enable"]
pub type GPIOGEN_R = crate::BitReader;
#[doc = "Field `GPIOGEN` writer - IO port G clock enable"]
pub type GPIOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHEN` reader - IO port H clock enable"]
pub type GPIOHEN_R = crate::BitReader;
#[doc = "Field `GPIOHEN` writer - IO port H clock enable"]
pub type GPIOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFSEN` reader - OTG full speed clock enable"]
pub type OTGFSEN_R = crate::BitReader;
#[doc = "Field `OTGFSEN` writer - OTG full speed clock enable"]
pub type OTGFSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCEN {
    #[doc = "0: ADC clock disabled"]
    Disabled = 0,
    #[doc = "1: ADC clock enabled"]
    Enabled = 1,
}
impl From<ADCEN> for bool {
    #[inline(always)]
    fn from(variant: ADCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - ADC clock enable"]
pub type ADCEN_R = crate::BitReader<ADCEN>;
impl ADCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCEN {
        match self.bits {
            false => ADCEN::Disabled,
            true => ADCEN::Enabled,
        }
    }
    #[doc = "ADC clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCEN::Disabled
    }
    #[doc = "ADC clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCEN::Enabled
    }
}
#[doc = "Field `ADCEN` writer - ADC clock enable"]
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCEN>;
impl<'a, REG> ADCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Disabled)
    }
    #[doc = "ADC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADCEN::Enabled)
    }
}
#[doc = "Field `AESEN` reader - AES accelerator clock enable"]
pub type AESEN_R = crate::BitReader;
#[doc = "Field `AESEN` writer - AES accelerator clock enable"]
pub type AESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGEN` reader - Random Number Generator clock enable"]
pub type RNGEN_R = crate::BitReader;
#[doc = "Field `RNGEN` writer - Random Number Generator clock enable"]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG full speed clock enable"]
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<AHB2ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<AHB2ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<AHB2ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<AHB2ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<AHB2ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<AHB2ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<AHB2ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<AHB2ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    #[doc = "Bit 12 - OTG full speed clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<AHB2ENRrs> {
        OTGFSEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<AHB2ENRrs> {
        ADCEN_W::new(self, 13)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<AHB2ENRrs> {
        AESEN_W::new(self, 16)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<AHB2ENRrs> {
        RNGEN_W::new(self, 18)
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2enr::R`](R) reader structure"]
impl crate::Readable for AHB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure"]
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
