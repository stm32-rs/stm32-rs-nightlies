///Register `LIFCR` writer
pub type W = crate::W<LIFCRrs>;
///Field `CFEIF0` writer - CFEIF0
pub type CFEIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF0` writer - CDMEIF0
pub type CDMEIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF0` writer - CTEIF0
pub type CTEIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF0` writer - CHTIF0
pub type CHTIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF0` writer - CTCIF0
pub type CTCIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF1` writer - CFEIF1
pub type CFEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF1` writer - CDMEIF1
pub type CDMEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF1` writer - CTEIF1
pub type CTEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF1` writer - CHTIF1
pub type CHTIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF1` writer - CTCIF1
pub type CTCIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF2` writer - CFEIF2
pub type CFEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF2` writer - CDMEIF2
pub type CDMEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF2` writer - CTEIF2
pub type CTEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF2` writer - CHTIF2
pub type CHTIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF2` writer - CTCIF2
pub type CTCIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF3` writer - CFEIF3
pub type CFEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF3` writer - CDMEIF3
pub type CDMEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF3` writer - CTEIF3
pub type CTEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF3` writer - CHTIF3
pub type CHTIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF3` writer - CTCIF3
pub type CTCIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<LIFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CFEIF0
    #[inline(always)]
    pub fn cfeif0(&mut self) -> CFEIF0_W<'_, LIFCRrs> {
        CFEIF0_W::new(self, 0)
    }
    ///Bit 2 - CDMEIF0
    #[inline(always)]
    pub fn cdmeif0(&mut self) -> CDMEIF0_W<'_, LIFCRrs> {
        CDMEIF0_W::new(self, 2)
    }
    ///Bit 3 - CTEIF0
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF0_W<'_, LIFCRrs> {
        CTEIF0_W::new(self, 3)
    }
    ///Bit 4 - CHTIF0
    #[inline(always)]
    pub fn chtif0(&mut self) -> CHTIF0_W<'_, LIFCRrs> {
        CHTIF0_W::new(self, 4)
    }
    ///Bit 5 - CTCIF0
    #[inline(always)]
    pub fn ctcif0(&mut self) -> CTCIF0_W<'_, LIFCRrs> {
        CTCIF0_W::new(self, 5)
    }
    ///Bit 6 - CFEIF1
    #[inline(always)]
    pub fn cfeif1(&mut self) -> CFEIF1_W<'_, LIFCRrs> {
        CFEIF1_W::new(self, 6)
    }
    ///Bit 8 - CDMEIF1
    #[inline(always)]
    pub fn cdmeif1(&mut self) -> CDMEIF1_W<'_, LIFCRrs> {
        CDMEIF1_W::new(self, 8)
    }
    ///Bit 9 - CTEIF1
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W<'_, LIFCRrs> {
        CTEIF1_W::new(self, 9)
    }
    ///Bit 10 - CHTIF1
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W<'_, LIFCRrs> {
        CHTIF1_W::new(self, 10)
    }
    ///Bit 11 - CTCIF1
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W<'_, LIFCRrs> {
        CTCIF1_W::new(self, 11)
    }
    ///Bit 16 - CFEIF2
    #[inline(always)]
    pub fn cfeif2(&mut self) -> CFEIF2_W<'_, LIFCRrs> {
        CFEIF2_W::new(self, 16)
    }
    ///Bit 18 - CDMEIF2
    #[inline(always)]
    pub fn cdmeif2(&mut self) -> CDMEIF2_W<'_, LIFCRrs> {
        CDMEIF2_W::new(self, 18)
    }
    ///Bit 19 - CTEIF2
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W<'_, LIFCRrs> {
        CTEIF2_W::new(self, 19)
    }
    ///Bit 20 - CHTIF2
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W<'_, LIFCRrs> {
        CHTIF2_W::new(self, 20)
    }
    ///Bit 21 - CTCIF2
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W<'_, LIFCRrs> {
        CTCIF2_W::new(self, 21)
    }
    ///Bit 22 - CFEIF3
    #[inline(always)]
    pub fn cfeif3(&mut self) -> CFEIF3_W<'_, LIFCRrs> {
        CFEIF3_W::new(self, 22)
    }
    ///Bit 24 - CDMEIF3
    #[inline(always)]
    pub fn cdmeif3(&mut self) -> CDMEIF3_W<'_, LIFCRrs> {
        CDMEIF3_W::new(self, 24)
    }
    ///Bit 25 - CTEIF3
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W<'_, LIFCRrs> {
        CTEIF3_W::new(self, 25)
    }
    ///Bit 26 - CHTIF3
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W<'_, LIFCRrs> {
        CHTIF3_W::new(self, 26)
    }
    ///Bit 27 - CTCIF3
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W<'_, LIFCRrs> {
        CTCIF3_W::new(self, 27)
    }
}
/**DMA low interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DMA1:LIFCR)*/
pub struct LIFCRrs;
impl crate::RegisterSpec for LIFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lifcr::W`](W) writer structure
impl crate::Writable for LIFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LIFCR to value 0
impl crate::Resettable for LIFCRrs {}
