///Register `MEMENCR` writer
pub type W = crate::W<MEMENCRrs>;
///Field `AXISRAM3ENC` writer - AXISRAM3 enable
pub type AXISRAM3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4ENC` writer - AXISRAM4 enable
pub type AXISRAM4ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5ENC` writer - AXISRAM5 enable
pub type AXISRAM5ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6ENC` writer - AXISRAM6 enable
pub type AXISRAM6ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1ENC` writer - AHBSRAM1 enable
pub type AHBSRAM1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2ENC` writer - AHBSRAM2 enable
pub type AHBSRAM2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMENC` writer - BKPSRAM enable
pub type BKPSRAMENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1ENC` writer - AXISRAM1 enable
pub type AXISRAM1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2ENC` writer - AXISRAM2 enable
pub type AXISRAM2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMENC` writer - FLEXRAM enable
pub type FLEXRAMENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERAMENC` writer - NPUCACHERAM enable
pub type NPUCACHERAMENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMENC` writer - VENCRAM enable
pub type VENCRAMENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTROMENC` writer - BOOTROM enable
pub type BOOTROMENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MEMENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - AXISRAM3 enable
    #[inline(always)]
    pub fn axisram3enc(&mut self) -> AXISRAM3ENC_W<'_, MEMENCRrs> {
        AXISRAM3ENC_W::new(self, 0)
    }
    ///Bit 1 - AXISRAM4 enable
    #[inline(always)]
    pub fn axisram4enc(&mut self) -> AXISRAM4ENC_W<'_, MEMENCRrs> {
        AXISRAM4ENC_W::new(self, 1)
    }
    ///Bit 2 - AXISRAM5 enable
    #[inline(always)]
    pub fn axisram5enc(&mut self) -> AXISRAM5ENC_W<'_, MEMENCRrs> {
        AXISRAM5ENC_W::new(self, 2)
    }
    ///Bit 3 - AXISRAM6 enable
    #[inline(always)]
    pub fn axisram6enc(&mut self) -> AXISRAM6ENC_W<'_, MEMENCRrs> {
        AXISRAM6ENC_W::new(self, 3)
    }
    ///Bit 4 - AHBSRAM1 enable
    #[inline(always)]
    pub fn ahbsram1enc(&mut self) -> AHBSRAM1ENC_W<'_, MEMENCRrs> {
        AHBSRAM1ENC_W::new(self, 4)
    }
    ///Bit 5 - AHBSRAM2 enable
    #[inline(always)]
    pub fn ahbsram2enc(&mut self) -> AHBSRAM2ENC_W<'_, MEMENCRrs> {
        AHBSRAM2ENC_W::new(self, 5)
    }
    ///Bit 6 - BKPSRAM enable
    #[inline(always)]
    pub fn bkpsramenc(&mut self) -> BKPSRAMENC_W<'_, MEMENCRrs> {
        BKPSRAMENC_W::new(self, 6)
    }
    ///Bit 7 - AXISRAM1 enable
    #[inline(always)]
    pub fn axisram1enc(&mut self) -> AXISRAM1ENC_W<'_, MEMENCRrs> {
        AXISRAM1ENC_W::new(self, 7)
    }
    ///Bit 8 - AXISRAM2 enable
    #[inline(always)]
    pub fn axisram2enc(&mut self) -> AXISRAM2ENC_W<'_, MEMENCRrs> {
        AXISRAM2ENC_W::new(self, 8)
    }
    ///Bit 9 - FLEXRAM enable
    #[inline(always)]
    pub fn flexramenc(&mut self) -> FLEXRAMENC_W<'_, MEMENCRrs> {
        FLEXRAMENC_W::new(self, 9)
    }
    ///Bit 10 - NPUCACHERAM enable
    #[inline(always)]
    pub fn npucacheramenc(&mut self) -> NPUCACHERAMENC_W<'_, MEMENCRrs> {
        NPUCACHERAMENC_W::new(self, 10)
    }
    ///Bit 11 - VENCRAM enable
    #[inline(always)]
    pub fn vencramenc(&mut self) -> VENCRAMENC_W<'_, MEMENCRrs> {
        VENCRAMENC_W::new(self, 11)
    }
    ///Bit 12 - BOOTROM enable
    #[inline(always)]
    pub fn bootromenc(&mut self) -> BOOTROMENC_W<'_, MEMENCRrs> {
        BOOTROMENC_W::new(self, 12)
    }
}
/**RCC memory enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:MEMENCR)*/
pub struct MEMENCRrs;
impl crate::RegisterSpec for MEMENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`memencr::W`](W) writer structure
impl crate::Writable for MEMENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMENCR to value 0
impl crate::Resettable for MEMENCRrs {}
