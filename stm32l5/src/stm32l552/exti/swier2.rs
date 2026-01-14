///Register `SWIER2` reader
pub type R = crate::R<SWIER2rs>;
///Register `SWIER2` writer
pub type W = crate::W<SWIER2rs>;
///Field `SWI35` reader - SWI35
pub type SWI35_R = crate::BitReader;
///Field `SWI35` writer - SWI35
pub type SWI35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI36` reader - SWI36
pub type SWI36_R = crate::BitReader;
///Field `SWI36` writer - SWI36
pub type SWI36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI37` reader - SWI37
pub type SWI37_R = crate::BitReader;
///Field `SWI37` writer - SWI37
pub type SWI37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI38` reader - SWI38
pub type SWI38_R = crate::BitReader;
///Field `SWI38` writer - SWI38
pub type SWI38_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - SWI35
    #[inline(always)]
    pub fn swi35(&self) -> SWI35_R {
        SWI35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SWI36
    #[inline(always)]
    pub fn swi36(&self) -> SWI36_R {
        SWI36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SWI37
    #[inline(always)]
    pub fn swi37(&self) -> SWI37_R {
        SWI37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SWI38
    #[inline(always)]
    pub fn swi38(&self) -> SWI38_R {
        SWI38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER2")
            .field("swi35", &self.swi35())
            .field("swi36", &self.swi36())
            .field("swi37", &self.swi37())
            .field("swi38", &self.swi38())
            .finish()
    }
}
impl W {
    ///Bit 3 - SWI35
    #[inline(always)]
    pub fn swi35(&mut self) -> SWI35_W<'_, SWIER2rs> {
        SWI35_W::new(self, 3)
    }
    ///Bit 4 - SWI36
    #[inline(always)]
    pub fn swi36(&mut self) -> SWI36_W<'_, SWIER2rs> {
        SWI36_W::new(self, 4)
    }
    ///Bit 5 - SWI37
    #[inline(always)]
    pub fn swi37(&mut self) -> SWI37_W<'_, SWIER2rs> {
        SWI37_W::new(self, 5)
    }
    ///Bit 6 - SWI38
    #[inline(always)]
    pub fn swi38(&mut self) -> SWI38_W<'_, SWIER2rs> {
        SWI38_W::new(self, 6)
    }
}
/**EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#EXTI:SWIER2)*/
pub struct SWIER2rs;
impl crate::RegisterSpec for SWIER2rs {
    type Ux = u32;
}
///`read()` method returns [`swier2::R`](R) reader structure
impl crate::Readable for SWIER2rs {}
///`write(|w| ..)` method takes [`swier2::W`](W) writer structure
impl crate::Writable for SWIER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER2 to value 0
impl crate::Resettable for SWIER2rs {}
