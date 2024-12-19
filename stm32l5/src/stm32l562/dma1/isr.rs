///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `GIF1` reader - Channel x global interrupt flag (x = 1 ..7)
pub type GIF1_R = crate::BitReader;
///Field `TCIF1` reader - Channel x transfer complete flag (x = 1 ..7)
pub type TCIF1_R = crate::BitReader;
///Field `HTIF1` reader - Channel x half transfer flag (x = 1 ..7)
pub type HTIF1_R = crate::BitReader;
///Field `TEIF1` reader - Channel x transfer error flag (x = 1 ..7)
pub type TEIF1_R = crate::BitReader;
///Field `GIF2` reader - Channel x global interrupt flag (x = 1 ..7)
pub type GIF2_R = crate::BitReader;
///Field `TCIF2` reader - Channel x transfer complete flag (x = 1 ..7)
pub type TCIF2_R = crate::BitReader;
///Field `HTIF2` reader - Channel x half transfer flag (x = 1 ..7)
pub type HTIF2_R = crate::BitReader;
///Field `TEIF2` reader - Channel x transfer error flag (x = 1 ..7)
pub type TEIF2_R = crate::BitReader;
///Field `GIF3` reader - Channel x global interrupt flag (x = 1 ..7)
pub type GIF3_R = crate::BitReader;
///Field `TCIF3` reader - Channel x transfer complete flag (x = 1 ..7)
pub type TCIF3_R = crate::BitReader;
///Field `HTIF3` reader - Channel x half transfer flag (x = 1 ..7)
pub type HTIF3_R = crate::BitReader;
///Field `TEIF3` reader - Channel x transfer error flag (x = 1 ..7)
pub type TEIF3_R = crate::BitReader;
///Field `GIF4` reader - Channel x global interrupt flag (x = 1 ..7)
pub type GIF4_R = crate::BitReader;
///Field `TCIF4` reader - Channel x transfer complete flag (x = 1 ..7)
pub type TCIF4_R = crate::BitReader;
///Field `HTIF4` reader - Channel x half transfer flag (x = 1 ..7)
pub type HTIF4_R = crate::BitReader;
///Field `TEIF4` reader - Channel x transfer error flag (x = 1 ..7)
pub type TEIF4_R = crate::BitReader;
///Field `GIF5` reader - Channel x global interrupt flag (x = 1 ..7)
pub type GIF5_R = crate::BitReader;
///Field `TCIF5` reader - Channel x transfer complete flag (x = 1 ..7)
pub type TCIF5_R = crate::BitReader;
///Field `HTIF5` reader - Channel x half transfer flag (x = 1 ..7)
pub type HTIF5_R = crate::BitReader;
///Field `TEIF5` reader - Channel x transfer error flag (x = 1 ..7)
pub type TEIF5_R = crate::BitReader;
///Field `GIF6` reader - Channel x global interrupt flag (x = 1 ..7)
pub type GIF6_R = crate::BitReader;
///Field `TCIF6` reader - Channel x transfer complete flag (x = 1 ..7)
pub type TCIF6_R = crate::BitReader;
///Field `HTIF6` reader - Channel x half transfer flag (x = 1 ..7)
pub type HTIF6_R = crate::BitReader;
///Field `TEIF6` reader - Channel x transfer error flag (x = 1 ..7)
pub type TEIF6_R = crate::BitReader;
///Field `GIF7` reader - Channel x global interrupt flag (x = 1 ..7)
pub type GIF7_R = crate::BitReader;
///Field `TCIF7` reader - Channel x transfer complete flag (x = 1 ..7)
pub type TCIF7_R = crate::BitReader;
///Field `HTIF7` reader - Channel x half transfer flag (x = 1 ..7)
pub type HTIF7_R = crate::BitReader;
///Field `TEIF7` reader - Channel x transfer error flag (x = 1 ..7)
pub type TEIF7_R = crate::BitReader;
///Field `GIF8` reader - global interrupt flag for channel 8
pub type GIF8_R = crate::BitReader;
///Field `TCIF8` reader - transfer complete (TC) flag for channel 8
pub type TCIF8_R = crate::BitReader;
///Field `HTIF8` reader - half transfer (HT) flag for channel 8
pub type HTIF8_R = crate::BitReader;
///Field `TEIF8` reader - transfer error (TE) flag for channel 8
pub type TEIF8_R = crate::BitReader;
impl R {
    ///Bit 0 - Channel x global interrupt flag (x = 1 ..7)
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel x transfer complete flag (x = 1 ..7)
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel x half transfer flag (x = 1 ..7)
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel x transfer error flag (x = 1 ..7)
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel x global interrupt flag (x = 1 ..7)
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel x transfer complete flag (x = 1 ..7)
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel x half transfer flag (x = 1 ..7)
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel x transfer error flag (x = 1 ..7)
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel x global interrupt flag (x = 1 ..7)
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel x transfer complete flag (x = 1 ..7)
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel x half transfer flag (x = 1 ..7)
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel x transfer error flag (x = 1 ..7)
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel x global interrupt flag (x = 1 ..7)
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel x transfer complete flag (x = 1 ..7)
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel x half transfer flag (x = 1 ..7)
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel x transfer error flag (x = 1 ..7)
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel x global interrupt flag (x = 1 ..7)
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel x transfer complete flag (x = 1 ..7)
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel x half transfer flag (x = 1 ..7)
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Channel x transfer error flag (x = 1 ..7)
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Channel x global interrupt flag (x = 1 ..7)
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Channel x transfer complete flag (x = 1 ..7)
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Channel x half transfer flag (x = 1 ..7)
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Channel x transfer error flag (x = 1 ..7)
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Channel x global interrupt flag (x = 1 ..7)
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Channel x transfer complete flag (x = 1 ..7)
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Channel x half transfer flag (x = 1 ..7)
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Channel x transfer error flag (x = 1 ..7)
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - global interrupt flag for channel 8
    #[inline(always)]
    pub fn gif8(&self) -> GIF8_R {
        GIF8_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - transfer complete (TC) flag for channel 8
    #[inline(always)]
    pub fn tcif8(&self) -> TCIF8_R {
        TCIF8_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - half transfer (HT) flag for channel 8
    #[inline(always)]
    pub fn htif8(&self) -> HTIF8_R {
        HTIF8_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - transfer error (TE) flag for channel 8
    #[inline(always)]
    pub fn teif8(&self) -> TEIF8_R {
        TEIF8_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("teif8", &self.teif8())
            .field("htif8", &self.htif8())
            .field("tcif8", &self.tcif8())
            .field("gif8", &self.gif8())
            .field("teif7", &self.teif7())
            .field("htif7", &self.htif7())
            .field("tcif7", &self.tcif7())
            .field("gif7", &self.gif7())
            .field("teif6", &self.teif6())
            .field("htif6", &self.htif6())
            .field("tcif6", &self.tcif6())
            .field("gif6", &self.gif6())
            .field("teif5", &self.teif5())
            .field("htif5", &self.htif5())
            .field("tcif5", &self.tcif5())
            .field("gif5", &self.gif5())
            .field("teif4", &self.teif4())
            .field("htif4", &self.htif4())
            .field("tcif4", &self.tcif4())
            .field("gif4", &self.gif4())
            .field("teif3", &self.teif3())
            .field("htif3", &self.htif3())
            .field("tcif3", &self.tcif3())
            .field("gif3", &self.gif3())
            .field("teif2", &self.teif2())
            .field("htif2", &self.htif2())
            .field("tcif2", &self.tcif2())
            .field("gif2", &self.gif2())
            .field("teif1", &self.teif1())
            .field("htif1", &self.htif1())
            .field("tcif1", &self.tcif1())
            .field("gif1", &self.gif1())
            .finish()
    }
}
/**interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#DMA1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
