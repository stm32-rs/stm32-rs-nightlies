///Register `POISONCFG` reader
pub type R = crate::R<POISONCFGrs>;
///Register `POISONCFG` writer
pub type W = crate::W<POISONCFGrs>;
///Field `WR_POISON_SLVERR_EN` reader - WR_POISON_SLVERR_EN
pub type WR_POISON_SLVERR_EN_R = crate::BitReader;
///Field `WR_POISON_SLVERR_EN` writer - WR_POISON_SLVERR_EN
pub type WR_POISON_SLVERR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WR_POISON_INTR_EN` reader - WR_POISON_INTR_EN
pub type WR_POISON_INTR_EN_R = crate::BitReader;
///Field `WR_POISON_INTR_EN` writer - WR_POISON_INTR_EN
pub type WR_POISON_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WR_POISON_INTR_CLR` reader - WR_POISON_INTR_CLR
pub type WR_POISON_INTR_CLR_R = crate::BitReader;
///Field `WR_POISON_INTR_CLR` writer - WR_POISON_INTR_CLR
pub type WR_POISON_INTR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RD_POISON_SLVERR_EN` reader - RD_POISON_SLVERR_EN
pub type RD_POISON_SLVERR_EN_R = crate::BitReader;
///Field `RD_POISON_SLVERR_EN` writer - RD_POISON_SLVERR_EN
pub type RD_POISON_SLVERR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RD_POISON_INTR_EN` reader - RD_POISON_INTR_EN
pub type RD_POISON_INTR_EN_R = crate::BitReader;
///Field `RD_POISON_INTR_EN` writer - RD_POISON_INTR_EN
pub type RD_POISON_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RD_POISON_INTR_CLR` reader - RD_POISON_INTR_CLR
pub type RD_POISON_INTR_CLR_R = crate::BitReader;
///Field `RD_POISON_INTR_CLR` writer - RD_POISON_INTR_CLR
pub type RD_POISON_INTR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - WR_POISON_SLVERR_EN
    #[inline(always)]
    pub fn wr_poison_slverr_en(&self) -> WR_POISON_SLVERR_EN_R {
        WR_POISON_SLVERR_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - WR_POISON_INTR_EN
    #[inline(always)]
    pub fn wr_poison_intr_en(&self) -> WR_POISON_INTR_EN_R {
        WR_POISON_INTR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - WR_POISON_INTR_CLR
    #[inline(always)]
    pub fn wr_poison_intr_clr(&self) -> WR_POISON_INTR_CLR_R {
        WR_POISON_INTR_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - RD_POISON_SLVERR_EN
    #[inline(always)]
    pub fn rd_poison_slverr_en(&self) -> RD_POISON_SLVERR_EN_R {
        RD_POISON_SLVERR_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - RD_POISON_INTR_EN
    #[inline(always)]
    pub fn rd_poison_intr_en(&self) -> RD_POISON_INTR_EN_R {
        RD_POISON_INTR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - RD_POISON_INTR_CLR
    #[inline(always)]
    pub fn rd_poison_intr_clr(&self) -> RD_POISON_INTR_CLR_R {
        RD_POISON_INTR_CLR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POISONCFG")
            .field("wr_poison_slverr_en", &self.wr_poison_slverr_en())
            .field("wr_poison_intr_en", &self.wr_poison_intr_en())
            .field("wr_poison_intr_clr", &self.wr_poison_intr_clr())
            .field("rd_poison_slverr_en", &self.rd_poison_slverr_en())
            .field("rd_poison_intr_en", &self.rd_poison_intr_en())
            .field("rd_poison_intr_clr", &self.rd_poison_intr_clr())
            .finish()
    }
}
impl W {
    ///Bit 0 - WR_POISON_SLVERR_EN
    #[inline(always)]
    pub fn wr_poison_slverr_en(&mut self) -> WR_POISON_SLVERR_EN_W<'_, POISONCFGrs> {
        WR_POISON_SLVERR_EN_W::new(self, 0)
    }
    ///Bit 4 - WR_POISON_INTR_EN
    #[inline(always)]
    pub fn wr_poison_intr_en(&mut self) -> WR_POISON_INTR_EN_W<'_, POISONCFGrs> {
        WR_POISON_INTR_EN_W::new(self, 4)
    }
    ///Bit 8 - WR_POISON_INTR_CLR
    #[inline(always)]
    pub fn wr_poison_intr_clr(&mut self) -> WR_POISON_INTR_CLR_W<'_, POISONCFGrs> {
        WR_POISON_INTR_CLR_W::new(self, 8)
    }
    ///Bit 16 - RD_POISON_SLVERR_EN
    #[inline(always)]
    pub fn rd_poison_slverr_en(&mut self) -> RD_POISON_SLVERR_EN_W<'_, POISONCFGrs> {
        RD_POISON_SLVERR_EN_W::new(self, 16)
    }
    ///Bit 20 - RD_POISON_INTR_EN
    #[inline(always)]
    pub fn rd_poison_intr_en(&mut self) -> RD_POISON_INTR_EN_W<'_, POISONCFGrs> {
        RD_POISON_INTR_EN_W::new(self, 20)
    }
    ///Bit 24 - RD_POISON_INTR_CLR
    #[inline(always)]
    pub fn rd_poison_intr_clr(&mut self) -> RD_POISON_INTR_CLR_W<'_, POISONCFGrs> {
        RD_POISON_INTR_CLR_W::new(self, 24)
    }
}
/**AXI Poison configuration register common for all AXI ports.

You can [`read`](crate::Reg::read) this register and get [`poisoncfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poisoncfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:POISONCFG)*/
pub struct POISONCFGrs;
impl crate::RegisterSpec for POISONCFGrs {
    type Ux = u32;
}
///`read()` method returns [`poisoncfg::R`](R) reader structure
impl crate::Readable for POISONCFGrs {}
///`write(|w| ..)` method takes [`poisoncfg::W`](W) writer structure
impl crate::Writable for POISONCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POISONCFG to value 0x0011_0011
impl crate::Resettable for POISONCFGrs {
    const RESET_VALUE: u32 = 0x0011_0011;
}
