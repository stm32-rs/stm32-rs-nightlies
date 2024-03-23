#[doc = "Register `RCC_MP_AHB4ENCLRR` reader"]
pub type R = crate::R<RCC_MP_AHB4ENCLRRrs>;
#[doc = "Register `RCC_MP_AHB4ENCLRR` writer"]
pub type W = crate::W<RCC_MP_AHB4ENCLRRrs>;
#[doc = "Field `GPIOAEN` reader - GPIOAEN"]
pub type GPIOAEN_R = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - GPIOAEN"]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - GPIOBEN"]
pub type GPIOBEN_R = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - GPIOBEN"]
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - GPIOCEN"]
pub type GPIOCEN_R = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - GPIOCEN"]
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODEN` reader - GPIODEN"]
pub type GPIODEN_R = crate::BitReader;
#[doc = "Field `GPIODEN` writer - GPIODEN"]
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOEEN` reader - GPIOEEN"]
pub type GPIOEEN_R = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - GPIOEEN"]
pub type GPIOEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFEN` reader - GPIOFEN"]
pub type GPIOFEN_R = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - GPIOFEN"]
pub type GPIOFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGEN` reader - GPIOGEN"]
pub type GPIOGEN_R = crate::BitReader;
#[doc = "Field `GPIOGEN` writer - GPIOGEN"]
pub type GPIOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHEN` reader - GPIOHEN"]
pub type GPIOHEN_R = crate::BitReader;
#[doc = "Field `GPIOHEN` writer - GPIOHEN"]
pub type GPIOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOIEN` reader - GPIOIEN"]
pub type GPIOIEN_R = crate::BitReader;
#[doc = "Field `GPIOIEN` writer - GPIOIEN"]
pub type GPIOIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOJEN` reader - GPIOJEN"]
pub type GPIOJEN_R = crate::BitReader;
#[doc = "Field `GPIOJEN` writer - GPIOJEN"]
pub type GPIOJEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOKEN` reader - GPIOKEN"]
pub type GPIOKEN_R = crate::BitReader;
#[doc = "Field `GPIOKEN` writer - GPIOKEN"]
pub type GPIOKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOAEN"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOBEN"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOCEN"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIODEN"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOEEN"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOFEN"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOGEN"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOHEN"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOIEN"]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOJEN"]
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOKEN"]
    #[inline(always)]
    pub fn gpioken(&self) -> GPIOKEN_R {
        GPIOKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOAEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<RCC_MP_AHB4ENCLRRrs> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOBEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<RCC_MP_AHB4ENCLRRrs> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOCEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<RCC_MP_AHB4ENCLRRrs> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIODEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<RCC_MP_AHB4ENCLRRrs> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOEEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<RCC_MP_AHB4ENCLRRrs> {
        GPIOEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOFEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<RCC_MP_AHB4ENCLRRrs> {
        GPIOFEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOGEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<RCC_MP_AHB4ENCLRRrs> {
        GPIOGEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOHEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<RCC_MP_AHB4ENCLRRrs> {
        GPIOHEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOIEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<RCC_MP_AHB4ENCLRRrs> {
        GPIOIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - GPIOJEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiojen(&mut self) -> GPIOJEN_W<RCC_MP_AHB4ENCLRRrs> {
        GPIOJEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - GPIOKEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioken(&mut self) -> GPIOKEN_W<RCC_MP_AHB4ENCLRRrs> {
        GPIOKEN_W::new(self, 10)
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb4enclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb4enclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_AHB4ENCLRRrs;
impl crate::RegisterSpec for RCC_MP_AHB4ENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_ahb4enclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_AHB4ENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_ahb4enclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_AHB4ENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_AHB4ENCLRR to value 0"]
impl crate::Resettable for RCC_MP_AHB4ENCLRRrs {
    const RESET_VALUE: u32 = 0;
}
