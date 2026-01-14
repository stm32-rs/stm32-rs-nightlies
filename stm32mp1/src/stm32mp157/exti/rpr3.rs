///Register `RPR3` reader
pub type R = crate::R<RPR3rs>;
///Register `RPR3` writer
pub type W = crate::W<RPR3rs>;
///Field `RPIF65` reader - RPIF65
pub type RPIF65_R = crate::BitReader;
///Field `RPIF65` writer - RPIF65
pub type RPIF65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF66` reader - RPIF66
pub type RPIF66_R = crate::BitReader;
///Field `RPIF66` writer - RPIF66
pub type RPIF66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF68` reader - RPIF68
pub type RPIF68_R = crate::BitReader;
///Field `RPIF68` writer - RPIF68
pub type RPIF68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF73` reader - RPIF73
pub type RPIF73_R = crate::BitReader;
///Field `RPIF73` writer - RPIF73
pub type RPIF73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF74` reader - RPIF74
pub type RPIF74_R = crate::BitReader;
///Field `RPIF74` writer - RPIF74
pub type RPIF74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - RPIF65
    #[inline(always)]
    pub fn rpif65(&self) -> RPIF65_R {
        RPIF65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RPIF66
    #[inline(always)]
    pub fn rpif66(&self) -> RPIF66_R {
        RPIF66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - RPIF68
    #[inline(always)]
    pub fn rpif68(&self) -> RPIF68_R {
        RPIF68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - RPIF73
    #[inline(always)]
    pub fn rpif73(&self) -> RPIF73_R {
        RPIF73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RPIF74
    #[inline(always)]
    pub fn rpif74(&self) -> RPIF74_R {
        RPIF74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR3")
            .field("rpif65", &self.rpif65())
            .field("rpif66", &self.rpif66())
            .field("rpif68", &self.rpif68())
            .field("rpif73", &self.rpif73())
            .field("rpif74", &self.rpif74())
            .finish()
    }
}
impl W {
    ///Bit 1 - RPIF65
    #[inline(always)]
    pub fn rpif65(&mut self) -> RPIF65_W<'_, RPR3rs> {
        RPIF65_W::new(self, 1)
    }
    ///Bit 2 - RPIF66
    #[inline(always)]
    pub fn rpif66(&mut self) -> RPIF66_W<'_, RPR3rs> {
        RPIF66_W::new(self, 2)
    }
    ///Bit 4 - RPIF68
    #[inline(always)]
    pub fn rpif68(&mut self) -> RPIF68_W<'_, RPR3rs> {
        RPIF68_W::new(self, 4)
    }
    ///Bit 9 - RPIF73
    #[inline(always)]
    pub fn rpif73(&mut self) -> RPIF73_W<'_, RPR3rs> {
        RPIF73_W::new(self, 9)
    }
    ///Bit 10 - RPIF74
    #[inline(always)]
    pub fn rpif74(&mut self) -> RPIF74_W<'_, RPR3rs> {
        RPIF74_W::new(self, 10)
    }
}
/**Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`rpr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:RPR3)*/
pub struct RPR3rs;
impl crate::RegisterSpec for RPR3rs {
    type Ux = u32;
}
///`read()` method returns [`rpr3::R`](R) reader structure
impl crate::Readable for RPR3rs {}
///`write(|w| ..)` method takes [`rpr3::W`](W) writer structure
impl crate::Writable for RPR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RPR3 to value 0
impl crate::Resettable for RPR3rs {}
