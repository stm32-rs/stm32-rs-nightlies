///Register `WISR` reader
pub type R = crate::R<WISRrs>;
///Register `WISR` writer
pub type W = crate::W<WISRrs>;
///Field `TEIF` reader - Tearing effect interrupt flag
pub type TEIF_R = crate::BitReader;
///Field `TEIF` writer - Tearing effect interrupt flag
pub type TEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERIF` reader - End of refresh interrupt flag
pub type ERIF_R = crate::BitReader;
///Field `ERIF` writer - End of refresh interrupt flag
pub type ERIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSY` reader - Busy flag
pub type BUSY_R = crate::BitReader;
///Field `BUSY` writer - Busy flag
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLLS` reader - PLL lock status
pub type PLLLS_R = crate::BitReader;
///Field `PLLLS` writer - PLL lock status
pub type PLLLS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLLIF` reader - PLL lock interrupt flag
pub type PLLLIF_R = crate::BitReader;
///Field `PLLLIF` writer - PLL lock interrupt flag
pub type PLLLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLUIF` reader - PLL unlock interrupt flag
pub type PLLUIF_R = crate::BitReader;
///Field `PLLUIF` writer - PLL unlock interrupt flag
pub type PLLUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRS` reader - Regulator ready status
pub type RRS_R = crate::BitReader;
///Field `RRS` writer - Regulator ready status
pub type RRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRIF` reader - Regulator ready interrupt flag
pub type RRIF_R = crate::BitReader;
///Field `RRIF` writer - Regulator ready interrupt flag
pub type RRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tearing effect interrupt flag
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of refresh interrupt flag
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Busy flag
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - PLL lock status
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL lock interrupt flag
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL unlock interrupt flag
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Regulator ready status
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Regulator ready interrupt flag
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
impl W {
    ///Bit 0 - Tearing effect interrupt flag
    #[inline(always)]
    pub fn teif(&mut self) -> TEIF_W<WISRrs> {
        TEIF_W::new(self, 0)
    }
    ///Bit 1 - End of refresh interrupt flag
    #[inline(always)]
    pub fn erif(&mut self) -> ERIF_W<WISRrs> {
        ERIF_W::new(self, 1)
    }
    ///Bit 2 - Busy flag
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<WISRrs> {
        BUSY_W::new(self, 2)
    }
    ///Bit 8 - PLL lock status
    #[inline(always)]
    pub fn pllls(&mut self) -> PLLLS_W<WISRrs> {
        PLLLS_W::new(self, 8)
    }
    ///Bit 9 - PLL lock interrupt flag
    #[inline(always)]
    pub fn plllif(&mut self) -> PLLLIF_W<WISRrs> {
        PLLLIF_W::new(self, 9)
    }
    ///Bit 10 - PLL unlock interrupt flag
    #[inline(always)]
    pub fn plluif(&mut self) -> PLLUIF_W<WISRrs> {
        PLLUIF_W::new(self, 10)
    }
    ///Bit 12 - Regulator ready status
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W<WISRrs> {
        RRS_W::new(self, 12)
    }
    ///Bit 13 - Regulator ready interrupt flag
    #[inline(always)]
    pub fn rrif(&mut self) -> RRIF_W<WISRrs> {
        RRIF_W::new(self, 13)
    }
}
/**DSI wrapper interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`wisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WISR)*/
pub struct WISRrs;
impl crate::RegisterSpec for WISRrs {
    type Ux = u32;
}
///`read()` method returns [`wisr::R`](R) reader structure
impl crate::Readable for WISRrs {}
///`write(|w| ..)` method takes [`wisr::W`](W) writer structure
impl crate::Writable for WISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WISR to value 0
impl crate::Resettable for WISRrs {
    const RESET_VALUE: u32 = 0;
}
