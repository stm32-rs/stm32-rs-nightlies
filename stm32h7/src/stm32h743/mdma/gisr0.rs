#[doc = "Register `GISR0` reader"]
pub type R = crate::R<GISR0rs>;
#[doc = "Field `GIF(0-15)` reader - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
pub type GIF_R = crate::BitReader;
impl R {
    #[doc = "Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `GIF0` field"]
    #[inline(always)]
    pub fn gif(&self, n: u8) -> GIF_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        GIF_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif_iter(&self) -> impl Iterator<Item = GIF_R> + '_ {
        (0..16).map(move |n| GIF_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF_R {
        GIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif8(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif9(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif10(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif11(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif12(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif13(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif14(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel x global interrupt flag (x=...) This bit is set and reset by hardware. It is a logical OR of all the Channel x interrupt flags (CTCIFx, BTIFx, BRTIFx, TEIFx) which are enabled in the interrupt mask register (CTCIEx, BTIEx, BRTIEx, TEIEx)"]
    #[inline(always)]
    pub fn gif15(&self) -> GIF_R {
        GIF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "MDMA Global Interrupt/Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gisr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GISR0rs;
impl crate::RegisterSpec for GISR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gisr0::R`](R) reader structure"]
impl crate::Readable for GISR0rs {}
#[doc = "`reset()` method sets GISR0 to value 0"]
impl crate::Resettable for GISR0rs {
    const RESET_VALUE: u32 = 0;
}
