///Register `MEMRSTCR` writer
pub type W = crate::W<MEMRSTCRrs>;
///Field `AXISRAM3RSTC` writer - AXISRAM3 reset
pub type AXISRAM3RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4RSTC` writer - AXISRAM4 reset
pub type AXISRAM4RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5RSTC` writer - AXISRAM5 reset
pub type AXISRAM5RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6RSTC` writer - AXISRAM6 reset
pub type AXISRAM6RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1RSTC` writer - AHBSRAM1 reset
pub type AHBSRAM1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2RSTC` writer - AHBSRAM2 reset
pub type AHBSRAM2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1RSTC` writer - AXISRAM1 reset
pub type AXISRAM1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2RSTC` writer - AXISRAM2 reset
pub type AXISRAM2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMRSTC` writer - FLEXRAM reset
pub type FLEXRAMRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERAMRSTC` writer - NPUCACHERAM reset
pub type NPUCACHERAMRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMRSTC` writer - VENCRAM reset
pub type VENCRAMRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTROMRSTC` writer - BOOTROM reset
pub type BOOTROMRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MEMRSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - AXISRAM3 reset
    #[inline(always)]
    pub fn axisram3rstc(&mut self) -> AXISRAM3RSTC_W<'_, MEMRSTCRrs> {
        AXISRAM3RSTC_W::new(self, 0)
    }
    ///Bit 1 - AXISRAM4 reset
    #[inline(always)]
    pub fn axisram4rstc(&mut self) -> AXISRAM4RSTC_W<'_, MEMRSTCRrs> {
        AXISRAM4RSTC_W::new(self, 1)
    }
    ///Bit 2 - AXISRAM5 reset
    #[inline(always)]
    pub fn axisram5rstc(&mut self) -> AXISRAM5RSTC_W<'_, MEMRSTCRrs> {
        AXISRAM5RSTC_W::new(self, 2)
    }
    ///Bit 3 - AXISRAM6 reset
    #[inline(always)]
    pub fn axisram6rstc(&mut self) -> AXISRAM6RSTC_W<'_, MEMRSTCRrs> {
        AXISRAM6RSTC_W::new(self, 3)
    }
    ///Bit 4 - AHBSRAM1 reset
    #[inline(always)]
    pub fn ahbsram1rstc(&mut self) -> AHBSRAM1RSTC_W<'_, MEMRSTCRrs> {
        AHBSRAM1RSTC_W::new(self, 4)
    }
    ///Bit 5 - AHBSRAM2 reset
    #[inline(always)]
    pub fn ahbsram2rstc(&mut self) -> AHBSRAM2RSTC_W<'_, MEMRSTCRrs> {
        AHBSRAM2RSTC_W::new(self, 5)
    }
    ///Bit 7 - AXISRAM1 reset
    #[inline(always)]
    pub fn axisram1rstc(&mut self) -> AXISRAM1RSTC_W<'_, MEMRSTCRrs> {
        AXISRAM1RSTC_W::new(self, 7)
    }
    ///Bit 8 - AXISRAM2 reset
    #[inline(always)]
    pub fn axisram2rstc(&mut self) -> AXISRAM2RSTC_W<'_, MEMRSTCRrs> {
        AXISRAM2RSTC_W::new(self, 8)
    }
    ///Bit 9 - FLEXRAM reset
    #[inline(always)]
    pub fn flexramrstc(&mut self) -> FLEXRAMRSTC_W<'_, MEMRSTCRrs> {
        FLEXRAMRSTC_W::new(self, 9)
    }
    ///Bit 10 - NPUCACHERAM reset
    #[inline(always)]
    pub fn npucacheramrstc(&mut self) -> NPUCACHERAMRSTC_W<'_, MEMRSTCRrs> {
        NPUCACHERAMRSTC_W::new(self, 10)
    }
    ///Bit 11 - VENCRAM reset
    #[inline(always)]
    pub fn vencramrstc(&mut self) -> VENCRAMRSTC_W<'_, MEMRSTCRrs> {
        VENCRAMRSTC_W::new(self, 11)
    }
    ///Bit 12 - BOOTROM reset
    #[inline(always)]
    pub fn bootromrstc(&mut self) -> BOOTROMRSTC_W<'_, MEMRSTCRrs> {
        BOOTROMRSTC_W::new(self, 12)
    }
}
/**RCC memory reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:MEMRSTCR)*/
pub struct MEMRSTCRrs;
impl crate::RegisterSpec for MEMRSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`memrstcr::W`](W) writer structure
impl crate::Writable for MEMRSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMRSTCR to value 0
impl crate::Resettable for MEMRSTCRrs {}
