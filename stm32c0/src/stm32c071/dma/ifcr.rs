///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
///Field `CGIF1` writer - Global interrupt flag clear for channel 1
pub type CGIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF1` writer - Transfer complete flag clear for channel 1
pub type CTCIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF1` writer - Half transfer flag clear for channel 1
pub type CHTIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF1` writer - Transfer error flag clear for channel 1
pub type CTEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF2` writer - Global interrupt flag clear for channel 2
pub type CGIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF2` writer - Transfer complete flag clear for channel 2
pub type CTCIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF2` writer - Half transfer flag clear for channel 2
pub type CHTIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF2` writer - Transfer error flag clear for channel 2
pub type CTEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF3` writer - Global interrupt flag clear for channel 3
pub type CGIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF3` writer - Transfer complete flag clear for channel 3
pub type CTCIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF3` writer - Half transfer flag clear for channel 3
pub type CHTIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF3` writer - Transfer error flag clear for channel 3
pub type CTEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF4` writer - Global interrupt flag clear for channel 4
pub type CGIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF4` writer - Transfer complete flag clear for channel 4
pub type CTCIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF4` writer - Half transfer flag clear for channel 4
pub type CHTIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF4` writer - Transfer error flag clear for channel 4
pub type CTEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF5` writer - Global interrupt flag clear for channel 5
pub type CGIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF5` writer - Transfer complete flag clear for channel 5
pub type CTCIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF5` writer - Half transfer flag clear for channel 5
pub type CHTIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF5` writer - Transfer error flag clear for channel 5
pub type CTEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Global interrupt flag clear for channel 1
    #[inline(always)]
    pub fn cgif1(&mut self) -> CGIF1_W<IFCRrs> {
        CGIF1_W::new(self, 0)
    }
    ///Bit 1 - Transfer complete flag clear for channel 1
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W<IFCRrs> {
        CTCIF1_W::new(self, 1)
    }
    ///Bit 2 - Half transfer flag clear for channel 1
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W<IFCRrs> {
        CHTIF1_W::new(self, 2)
    }
    ///Bit 3 - Transfer error flag clear for channel 1
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W<IFCRrs> {
        CTEIF1_W::new(self, 3)
    }
    ///Bit 4 - Global interrupt flag clear for channel 2
    #[inline(always)]
    pub fn cgif2(&mut self) -> CGIF2_W<IFCRrs> {
        CGIF2_W::new(self, 4)
    }
    ///Bit 5 - Transfer complete flag clear for channel 2
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W<IFCRrs> {
        CTCIF2_W::new(self, 5)
    }
    ///Bit 6 - Half transfer flag clear for channel 2
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W<IFCRrs> {
        CHTIF2_W::new(self, 6)
    }
    ///Bit 7 - Transfer error flag clear for channel 2
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W<IFCRrs> {
        CTEIF2_W::new(self, 7)
    }
    ///Bit 8 - Global interrupt flag clear for channel 3
    #[inline(always)]
    pub fn cgif3(&mut self) -> CGIF3_W<IFCRrs> {
        CGIF3_W::new(self, 8)
    }
    ///Bit 9 - Transfer complete flag clear for channel 3
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W<IFCRrs> {
        CTCIF3_W::new(self, 9)
    }
    ///Bit 10 - Half transfer flag clear for channel 3
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W<IFCRrs> {
        CHTIF3_W::new(self, 10)
    }
    ///Bit 11 - Transfer error flag clear for channel 3
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W<IFCRrs> {
        CTEIF3_W::new(self, 11)
    }
    ///Bit 12 - Global interrupt flag clear for channel 4
    #[inline(always)]
    pub fn cgif4(&mut self) -> CGIF4_W<IFCRrs> {
        CGIF4_W::new(self, 12)
    }
    ///Bit 13 - Transfer complete flag clear for channel 4
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W<IFCRrs> {
        CTCIF4_W::new(self, 13)
    }
    ///Bit 14 - Half transfer flag clear for channel 4
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W<IFCRrs> {
        CHTIF4_W::new(self, 14)
    }
    ///Bit 15 - Transfer error flag clear for channel 4
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W<IFCRrs> {
        CTEIF4_W::new(self, 15)
    }
    ///Bit 16 - Global interrupt flag clear for channel 5
    #[inline(always)]
    pub fn cgif5(&mut self) -> CGIF5_W<IFCRrs> {
        CGIF5_W::new(self, 16)
    }
    ///Bit 17 - Transfer complete flag clear for channel 5
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W<IFCRrs> {
        CTCIF5_W::new(self, 17)
    }
    ///Bit 18 - Half transfer flag clear for channel 5
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W<IFCRrs> {
        CHTIF5_W::new(self, 18)
    }
    ///Bit 19 - Transfer error flag clear for channel 5
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W<IFCRrs> {
        CTEIF5_W::new(self, 19)
    }
}
/**DMA interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#DMA:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}
