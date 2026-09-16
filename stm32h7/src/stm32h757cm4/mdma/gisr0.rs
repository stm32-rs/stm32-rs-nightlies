///Register `GISR0` reader
pub type R = crate::R<GISR0rs>;
///Field `GIF(0-15)` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
pub type GIF_R = crate::BitReader;
impl R {
    ///Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `GIF0` field.</div>
    #[inline(always)]
    pub fn gif(&self, n: u8) -> GIF_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        GIF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif_iter(&self) -> impl Iterator<Item = GIF_R> + '_ {
        (0..16).map(move |n| GIF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif0(&self) -> GIF_R {
        GIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif1(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif2(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif3(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif4(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif5(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif6(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif7(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif8(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif9(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif10(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif11(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif12(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif13(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif14(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)
    #[inline(always)]
    pub fn gif15(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GISR0")
            .field("gif0", &self.gif0())
            .field("gif1", &self.gif1())
            .field("gif2", &self.gif2())
            .field("gif3", &self.gif3())
            .field("gif4", &self.gif4())
            .field("gif5", &self.gif5())
            .field("gif6", &self.gif6())
            .field("gif7", &self.gif7())
            .field("gif8", &self.gif8())
            .field("gif9", &self.gif9())
            .field("gif10", &self.gif10())
            .field("gif11", &self.gif11())
            .field("gif12", &self.gif12())
            .field("gif13", &self.gif13())
            .field("gif14", &self.gif14())
            .field("gif15", &self.gif15())
            .finish()
    }
}
/**MDMA Global Interrupt/Status Register

You can [`read`](crate::Reg::read) this register and get [`gisr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#MDMA:GISR0)*/
pub struct GISR0rs;
impl crate::RegisterSpec for GISR0rs {
    type Ux = u32;
}
///`read()` method returns [`gisr0::R`](R) reader structure
impl crate::Readable for GISR0rs {}
///`reset()` method sets GISR0 to value 0
impl crate::Resettable for GISR0rs {}
