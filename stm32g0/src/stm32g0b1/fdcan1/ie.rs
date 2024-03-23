#[doc = "Register `IE` reader"]
pub type R = crate::R<IErs>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IErs>;
#[doc = "Field `RF0NE` reader - Rx FIFO 0 new message interrupt enable"]
pub type RF0NE_R = crate::BitReader;
#[doc = "Field `RF0NE` writer - Rx FIFO 0 new message interrupt enable"]
pub type RF0NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0FE` reader - Rx FIFO 0 full interrupt enable"]
pub type RF0FE_R = crate::BitReader;
#[doc = "Field `RF0FE` writer - Rx FIFO 0 full interrupt enable"]
pub type RF0FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0LE` reader - Rx FIFO 0 message lost interrupt enable"]
pub type RF0LE_R = crate::BitReader;
#[doc = "Field `RF0LE` writer - Rx FIFO 0 message lost interrupt enable"]
pub type RF0LE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1NE` reader - Rx FIFO 1 new message interrupt enable"]
pub type RF1NE_R = crate::BitReader;
#[doc = "Field `RF1NE` writer - Rx FIFO 1 new message interrupt enable"]
pub type RF1NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1FE` reader - Rx FIFO 1 full interrupt enable"]
pub type RF1FE_R = crate::BitReader;
#[doc = "Field `RF1FE` writer - Rx FIFO 1 full interrupt enable"]
pub type RF1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1LE` reader - Rx FIFO 1 message lost interrupt enable"]
pub type RF1LE_R = crate::BitReader;
#[doc = "Field `RF1LE` writer - Rx FIFO 1 message lost interrupt enable"]
pub type RF1LE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPME` reader - High-priority message interrupt enable"]
pub type HPME_R = crate::BitReader;
#[doc = "Field `HPME` writer - High-priority message interrupt enable"]
pub type HPME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCE` reader - Transmission completed interrupt enable"]
pub type TCE_R = crate::BitReader;
#[doc = "Field `TCE` writer - Transmission completed interrupt enable"]
pub type TCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCFE` reader - Transmission cancellation finished interrupt enable"]
pub type TCFE_R = crate::BitReader;
#[doc = "Field `TCFE` writer - Transmission cancellation finished interrupt enable"]
pub type TCFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFEE` reader - Tx FIFO empty interrupt enable"]
pub type TFEE_R = crate::BitReader;
#[doc = "Field `TFEE` writer - Tx FIFO empty interrupt enable"]
pub type TFEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFNE` reader - Tx event FIFO new entry interrupt enable"]
pub type TEFNE_R = crate::BitReader;
#[doc = "Field `TEFNE` writer - Tx event FIFO new entry interrupt enable"]
pub type TEFNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFFE` reader - Tx event FIFO full interrupt enable"]
pub type TEFFE_R = crate::BitReader;
#[doc = "Field `TEFFE` writer - Tx event FIFO full interrupt enable"]
pub type TEFFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFLE` reader - Tx event FIFO element lost interrupt enable"]
pub type TEFLE_R = crate::BitReader;
#[doc = "Field `TEFLE` writer - Tx event FIFO element lost interrupt enable"]
pub type TEFLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSWE` reader - Timestamp wraparound interrupt enable"]
pub type TSWE_R = crate::BitReader;
#[doc = "Field `TSWE` writer - Timestamp wraparound interrupt enable"]
pub type TSWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAFE` reader - Message RAM access failure interrupt enable"]
pub type MRAFE_R = crate::BitReader;
#[doc = "Field `MRAFE` writer - Message RAM access failure interrupt enable"]
pub type MRAFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOOE` reader - Timeout occurred interrupt enable"]
pub type TOOE_R = crate::BitReader;
#[doc = "Field `TOOE` writer - Timeout occurred interrupt enable"]
pub type TOOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELOE` reader - Error logging overflow interrupt enable"]
pub type ELOE_R = crate::BitReader;
#[doc = "Field `ELOE` writer - Error logging overflow interrupt enable"]
pub type ELOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPE` reader - Error passive interrupt enable"]
pub type EPE_R = crate::BitReader;
#[doc = "Field `EPE` writer - Error passive interrupt enable"]
pub type EPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWE` reader - Warning status interrupt enable"]
pub type EWE_R = crate::BitReader;
#[doc = "Field `EWE` writer - Warning status interrupt enable"]
pub type EWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOE` reader - Bus_Off status"]
pub type BOE_R = crate::BitReader;
#[doc = "Field `BOE` writer - Bus_Off status"]
pub type BOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDIE` reader - Watchdog interrupt enable"]
pub type WDIE_R = crate::BitReader;
#[doc = "Field `WDIE` writer - Watchdog interrupt enable"]
pub type WDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEAE` reader - Protocol error in arbitration phase enable"]
pub type PEAE_R = crate::BitReader;
#[doc = "Field `PEAE` writer - Protocol error in arbitration phase enable"]
pub type PEAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEDE` reader - Protocol error in data phase enable"]
pub type PEDE_R = crate::BitReader;
#[doc = "Field `PEDE` writer - Protocol error in data phase enable"]
pub type PEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARAE` reader - Access to reserved address enable"]
pub type ARAE_R = crate::BitReader;
#[doc = "Field `ARAE` writer - Access to reserved address enable"]
pub type ARAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable"]
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost interrupt enable"]
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message interrupt enable"]
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost interrupt enable"]
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High-priority message interrupt enable"]
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmission completed interrupt enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmission cancellation finished interrupt enable"]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn tfee(&self) -> TFEE_R {
        TFEE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx event FIFO new entry interrupt enable"]
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx event FIFO full interrupt enable"]
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost interrupt enable"]
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timestamp wraparound interrupt enable"]
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message RAM access failure interrupt enable"]
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout occurred interrupt enable"]
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Error logging overflow interrupt enable"]
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Warning status interrupt enable"]
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Watchdog interrupt enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase enable"]
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protocol error in data phase enable"]
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Access to reserved address enable"]
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0ne(&mut self) -> RF0NE_W<IErs> {
        RF0NE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0fe(&mut self) -> RF0FE_W<IErs> {
        RF0FE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rx FIFO 0 message lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0le(&mut self) -> RF0LE_W<IErs> {
        RF0LE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rx FIFO 1 new message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1ne(&mut self) -> RF1NE_W<IErs> {
        RF1NE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO 1 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1fe(&mut self) -> RF1FE_W<IErs> {
        RF1FE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO 1 message lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1le(&mut self) -> RF1LE_W<IErs> {
        RF1LE_W::new(self, 5)
    }
    #[doc = "Bit 6 - High-priority message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpme(&mut self) -> HPME_W<IErs> {
        HPME_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmission completed interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<IErs> {
        TCE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Transmission cancellation finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcfe(&mut self) -> TCFE_W<IErs> {
        TCFE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfee(&mut self) -> TFEE_W<IErs> {
        TFEE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Tx event FIFO new entry interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tefne(&mut self) -> TEFNE_W<IErs> {
        TEFNE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Tx event FIFO full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teffe(&mut self) -> TEFFE_W<IErs> {
        TEFFE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Tx event FIFO element lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tefle(&mut self) -> TEFLE_W<IErs> {
        TEFLE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Timestamp wraparound interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tswe(&mut self) -> TSWE_W<IErs> {
        TSWE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Message RAM access failure interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn mrafe(&mut self) -> MRAFE_W<IErs> {
        MRAFE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Timeout occurred interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tooe(&mut self) -> TOOE_W<IErs> {
        TOOE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Error logging overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eloe(&mut self) -> ELOE_W<IErs> {
        ELOE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Error passive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epe(&mut self) -> EPE_W<IErs> {
        EPE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Warning status interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewe(&mut self) -> EWE_W<IErs> {
        EWE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Bus_Off status"]
    #[inline(always)]
    #[must_use]
    pub fn boe(&mut self) -> BOE_W<IErs> {
        BOE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Watchdog interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdie(&mut self) -> WDIE_W<IErs> {
        WDIE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Protocol error in arbitration phase enable"]
    #[inline(always)]
    #[must_use]
    pub fn peae(&mut self) -> PEAE_W<IErs> {
        PEAE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Protocol error in data phase enable"]
    #[inline(always)]
    #[must_use]
    pub fn pede(&mut self) -> PEDE_W<IErs> {
        PEDE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Access to reserved address enable"]
    #[inline(always)]
    #[must_use]
    pub fn arae(&mut self) -> ARAE_W<IErs> {
        ARAE_W::new(self, 23)
    }
}
#[doc = "FDCAN interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IErs;
impl crate::RegisterSpec for IErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IErs {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IErs {
    const RESET_VALUE: u32 = 0;
}
