///Register `SWIER2` reader
pub type R = crate::R<SWIER2rs>;
///Register `SWIER2` writer
pub type W = crate::W<SWIER2rs>;
///Field `SWI39` reader - Software interrupt on event x
pub type SWI39_R = crate::BitReader;
///Field `SWI39` writer - Software interrupt on event x
pub type SWI39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI40` reader - Software interrupt on event x
pub type SWI40_R = crate::BitReader;
///Field `SWI40` writer - Software interrupt on event x
pub type SWI40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI51` reader - Software interrupt on event 51
pub type SWI51_R = crate::BitReader;
///Field `SWI51` writer - Software interrupt on event 51
pub type SWI51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI54` reader - Software interrupt on event 54
pub type SWI54_R = crate::BitReader;
///Field `SWI54` writer - Software interrupt on event 54
pub type SWI54_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI56` reader - Software interrupt on event 56
pub type SWI56_R = crate::BitReader;
///Field `SWI56` writer - Software interrupt on event 56
pub type SWI56_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - Software interrupt on event x
    #[inline(always)]
    pub fn swi39(&self) -> SWI39_R {
        SWI39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Software interrupt on event x
    #[inline(always)]
    pub fn swi40(&self) -> SWI40_R {
        SWI40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 19 - Software interrupt on event 51
    #[inline(always)]
    pub fn swi51(&self) -> SWI51_R {
        SWI51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Software interrupt on event 54
    #[inline(always)]
    pub fn swi54(&self) -> SWI54_R {
        SWI54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Software interrupt on event 56
    #[inline(always)]
    pub fn swi56(&self) -> SWI56_R {
        SWI56_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER2")
            .field("swi39", &self.swi39())
            .field("swi40", &self.swi40())
            .field("swi51", &self.swi51())
            .field("swi54", &self.swi54())
            .field("swi56", &self.swi56())
            .finish()
    }
}
impl W {
    ///Bit 7 - Software interrupt on event x
    #[inline(always)]
    pub fn swi39(&mut self) -> SWI39_W<'_, SWIER2rs> {
        SWI39_W::new(self, 7)
    }
    ///Bit 8 - Software interrupt on event x
    #[inline(always)]
    pub fn swi40(&mut self) -> SWI40_W<'_, SWIER2rs> {
        SWI40_W::new(self, 8)
    }
    ///Bit 19 - Software interrupt on event 51
    #[inline(always)]
    pub fn swi51(&mut self) -> SWI51_W<'_, SWIER2rs> {
        SWI51_W::new(self, 19)
    }
    ///Bit 22 - Software interrupt on event 54
    #[inline(always)]
    pub fn swi54(&mut self) -> SWI54_W<'_, SWIER2rs> {
        SWI54_W::new(self, 22)
    }
    ///Bit 24 - Software interrupt on event 56
    #[inline(always)]
    pub fn swi56(&mut self) -> SWI56_W<'_, SWIER2rs> {
        SWI56_W::new(self, 24)
    }
}
/**EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#EXTI:SWIER2)*/
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
