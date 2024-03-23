#[doc = "Register `DSI_WISR` reader"]
pub type R = crate::R<DSI_WISRrs>;
#[doc = "Field `TEIF` reader - Tearing effect interrupt flag This bit is set when a tearing effect event occurs."]
pub type TEIF_R = crate::BitReader;
#[doc = "Field `ERIF` reader - End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished."]
pub type ERIF_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing."]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `PLLLS` reader - PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked."]
pub type PLLLS_R = crate::BitReader;
#[doc = "Field `PLLLIF` reader - PLL lock interrupt flag This bit is set when the PLL becomes locked."]
pub type PLLLIF_R = crate::BitReader;
#[doc = "Field `PLLUIF` reader - PLL unlock interrupt flag This bit is set when the PLL becomes unlocked."]
pub type PLLUIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Tearing effect interrupt flag This bit is set when a tearing effect event occurs."]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of refresh interrupt flag This bit is set when the transfer of a frame in adapted command mode is finished."]
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy flag This bit is set when the transfer of a frame in adapted command mode is ongoing."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL lock status This bit is set when the PLL is locked and cleared when it is unlocked."]
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL lock interrupt flag This bit is set when the PLL becomes locked."]
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLL unlock interrupt flag This bit is set when the PLL becomes unlocked."]
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "DSI Wrapper interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WISRrs;
impl crate::RegisterSpec for DSI_WISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wisr::R`](R) reader structure"]
impl crate::Readable for DSI_WISRrs {}
#[doc = "`reset()` method sets DSI_WISR to value 0"]
impl crate::Resettable for DSI_WISRrs {
    const RESET_VALUE: u32 = 0;
}
