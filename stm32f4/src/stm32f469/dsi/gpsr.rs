#[doc = "Register `GPSR` reader"]
pub type R = crate::R<GPSRrs>;
#[doc = "Register `GPSR` writer"]
pub type W = crate::W<GPSRrs>;
#[doc = "Field `CMDFE` reader - Tearing Effect Acknowledge Request Enable"]
pub type CMDFE_R = crate::BitReader;
#[doc = "Field `CMDFE` writer - Tearing Effect Acknowledge Request Enable"]
pub type CMDFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDFF` reader - Acknowledge Request Enable"]
pub type CMDFF_R = crate::BitReader;
#[doc = "Field `CMDFF` writer - Acknowledge Request Enable"]
pub type CMDFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRFE` reader - PWRFE"]
pub type PWRFE_R = crate::BitReader;
#[doc = "Field `PWRFE` writer - PWRFE"]
pub type PWRFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRFF` reader - PWRFF"]
pub type PWRFF_R = crate::BitReader;
#[doc = "Field `PWRFF` writer - PWRFF"]
pub type PWRFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRDFE` reader - PRDFE"]
pub type PRDFE_R = crate::BitReader;
#[doc = "Field `PRDFE` writer - PRDFE"]
pub type PRDFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRDFF` reader - PRDFF"]
pub type PRDFF_R = crate::BitReader;
#[doc = "Field `PRDFF` writer - PRDFF"]
pub type PRDFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCB` reader - RCB"]
pub type RCB_R = crate::BitReader;
#[doc = "Field `RCB` writer - RCB"]
pub type RCB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tearing Effect Acknowledge Request Enable"]
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledge Request Enable"]
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWRFE"]
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWRFF"]
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRDFE"]
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRDFF"]
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RCB"]
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tearing Effect Acknowledge Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdfe(&mut self) -> CMDFE_W<GPSRrs> {
        CMDFE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Acknowledge Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdff(&mut self) -> CMDFF_W<GPSRrs> {
        CMDFF_W::new(self, 1)
    }
    #[doc = "Bit 2 - PWRFE"]
    #[inline(always)]
    #[must_use]
    pub fn pwrfe(&mut self) -> PWRFE_W<GPSRrs> {
        PWRFE_W::new(self, 2)
    }
    #[doc = "Bit 3 - PWRFF"]
    #[inline(always)]
    #[must_use]
    pub fn pwrff(&mut self) -> PWRFF_W<GPSRrs> {
        PWRFF_W::new(self, 3)
    }
    #[doc = "Bit 4 - PRDFE"]
    #[inline(always)]
    #[must_use]
    pub fn prdfe(&mut self) -> PRDFE_W<GPSRrs> {
        PRDFE_W::new(self, 4)
    }
    #[doc = "Bit 5 - PRDFF"]
    #[inline(always)]
    #[must_use]
    pub fn prdff(&mut self) -> PRDFF_W<GPSRrs> {
        PRDFF_W::new(self, 5)
    }
    #[doc = "Bit 6 - RCB"]
    #[inline(always)]
    #[must_use]
    pub fn rcb(&mut self) -> RCB_W<GPSRrs> {
        RCB_W::new(self, 6)
    }
}
#[doc = "DSI Host Generic Packet Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets GPSR to value 0"]
impl crate::Resettable for GPSRrs {
    const RESET_VALUE: u32 = 0;
}
