#[doc = "Register `SR2` reader"]
pub type R = crate::R<SR2rs>;
#[doc = "Register `SR2` writer"]
pub type W = crate::W<SR2rs>;
#[doc = "Field `IRS` reader - IRS"]
pub type IRS_R = crate::BitReader;
#[doc = "Field `IRS` writer - IRS"]
pub type IRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILS` reader - ILS"]
pub type ILS_R = crate::BitReader;
#[doc = "Field `ILS` writer - ILS"]
pub type ILS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFS` reader - IFS"]
pub type IFS_R = crate::BitReader;
#[doc = "Field `IFS` writer - IFS"]
pub type IFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREN` reader - IREN"]
pub type IREN_R = crate::BitReader;
#[doc = "Field `IREN` writer - IREN"]
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILEN` reader - ILEN"]
pub type ILEN_R = crate::BitReader;
#[doc = "Field `ILEN` writer - ILEN"]
pub type ILEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFEN` reader - IFEN"]
pub type IFEN_R = crate::BitReader;
#[doc = "Field `IFEN` writer - IFEN"]
pub type IFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEMPT` reader - FEMPT"]
pub type FEMPT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FEMPT"]
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    #[must_use]
    pub fn irs(&mut self) -> IRS_W<SR2rs> {
        IRS_W::new(self, 0)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    #[must_use]
    pub fn ils(&mut self) -> ILS_W<SR2rs> {
        ILS_W::new(self, 1)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    #[must_use]
    pub fn ifs(&mut self) -> IFS_W<SR2rs> {
        IFS_W::new(self, 2)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<SR2rs> {
        IREN_W::new(self, 3)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    #[must_use]
    pub fn ilen(&mut self) -> ILEN_W<SR2rs> {
        ILEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    #[must_use]
    pub fn ifen(&mut self) -> IFEN_W<SR2rs> {
        IFEN_W::new(self, 5)
    }
}
#[doc = "FIFO status and interrupt register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for SR2rs {}
#[doc = "`write(|w| ..)` method takes [`sr2::W`](W) writer structure"]
impl crate::Writable for SR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR2 to value 0x40"]
impl crate::Resettable for SR2rs {
    const RESET_VALUE: u32 = 0x40;
}
