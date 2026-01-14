///Register `MEMRSTSR` writer
pub type W = crate::W<MEMRSTSRrs>;
///Field `AXISRAM3RSTS` writer - AXISRAM3 reset
pub type AXISRAM3RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4RSTS` writer - AXISRAM4 reset
pub type AXISRAM4RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5RSTS` writer - AXISRAM5 reset
pub type AXISRAM5RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6RSTS` writer - AXISRAM6 reset
pub type AXISRAM6RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1RSTS` writer - AHBSRAM1 reset
pub type AHBSRAM1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2RSTS` writer - AHBSRAM2 reset
pub type AHBSRAM2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1RSTS` writer - AXISRAM1 reset
pub type AXISRAM1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2RSTS` writer - AXISRAM2 reset
pub type AXISRAM2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMRSTS` writer - FLEXRAM reset
pub type FLEXRAMRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERAMRSTS` writer - NPUCACHERAM reset
pub type NPUCACHERAMRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMRSTS` writer - VENCRAM reset
pub type VENCRAMRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTROMRSTS` writer - BOOTROM reset
pub type BOOTROMRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MEMRSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - AXISRAM3 reset
    #[inline(always)]
    pub fn axisram3rsts(&mut self) -> AXISRAM3RSTS_W<'_, MEMRSTSRrs> {
        AXISRAM3RSTS_W::new(self, 0)
    }
    ///Bit 1 - AXISRAM4 reset
    #[inline(always)]
    pub fn axisram4rsts(&mut self) -> AXISRAM4RSTS_W<'_, MEMRSTSRrs> {
        AXISRAM4RSTS_W::new(self, 1)
    }
    ///Bit 2 - AXISRAM5 reset
    #[inline(always)]
    pub fn axisram5rsts(&mut self) -> AXISRAM5RSTS_W<'_, MEMRSTSRrs> {
        AXISRAM5RSTS_W::new(self, 2)
    }
    ///Bit 3 - AXISRAM6 reset
    #[inline(always)]
    pub fn axisram6rsts(&mut self) -> AXISRAM6RSTS_W<'_, MEMRSTSRrs> {
        AXISRAM6RSTS_W::new(self, 3)
    }
    ///Bit 4 - AHBSRAM1 reset
    #[inline(always)]
    pub fn ahbsram1rsts(&mut self) -> AHBSRAM1RSTS_W<'_, MEMRSTSRrs> {
        AHBSRAM1RSTS_W::new(self, 4)
    }
    ///Bit 5 - AHBSRAM2 reset
    #[inline(always)]
    pub fn ahbsram2rsts(&mut self) -> AHBSRAM2RSTS_W<'_, MEMRSTSRrs> {
        AHBSRAM2RSTS_W::new(self, 5)
    }
    ///Bit 7 - AXISRAM1 reset
    #[inline(always)]
    pub fn axisram1rsts(&mut self) -> AXISRAM1RSTS_W<'_, MEMRSTSRrs> {
        AXISRAM1RSTS_W::new(self, 7)
    }
    ///Bit 8 - AXISRAM2 reset
    #[inline(always)]
    pub fn axisram2rsts(&mut self) -> AXISRAM2RSTS_W<'_, MEMRSTSRrs> {
        AXISRAM2RSTS_W::new(self, 8)
    }
    ///Bit 9 - FLEXRAM reset
    #[inline(always)]
    pub fn flexramrsts(&mut self) -> FLEXRAMRSTS_W<'_, MEMRSTSRrs> {
        FLEXRAMRSTS_W::new(self, 9)
    }
    ///Bit 10 - NPUCACHERAM reset
    #[inline(always)]
    pub fn npucacheramrsts(&mut self) -> NPUCACHERAMRSTS_W<'_, MEMRSTSRrs> {
        NPUCACHERAMRSTS_W::new(self, 10)
    }
    ///Bit 11 - VENCRAM reset
    #[inline(always)]
    pub fn vencramrsts(&mut self) -> VENCRAMRSTS_W<'_, MEMRSTSRrs> {
        VENCRAMRSTS_W::new(self, 11)
    }
    ///Bit 12 - BOOTROM reset
    #[inline(always)]
    pub fn bootromrsts(&mut self) -> BOOTROMRSTS_W<'_, MEMRSTSRrs> {
        BOOTROMRSTS_W::new(self, 12)
    }
}
/**RCC memory reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:MEMRSTSR)*/
pub struct MEMRSTSRrs;
impl crate::RegisterSpec for MEMRSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`memrstsr::W`](W) writer structure
impl crate::Writable for MEMRSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMRSTSR to value 0
impl crate::Resettable for MEMRSTSRrs {}
