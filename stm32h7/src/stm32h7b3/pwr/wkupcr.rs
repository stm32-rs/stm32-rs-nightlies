#[doc = "Register `WKUPCR` reader"]
pub type R = crate::R<WKUPCRrs>;
#[doc = "Register `WKUPCR` writer"]
pub type W = crate::W<WKUPCRrs>;
#[doc = "Field `WKUPC1` reader - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC1_R = crate::BitReader;
#[doc = "Field `WKUPC1` writer - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPC2` reader - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC2_R = crate::BitReader;
#[doc = "Field `WKUPC2` writer - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPC3` reader - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC3_R = crate::BitReader;
#[doc = "Field `WKUPC3` writer - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPC4` reader - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC4_R = crate::BitReader;
#[doc = "Field `WKUPC4` writer - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPC5` reader - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC5_R = crate::BitReader;
#[doc = "Field `WKUPC5` writer - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPC6` reader - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC6_R = crate::BitReader;
#[doc = "Field `WKUPC6` writer - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc1(&self) -> WKUPC1_R {
        WKUPC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc2(&self) -> WKUPC2_R {
        WKUPC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc3(&self) -> WKUPC3_R {
        WKUPC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc4(&self) -> WKUPC4_R {
        WKUPC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc5(&self) -> WKUPC5_R {
        WKUPC5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc6(&self) -> WKUPC6_R {
        WKUPC6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn wkupc1(&mut self) -> WKUPC1_W<WKUPCRrs> {
        WKUPC1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn wkupc2(&mut self) -> WKUPC2_W<WKUPCRrs> {
        WKUPC2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn wkupc3(&mut self) -> WKUPC3_W<WKUPCRrs> {
        WKUPC3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn wkupc4(&mut self) -> WKUPC4_W<WKUPCRrs> {
        WKUPC4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn wkupc5(&mut self) -> WKUPC5_W<WKUPCRrs> {
        WKUPC5_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn wkupc6(&mut self) -> WKUPC6_W<WKUPCRrs> {
        WKUPC6_W::new(self, 5)
    }
}
#[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkupcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkupcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WKUPCRrs;
impl crate::RegisterSpec for WKUPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkupcr::R`](R) reader structure"]
impl crate::Readable for WKUPCRrs {}
#[doc = "`write(|w| ..)` method takes [`wkupcr::W`](W) writer structure"]
impl crate::Writable for WKUPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKUPCR to value 0"]
impl crate::Resettable for WKUPCRrs {
    const RESET_VALUE: u32 = 0;
}
