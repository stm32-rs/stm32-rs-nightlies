///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `GIF1` reader - GIF1: Channel 1 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 1 1: A TE, HT or TC event occurred on channel 1
pub type GIF1_R = crate::BitReader;
///Field `TCIF1` reader - TCIF1: Channel 1 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 1 1: A transfer complete (TC) event occurred on channel 1
pub type TCIF1_R = crate::BitReader;
///Field `HTIF1` reader - HTIF1: Channel 1 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 1 1: A half transfer (HT) event occurred on channel 1
pub type HTIF1_R = crate::BitReader;
///Field `TE1F1` reader - TEIF1: Channel 1 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 1 1: A transfer error (TE) occurred on channel 1
pub type TE1F1_R = crate::BitReader;
///Field `GIF2` reader - GIF2: Channel 2 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 2 1: A TE, HT or TC event occurred on channel 2
pub type GIF2_R = crate::BitReader;
///Field `TCIF2` reader - TCIF2: Channel 2 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 2 1: A transfer complete (TC) event occurred on channel 2
pub type TCIF2_R = crate::BitReader;
///Field `HTIF2` reader - HTIF2: Channel 2 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 2 1: A half transfer (HT) event occurred on channel 2
pub type HTIF2_R = crate::BitReader;
///Field `TE1F2` reader - TEIF2: Channel 2 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 2 1: A transfer error (TE) occurred on channel 2
pub type TE1F2_R = crate::BitReader;
///Field `GIF3` reader - GIF3: Channel 3 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 3 1: A TE, HT or TC event occurred on channel 3
pub type GIF3_R = crate::BitReader;
///Field `TCIF3` reader - TCIF3: Channel 3 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 3 1: A transfer complete (TC) event occurred on channel 3
pub type TCIF3_R = crate::BitReader;
///Field `HTIF3` reader - HTIF3: Channel 3 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 3 1: A half transfer (HT) event occurred on channel 3
pub type HTIF3_R = crate::BitReader;
///Field `TE1F3` reader - TEIF3: Channel 3 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 3 1: A transfer error (TE) occurred on channel 3
pub type TE1F3_R = crate::BitReader;
///Field `GIF4` reader - GIF4: Channel 4 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 4 1: A TE, HT or TC event occurred on channel 4
pub type GIF4_R = crate::BitReader;
///Field `TCIF4` reader - TCIF4: Channel 4 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 4 1: A transfer complete (TC) event occurred on channel 4
pub type TCIF4_R = crate::BitReader;
///Field `HTIF4` reader - HTIF4: Channel 4 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 4 1: A half transfer (HT) event occurred on channel 4
pub type HTIF4_R = crate::BitReader;
///Field `TE1F4` reader - TEIF4: Channel 4 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 4 1: A transfer error (TE) occurred on channel 4
pub type TE1F4_R = crate::BitReader;
///Field `GIF5` reader - GIF5: Channel 5 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 5 1: A TE, HT or TC event occurred on channel 5
pub type GIF5_R = crate::BitReader;
///Field `TCIF5` reader - TCIF5: Channel 5 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 5 1: A transfer complete (TC) event occurred on channel 5
pub type TCIF5_R = crate::BitReader;
///Field `HTIF5` reader - HTIF5: Channel 5 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 5 1: A half transfer (HT) event occurred on channel 5
pub type HTIF5_R = crate::BitReader;
///Field `TE1F5` reader - TEIF5: Channel 5 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 5 1: A transfer error (TE) occurred on channel 5
pub type TE1F5_R = crate::BitReader;
///Field `GIF6` reader - GIF6: Channel 6 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 6 1: A TE, HT or TC event occurred on channel 6
pub type GIF6_R = crate::BitReader;
///Field `TCIF6` reader - TCIF6: Channel 6 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 6 1: A transfer complete (TC) event occurred on channel 6
pub type TCIF6_R = crate::BitReader;
///Field `HTIF6` reader - HTIF6: Channel 6 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 6 1: A half transfer (HT) event occurred on channel 6
pub type HTIF6_R = crate::BitReader;
///Field `TE1F6` reader - TEIF6: Channel 6 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 6 1: A transfer error (TE) occurred on channel 6
pub type TE1F6_R = crate::BitReader;
///Field `GIF7` reader - GIF7: Channel 7 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 7 1: A TE, HT or TC event occurred on channel 7
pub type GIF7_R = crate::BitReader;
///Field `TCIF7` reader - TCIF7: Channel 7 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 7 1: A transfer complete (TC) event occurred on channel 7
pub type TCIF7_R = crate::BitReader;
///Field `HTIF7` reader - HTIF7: Channel 7 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 7 1: A half transfer (HT) event occurred on channel 7
pub type HTIF7_R = crate::BitReader;
///Field `TE1F7` reader - TEIF7: Channel 7 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 7 1: A transfer error (TE) occurred on channel 7
pub type TE1F7_R = crate::BitReader;
///Field `GIF8` reader - GIF8: Channel 8 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 8 1: A TE, HT or TC event occurred on channel 8
pub type GIF8_R = crate::BitReader;
///Field `TCIF8` reader - TCIF8: Channel 8 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 8 1: A transfer complete (TC) event occurred on channel 8
pub type TCIF8_R = crate::BitReader;
///Field `HTIF8` reader - HTIF8: Channel 8 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 8 1: A half transfer (HT) event occurred on channel 8
pub type HTIF8_R = crate::BitReader;
///Field `TE1F8` reader - TEIF8: Channel 8 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 8 1: A transfer error (TE) occurred on channel 8
pub type TE1F8_R = crate::BitReader;
impl R {
    ///Bit 0 - GIF1: Channel 1 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 1 1: A TE, HT or TC event occurred on channel 1
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TCIF1: Channel 1 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 1 1: A transfer complete (TC) event occurred on channel 1
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HTIF1: Channel 1 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 1 1: A half transfer (HT) event occurred on channel 1
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TEIF1: Channel 1 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 1 1: A transfer error (TE) occurred on channel 1
    #[inline(always)]
    pub fn te1f1(&self) -> TE1F1_R {
        TE1F1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GIF2: Channel 2 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 2 1: A TE, HT or TC event occurred on channel 2
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TCIF2: Channel 2 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 2 1: A transfer complete (TC) event occurred on channel 2
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HTIF2: Channel 2 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 2 1: A half transfer (HT) event occurred on channel 2
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TEIF2: Channel 2 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 2 1: A transfer error (TE) occurred on channel 2
    #[inline(always)]
    pub fn te1f2(&self) -> TE1F2_R {
        TE1F2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GIF3: Channel 3 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 3 1: A TE, HT or TC event occurred on channel 3
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TCIF3: Channel 3 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 3 1: A transfer complete (TC) event occurred on channel 3
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HTIF3: Channel 3 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 3 1: A half transfer (HT) event occurred on channel 3
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TEIF3: Channel 3 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 3 1: A transfer error (TE) occurred on channel 3
    #[inline(always)]
    pub fn te1f3(&self) -> TE1F3_R {
        TE1F3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GIF4: Channel 4 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 4 1: A TE, HT or TC event occurred on channel 4
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TCIF4: Channel 4 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 4 1: A transfer complete (TC) event occurred on channel 4
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - HTIF4: Channel 4 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 4 1: A half transfer (HT) event occurred on channel 4
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TEIF4: Channel 4 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 4 1: A transfer error (TE) occurred on channel 4
    #[inline(always)]
    pub fn te1f4(&self) -> TE1F4_R {
        TE1F4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - GIF5: Channel 5 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 5 1: A TE, HT or TC event occurred on channel 5
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TCIF5: Channel 5 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 5 1: A transfer complete (TC) event occurred on channel 5
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HTIF5: Channel 5 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 5 1: A half transfer (HT) event occurred on channel 5
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TEIF5: Channel 5 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 5 1: A transfer error (TE) occurred on channel 5
    #[inline(always)]
    pub fn te1f5(&self) -> TE1F5_R {
        TE1F5_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - GIF6: Channel 6 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 6 1: A TE, HT or TC event occurred on channel 6
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TCIF6: Channel 6 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 6 1: A transfer complete (TC) event occurred on channel 6
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - HTIF6: Channel 6 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 6 1: A half transfer (HT) event occurred on channel 6
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TEIF6: Channel 6 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 6 1: A transfer error (TE) occurred on channel 6
    #[inline(always)]
    pub fn te1f6(&self) -> TE1F6_R {
        TE1F6_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - GIF7: Channel 7 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 7 1: A TE, HT or TC event occurred on channel 7
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TCIF7: Channel 7 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 7 1: A transfer complete (TC) event occurred on channel 7
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - HTIF7: Channel 7 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 7 1: A half transfer (HT) event occurred on channel 7
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TEIF7: Channel 7 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 7 1: A transfer error (TE) occurred on channel 7
    #[inline(always)]
    pub fn te1f7(&self) -> TE1F7_R {
        TE1F7_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - GIF8: Channel 8 global interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No TE, HT or TC event on channel 8 1: A TE, HT or TC event occurred on channel 8
    #[inline(always)]
    pub fn gif8(&self) -> GIF8_R {
        GIF8_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - TCIF8: Channel 8 transfer complete flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer complete (TC) event on channel 8 1: A transfer complete (TC) event occurred on channel 8
    #[inline(always)]
    pub fn tcif8(&self) -> TCIF8_R {
        TCIF8_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - HTIF8: Channel 8 half transfer flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No half transfer (HT) event on channel 8 1: A half transfer (HT) event occurred on channel 8
    #[inline(always)]
    pub fn htif8(&self) -> HTIF8_R {
        HTIF8_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - TEIF8: Channel 8 transfer error flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register. 0: No transfer error (TE) on channel 8 1: A transfer error (TE) occurred on channel 8
    #[inline(always)]
    pub fn te1f8(&self) -> TE1F8_R {
        TE1F8_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("gif1", &self.gif1())
            .field("tcif1", &self.tcif1())
            .field("htif1", &self.htif1())
            .field("te1f1", &self.te1f1())
            .field("gif2", &self.gif2())
            .field("tcif2", &self.tcif2())
            .field("htif2", &self.htif2())
            .field("te1f2", &self.te1f2())
            .field("gif3", &self.gif3())
            .field("tcif3", &self.tcif3())
            .field("htif3", &self.htif3())
            .field("te1f3", &self.te1f3())
            .field("gif4", &self.gif4())
            .field("tcif4", &self.tcif4())
            .field("htif4", &self.htif4())
            .field("te1f4", &self.te1f4())
            .field("gif5", &self.gif5())
            .field("tcif5", &self.tcif5())
            .field("htif5", &self.htif5())
            .field("te1f5", &self.te1f5())
            .field("gif6", &self.gif6())
            .field("tcif6", &self.tcif6())
            .field("htif6", &self.htif6())
            .field("te1f6", &self.te1f6())
            .field("gif7", &self.gif7())
            .field("tcif7", &self.tcif7())
            .field("htif7", &self.htif7())
            .field("te1f7", &self.te1f7())
            .field("gif8", &self.gif8())
            .field("tcif8", &self.tcif8())
            .field("htif8", &self.htif8())
            .field("te1f8", &self.te1f8())
            .finish()
    }
}
/**DMA_ISR register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DMA:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
