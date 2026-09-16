///Register `ITENR` reader
pub type R = crate::R<ITENRrs>;
///Register `ITENR` writer
pub type W = crate::W<ITENRrs>;
///Field `TS1_ITEEN` reader - TS1_ITEEN
pub type TS1_ITEEN_R = crate::BitReader;
///Field `TS1_ITEEN` writer - TS1_ITEEN
pub type TS1_ITEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_ITLEN` reader - TS1_ITLEN
pub type TS1_ITLEN_R = crate::BitReader;
///Field `TS1_ITLEN` writer - TS1_ITLEN
pub type TS1_ITLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_ITHEN` reader - TS1_ITHEN
pub type TS1_ITHEN_R = crate::BitReader;
///Field `TS1_ITHEN` writer - TS1_ITHEN
pub type TS1_ITHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_AITEEN` reader - TS1_AITEEN
pub type TS1_AITEEN_R = crate::BitReader;
///Field `TS1_AITEEN` writer - TS1_AITEEN
pub type TS1_AITEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_AITLEN` reader - TS1_AITLEN
pub type TS1_AITLEN_R = crate::BitReader;
///Field `TS1_AITLEN` writer - TS1_AITLEN
pub type TS1_AITLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_AITHEN` reader - TS1_AITHEN
pub type TS1_AITHEN_R = crate::BitReader;
///Field `TS1_AITHEN` writer - TS1_AITHEN
pub type TS1_AITHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TS1_ITEEN
    #[inline(always)]
    pub fn ts1_iteen(&self) -> TS1_ITEEN_R {
        TS1_ITEEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TS1_ITLEN
    #[inline(always)]
    pub fn ts1_itlen(&self) -> TS1_ITLEN_R {
        TS1_ITLEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TS1_ITHEN
    #[inline(always)]
    pub fn ts1_ithen(&self) -> TS1_ITHEN_R {
        TS1_ITHEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - TS1_AITEEN
    #[inline(always)]
    pub fn ts1_aiteen(&self) -> TS1_AITEEN_R {
        TS1_AITEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TS1_AITLEN
    #[inline(always)]
    pub fn ts1_aitlen(&self) -> TS1_AITLEN_R {
        TS1_AITLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TS1_AITHEN
    #[inline(always)]
    pub fn ts1_aithen(&self) -> TS1_AITHEN_R {
        TS1_AITHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITENR")
            .field("ts1_iteen", &self.ts1_iteen())
            .field("ts1_itlen", &self.ts1_itlen())
            .field("ts1_ithen", &self.ts1_ithen())
            .field("ts1_aiteen", &self.ts1_aiteen())
            .field("ts1_aitlen", &self.ts1_aitlen())
            .field("ts1_aithen", &self.ts1_aithen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TS1_ITEEN
    #[inline(always)]
    pub fn ts1_iteen(&mut self) -> TS1_ITEEN_W<'_, ITENRrs> {
        TS1_ITEEN_W::new(self, 0)
    }
    ///Bit 1 - TS1_ITLEN
    #[inline(always)]
    pub fn ts1_itlen(&mut self) -> TS1_ITLEN_W<'_, ITENRrs> {
        TS1_ITLEN_W::new(self, 1)
    }
    ///Bit 2 - TS1_ITHEN
    #[inline(always)]
    pub fn ts1_ithen(&mut self) -> TS1_ITHEN_W<'_, ITENRrs> {
        TS1_ITHEN_W::new(self, 2)
    }
    ///Bit 4 - TS1_AITEEN
    #[inline(always)]
    pub fn ts1_aiteen(&mut self) -> TS1_AITEEN_W<'_, ITENRrs> {
        TS1_AITEEN_W::new(self, 4)
    }
    ///Bit 5 - TS1_AITLEN
    #[inline(always)]
    pub fn ts1_aitlen(&mut self) -> TS1_AITLEN_W<'_, ITENRrs> {
        TS1_AITLEN_W::new(self, 5)
    }
    ///Bit 6 - TS1_AITHEN
    #[inline(always)]
    pub fn ts1_aithen(&mut self) -> TS1_AITHEN_W<'_, ITENRrs> {
        TS1_AITHEN_W::new(self, 6)
    }
}
/**Temperature sensor interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`itenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DTS:ITENR)*/
pub struct ITENRrs;
impl crate::RegisterSpec for ITENRrs {
    type Ux = u32;
}
///`read()` method returns [`itenr::R`](R) reader structure
impl crate::Readable for ITENRrs {}
///`write(|w| ..)` method takes [`itenr::W`](W) writer structure
impl crate::Writable for ITENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ITENR to value 0
impl crate::Resettable for ITENRrs {}
