#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `EOP` reader - End of operation"]
pub type EOP_R = crate::BitReader;
#[doc = "Field `EOP` writer - End of operation"]
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERR` reader - Operation error"]
pub type OPERR_R = crate::BitReader;
#[doc = "Field `OPERR` writer - Operation error"]
pub type OPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERR` reader - Write protection error"]
pub type WRPERR_R = crate::BitReader;
#[doc = "Field `WRPERR` writer - Write protection error"]
pub type WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAERR` reader - Programming alignment error"]
pub type PGAERR_R = crate::BitReader;
#[doc = "Field `PGAERR` writer - Programming alignment error"]
pub type PGAERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGPERR` reader - Programming parallelism error"]
pub type PGPERR_R = crate::BitReader;
#[doc = "Field `PGPERR` writer - Programming parallelism error"]
pub type PGPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERR` reader - Programming sequence error"]
pub type PGSERR_R = crate::BitReader;
#[doc = "Field `PGSERR` writer - Programming sequence error"]
pub type PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSY` reader - Busy"]
pub type BSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Programming parallelism error"]
    #[inline(always)]
    pub fn pgperr(&self) -> PGPERR_R {
        PGPERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<SRrs> {
        EOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OPERR_W<SRrs> {
        OPERR_W::new(self, 1)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<SRrs> {
        WRPERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<SRrs> {
        PGAERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Programming parallelism error"]
    #[inline(always)]
    #[must_use]
    pub fn pgperr(&mut self) -> PGPERR_W<SRrs> {
        PGPERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PGSERR_W<SRrs> {
        PGSERR_W::new(self, 7)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
