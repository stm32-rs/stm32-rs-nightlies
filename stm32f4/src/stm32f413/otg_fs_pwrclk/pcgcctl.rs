///Register `PCGCCTL` reader
pub type R = crate::R<PCGCCTLrs>;
///Register `PCGCCTL` writer
pub type W = crate::W<PCGCCTLrs>;
///Field `STPPCLK` reader - Stop PHY clock
pub type STPPCLK_R = crate::BitReader;
///Field `STPPCLK` writer - Stop PHY clock
pub type STPPCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GATEHCLK` reader - Gate HCLK
pub type GATEHCLK_R = crate::BitReader;
///Field `GATEHCLK` writer - Gate HCLK
pub type GATEHCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHYSUSP` reader - PHY Suspended
pub type PHYSUSP_R = crate::BitReader;
///Field `PHYSUSP` writer - PHY Suspended
pub type PHYSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Stop PHY clock
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Gate HCLK
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - PHY Suspended
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCGCCTL")
            .field("stppclk", &self.stppclk())
            .field("gatehclk", &self.gatehclk())
            .field("physusp", &self.physusp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Stop PHY clock
    #[inline(always)]
    pub fn stppclk(&mut self) -> STPPCLK_W<'_, PCGCCTLrs> {
        STPPCLK_W::new(self, 0)
    }
    ///Bit 1 - Gate HCLK
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<'_, PCGCCTLrs> {
        GATEHCLK_W::new(self, 1)
    }
    ///Bit 4 - PHY Suspended
    #[inline(always)]
    pub fn physusp(&mut self) -> PHYSUSP_W<'_, PCGCCTLrs> {
        PHYSUSP_W::new(self, 4)
    }
}
/**OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)

You can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#OTG_FS_PWRCLK:PCGCCTL)*/
pub struct PCGCCTLrs;
impl crate::RegisterSpec for PCGCCTLrs {
    type Ux = u32;
}
///`read()` method returns [`pcgcctl::R`](R) reader structure
impl crate::Readable for PCGCCTLrs {}
///`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure
impl crate::Writable for PCGCCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCGCCTL to value 0
impl crate::Resettable for PCGCCTLrs {}
