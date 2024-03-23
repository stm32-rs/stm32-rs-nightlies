#[doc = "Register `LTDC_ISR` reader"]
pub type R = crate::R<LTDC_ISRrs>;
#[doc = "Field `LIF` reader - line interrupt flag"]
pub type LIF_R = crate::BitReader;
#[doc = "Field `FUIF` reader - FIFO underrun interrupt flag"]
pub type FUIF_R = crate::BitReader;
#[doc = "Field `TERRIF` reader - transfer error interrupt flag"]
pub type TERRIF_R = crate::BitReader;
#[doc = "Field `RRIF` reader - register reload interrupt flag"]
pub type RRIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - line interrupt flag"]
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO underrun interrupt flag"]
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - transfer error interrupt flag"]
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - register reload interrupt flag"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "LTDC interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_ISRrs;
impl crate::RegisterSpec for LTDC_ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_isr::R`](R) reader structure"]
impl crate::Readable for LTDC_ISRrs {}
#[doc = "`reset()` method sets LTDC_ISR to value 0"]
impl crate::Resettable for LTDC_ISRrs {
    const RESET_VALUE: u32 = 0;
}
