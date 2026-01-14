///Register `HIFCR` writer
pub type W = crate::W<HIFCRrs>;
///Field `CFEIF4` writer - CFEIF4
pub type CFEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF4` writer - CDMEIF4
pub type CDMEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF4` writer - CTEIF4
pub type CTEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF4` writer - CHTIF4
pub type CHTIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF4` writer - CTCIF4
pub type CTCIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF5` writer - CFEIF5
pub type CFEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF5` writer - CDMEIF5
pub type CDMEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF5` writer - CTEIF5
pub type CTEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF5` writer - CHTIF5
pub type CHTIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF5` writer - CTCIF5
pub type CTCIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF6` writer - CFEIF6
pub type CFEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF6` writer - CDMEIF6
pub type CDMEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF6` writer - CTEIF6
pub type CTEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF6` writer - CHTIF6
pub type CHTIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF6` writer - CTCIF6
pub type CTCIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFEIF7` writer - CFEIF7
pub type CFEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDMEIF7` writer - CDMEIF7
pub type CDMEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF7` writer - CTEIF7
pub type CTEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF7` writer - CHTIF7
pub type CHTIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF7` writer - CTCIF7
pub type CTCIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<HIFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CFEIF4
    #[inline(always)]
    pub fn cfeif4(&mut self) -> CFEIF4_W<'_, HIFCRrs> {
        CFEIF4_W::new(self, 0)
    }
    ///Bit 2 - CDMEIF4
    #[inline(always)]
    pub fn cdmeif4(&mut self) -> CDMEIF4_W<'_, HIFCRrs> {
        CDMEIF4_W::new(self, 2)
    }
    ///Bit 3 - CTEIF4
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W<'_, HIFCRrs> {
        CTEIF4_W::new(self, 3)
    }
    ///Bit 4 - CHTIF4
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W<'_, HIFCRrs> {
        CHTIF4_W::new(self, 4)
    }
    ///Bit 5 - CTCIF4
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W<'_, HIFCRrs> {
        CTCIF4_W::new(self, 5)
    }
    ///Bit 6 - CFEIF5
    #[inline(always)]
    pub fn cfeif5(&mut self) -> CFEIF5_W<'_, HIFCRrs> {
        CFEIF5_W::new(self, 6)
    }
    ///Bit 8 - CDMEIF5
    #[inline(always)]
    pub fn cdmeif5(&mut self) -> CDMEIF5_W<'_, HIFCRrs> {
        CDMEIF5_W::new(self, 8)
    }
    ///Bit 9 - CTEIF5
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W<'_, HIFCRrs> {
        CTEIF5_W::new(self, 9)
    }
    ///Bit 10 - CHTIF5
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W<'_, HIFCRrs> {
        CHTIF5_W::new(self, 10)
    }
    ///Bit 11 - CTCIF5
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W<'_, HIFCRrs> {
        CTCIF5_W::new(self, 11)
    }
    ///Bit 16 - CFEIF6
    #[inline(always)]
    pub fn cfeif6(&mut self) -> CFEIF6_W<'_, HIFCRrs> {
        CFEIF6_W::new(self, 16)
    }
    ///Bit 18 - CDMEIF6
    #[inline(always)]
    pub fn cdmeif6(&mut self) -> CDMEIF6_W<'_, HIFCRrs> {
        CDMEIF6_W::new(self, 18)
    }
    ///Bit 19 - CTEIF6
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W<'_, HIFCRrs> {
        CTEIF6_W::new(self, 19)
    }
    ///Bit 20 - CHTIF6
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W<'_, HIFCRrs> {
        CHTIF6_W::new(self, 20)
    }
    ///Bit 21 - CTCIF6
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W<'_, HIFCRrs> {
        CTCIF6_W::new(self, 21)
    }
    ///Bit 22 - CFEIF7
    #[inline(always)]
    pub fn cfeif7(&mut self) -> CFEIF7_W<'_, HIFCRrs> {
        CFEIF7_W::new(self, 22)
    }
    ///Bit 24 - CDMEIF7
    #[inline(always)]
    pub fn cdmeif7(&mut self) -> CDMEIF7_W<'_, HIFCRrs> {
        CDMEIF7_W::new(self, 24)
    }
    ///Bit 25 - CTEIF7
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W<'_, HIFCRrs> {
        CTEIF7_W::new(self, 25)
    }
    ///Bit 26 - CHTIF7
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W<'_, HIFCRrs> {
        CHTIF7_W::new(self, 26)
    }
    ///Bit 27 - CTCIF7
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W<'_, HIFCRrs> {
        CTCIF7_W::new(self, 27)
    }
}
/**DMA high interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DMA1:HIFCR)*/
pub struct HIFCRrs;
impl crate::RegisterSpec for HIFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`hifcr::W`](W) writer structure
impl crate::Writable for HIFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HIFCR to value 0
impl crate::Resettable for HIFCRrs {}
