#[doc = "Register `WISR` reader"]
pub type R = crate::R<WISRrs>;
#[doc = "Field `TEIF` reader - Tearing Effect Interrupt Flag"]
pub type TEIF_R = crate::BitReader;
#[doc = "Field `ERIF` reader - End of Refresh Interrupt Flag"]
pub type ERIF_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy Flag"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `PLLLS` reader - PLL Lock Status"]
pub type PLLLS_R = crate::BitReader;
#[doc = "Field `PLLLIF` reader - PLL Lock Interrupt Flag"]
pub type PLLLIF_R = crate::BitReader;
#[doc = "Field `PLLUIF` reader - PLL Unlock Interrupt Flag"]
pub type PLLUIF_R = crate::BitReader;
#[doc = "Field `RRS` reader - Regulator Ready Status"]
pub type RRS_R = crate::BitReader;
#[doc = "Field `RRIF` reader - Regulator Ready Interrupt Flag"]
pub type RRIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Tearing Effect Interrupt Flag"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Refresh Interrupt Flag"]
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL Lock Status"]
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLL Unlock Interrupt Flag"]
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Regulator Ready Status"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Regulator Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "DSI Wrapper Interrupt &amp; Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WISRrs;
impl crate::RegisterSpec for WISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wisr::R`](R) reader structure"]
impl crate::Readable for WISRrs {}
#[doc = "`reset()` method sets WISR to value 0"]
impl crate::Resettable for WISRrs {
    const RESET_VALUE: u32 = 0;
}
