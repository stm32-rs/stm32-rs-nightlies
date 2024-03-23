#[doc = "Register `C2AHB2ENR` reader"]
pub type R = crate::R<C2AHB2ENRrs>;
#[doc = "Register `C2AHB2ENR` writer"]
pub type W = crate::W<C2AHB2ENRrs>;
#[doc = "Field `GPIOAEN` reader - CPU2 IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - CPU2 IO port A clock enable"]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - CPU2 IO port B clock enable"]
pub type GPIOBEN_R = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - CPU2 IO port B clock enable"]
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - CPU2 IO port C clock enable"]
pub type GPIOCEN_R = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - CPU2 IO port C clock enable"]
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODEN` reader - CPU2 IO port D clock enable"]
pub type GPIODEN_R = crate::BitReader;
#[doc = "Field `GPIODEN` writer - CPU2 IO port D clock enable"]
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOEEN` reader - CPU2 IO port E clock enable"]
pub type GPIOEEN_R = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - CPU2 IO port E clock enable"]
pub type GPIOEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHEN` reader - CPU2 IO port H clock enable"]
pub type GPIOHEN_R = crate::BitReader;
#[doc = "Field `GPIOHEN` writer - CPU2 IO port H clock enable"]
pub type GPIOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCEN` reader - CPU2 ADC clock enable"]
pub type ADCEN_R = crate::BitReader;
#[doc = "Field `ADCEN` writer - CPU2 ADC clock enable"]
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES1EN` reader - CPU2 AES1 accelerator clock enable"]
pub type AES1EN_R = crate::BitReader;
#[doc = "Field `AES1EN` writer - CPU2 AES1 accelerator clock enable"]
pub type AES1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU2 IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU2 IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU2 IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU2 IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU2 IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU2 IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU2 ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU2 AES1 accelerator clock enable"]
    #[inline(always)]
    pub fn aes1en(&self) -> AES1EN_R {
        AES1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 IO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<C2AHB2ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU2 IO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<C2AHB2ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU2 IO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<C2AHB2ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU2 IO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<C2AHB2ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU2 IO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<C2AHB2ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    #[doc = "Bit 7 - CPU2 IO port H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<C2AHB2ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    #[doc = "Bit 13 - CPU2 ADC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<C2AHB2ENRrs> {
        ADCEN_W::new(self, 13)
    }
    #[doc = "Bit 16 - CPU2 AES1 accelerator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn aes1en(&mut self) -> AES1EN_W<C2AHB2ENRrs> {
        AES1EN_W::new(self, 16)
    }
}
#[doc = "CPU2 AHB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ahb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ahb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2AHB2ENRrs;
impl crate::RegisterSpec for C2AHB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2ahb2enr::R`](R) reader structure"]
impl crate::Readable for C2AHB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2ahb2enr::W`](W) writer structure"]
impl crate::Writable for C2AHB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2AHB2ENR to value 0"]
impl crate::Resettable for C2AHB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
