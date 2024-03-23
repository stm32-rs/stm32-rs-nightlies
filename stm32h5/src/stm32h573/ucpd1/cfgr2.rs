#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Field `RXFILTDIS` reader - BMC decoder Rx pre-filter enable The sampling clock is that of the receiver (that is, after pre-scaler)."]
pub type RXFILTDIS_R = crate::BitReader;
#[doc = "Field `RXFILTDIS` writer - BMC decoder Rx pre-filter enable The sampling clock is that of the receiver (that is, after pre-scaler)."]
pub type RXFILTDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFILT2N3` reader - BMC decoder Rx pre-filter sampling method Number of consistent consecutive samples before confirming a new value."]
pub type RXFILT2N3_R = crate::BitReader;
#[doc = "Field `RXFILT2N3` writer - BMC decoder Rx pre-filter sampling method Number of consistent consecutive samples before confirming a new value."]
pub type RXFILT2N3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCECLK` reader - Force ClkReq clock request"]
pub type FORCECLK_R = crate::BitReader;
#[doc = "Field `FORCECLK` writer - Force ClkReq clock request"]
pub type FORCECLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN` reader - Wakeup from Stop mode enable Setting the bit enables the UCPD_ASYNC_INT signal."]
pub type WUPEN_R = crate::BitReader;
#[doc = "Field `WUPEN` writer - Wakeup from Stop mode enable Setting the bit enables the UCPD_ASYNC_INT signal."]
pub type WUPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXAFILTEN` reader - Rx analog filter enable Setting the bit enables the Rx analog filter required for optimum Power Delivery reception."]
pub type RXAFILTEN_R = crate::BitReader;
#[doc = "Field `RXAFILTEN` writer - Rx analog filter enable Setting the bit enables the Rx analog filter required for optimum Power Delivery reception."]
pub type RXAFILTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BMC decoder Rx pre-filter enable The sampling clock is that of the receiver (that is, after pre-scaler)."]
    #[inline(always)]
    pub fn rxfiltdis(&self) -> RXFILTDIS_R {
        RXFILTDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BMC decoder Rx pre-filter sampling method Number of consistent consecutive samples before confirming a new value."]
    #[inline(always)]
    pub fn rxfilt2n3(&self) -> RXFILT2N3_R {
        RXFILT2N3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force ClkReq clock request"]
    #[inline(always)]
    pub fn forceclk(&self) -> FORCECLK_R {
        FORCECLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup from Stop mode enable Setting the bit enables the UCPD_ASYNC_INT signal."]
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Rx analog filter enable Setting the bit enables the Rx analog filter required for optimum Power Delivery reception."]
    #[inline(always)]
    pub fn rxafilten(&self) -> RXAFILTEN_R {
        RXAFILTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BMC decoder Rx pre-filter enable The sampling clock is that of the receiver (that is, after pre-scaler)."]
    #[inline(always)]
    #[must_use]
    pub fn rxfiltdis(&mut self) -> RXFILTDIS_W<CFGR2rs> {
        RXFILTDIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - BMC decoder Rx pre-filter sampling method Number of consistent consecutive samples before confirming a new value."]
    #[inline(always)]
    #[must_use]
    pub fn rxfilt2n3(&mut self) -> RXFILT2N3_W<CFGR2rs> {
        RXFILT2N3_W::new(self, 1)
    }
    #[doc = "Bit 2 - Force ClkReq clock request"]
    #[inline(always)]
    #[must_use]
    pub fn forceclk(&mut self) -> FORCECLK_W<CFGR2rs> {
        FORCECLK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Wakeup from Stop mode enable Setting the bit enables the UCPD_ASYNC_INT signal."]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WUPEN_W<CFGR2rs> {
        WUPEN_W::new(self, 3)
    }
    #[doc = "Bit 8 - Rx analog filter enable Setting the bit enables the Rx analog filter required for optimum Power Delivery reception."]
    #[inline(always)]
    #[must_use]
    pub fn rxafilten(&mut self) -> RXAFILTEN_W<CFGR2rs> {
        RXAFILTEN_W::new(self, 8)
    }
}
#[doc = "UCPD configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
