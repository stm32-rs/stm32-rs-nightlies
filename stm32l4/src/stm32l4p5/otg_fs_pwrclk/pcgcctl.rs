#[doc = "Register `PCGCCTL` reader"]
pub type R = crate::R<PCGCCTLrs>;
#[doc = "Register `PCGCCTL` writer"]
pub type W = crate::W<PCGCCTLrs>;
#[doc = "Field `STPPCLK` reader - Stop PHY clock"]
pub type STPPCLK_R = crate::BitReader;
#[doc = "Field `STPPCLK` writer - Stop PHY clock"]
pub type STPPCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GATEHCLK` reader - Gate HCLK"]
pub type GATEHCLK_R = crate::BitReader;
#[doc = "Field `GATEHCLK` writer - Gate HCLK"]
pub type GATEHCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSUSP` reader - PHY Suspended"]
pub type PHYSUSP_R = crate::BitReader;
#[doc = "Field `PHYSUSP` writer - PHY Suspended"]
pub type PHYSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENL1GTG` reader - Enable sleep clock gating"]
pub type ENL1GTG_R = crate::BitReader;
#[doc = "Field `ENL1GTG` writer - Enable sleep clock gating"]
pub type ENL1GTG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSLEEP` reader - PHY in Sleep"]
pub type PHYSLEEP_R = crate::BitReader;
#[doc = "Field `PHYSLEEP` writer - PHY in Sleep"]
pub type PHYSLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - Deep Sleep"]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - Deep Sleep"]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable sleep clock gating"]
    #[inline(always)]
    pub fn enl1gtg(&self) -> ENL1GTG_R {
        ENL1GTG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PHY in Sleep"]
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Deep Sleep"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    #[must_use]
    pub fn stppclk(&mut self) -> STPPCLK_W<PCGCCTLrs> {
        STPPCLK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<PCGCCTLrs> {
        GATEHCLK_W::new(self, 1)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    #[must_use]
    pub fn physusp(&mut self) -> PHYSUSP_W<PCGCCTLrs> {
        PHYSUSP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable sleep clock gating"]
    #[inline(always)]
    #[must_use]
    pub fn enl1gtg(&mut self) -> ENL1GTG_W<PCGCCTLrs> {
        ENL1GTG_W::new(self, 5)
    }
    #[doc = "Bit 6 - PHY in Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn physleep(&mut self) -> PHYSLEEP_W<PCGCCTLrs> {
        PHYSLEEP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Deep Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<PCGCCTLrs> {
        SUSP_W::new(self, 7)
    }
}
#[doc = "OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCGCCTLrs;
impl crate::RegisterSpec for PCGCCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcgcctl::R`](R) reader structure"]
impl crate::Readable for PCGCCTLrs {}
#[doc = "`write(|w| ..)` method takes [`pcgcctl::W`](W) writer structure"]
impl crate::Writable for PCGCCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCGCCTL to value 0"]
impl crate::Resettable for PCGCCTLrs {
    const RESET_VALUE: u32 = 0;
}
