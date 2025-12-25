///Register `PCR` reader
pub type R = crate::R<PCRrs>;
///Register `PCR` writer
pub type W = crate::W<PCRrs>;
///Field `PWRDOWN` reader - Power down
pub type PWRDOWN_R = crate::BitReader;
///Field `PWRDOWN` writer - Power down
pub type PWRDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLEN` reader - Clock lane enable
pub type CLEN_R = crate::BitReader;
///Field `CLEN` writer - Clock lane enable
pub type CLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DL0EN` reader - D-PHY_RX data lane 0 enable
pub type DL0EN_R = crate::BitReader;
///Field `DL0EN` writer - D-PHY_RX data lane 0 enable
pub type DL0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DL1EN` reader - D-PHY_RX data lane 1 enable
pub type DL1EN_R = crate::BitReader;
///Field `DL1EN` writer - D-PHY_RX data lane 1 enable
pub type DL1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Power down
    #[inline(always)]
    pub fn pwrdown(&self) -> PWRDOWN_R {
        PWRDOWN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock lane enable
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - D-PHY_RX data lane 0 enable
    #[inline(always)]
    pub fn dl0en(&self) -> DL0EN_R {
        DL0EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - D-PHY_RX data lane 1 enable
    #[inline(always)]
    pub fn dl1en(&self) -> DL1EN_R {
        DL1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCR")
            .field("pwrdown", &self.pwrdown())
            .field("clen", &self.clen())
            .field("dl0en", &self.dl0en())
            .field("dl1en", &self.dl1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power down
    #[inline(always)]
    pub fn pwrdown(&mut self) -> PWRDOWN_W<'_, PCRrs> {
        PWRDOWN_W::new(self, 0)
    }
    ///Bit 1 - Clock lane enable
    #[inline(always)]
    pub fn clen(&mut self) -> CLEN_W<'_, PCRrs> {
        CLEN_W::new(self, 1)
    }
    ///Bit 2 - D-PHY_RX data lane 0 enable
    #[inline(always)]
    pub fn dl0en(&mut self) -> DL0EN_W<'_, PCRrs> {
        DL0EN_W::new(self, 2)
    }
    ///Bit 3 - D-PHY_RX data lane 1 enable
    #[inline(always)]
    pub fn dl1en(&mut self) -> DL1EN_W<'_, PCRrs> {
        DL1EN_W::new(self, 3)
    }
}
/**CSI-2 Host DPHY_RX control register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#CSI:PCR)*/
pub struct PCRrs;
impl crate::RegisterSpec for PCRrs {
    type Ux = u32;
}
///`read()` method returns [`pcr::R`](R) reader structure
impl crate::Readable for PCRrs {}
///`write(|w| ..)` method takes [`pcr::W`](W) writer structure
impl crate::Writable for PCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCR to value 0
impl crate::Resettable for PCRrs {}
