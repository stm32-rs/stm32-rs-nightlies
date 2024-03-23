#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `GIF1` reader - global interrupt flag for channel 1"]
pub type GIF1_R = crate::BitReader;
#[doc = "Field `TCIF1` reader - transfer complete (TC) flag for channel 1"]
pub type TCIF1_R = crate::BitReader;
#[doc = "Field `HTIF1` reader - half transfer (HT) flag for channel 1"]
pub type HTIF1_R = crate::BitReader;
#[doc = "Field `TEIF1` reader - transfer error (TE) flag for channel 1"]
pub type TEIF1_R = crate::BitReader;
#[doc = "Field `GIF2` reader - global interrupt flag for channel 2"]
pub type GIF2_R = crate::BitReader;
#[doc = "Field `TCIF2` reader - transfer complete (TC) flag for channel 2"]
pub type TCIF2_R = crate::BitReader;
#[doc = "Field `HTIF2` reader - half transfer (HT) flag for channel 2"]
pub type HTIF2_R = crate::BitReader;
#[doc = "Field `TEIF2` reader - transfer error (TE) flag for channel 2"]
pub type TEIF2_R = crate::BitReader;
#[doc = "Field `GIF3` reader - global interrupt flag for channel 3"]
pub type GIF3_R = crate::BitReader;
#[doc = "Field `TCIF3` reader - transfer complete (TC) flag for channel 3"]
pub type TCIF3_R = crate::BitReader;
#[doc = "Field `HTIF3` reader - half transfer (HT) flag for channel 3"]
pub type HTIF3_R = crate::BitReader;
#[doc = "Field `TEIF3` reader - transfer error (TE) flag for channel 3"]
pub type TEIF3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - global interrupt flag for channel 1"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - transfer complete (TC) flag for channel 1"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - half transfer (HT) flag for channel 1"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transfer error (TE) flag for channel 1"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - global interrupt flag for channel 2"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - transfer complete (TC) flag for channel 2"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - half transfer (HT) flag for channel 2"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - transfer error (TE) flag for channel 2"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - global interrupt flag for channel 3"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - transfer complete (TC) flag for channel 3"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - half transfer (HT) flag for channel 3"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - transfer error (TE) flag for channel 3"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "DMA interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
