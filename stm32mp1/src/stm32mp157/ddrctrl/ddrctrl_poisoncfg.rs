#[doc = "Register `DDRCTRL_POISONCFG` reader"]
pub type R = crate::R<DDRCTRL_POISONCFGrs>;
#[doc = "Register `DDRCTRL_POISONCFG` writer"]
pub type W = crate::W<DDRCTRL_POISONCFGrs>;
#[doc = "Field `WR_POISON_SLVERR_EN` reader - WR_POISON_SLVERR_EN"]
pub type WR_POISON_SLVERR_EN_R = crate::BitReader;
#[doc = "Field `WR_POISON_SLVERR_EN` writer - WR_POISON_SLVERR_EN"]
pub type WR_POISON_SLVERR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_POISON_INTR_EN` reader - WR_POISON_INTR_EN"]
pub type WR_POISON_INTR_EN_R = crate::BitReader;
#[doc = "Field `WR_POISON_INTR_EN` writer - WR_POISON_INTR_EN"]
pub type WR_POISON_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_POISON_INTR_CLR` reader - WR_POISON_INTR_CLR"]
pub type WR_POISON_INTR_CLR_R = crate::BitReader;
#[doc = "Field `WR_POISON_INTR_CLR` writer - WR_POISON_INTR_CLR"]
pub type WR_POISON_INTR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_POISON_SLVERR_EN` reader - RD_POISON_SLVERR_EN"]
pub type RD_POISON_SLVERR_EN_R = crate::BitReader;
#[doc = "Field `RD_POISON_SLVERR_EN` writer - RD_POISON_SLVERR_EN"]
pub type RD_POISON_SLVERR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_POISON_INTR_EN` reader - RD_POISON_INTR_EN"]
pub type RD_POISON_INTR_EN_R = crate::BitReader;
#[doc = "Field `RD_POISON_INTR_EN` writer - RD_POISON_INTR_EN"]
pub type RD_POISON_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_POISON_INTR_CLR` reader - RD_POISON_INTR_CLR"]
pub type RD_POISON_INTR_CLR_R = crate::BitReader;
#[doc = "Field `RD_POISON_INTR_CLR` writer - RD_POISON_INTR_CLR"]
pub type RD_POISON_INTR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WR_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn wr_poison_slverr_en(&self) -> WR_POISON_SLVERR_EN_R {
        WR_POISON_SLVERR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - WR_POISON_INTR_EN"]
    #[inline(always)]
    pub fn wr_poison_intr_en(&self) -> WR_POISON_INTR_EN_R {
        WR_POISON_INTR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - WR_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn wr_poison_intr_clr(&self) -> WR_POISON_INTR_CLR_R {
        WR_POISON_INTR_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - RD_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn rd_poison_slverr_en(&self) -> RD_POISON_SLVERR_EN_R {
        RD_POISON_SLVERR_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - RD_POISON_INTR_EN"]
    #[inline(always)]
    pub fn rd_poison_intr_en(&self) -> RD_POISON_INTR_EN_R {
        RD_POISON_INTR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - RD_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn rd_poison_intr_clr(&self) -> RD_POISON_INTR_CLR_R {
        RD_POISON_INTR_CLR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WR_POISON_SLVERR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn wr_poison_slverr_en(&mut self) -> WR_POISON_SLVERR_EN_W<DDRCTRL_POISONCFGrs> {
        WR_POISON_SLVERR_EN_W::new(self, 0)
    }
    #[doc = "Bit 4 - WR_POISON_INTR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn wr_poison_intr_en(&mut self) -> WR_POISON_INTR_EN_W<DDRCTRL_POISONCFGrs> {
        WR_POISON_INTR_EN_W::new(self, 4)
    }
    #[doc = "Bit 8 - WR_POISON_INTR_CLR"]
    #[inline(always)]
    #[must_use]
    pub fn wr_poison_intr_clr(&mut self) -> WR_POISON_INTR_CLR_W<DDRCTRL_POISONCFGrs> {
        WR_POISON_INTR_CLR_W::new(self, 8)
    }
    #[doc = "Bit 16 - RD_POISON_SLVERR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rd_poison_slverr_en(&mut self) -> RD_POISON_SLVERR_EN_W<DDRCTRL_POISONCFGrs> {
        RD_POISON_SLVERR_EN_W::new(self, 16)
    }
    #[doc = "Bit 20 - RD_POISON_INTR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rd_poison_intr_en(&mut self) -> RD_POISON_INTR_EN_W<DDRCTRL_POISONCFGrs> {
        RD_POISON_INTR_EN_W::new(self, 20)
    }
    #[doc = "Bit 24 - RD_POISON_INTR_CLR"]
    #[inline(always)]
    #[must_use]
    pub fn rd_poison_intr_clr(&mut self) -> RD_POISON_INTR_CLR_W<DDRCTRL_POISONCFGrs> {
        RD_POISON_INTR_CLR_W::new(self, 24)
    }
}
#[doc = "AXI Poison configuration register common for all AXI ports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_poisoncfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_poisoncfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_POISONCFGrs;
impl crate::RegisterSpec for DDRCTRL_POISONCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_poisoncfg::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_POISONCFGrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_poisoncfg::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_POISONCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_POISONCFG to value 0x0011_0011"]
impl crate::Resettable for DDRCTRL_POISONCFGrs {
    const RESET_VALUE: u32 = 0x0011_0011;
}
