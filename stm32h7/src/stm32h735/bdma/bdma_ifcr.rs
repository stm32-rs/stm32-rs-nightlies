///Register `BDMA_IFCR` writer
pub type W = crate::W<BDMA_IFCRrs>;
///Field `CGIF0` writer - global interrupt flag clear for channel 0
pub type CGIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF0` writer - transfer complete flag clear for channel 0
pub type CTCIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF0` writer - half transfer flag clear for channel 0
pub type CHTIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF0` writer - transfer error flag clear for channel 0
pub type CTEIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF1` writer - global interrupt flag clear for channel 0
pub type CGIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF1` writer - transfer complete flag clear for channel 1
pub type CTCIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF1` writer - half transfer flag clear for channel 1
pub type CHTIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF1` writer - transfer error flag clear for channel 1
pub type CTEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF2` writer - global interrupt flag clear for channel 2
pub type CGIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF2` writer - transfer complete flag clear for channel 2
pub type CTCIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF2` writer - half transfer flag clear for channe2
pub type CHTIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF2` writer - transfer error flag clear for channel 2
pub type CTEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF3` writer - global interrupt flag clear for channel 3
pub type CGIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF3` writer - transfer complete flag clear for channel 3
pub type CTCIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF3` writer - half transfer flag clear for channel 3
pub type CHTIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF3` writer - transfer error flag clear for channel 3
pub type CTEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF4` writer - global interrupt flag clear for channel 4
pub type CGIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF4` writer - transfer complete flag clear for channel 4
pub type CTCIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF4` writer - half transfer flag clear for channel 4
pub type CHTIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF4` writer - transfer error flag clear for channel 4
pub type CTEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF5` writer - global interrupt flag clear for channel 5
pub type CGIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF5` writer - transfer complete flag clear for channel 5
pub type CTCIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF5` writer - half transfer flag clear for channel 5
pub type CHTIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF5` writer - transfer error flag clear for channel 5
pub type CTEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF6` writer - global interrupt flag clear for channel 6
pub type CGIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF6` writer - transfer complete flag clear for channel 6
pub type CTCIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF6` writer - half transfer flag clear for channel 6
pub type CHTIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF6` writer - transfer error flag clear for channel 6
pub type CTEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF7` writer - global interrupt flag clear for channel 7
pub type CGIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF7` writer - transfer complete flag clear for channel 7
pub type CTCIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF7` writer - half transfer flag clear for channel 7
pub type CHTIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF7` writer - transfer error flag clear for channel 7
pub type CTEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BDMA_IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - global interrupt flag clear for channel 0
    #[inline(always)]
    pub fn cgif0(&mut self) -> CGIF0_W<BDMA_IFCRrs> {
        CGIF0_W::new(self, 0)
    }
    ///Bit 1 - transfer complete flag clear for channel 0
    #[inline(always)]
    pub fn ctcif0(&mut self) -> CTCIF0_W<BDMA_IFCRrs> {
        CTCIF0_W::new(self, 1)
    }
    ///Bit 2 - half transfer flag clear for channel 0
    #[inline(always)]
    pub fn chtif0(&mut self) -> CHTIF0_W<BDMA_IFCRrs> {
        CHTIF0_W::new(self, 2)
    }
    ///Bit 3 - transfer error flag clear for channel 0
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF0_W<BDMA_IFCRrs> {
        CTEIF0_W::new(self, 3)
    }
    ///Bit 4 - global interrupt flag clear for channel 0
    #[inline(always)]
    pub fn cgif1(&mut self) -> CGIF1_W<BDMA_IFCRrs> {
        CGIF1_W::new(self, 4)
    }
    ///Bit 5 - transfer complete flag clear for channel 1
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W<BDMA_IFCRrs> {
        CTCIF1_W::new(self, 5)
    }
    ///Bit 6 - half transfer flag clear for channel 1
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W<BDMA_IFCRrs> {
        CHTIF1_W::new(self, 6)
    }
    ///Bit 7 - transfer error flag clear for channel 1
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W<BDMA_IFCRrs> {
        CTEIF1_W::new(self, 7)
    }
    ///Bit 8 - global interrupt flag clear for channel 2
    #[inline(always)]
    pub fn cgif2(&mut self) -> CGIF2_W<BDMA_IFCRrs> {
        CGIF2_W::new(self, 8)
    }
    ///Bit 9 - transfer complete flag clear for channel 2
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W<BDMA_IFCRrs> {
        CTCIF2_W::new(self, 9)
    }
    ///Bit 10 - half transfer flag clear for channe2
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W<BDMA_IFCRrs> {
        CHTIF2_W::new(self, 10)
    }
    ///Bit 11 - transfer error flag clear for channel 2
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W<BDMA_IFCRrs> {
        CTEIF2_W::new(self, 11)
    }
    ///Bit 12 - global interrupt flag clear for channel 3
    #[inline(always)]
    pub fn cgif3(&mut self) -> CGIF3_W<BDMA_IFCRrs> {
        CGIF3_W::new(self, 12)
    }
    ///Bit 13 - transfer complete flag clear for channel 3
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W<BDMA_IFCRrs> {
        CTCIF3_W::new(self, 13)
    }
    ///Bit 14 - half transfer flag clear for channel 3
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W<BDMA_IFCRrs> {
        CHTIF3_W::new(self, 14)
    }
    ///Bit 15 - transfer error flag clear for channel 3
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W<BDMA_IFCRrs> {
        CTEIF3_W::new(self, 15)
    }
    ///Bit 16 - global interrupt flag clear for channel 4
    #[inline(always)]
    pub fn cgif4(&mut self) -> CGIF4_W<BDMA_IFCRrs> {
        CGIF4_W::new(self, 16)
    }
    ///Bit 17 - transfer complete flag clear for channel 4
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W<BDMA_IFCRrs> {
        CTCIF4_W::new(self, 17)
    }
    ///Bit 18 - half transfer flag clear for channel 4
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W<BDMA_IFCRrs> {
        CHTIF4_W::new(self, 18)
    }
    ///Bit 19 - transfer error flag clear for channel 4
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W<BDMA_IFCRrs> {
        CTEIF4_W::new(self, 19)
    }
    ///Bit 20 - global interrupt flag clear for channel 5
    #[inline(always)]
    pub fn cgif5(&mut self) -> CGIF5_W<BDMA_IFCRrs> {
        CGIF5_W::new(self, 20)
    }
    ///Bit 21 - transfer complete flag clear for channel 5
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W<BDMA_IFCRrs> {
        CTCIF5_W::new(self, 21)
    }
    ///Bit 22 - half transfer flag clear for channel 5
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W<BDMA_IFCRrs> {
        CHTIF5_W::new(self, 22)
    }
    ///Bit 23 - transfer error flag clear for channel 5
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W<BDMA_IFCRrs> {
        CTEIF5_W::new(self, 23)
    }
    ///Bit 24 - global interrupt flag clear for channel 6
    #[inline(always)]
    pub fn cgif6(&mut self) -> CGIF6_W<BDMA_IFCRrs> {
        CGIF6_W::new(self, 24)
    }
    ///Bit 25 - transfer complete flag clear for channel 6
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W<BDMA_IFCRrs> {
        CTCIF6_W::new(self, 25)
    }
    ///Bit 26 - half transfer flag clear for channel 6
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W<BDMA_IFCRrs> {
        CHTIF6_W::new(self, 26)
    }
    ///Bit 27 - transfer error flag clear for channel 6
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W<BDMA_IFCRrs> {
        CTEIF6_W::new(self, 27)
    }
    ///Bit 28 - global interrupt flag clear for channel 7
    #[inline(always)]
    pub fn cgif7(&mut self) -> CGIF7_W<BDMA_IFCRrs> {
        CGIF7_W::new(self, 28)
    }
    ///Bit 29 - transfer complete flag clear for channel 7
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W<BDMA_IFCRrs> {
        CTCIF7_W::new(self, 29)
    }
    ///Bit 30 - half transfer flag clear for channel 7
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W<BDMA_IFCRrs> {
        CHTIF7_W::new(self, 30)
    }
    ///Bit 31 - transfer error flag clear for channel 7
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W<BDMA_IFCRrs> {
        CTEIF7_W::new(self, 31)
    }
}
/**BDMA interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdma_ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#BDMA:BDMA_IFCR)*/
pub struct BDMA_IFCRrs;
impl crate::RegisterSpec for BDMA_IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bdma_ifcr::W`](W) writer structure
impl crate::Writable for BDMA_IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BDMA_IFCR to value 0
impl crate::Resettable for BDMA_IFCRrs {
    const RESET_VALUE: u32 = 0;
}
