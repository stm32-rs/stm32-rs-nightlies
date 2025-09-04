///Register `ICIFR` reader
pub type R = crate::R<ICIFRrs>;
///Register `ICIFR` writer
pub type W = crate::W<ICIFRrs>;
///Field `TS1_CITEF` reader - TS1_CITEF
pub type TS1_CITEF_R = crate::BitReader;
///Field `TS1_CITEF` writer - TS1_CITEF
pub type TS1_CITEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_CITLF` reader - TS1_CITLF
pub type TS1_CITLF_R = crate::BitReader;
///Field `TS1_CITLF` writer - TS1_CITLF
pub type TS1_CITLF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_CITHF` reader - TS1_CITHF
pub type TS1_CITHF_R = crate::BitReader;
///Field `TS1_CITHF` writer - TS1_CITHF
pub type TS1_CITHF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_CAITEF` reader - TS1_CAITEF
pub type TS1_CAITEF_R = crate::BitReader;
///Field `TS1_CAITEF` writer - TS1_CAITEF
pub type TS1_CAITEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_CAITLF` reader - TS1_CAITLF
pub type TS1_CAITLF_R = crate::BitReader;
///Field `TS1_CAITLF` writer - TS1_CAITLF
pub type TS1_CAITLF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TS1_CAITHF` reader - TS1_CAITHF
pub type TS1_CAITHF_R = crate::BitReader;
///Field `TS1_CAITHF` writer - TS1_CAITHF
pub type TS1_CAITHF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TS1_CITEF
    #[inline(always)]
    pub fn ts1_citef(&self) -> TS1_CITEF_R {
        TS1_CITEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TS1_CITLF
    #[inline(always)]
    pub fn ts1_citlf(&self) -> TS1_CITLF_R {
        TS1_CITLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TS1_CITHF
    #[inline(always)]
    pub fn ts1_cithf(&self) -> TS1_CITHF_R {
        TS1_CITHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - TS1_CAITEF
    #[inline(always)]
    pub fn ts1_caitef(&self) -> TS1_CAITEF_R {
        TS1_CAITEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TS1_CAITLF
    #[inline(always)]
    pub fn ts1_caitlf(&self) -> TS1_CAITLF_R {
        TS1_CAITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TS1_CAITHF
    #[inline(always)]
    pub fn ts1_caithf(&self) -> TS1_CAITHF_R {
        TS1_CAITHF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICIFR")
            .field("ts1_citef", &self.ts1_citef())
            .field("ts1_citlf", &self.ts1_citlf())
            .field("ts1_cithf", &self.ts1_cithf())
            .field("ts1_caitef", &self.ts1_caitef())
            .field("ts1_caitlf", &self.ts1_caitlf())
            .field("ts1_caithf", &self.ts1_caithf())
            .finish()
    }
}
impl W {
    ///Bit 0 - TS1_CITEF
    #[inline(always)]
    pub fn ts1_citef(&mut self) -> TS1_CITEF_W<ICIFRrs> {
        TS1_CITEF_W::new(self, 0)
    }
    ///Bit 1 - TS1_CITLF
    #[inline(always)]
    pub fn ts1_citlf(&mut self) -> TS1_CITLF_W<ICIFRrs> {
        TS1_CITLF_W::new(self, 1)
    }
    ///Bit 2 - TS1_CITHF
    #[inline(always)]
    pub fn ts1_cithf(&mut self) -> TS1_CITHF_W<ICIFRrs> {
        TS1_CITHF_W::new(self, 2)
    }
    ///Bit 4 - TS1_CAITEF
    #[inline(always)]
    pub fn ts1_caitef(&mut self) -> TS1_CAITEF_W<ICIFRrs> {
        TS1_CAITEF_W::new(self, 4)
    }
    ///Bit 5 - TS1_CAITLF
    #[inline(always)]
    pub fn ts1_caitlf(&mut self) -> TS1_CAITLF_W<ICIFRrs> {
        TS1_CAITLF_W::new(self, 5)
    }
    ///Bit 6 - TS1_CAITHF
    #[inline(always)]
    pub fn ts1_caithf(&mut self) -> TS1_CAITHF_W<ICIFRrs> {
        TS1_CAITHF_W::new(self, 6)
    }
}
/**DTS_ICIFR is the control register for the interrupt flags.

You can [`read`](crate::Reg::read) this register and get [`icifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DTS:ICIFR)*/
pub struct ICIFRrs;
impl crate::RegisterSpec for ICIFRrs {
    type Ux = u32;
}
///`read()` method returns [`icifr::R`](R) reader structure
impl crate::Readable for ICIFRrs {}
///`write(|w| ..)` method takes [`icifr::W`](W) writer structure
impl crate::Writable for ICIFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICIFR to value 0
impl crate::Resettable for ICIFRrs {}
