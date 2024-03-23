#[doc = "Register `SECCFGR` writer"]
pub type W = crate::W<SECCFGRrs>;
#[doc = "Field `SEC0` writer - I/O pin of Port x secure bit enable"]
pub type SEC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC1` writer - I/O pin of Port x secure bit enable"]
pub type SEC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC2` writer - I/O pin of Port x secure bit enable"]
pub type SEC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC3` writer - I/O pin of Port x secure bit enable"]
pub type SEC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC4` writer - I/O pin of Port x secure bit enable"]
pub type SEC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC5` writer - I/O pin of Port x secure bit enable"]
pub type SEC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC6` writer - I/O pin of Port x secure bit enable"]
pub type SEC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC7` writer - I/O pin of Port x secure bit enable"]
pub type SEC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC8` writer - I/O pin of Port x secure bit enable"]
pub type SEC8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC9` writer - I/O pin of Port x secure bit enable"]
pub type SEC9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC10` writer - I/O pin of Port x secure bit enable"]
pub type SEC10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC11` writer - I/O pin of Port x secure bit enable"]
pub type SEC11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC12` writer - I/O pin of Port x secure bit enable"]
pub type SEC12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC13` writer - I/O pin of Port x secure bit enable"]
pub type SEC13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC14` writer - I/O pin of Port x secure bit enable"]
pub type SEC14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC15` writer - I/O pin of Port x secure bit enable"]
pub type SEC15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<SECCFGRrs> {
        SEC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<SECCFGRrs> {
        SEC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<SECCFGRrs> {
        SEC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<SECCFGRrs> {
        SEC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec4(&mut self) -> SEC4_W<SECCFGRrs> {
        SEC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec5(&mut self) -> SEC5_W<SECCFGRrs> {
        SEC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec6(&mut self) -> SEC6_W<SECCFGRrs> {
        SEC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec7(&mut self) -> SEC7_W<SECCFGRrs> {
        SEC7_W::new(self, 7)
    }
    #[doc = "Bit 8 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec8(&mut self) -> SEC8_W<SECCFGRrs> {
        SEC8_W::new(self, 8)
    }
    #[doc = "Bit 9 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec9(&mut self) -> SEC9_W<SECCFGRrs> {
        SEC9_W::new(self, 9)
    }
    #[doc = "Bit 10 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec10(&mut self) -> SEC10_W<SECCFGRrs> {
        SEC10_W::new(self, 10)
    }
    #[doc = "Bit 11 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec11(&mut self) -> SEC11_W<SECCFGRrs> {
        SEC11_W::new(self, 11)
    }
    #[doc = "Bit 12 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec12(&mut self) -> SEC12_W<SECCFGRrs> {
        SEC12_W::new(self, 12)
    }
    #[doc = "Bit 13 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec13(&mut self) -> SEC13_W<SECCFGRrs> {
        SEC13_W::new(self, 13)
    }
    #[doc = "Bit 14 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec14(&mut self) -> SEC14_W<SECCFGRrs> {
        SEC14_W::new(self, 14)
    }
    #[doc = "Bit 15 - I/O pin of Port x secure bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec15(&mut self) -> SEC15_W<SECCFGRrs> {
        SEC15_W::new(self, 15)
    }
}
#[doc = "GPIO secure configuration register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure"]
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
