///Register `WISR` reader
pub type R = crate::R<WISRrs>;
///Field `TEIF` reader - Tearing Effect Interrupt Flag
pub type TEIF_R = crate::BitReader;
///Field `ERIF` reader - End of Refresh Interrupt Flag
pub type ERIF_R = crate::BitReader;
///Field `BUSY` reader - Busy Flag
pub type BUSY_R = crate::BitReader;
///Field `PLLLS` reader - PLL Lock Status
pub type PLLLS_R = crate::BitReader;
///Field `PLLLIF` reader - PLL Lock Interrupt Flag
pub type PLLLIF_R = crate::BitReader;
///Field `PLLUIF` reader - PLL Unlock Interrupt Flag
pub type PLLUIF_R = crate::BitReader;
///Field `RRS` reader - Regulator Ready Status
pub type RRS_R = crate::BitReader;
///Field `RRIF` reader - Regulator Ready Interrupt Flag
pub type RRIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Tearing Effect Interrupt Flag
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of Refresh Interrupt Flag
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Busy Flag
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - PLL Lock Status
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL Lock Interrupt Flag
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL Unlock Interrupt Flag
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Regulator Ready Status
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Regulator Ready Interrupt Flag
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WISR")
            .field("rrif", &self.rrif())
            .field("rrs", &self.rrs())
            .field("plluif", &self.plluif())
            .field("plllif", &self.plllif())
            .field("pllls", &self.pllls())
            .field("busy", &self.busy())
            .field("erif", &self.erif())
            .field("teif", &self.teif())
            .finish()
    }
}
/**DSI Wrapper Interrupt & Status Register

You can [`read`](crate::Reg::read) this register and get [`wisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WISR)*/
pub struct WISRrs;
impl crate::RegisterSpec for WISRrs {
    type Ux = u32;
}
///`read()` method returns [`wisr::R`](R) reader structure
impl crate::Readable for WISRrs {}
///`reset()` method sets WISR to value 0
impl crate::Resettable for WISRrs {}
