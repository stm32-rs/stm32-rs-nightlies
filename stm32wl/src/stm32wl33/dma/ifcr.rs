///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
///Field `CGIF1` writer - CGIF1: Channel 1 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
pub type CGIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF1` writer - CTCIF1: Channel 1 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
pub type CTCIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF1` writer - CHTIF1: Channel 1 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register
pub type CHTIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF1` writer - CTEIF1: Channel 1 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
pub type CTEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF2` writer - CGIF2: Channel 2 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
pub type CGIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF2` writer - CTCIF2: Channel 2 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
pub type CTCIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF2` writer - CHTIF2: Channel 2 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register
pub type CHTIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF2` writer - CTEIF2: Channel 2 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
pub type CTEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF3` writer - CGIF3: Channel 3 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
pub type CGIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF3` writer - CTCIF3: Channel 3 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
pub type CTCIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF3` writer - CHTIF3: Channel 3 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register
pub type CHTIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF3` writer - CTEIF3: Channel 3 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
pub type CTEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF4` writer - CGIF4: Channel 4 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
pub type CGIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF4` writer - CTCIF4: Channel 4 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
pub type CTCIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF4` writer - CHTIF4: Channel 4 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register
pub type CHTIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF4` writer - CTEIF4: Channel 4 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
pub type CTEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF5` writer - CGIF5: Channel 5 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
pub type CGIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF5` writer - CTCIF5: Channel 5 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
pub type CTCIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF5` writer - CHTIF5: Channel 5 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register
pub type CHTIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF5` writer - CTEIF5: Channel 5 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
pub type CTEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF6` writer - CGIF6: Channel 6 global interrupt clear This bit is set and cleared by software. 0: No effect. 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
pub type CGIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF6` writer - CTCIF6: Channel 6 transfer complete clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding TCIF flag in the DMA_ISR register
pub type CTCIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF6` writer - CHTIF6: Channel 6 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register
pub type CHTIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF6` writer - CTEIF6: Channel 6 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
pub type CTEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF7` writer - CGIF7: Channel 7 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
pub type CGIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF7` writer - CTCIF7: Channel 7 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
pub type CTCIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF7` writer - CHTIF7: Channel 7 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register
pub type CHTIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF7` writer - CTEIF7: Channel 7 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
pub type CTEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF8` writer - CGIF8: Channel 8 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
pub type CGIF8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF8` writer - CTCIF8: Channel 8 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
pub type CTCIF8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF8` writer - CHTIF8: Channel 8 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register
pub type CHTIF8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF8` writer - CTEIF8: Channel 8 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
pub type CTEIF8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CGIF1: Channel 1 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
    #[inline(always)]
    pub fn cgif1(&mut self) -> CGIF1_W<'_, IFCRrs> {
        CGIF1_W::new(self, 0)
    }
    ///Bit 1 - CTCIF1: Channel 1 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W<'_, IFCRrs> {
        CTCIF1_W::new(self, 1)
    }
    ///Bit 2 - CHTIF1: Channel 1 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W<'_, IFCRrs> {
        CHTIF1_W::new(self, 2)
    }
    ///Bit 3 - CTEIF1: Channel 1 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W<'_, IFCRrs> {
        CTEIF1_W::new(self, 3)
    }
    ///Bit 4 - CGIF2: Channel 2 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
    #[inline(always)]
    pub fn cgif2(&mut self) -> CGIF2_W<'_, IFCRrs> {
        CGIF2_W::new(self, 4)
    }
    ///Bit 5 - CTCIF2: Channel 2 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W<'_, IFCRrs> {
        CTCIF2_W::new(self, 5)
    }
    ///Bit 6 - CHTIF2: Channel 2 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W<'_, IFCRrs> {
        CHTIF2_W::new(self, 6)
    }
    ///Bit 7 - CTEIF2: Channel 2 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W<'_, IFCRrs> {
        CTEIF2_W::new(self, 7)
    }
    ///Bit 8 - CGIF3: Channel 3 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
    #[inline(always)]
    pub fn cgif3(&mut self) -> CGIF3_W<'_, IFCRrs> {
        CGIF3_W::new(self, 8)
    }
    ///Bit 9 - CTCIF3: Channel 3 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W<'_, IFCRrs> {
        CTCIF3_W::new(self, 9)
    }
    ///Bit 10 - CHTIF3: Channel 3 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W<'_, IFCRrs> {
        CHTIF3_W::new(self, 10)
    }
    ///Bit 11 - CTEIF3: Channel 3 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W<'_, IFCRrs> {
        CTEIF3_W::new(self, 11)
    }
    ///Bit 12 - CGIF4: Channel 4 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
    #[inline(always)]
    pub fn cgif4(&mut self) -> CGIF4_W<'_, IFCRrs> {
        CGIF4_W::new(self, 12)
    }
    ///Bit 13 - CTCIF4: Channel 4 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W<'_, IFCRrs> {
        CTCIF4_W::new(self, 13)
    }
    ///Bit 14 - CHTIF4: Channel 4 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W<'_, IFCRrs> {
        CHTIF4_W::new(self, 14)
    }
    ///Bit 15 - CTEIF4: Channel 4 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W<'_, IFCRrs> {
        CTEIF4_W::new(self, 15)
    }
    ///Bit 16 - CGIF5: Channel 5 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
    #[inline(always)]
    pub fn cgif5(&mut self) -> CGIF5_W<'_, IFCRrs> {
        CGIF5_W::new(self, 16)
    }
    ///Bit 17 - CTCIF5: Channel 5 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W<'_, IFCRrs> {
        CTCIF5_W::new(self, 17)
    }
    ///Bit 18 - CHTIF5: Channel 5 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W<'_, IFCRrs> {
        CHTIF5_W::new(self, 18)
    }
    ///Bit 19 - CTEIF5: Channel 5 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W<'_, IFCRrs> {
        CTEIF5_W::new(self, 19)
    }
    ///Bit 20 - CGIF6: Channel 6 global interrupt clear This bit is set and cleared by software. 0: No effect. 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
    #[inline(always)]
    pub fn cgif6(&mut self) -> CGIF6_W<'_, IFCRrs> {
        CGIF6_W::new(self, 20)
    }
    ///Bit 21 - CTCIF6: Channel 6 transfer complete clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding TCIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W<'_, IFCRrs> {
        CTCIF6_W::new(self, 21)
    }
    ///Bit 22 - CHTIF6: Channel 6 half transfer clear This bit is set and cleared by software. 0: No effect. 1: Clears the corresponding HTIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W<'_, IFCRrs> {
        CHTIF6_W::new(self, 22)
    }
    ///Bit 23 - CTEIF6: Channel 6 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W<'_, IFCRrs> {
        CTEIF6_W::new(self, 23)
    }
    ///Bit 24 - CGIF7: Channel 7 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
    #[inline(always)]
    pub fn cgif7(&mut self) -> CGIF7_W<'_, IFCRrs> {
        CGIF7_W::new(self, 24)
    }
    ///Bit 25 - CTCIF7: Channel 7 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W<'_, IFCRrs> {
        CTCIF7_W::new(self, 25)
    }
    ///Bit 26 - CHTIF7: Channel 7 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W<'_, IFCRrs> {
        CHTIF7_W::new(self, 26)
    }
    ///Bit 27 - CTEIF7: Channel 7 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W<'_, IFCRrs> {
        CTEIF7_W::new(self, 27)
    }
    ///Bit 28 - CGIF8: Channel 8 global interrupt clear This bit is set and cleared by software. 0: No effect 1: Clears the GIF, TEIF, HTIF and TCIF flags in the DMA_ISR register
    #[inline(always)]
    pub fn cgif8(&mut self) -> CGIF8_W<'_, IFCRrs> {
        CGIF8_W::new(self, 28)
    }
    ///Bit 29 - CTCIF8: Channel 8 transfer complete clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TCIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn ctcif8(&mut self) -> CTCIF8_W<'_, IFCRrs> {
        CTCIF8_W::new(self, 29)
    }
    ///Bit 30 - CHTIF8: Channel 8 half transfer clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding HTIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn chtif8(&mut self) -> CHTIF8_W<'_, IFCRrs> {
        CHTIF8_W::new(self, 30)
    }
    ///Bit 31 - CTEIF8: Channel 8 transfer error clear This bit is set and cleared by software. 0: No effect 1: Clears the corresponding TEIF flag in the DMA_ISR register
    #[inline(always)]
    pub fn cteif8(&mut self) -> CTEIF8_W<'_, IFCRrs> {
        CTEIF8_W::new(self, 31)
    }
}
/**DMA_IFCR register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {}
