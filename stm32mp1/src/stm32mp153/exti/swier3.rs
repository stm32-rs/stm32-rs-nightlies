///Register `SWIER3` reader
pub type R = crate::R<SWIER3rs>;
///Register `SWIER3` writer
pub type W = crate::W<SWIER3rs>;
///Field `SWI65` reader - SWI65
pub type SWI65_R = crate::BitReader;
///Field `SWI65` writer - SWI65
pub type SWI65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI66` reader - SWI66
pub type SWI66_R = crate::BitReader;
///Field `SWI66` writer - SWI66
pub type SWI66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI68` reader - SWI68
pub type SWI68_R = crate::BitReader;
///Field `SWI68` writer - SWI68
pub type SWI68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI73` reader - SWI73
pub type SWI73_R = crate::BitReader;
///Field `SWI73` writer - SWI73
pub type SWI73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI74` reader - SWI74
pub type SWI74_R = crate::BitReader;
///Field `SWI74` writer - SWI74
pub type SWI74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - SWI65
    #[inline(always)]
    pub fn swi65(&self) -> SWI65_R {
        SWI65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SWI66
    #[inline(always)]
    pub fn swi66(&self) -> SWI66_R {
        SWI66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SWI68
    #[inline(always)]
    pub fn swi68(&self) -> SWI68_R {
        SWI68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - SWI73
    #[inline(always)]
    pub fn swi73(&self) -> SWI73_R {
        SWI73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SWI74
    #[inline(always)]
    pub fn swi74(&self) -> SWI74_R {
        SWI74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER3")
            .field("swi65", &self.swi65())
            .field("swi66", &self.swi66())
            .field("swi68", &self.swi68())
            .field("swi73", &self.swi73())
            .field("swi74", &self.swi74())
            .finish()
    }
}
impl W {
    ///Bit 1 - SWI65
    #[inline(always)]
    pub fn swi65(&mut self) -> SWI65_W<'_, SWIER3rs> {
        SWI65_W::new(self, 1)
    }
    ///Bit 2 - SWI66
    #[inline(always)]
    pub fn swi66(&mut self) -> SWI66_W<'_, SWIER3rs> {
        SWI66_W::new(self, 2)
    }
    ///Bit 4 - SWI68
    #[inline(always)]
    pub fn swi68(&mut self) -> SWI68_W<'_, SWIER3rs> {
        SWI68_W::new(self, 4)
    }
    ///Bit 9 - SWI73
    #[inline(always)]
    pub fn swi73(&mut self) -> SWI73_W<'_, SWIER3rs> {
        SWI73_W::new(self, 9)
    }
    ///Bit 10 - SWI74
    #[inline(always)]
    pub fn swi74(&mut self) -> SWI74_W<'_, SWIER3rs> {
        SWI74_W::new(self, 10)
    }
}
/**Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`swier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:SWIER3)*/
pub struct SWIER3rs;
impl crate::RegisterSpec for SWIER3rs {
    type Ux = u32;
}
///`read()` method returns [`swier3::R`](R) reader structure
impl crate::Readable for SWIER3rs {}
///`write(|w| ..)` method takes [`swier3::W`](W) writer structure
impl crate::Writable for SWIER3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER3 to value 0
impl crate::Resettable for SWIER3rs {}
