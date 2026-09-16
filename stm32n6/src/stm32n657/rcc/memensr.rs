///Register `MEMENSR` writer
pub type W = crate::W<MEMENSRrs>;
///Field `AXISRAM3ENS` writer - AXISRAM3 enable
pub type AXISRAM3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4ENS` writer - AXISRAM4 enable
pub type AXISRAM4ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5ENS` writer - AXISRAM5 enable
pub type AXISRAM5ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6ENS` writer - AXISRAM6 enable
pub type AXISRAM6ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1ENS` writer - AHBSRAM1 enable
pub type AHBSRAM1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2ENS` writer - AHBSRAM2 enable
pub type AHBSRAM2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMENS` writer - BKPSRAM enable
pub type BKPSRAMENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1ENS` writer - AXISRAM1 enable
pub type AXISRAM1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2ENS` writer - AXISRAM2 enable
pub type AXISRAM2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMENS` writer - FLEXRAM enable
pub type FLEXRAMENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERAMENS` writer - NPUCACHERAM enable
pub type NPUCACHERAMENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMENS` writer - VENCRAM enable
pub type VENCRAMENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTROMENS` writer - BOOTROM enable
pub type BOOTROMENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MEMENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - AXISRAM3 enable
    #[inline(always)]
    pub fn axisram3ens(&mut self) -> AXISRAM3ENS_W<'_, MEMENSRrs> {
        AXISRAM3ENS_W::new(self, 0)
    }
    ///Bit 1 - AXISRAM4 enable
    #[inline(always)]
    pub fn axisram4ens(&mut self) -> AXISRAM4ENS_W<'_, MEMENSRrs> {
        AXISRAM4ENS_W::new(self, 1)
    }
    ///Bit 2 - AXISRAM5 enable
    #[inline(always)]
    pub fn axisram5ens(&mut self) -> AXISRAM5ENS_W<'_, MEMENSRrs> {
        AXISRAM5ENS_W::new(self, 2)
    }
    ///Bit 3 - AXISRAM6 enable
    #[inline(always)]
    pub fn axisram6ens(&mut self) -> AXISRAM6ENS_W<'_, MEMENSRrs> {
        AXISRAM6ENS_W::new(self, 3)
    }
    ///Bit 4 - AHBSRAM1 enable
    #[inline(always)]
    pub fn ahbsram1ens(&mut self) -> AHBSRAM1ENS_W<'_, MEMENSRrs> {
        AHBSRAM1ENS_W::new(self, 4)
    }
    ///Bit 5 - AHBSRAM2 enable
    #[inline(always)]
    pub fn ahbsram2ens(&mut self) -> AHBSRAM2ENS_W<'_, MEMENSRrs> {
        AHBSRAM2ENS_W::new(self, 5)
    }
    ///Bit 6 - BKPSRAM enable
    #[inline(always)]
    pub fn bkpsramens(&mut self) -> BKPSRAMENS_W<'_, MEMENSRrs> {
        BKPSRAMENS_W::new(self, 6)
    }
    ///Bit 7 - AXISRAM1 enable
    #[inline(always)]
    pub fn axisram1ens(&mut self) -> AXISRAM1ENS_W<'_, MEMENSRrs> {
        AXISRAM1ENS_W::new(self, 7)
    }
    ///Bit 8 - AXISRAM2 enable
    #[inline(always)]
    pub fn axisram2ens(&mut self) -> AXISRAM2ENS_W<'_, MEMENSRrs> {
        AXISRAM2ENS_W::new(self, 8)
    }
    ///Bit 9 - FLEXRAM enable
    #[inline(always)]
    pub fn flexramens(&mut self) -> FLEXRAMENS_W<'_, MEMENSRrs> {
        FLEXRAMENS_W::new(self, 9)
    }
    ///Bit 10 - NPUCACHERAM enable
    #[inline(always)]
    pub fn npucacheramens(&mut self) -> NPUCACHERAMENS_W<'_, MEMENSRrs> {
        NPUCACHERAMENS_W::new(self, 10)
    }
    ///Bit 11 - VENCRAM enable
    #[inline(always)]
    pub fn vencramens(&mut self) -> VENCRAMENS_W<'_, MEMENSRrs> {
        VENCRAMENS_W::new(self, 11)
    }
    ///Bit 12 - BOOTROM enable
    #[inline(always)]
    pub fn bootromens(&mut self) -> BOOTROMENS_W<'_, MEMENSRrs> {
        BOOTROMENS_W::new(self, 12)
    }
}
/**RCC memory enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:MEMENSR)*/
pub struct MEMENSRrs;
impl crate::RegisterSpec for MEMENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`memensr::W`](W) writer structure
impl crate::Writable for MEMENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMENSR to value 0
impl crate::Resettable for MEMENSRrs {}
