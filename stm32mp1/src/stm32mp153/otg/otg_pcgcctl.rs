#[doc = "Register `OTG_PCGCCTL` reader"]
pub type R = crate::R<OTG_PCGCCTLrs>;
#[doc = "Register `OTG_PCGCCTL` writer"]
pub type W = crate::W<OTG_PCGCCTLrs>;
#[doc = "Field `STPPCLK` reader - STPPCLK"]
pub type STPPCLK_R = crate::BitReader;
#[doc = "Field `STPPCLK` writer - STPPCLK"]
pub type STPPCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GATEHCLK` reader - GATEHCLK"]
pub type GATEHCLK_R = crate::BitReader;
#[doc = "Field `GATEHCLK` writer - GATEHCLK"]
pub type GATEHCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSUSP` reader - PHYSUSP"]
pub type PHYSUSP_R = crate::BitReader;
#[doc = "Field `ENL1GTG` reader - ENL1GTG"]
pub type ENL1GTG_R = crate::BitReader;
#[doc = "Field `ENL1GTG` writer - ENL1GTG"]
pub type ENL1GTG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSLEEP` reader - PHYSLEEP"]
pub type PHYSLEEP_R = crate::BitReader;
#[doc = "Field `SUSP` reader - SUSP"]
pub type SUSP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - STPPCLK"]
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GATEHCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - PHYSUSP"]
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ENL1GTG"]
    #[inline(always)]
    pub fn enl1gtg(&self) -> ENL1GTG_R {
        ENL1GTG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PHYSLEEP"]
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SUSP"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STPPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn stppclk(&mut self) -> STPPCLK_W<OTG_PCGCCTLrs> {
        STPPCLK_W::new(self, 0)
    }
    #[doc = "Bit 1 - GATEHCLK"]
    #[inline(always)]
    #[must_use]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<OTG_PCGCCTLrs> {
        GATEHCLK_W::new(self, 1)
    }
    #[doc = "Bit 5 - ENL1GTG"]
    #[inline(always)]
    #[must_use]
    pub fn enl1gtg(&mut self) -> ENL1GTG_W<OTG_PCGCCTLrs> {
        ENL1GTG_W::new(self, 5)
    }
}
#[doc = "This register is available in host and device modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_pcgcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_pcgcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_PCGCCTLrs;
impl crate::RegisterSpec for OTG_PCGCCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_pcgcctl::R`](R) reader structure"]
impl crate::Readable for OTG_PCGCCTLrs {}
#[doc = "`write(|w| ..)` method takes [`otg_pcgcctl::W`](W) writer structure"]
impl crate::Writable for OTG_PCGCCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_PCGCCTL to value 0x200b_8000"]
impl crate::Resettable for OTG_PCGCCTLrs {
    const RESET_VALUE: u32 = 0x200b_8000;
}
