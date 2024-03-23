#[doc = "Register `IOPSMENR` reader"]
pub type R = crate::R<IOPSMENRrs>;
#[doc = "Register `IOPSMENR` writer"]
pub type W = crate::W<IOPSMENRrs>;
#[doc = "Field `GPIOASMEN` reader - I/O port A clock enable during Sleep mode"]
pub type GPIOASMEN_R = crate::BitReader;
#[doc = "Field `GPIOASMEN` writer - I/O port A clock enable during Sleep mode"]
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBSMEN` reader - I/O port B clock enable during Sleep mode"]
pub type GPIOBSMEN_R = crate::BitReader;
#[doc = "Field `GPIOBSMEN` writer - I/O port B clock enable during Sleep mode"]
pub type GPIOBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCSMEN` reader - I/O port C clock enable during Sleep mode"]
pub type GPIOCSMEN_R = crate::BitReader;
#[doc = "Field `GPIOCSMEN` writer - I/O port C clock enable during Sleep mode"]
pub type GPIOCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODSMEN` reader - I/O port D clock enable during Sleep mode"]
pub type GPIODSMEN_R = crate::BitReader;
#[doc = "Field `GPIODSMEN` writer - I/O port D clock enable during Sleep mode"]
pub type GPIODSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOESMEN` reader - I/O port E clock enable during Sleep mode"]
pub type GPIOESMEN_R = crate::BitReader;
#[doc = "Field `GPIOESMEN` writer - I/O port E clock enable during Sleep mode"]
pub type GPIOESMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFSMEN` reader - I/O port F clock enable during Sleep mode"]
pub type GPIOFSMEN_R = crate::BitReader;
#[doc = "Field `GPIOFSMEN` writer - I/O port F clock enable during Sleep mode"]
pub type GPIOFSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<IOPSMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<IOPSMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<IOPSMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<IOPSMENRrs> {
        GPIODSMEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port E clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<IOPSMENRrs> {
        GPIOESMEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<IOPSMENRrs> {
        GPIOFSMEN_W::new(self, 5)
    }
}
#[doc = "GPIO in Sleep mode clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iopsmenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iopsmenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOPSMENRrs;
impl crate::RegisterSpec for IOPSMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopsmenr::R`](R) reader structure"]
impl crate::Readable for IOPSMENRrs {}
#[doc = "`write(|w| ..)` method takes [`iopsmenr::W`](W) writer structure"]
impl crate::Writable for IOPSMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOPSMENR to value 0x3f"]
impl crate::Resettable for IOPSMENRrs {
    const RESET_VALUE: u32 = 0x3f;
}
