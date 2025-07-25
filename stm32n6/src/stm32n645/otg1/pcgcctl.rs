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
///Field `PHYSUSP` reader - PHY suspended
pub type PHYSUSP_R = crate::BitReader;
///Field `ENL1GTG` reader - Enable sleep clock gating
pub type ENL1GTG_R = crate::BitReader;
///Field `ENL1GTG` writer - Enable sleep clock gating
pub type ENL1GTG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHYSLEEP` reader - PHY in Sleep
pub type PHYSLEEP_R = crate::BitReader;
///Field `SUSP` reader - Deep Sleep
pub type SUSP_R = crate::BitReader;
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
    ///Bit 4 - PHY suspended
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable sleep clock gating
    #[inline(always)]
    pub fn enl1gtg(&self) -> ENL1GTG_R {
        ENL1GTG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PHY in Sleep
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Deep Sleep
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCGCCTL")
            .field("stppclk", &self.stppclk())
            .field("gatehclk", &self.gatehclk())
            .field("physusp", &self.physusp())
            .field("enl1gtg", &self.enl1gtg())
            .field("physleep", &self.physleep())
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Stop PHY clock
    #[inline(always)]
    pub fn stppclk(&mut self) -> STPPCLK_W<PCGCCTLrs> {
        STPPCLK_W::new(self, 0)
    }
    ///Bit 1 - Gate HCLK
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<PCGCCTLrs> {
        GATEHCLK_W::new(self, 1)
    }
    ///Bit 5 - Enable sleep clock gating
    #[inline(always)]
    pub fn enl1gtg(&mut self) -> ENL1GTG_W<PCGCCTLrs> {
        ENL1GTG_W::new(self, 5)
    }
}
/**OTG power and clock gating control register

You can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#OTG1:PCGCCTL)*/
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
///`reset()` method sets PCGCCTL to value 0x200b_8000
impl crate::Resettable for PCGCCTLrs {
    const RESET_VALUE: u32 = 0x200b_8000;
}
