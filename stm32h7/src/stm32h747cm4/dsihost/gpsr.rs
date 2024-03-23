#[doc = "Register `GPSR` reader"]
pub type R = crate::R<GPSRrs>;
#[doc = "Register `GPSR` writer"]
pub type W = crate::W<GPSRrs>;
#[doc = "Field `CMDFE` reader - Command FIFO empty"]
pub type CMDFE_R = crate::BitReader;
#[doc = "Field `CMDFE` writer - Command FIFO empty"]
pub type CMDFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDFF` reader - Command FIFO full"]
pub type CMDFF_R = crate::BitReader;
#[doc = "Field `CMDFF` writer - Command FIFO full"]
pub type CMDFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRFE` reader - Payload write FIFO empty"]
pub type PWRFE_R = crate::BitReader;
#[doc = "Field `PWRFE` writer - Payload write FIFO empty"]
pub type PWRFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRFF` reader - Payload write FIFO full"]
pub type PWRFF_R = crate::BitReader;
#[doc = "Field `PWRFF` writer - Payload write FIFO full"]
pub type PWRFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRDFE` reader - Payload read FIFO empty"]
pub type PRDFE_R = crate::BitReader;
#[doc = "Field `PRDFE` writer - Payload read FIFO empty"]
pub type PRDFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRDFF` reader - Payload read FIFO full"]
pub type PRDFF_R = crate::BitReader;
#[doc = "Field `PRDFF` writer - Payload read FIFO full"]
pub type PRDFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB` reader - Read command busy"]
pub type RCB_R = crate::BitReader;
#[doc = "Field `RCB` writer - Read command busy"]
pub type RCB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command FIFO empty"]
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command FIFO full"]
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Payload write FIFO empty"]
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Payload write FIFO full"]
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Payload read FIFO empty"]
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Payload read FIFO full"]
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read command busy"]
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn cmdfe(&mut self) -> CMDFE_W<GPSRrs> {
        CMDFE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Command FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn cmdff(&mut self) -> CMDFF_W<GPSRrs> {
        CMDFF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Payload write FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn pwrfe(&mut self) -> PWRFE_W<GPSRrs> {
        PWRFE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Payload write FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn pwrff(&mut self) -> PWRFF_W<GPSRrs> {
        PWRFF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Payload read FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn prdfe(&mut self) -> PRDFE_W<GPSRrs> {
        PRDFE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Payload read FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn prdff(&mut self) -> PRDFF_W<GPSRrs> {
        PRDFF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Read command busy"]
    #[inline(always)]
    #[must_use]
    pub fn rcb(&mut self) -> RCB_W<GPSRrs> {
        RCB_W::new(self, 6)
    }
}
#[doc = "DSI Host generic packet status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPSRrs;
impl crate::RegisterSpec for GPSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpsr::R`](R) reader structure"]
impl crate::Readable for GPSRrs {}
#[doc = "`write(|w| ..)` method takes [`gpsr::W`](W) writer structure"]
impl crate::Writable for GPSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPSR to value 0x15"]
impl crate::Resettable for GPSRrs {
    const RESET_VALUE: u32 = 0x15;
}
