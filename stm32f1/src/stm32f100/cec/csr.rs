#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Field `TSOM` reader - Tx start of message"]
pub type TSOM_R = crate::BitReader;
#[doc = "Field `TSOM` writer - Tx start of message"]
pub type TSOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEOM` reader - Tx end of message"]
pub type TEOM_R = crate::BitReader;
#[doc = "Field `TEOM` writer - Tx end of message"]
pub type TEOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERR` reader - Tx error"]
pub type TERR_R = crate::BitReader;
#[doc = "Field `TERR` writer - Tx error"]
pub type TERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBTRF` reader - Tx byte transfer request or block transfer finished"]
pub type TBTRF_R = crate::BitReader;
#[doc = "Field `TBTRF` writer - Tx byte transfer request or block transfer finished"]
pub type TBTRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSOM` reader - Rx start of message"]
pub type RSOM_R = crate::BitReader;
#[doc = "Field `RSOM` writer - Rx start of message"]
pub type RSOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REOM` reader - Rx end of message"]
pub type REOM_R = crate::BitReader;
#[doc = "Field `REOM` writer - Rx end of message"]
pub type REOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RERR` reader - Rx error"]
pub type RERR_R = crate::BitReader;
#[doc = "Field `RERR` writer - Rx error"]
pub type RERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBTF` reader - Rx byte/block transfer finished"]
pub type RBTF_R = crate::BitReader;
#[doc = "Field `RBTF` writer - Rx byte/block transfer finished"]
pub type RBTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tx start of message"]
    #[inline(always)]
    pub fn tsom(&self) -> TSOM_R {
        TSOM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx end of message"]
    #[inline(always)]
    pub fn teom(&self) -> TEOM_R {
        TEOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tx error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx byte transfer request or block transfer finished"]
    #[inline(always)]
    pub fn tbtrf(&self) -> TBTRF_R {
        TBTRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx start of message"]
    #[inline(always)]
    pub fn rsom(&self) -> RSOM_R {
        RSOM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx end of message"]
    #[inline(always)]
    pub fn reom(&self) -> REOM_R {
        REOM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx error"]
    #[inline(always)]
    pub fn rerr(&self) -> RERR_R {
        RERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx byte/block transfer finished"]
    #[inline(always)]
    pub fn rbtf(&self) -> RBTF_R {
        RBTF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tx start of message"]
    #[inline(always)]
    #[must_use]
    pub fn tsom(&mut self) -> TSOM_W<CSRrs> {
        TSOM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tx end of message"]
    #[inline(always)]
    #[must_use]
    pub fn teom(&mut self) -> TEOM_W<CSRrs> {
        TEOM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tx error"]
    #[inline(always)]
    #[must_use]
    pub fn terr(&mut self) -> TERR_W<CSRrs> {
        TERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Tx byte transfer request or block transfer finished"]
    #[inline(always)]
    #[must_use]
    pub fn tbtrf(&mut self) -> TBTRF_W<CSRrs> {
        TBTRF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rx start of message"]
    #[inline(always)]
    #[must_use]
    pub fn rsom(&mut self) -> RSOM_W<CSRrs> {
        RSOM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rx end of message"]
    #[inline(always)]
    #[must_use]
    pub fn reom(&mut self) -> REOM_W<CSRrs> {
        REOM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rx error"]
    #[inline(always)]
    #[must_use]
    pub fn rerr(&mut self) -> RERR_W<CSRrs> {
        RERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rx byte/block transfer finished"]
    #[inline(always)]
    #[must_use]
    pub fn rbtf(&mut self) -> RBTF_W<CSRrs> {
        RBTF_W::new(self, 7)
    }
}
#[doc = "CEC control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
