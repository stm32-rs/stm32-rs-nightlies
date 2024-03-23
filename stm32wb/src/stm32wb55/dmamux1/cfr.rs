#[doc = "Register `CFR` writer"]
pub type W = crate::W<CFRrs>;
#[doc = "Field `CSOF0` writer - Synchronization Clear Overrun Flag 0"]
pub type CSOF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF1` writer - Synchronization Clear Overrun Flag 1"]
pub type CSOF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF2` writer - Synchronization Clear Overrun Flag 2"]
pub type CSOF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF3` writer - Synchronization Clear Overrun Flag 3"]
pub type CSOF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF4` writer - Synchronization Clear Overrun Flag 4"]
pub type CSOF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF5` writer - Synchronization Clear Overrun Flag 5"]
pub type CSOF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF6` writer - Synchronization Clear Overrun Flag 6"]
pub type CSOF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF7` writer - Synchronization Clear Overrun Flag 7"]
pub type CSOF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF8` writer - Synchronization Clear Overrun Flag 8"]
pub type CSOF8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF9` writer - Synchronization Clear Overrun Flag 9"]
pub type CSOF9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF10` writer - Synchronization Clear Overrun Flag 10"]
pub type CSOF10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF11` writer - Synchronization Clear Overrun Flag 11"]
pub type CSOF11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF12` writer - Synchronization Clear Overrun Flag 12"]
pub type CSOF12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF13` writer - Synchronization Clear Overrun Flag 13"]
pub type CSOF13_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Synchronization Clear Overrun Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF0_W<CFRrs> {
        CSOF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Synchronization Clear Overrun Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF1_W<CFRrs> {
        CSOF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization Clear Overrun Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF2_W<CFRrs> {
        CSOF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronization Clear Overrun Flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> CSOF3_W<CFRrs> {
        CSOF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Synchronization Clear Overrun Flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn csof4(&mut self) -> CSOF4_W<CFRrs> {
        CSOF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Synchronization Clear Overrun Flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn csof5(&mut self) -> CSOF5_W<CFRrs> {
        CSOF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Synchronization Clear Overrun Flag 6"]
    #[inline(always)]
    #[must_use]
    pub fn csof6(&mut self) -> CSOF6_W<CFRrs> {
        CSOF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Synchronization Clear Overrun Flag 7"]
    #[inline(always)]
    #[must_use]
    pub fn csof7(&mut self) -> CSOF7_W<CFRrs> {
        CSOF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Synchronization Clear Overrun Flag 8"]
    #[inline(always)]
    #[must_use]
    pub fn csof8(&mut self) -> CSOF8_W<CFRrs> {
        CSOF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Synchronization Clear Overrun Flag 9"]
    #[inline(always)]
    #[must_use]
    pub fn csof9(&mut self) -> CSOF9_W<CFRrs> {
        CSOF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Synchronization Clear Overrun Flag 10"]
    #[inline(always)]
    #[must_use]
    pub fn csof10(&mut self) -> CSOF10_W<CFRrs> {
        CSOF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronization Clear Overrun Flag 11"]
    #[inline(always)]
    #[must_use]
    pub fn csof11(&mut self) -> CSOF11_W<CFRrs> {
        CSOF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Synchronization Clear Overrun Flag 12"]
    #[inline(always)]
    #[must_use]
    pub fn csof12(&mut self) -> CSOF12_W<CFRrs> {
        CSOF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    #[must_use]
    pub fn csof13(&mut self) -> CSOF13_W<CFRrs> {
        CSOF13_W::new(self, 13)
    }
}
#[doc = "DMA Channel Clear Flag Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFR to value 0"]
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u32 = 0;
}
