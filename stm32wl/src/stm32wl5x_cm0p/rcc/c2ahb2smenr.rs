#[doc = "Register `C2AHB2SMENR` reader"]
pub type R = crate::R<C2AHB2SMENRrs>;
#[doc = "Register `C2AHB2SMENR` writer"]
pub type W = crate::W<C2AHB2SMENRrs>;
#[doc = "Field `GPIOASMEN` reader - IO port A clock enable during CPU2 CSleep mode."]
pub type GPIOASMEN_R = crate::BitReader;
#[doc = "Field `GPIOASMEN` writer - IO port A clock enable during CPU2 CSleep mode."]
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBSMEN` reader - IO port B clock enable during CPU2 CSleep mode."]
pub type GPIOBSMEN_R = crate::BitReader;
#[doc = "Field `GPIOBSMEN` writer - IO port B clock enable during CPU2 CSleep mode."]
pub type GPIOBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCSMEN` reader - IO port C clock enable during CPU2 CSleep mode."]
pub type GPIOCSMEN_R = crate::BitReader;
#[doc = "Field `GPIOCSMEN` writer - IO port C clock enable during CPU2 CSleep mode."]
pub type GPIOCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHSMEN` reader - IO port H clock enable during CPU2 CSleep mode."]
pub type GPIOHSMEN_R = crate::BitReader;
#[doc = "Field `GPIOHSMEN` writer - IO port H clock enable during CPU2 CSleep mode."]
pub type GPIOHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<C2AHB2SMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<C2AHB2SMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<C2AHB2SMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    #[doc = "Bit 7 - IO port H clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<C2AHB2SMENRrs> {
        GPIOHSMEN_W::new(self, 7)
    }
}
#[doc = "CPU2 AHB2 peripheral clocks enable in Sleep modes register \\[dual core device only\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ahb2smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ahb2smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2AHB2SMENRrs;
impl crate::RegisterSpec for C2AHB2SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2ahb2smenr::R`](R) reader structure"]
impl crate::Readable for C2AHB2SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2ahb2smenr::W`](W) writer structure"]
impl crate::Writable for C2AHB2SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2AHB2SMENR to value 0x87"]
impl crate::Resettable for C2AHB2SMENRrs {
    const RESET_VALUE: u32 = 0x87;
}
