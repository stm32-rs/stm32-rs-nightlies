#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `GIF1` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF1_R = crate::BitReader;
#[doc = "Field `TCIF1` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF1_R = crate::BitReader;
#[doc = "Field `HTIF1` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF1_R = crate::BitReader;
#[doc = "Field `TEIF1` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF1_R = crate::BitReader;
#[doc = "Field `GIF2` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF2_R = crate::BitReader;
#[doc = "Field `TCIF2` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF2_R = crate::BitReader;
#[doc = "Field `HTIF2` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF2_R = crate::BitReader;
#[doc = "Field `TEIF2` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF2_R = crate::BitReader;
#[doc = "Field `GIF3` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF3_R = crate::BitReader;
#[doc = "Field `TCIF3` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF3_R = crate::BitReader;
#[doc = "Field `HTIF3` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF3_R = crate::BitReader;
#[doc = "Field `TEIF3` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF3_R = crate::BitReader;
#[doc = "Field `GIF4` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF4_R = crate::BitReader;
#[doc = "Field `TCIF4` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF4_R = crate::BitReader;
#[doc = "Field `HTIF4` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF4_R = crate::BitReader;
#[doc = "Field `TEIF4` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF4_R = crate::BitReader;
#[doc = "Field `GIF5` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF5_R = crate::BitReader;
#[doc = "Field `TCIF5` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF5_R = crate::BitReader;
#[doc = "Field `HTIF5` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF5_R = crate::BitReader;
#[doc = "Field `TEIF5` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF5_R = crate::BitReader;
#[doc = "Field `GIF6` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF6_R = crate::BitReader;
#[doc = "Field `TCIF6` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF6_R = crate::BitReader;
#[doc = "Field `HTIF6` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF6_R = crate::BitReader;
#[doc = "Field `TEIF6` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF6_R = crate::BitReader;
#[doc = "Field `GIF7` reader - Channel x global interrupt flag (x = 1 ..7)"]
pub type GIF7_R = crate::BitReader;
#[doc = "Field `TCIF7` reader - Channel x transfer complete flag (x = 1 ..7)"]
pub type TCIF7_R = crate::BitReader;
#[doc = "Field `HTIF7` reader - Channel x half transfer flag (x = 1 ..7)"]
pub type HTIF7_R = crate::BitReader;
#[doc = "Field `TEIF7` reader - Channel x transfer error flag (x = 1 ..7)"]
pub type TEIF7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
