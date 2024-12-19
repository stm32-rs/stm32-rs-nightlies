///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `GIF1` reader - Global interrupt flag for channel 1
pub type GIF1_R = crate::BitReader;
///Field `TCIF1` reader - Transfer complete (TC) flag for channel 1
pub type TCIF1_R = crate::BitReader;
///Field `HTIF1` reader - Half transfer (HT) flag for channel 1
pub type HTIF1_R = crate::BitReader;
///Field `TEIF1` reader - Transfer error (TE) flag for channel 1
pub type TEIF1_R = crate::BitReader;
///Field `GIF2` reader - Global interrupt flag for channel 2
pub type GIF2_R = crate::BitReader;
///Field `TCIF2` reader - Transfer complete (TC) flag for channel 2
pub type TCIF2_R = crate::BitReader;
///Field `HTIF2` reader - Half transfer (HT) flag for channel 2
pub type HTIF2_R = crate::BitReader;
///Field `TEIF2` reader - Transfer error (TE) flag for channel 2
pub type TEIF2_R = crate::BitReader;
///Field `GIF3` reader - Global interrupt flag for channel 3
pub type GIF3_R = crate::BitReader;
///Field `TCIF3` reader - Transfer complete (TC) flag for channel 3
pub type TCIF3_R = crate::BitReader;
///Field `HTIF3` reader - Half transfer (HT) flag for channel 3
pub type HTIF3_R = crate::BitReader;
///Field `TEIF3` reader - Transfer error (TE) flag for channel 3
pub type TEIF3_R = crate::BitReader;
///Field `GIF4` reader - global interrupt flag for channel 4
pub type GIF4_R = crate::BitReader;
///Field `TCIF4` reader - Transfer complete (TC) flag for channel 4
pub type TCIF4_R = crate::BitReader;
///Field `HTIF4` reader - Half transfer (HT) flag for channel 4
pub type HTIF4_R = crate::BitReader;
///Field `TEIF4` reader - Transfer error (TE) flag for channel 4
pub type TEIF4_R = crate::BitReader;
///Field `GIF5` reader - global interrupt flag for channel 5
pub type GIF5_R = crate::BitReader;
///Field `TCIF5` reader - Transfer complete (TC) flag for channel 5
pub type TCIF5_R = crate::BitReader;
///Field `HTIF5` reader - Half transfer (HT) flag for channel 5
pub type HTIF5_R = crate::BitReader;
///Field `TEIF5` reader - Transfer error (TE) flag for channel 5
pub type TEIF5_R = crate::BitReader;
impl R {
    ///Bit 0 - Global interrupt flag for channel 1
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer complete (TC) flag for channel 1
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Half transfer (HT) flag for channel 1
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transfer error (TE) flag for channel 1
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Global interrupt flag for channel 2
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transfer complete (TC) flag for channel 2
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Half transfer (HT) flag for channel 2
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transfer error (TE) flag for channel 2
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Global interrupt flag for channel 3
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transfer complete (TC) flag for channel 3
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Half transfer (HT) flag for channel 3
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Transfer error (TE) flag for channel 3
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - global interrupt flag for channel 4
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Transfer complete (TC) flag for channel 4
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Half transfer (HT) flag for channel 4
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Transfer error (TE) flag for channel 4
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - global interrupt flag for channel 5
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Transfer complete (TC) flag for channel 5
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Half transfer (HT) flag for channel 5
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Transfer error (TE) flag for channel 5
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("gif1", &self.gif1())
            .field("tcif1", &self.tcif1())
            .field("htif1", &self.htif1())
            .field("teif1", &self.teif1())
            .field("gif2", &self.gif2())
            .field("tcif2", &self.tcif2())
            .field("htif2", &self.htif2())
            .field("teif2", &self.teif2())
            .field("gif3", &self.gif3())
            .field("tcif3", &self.tcif3())
            .field("htif3", &self.htif3())
            .field("teif3", &self.teif3())
            .field("gif4", &self.gif4())
            .field("tcif4", &self.tcif4())
            .field("htif4", &self.htif4())
            .field("teif4", &self.teif4())
            .field("gif5", &self.gif5())
            .field("tcif5", &self.tcif5())
            .field("htif5", &self.htif5())
            .field("teif5", &self.teif5())
            .finish()
    }
}
/**DMA interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#DMA:ISR)*/
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
