#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `LIF` reader - Line Interrupt flag"]
pub type LIF_R = crate::BitReader;
#[doc = "Field `FUIF` reader - FIFO Underrun Interrupt flag"]
pub type FUIF_R = crate::BitReader;
#[doc = "Field `TERRIF` reader - Transfer Error interrupt flag"]
pub type TERRIF_R = crate::BitReader;
#[doc = "Field `RRIF` reader - Register Reload Interrupt Flag"]
pub type RRIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Line Interrupt flag"]
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt flag"]
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Error interrupt flag"]
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Register Reload Interrupt Flag"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "LTDC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
