///Register `MC_AHB4ENCLRR` reader
pub type R = crate::R<MC_AHB4ENCLRRrs>;
///Register `MC_AHB4ENCLRR` writer
pub type W = crate::W<MC_AHB4ENCLRRrs>;
///Field `GPIOAEN` reader - GPIOAEN
pub type GPIOAEN_R = crate::BitReader;
///Field `GPIOAEN` writer - GPIOAEN
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBEN` reader - GPIOBEN
pub type GPIOBEN_R = crate::BitReader;
///Field `GPIOBEN` writer - GPIOBEN
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCEN` reader - GPIOCEN
pub type GPIOCEN_R = crate::BitReader;
///Field `GPIOCEN` writer - GPIOCEN
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODEN` reader - GPIODEN
pub type GPIODEN_R = crate::BitReader;
///Field `GPIODEN` writer - GPIODEN
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOEEN` reader - GPIOEEN
pub type GPIOEEN_R = crate::BitReader;
///Field `GPIOEEN` writer - GPIOEEN
pub type GPIOEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFEN` reader - GPIOFEN
pub type GPIOFEN_R = crate::BitReader;
///Field `GPIOFEN` writer - GPIOFEN
pub type GPIOFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGEN` reader - GPIOGEN
pub type GPIOGEN_R = crate::BitReader;
///Field `GPIOGEN` writer - GPIOGEN
pub type GPIOGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHEN` reader - GPIOHEN
pub type GPIOHEN_R = crate::BitReader;
///Field `GPIOHEN` writer - GPIOHEN
pub type GPIOHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOIEN` reader - GPIOIEN
pub type GPIOIEN_R = crate::BitReader;
///Field `GPIOIEN` writer - GPIOIEN
pub type GPIOIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOJEN` reader - GPIOJEN
pub type GPIOJEN_R = crate::BitReader;
///Field `GPIOJEN` writer - GPIOJEN
pub type GPIOJEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOKEN` reader - GPIOKEN
pub type GPIOKEN_R = crate::BitReader;
///Field `GPIOKEN` writer - GPIOKEN
pub type GPIOKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIOAEN
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOBEN
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOCEN
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIODEN
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOEEN
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOFEN
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOGEN
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOHEN
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOIEN
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIOJEN
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPIOKEN
    #[inline(always)]
    pub fn gpioken(&self) -> GPIOKEN_R {
        GPIOKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_AHB4ENCLRR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .field("gpiogen", &self.gpiogen())
            .field("gpiohen", &self.gpiohen())
            .field("gpioien", &self.gpioien())
            .field("gpiojen", &self.gpiojen())
            .field("gpioken", &self.gpioken())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOAEN
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, MC_AHB4ENCLRRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - GPIOBEN
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, MC_AHB4ENCLRRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - GPIOCEN
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, MC_AHB4ENCLRRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - GPIODEN
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, MC_AHB4ENCLRRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - GPIOEEN
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, MC_AHB4ENCLRRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - GPIOFEN
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, MC_AHB4ENCLRRrs> {
        GPIOFEN_W::new(self, 5)
    }
    ///Bit 6 - GPIOGEN
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<'_, MC_AHB4ENCLRRrs> {
        GPIOGEN_W::new(self, 6)
    }
    ///Bit 7 - GPIOHEN
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<'_, MC_AHB4ENCLRRrs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 8 - GPIOIEN
    #[inline(always)]
    pub fn gpioien(&mut self) -> GPIOIEN_W<'_, MC_AHB4ENCLRRrs> {
        GPIOIEN_W::new(self, 8)
    }
    ///Bit 9 - GPIOJEN
    #[inline(always)]
    pub fn gpiojen(&mut self) -> GPIOJEN_W<'_, MC_AHB4ENCLRRrs> {
        GPIOJEN_W::new(self, 9)
    }
    ///Bit 10 - GPIOKEN
    #[inline(always)]
    pub fn gpioken(&mut self) -> GPIOKEN_W<'_, MC_AHB4ENCLRRrs> {
        GPIOKEN_W::new(self, 10)
    }
}
/**This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb4enclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb4enclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MC_AHB4ENCLRR)*/
pub struct MC_AHB4ENCLRRrs;
impl crate::RegisterSpec for MC_AHB4ENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_ahb4enclrr::R`](R) reader structure
impl crate::Readable for MC_AHB4ENCLRRrs {}
///`write(|w| ..)` method takes [`mc_ahb4enclrr::W`](W) writer structure
impl crate::Writable for MC_AHB4ENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_AHB4ENCLRR to value 0
impl crate::Resettable for MC_AHB4ENCLRRrs {}
