#[doc = "Register `CCFR` reader"]
pub type R = crate::R<CCFRrs>;
#[doc = "Register `CCFR` writer"]
pub type W = crate::W<CCFRrs>;
#[doc = "Field `CSOF0` reader - Synchronization Clear Overrun Flag 0"]
pub type CSOF0_R = crate::BitReader;
#[doc = "Field `CSOF0` writer - Synchronization Clear Overrun Flag 0"]
pub type CSOF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF1` reader - Synchronization Clear Overrun Flag 1"]
pub type CSOF1_R = crate::BitReader;
#[doc = "Field `CSOF1` writer - Synchronization Clear Overrun Flag 1"]
pub type CSOF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF2` reader - Synchronization Clear Overrun Flag 2"]
pub type CSOF2_R = crate::BitReader;
#[doc = "Field `CSOF2` writer - Synchronization Clear Overrun Flag 2"]
pub type CSOF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF3` reader - Synchronization Clear Overrun Flag 3"]
pub type CSOF3_R = crate::BitReader;
#[doc = "Field `CSOF3` writer - Synchronization Clear Overrun Flag 3"]
pub type CSOF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF4` reader - Synchronization Clear Overrun Flag 4"]
pub type CSOF4_R = crate::BitReader;
#[doc = "Field `CSOF4` writer - Synchronization Clear Overrun Flag 4"]
pub type CSOF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF5` reader - Synchronization Clear Overrun Flag 5"]
pub type CSOF5_R = crate::BitReader;
#[doc = "Field `CSOF5` writer - Synchronization Clear Overrun Flag 5"]
pub type CSOF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF6` reader - Synchronization Clear Overrun Flag 6"]
pub type CSOF6_R = crate::BitReader;
#[doc = "Field `CSOF6` writer - Synchronization Clear Overrun Flag 6"]
pub type CSOF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF7` reader - Synchronization Clear Overrun Flag 7"]
pub type CSOF7_R = crate::BitReader;
#[doc = "Field `CSOF7` writer - Synchronization Clear Overrun Flag 7"]
pub type CSOF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF8` reader - Synchronization Clear Overrun Flag 8"]
pub type CSOF8_R = crate::BitReader;
#[doc = "Field `CSOF8` writer - Synchronization Clear Overrun Flag 8"]
pub type CSOF8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF9` reader - Synchronization Clear Overrun Flag 9"]
pub type CSOF9_R = crate::BitReader;
#[doc = "Field `CSOF9` writer - Synchronization Clear Overrun Flag 9"]
pub type CSOF9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF10` reader - Synchronization Clear Overrun Flag 10"]
pub type CSOF10_R = crate::BitReader;
#[doc = "Field `CSOF10` writer - Synchronization Clear Overrun Flag 10"]
pub type CSOF10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF11` reader - Synchronization Clear Overrun Flag 11"]
pub type CSOF11_R = crate::BitReader;
#[doc = "Field `CSOF11` writer - Synchronization Clear Overrun Flag 11"]
pub type CSOF11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF12` reader - Synchronization Clear Overrun Flag 12"]
pub type CSOF12_R = crate::BitReader;
#[doc = "Field `CSOF12` writer - Synchronization Clear Overrun Flag 12"]
pub type CSOF12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF13` reader - Synchronization Clear Overrun Flag 13"]
pub type CSOF13_R = crate::BitReader;
#[doc = "Field `CSOF13` writer - Synchronization Clear Overrun Flag 13"]
pub type CSOF13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF14` reader - Synchronization Clear Overrun Flag 13"]
pub type CSOF14_R = crate::BitReader;
#[doc = "Field `CSOF14` writer - Synchronization Clear Overrun Flag 13"]
pub type CSOF14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF15` reader - Synchronization Clear Overrun Flag 13"]
pub type CSOF15_R = crate::BitReader;
#[doc = "Field `CSOF15` writer - Synchronization Clear Overrun Flag 13"]
pub type CSOF15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Synchronization Clear Overrun Flag 0"]
    #[inline(always)]
    pub fn csof0(&self) -> CSOF0_R {
        CSOF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronization Clear Overrun Flag 1"]
    #[inline(always)]
    pub fn csof1(&self) -> CSOF1_R {
        CSOF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization Clear Overrun Flag 2"]
    #[inline(always)]
    pub fn csof2(&self) -> CSOF2_R {
        CSOF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Clear Overrun Flag 3"]
    #[inline(always)]
    pub fn csof3(&self) -> CSOF3_R {
        CSOF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization Clear Overrun Flag 4"]
    #[inline(always)]
    pub fn csof4(&self) -> CSOF4_R {
        CSOF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization Clear Overrun Flag 5"]
    #[inline(always)]
    pub fn csof5(&self) -> CSOF5_R {
        CSOF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization Clear Overrun Flag 6"]
    #[inline(always)]
    pub fn csof6(&self) -> CSOF6_R {
        CSOF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization Clear Overrun Flag 7"]
    #[inline(always)]
    pub fn csof7(&self) -> CSOF7_R {
        CSOF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronization Clear Overrun Flag 8"]
    #[inline(always)]
    pub fn csof8(&self) -> CSOF8_R {
        CSOF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Synchronization Clear Overrun Flag 9"]
    #[inline(always)]
    pub fn csof9(&self) -> CSOF9_R {
        CSOF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization Clear Overrun Flag 10"]
    #[inline(always)]
    pub fn csof10(&self) -> CSOF10_R {
        CSOF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization Clear Overrun Flag 11"]
    #[inline(always)]
    pub fn csof11(&self) -> CSOF11_R {
        CSOF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Synchronization Clear Overrun Flag 12"]
    #[inline(always)]
    pub fn csof12(&self) -> CSOF12_R {
        CSOF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof13(&self) -> CSOF13_R {
        CSOF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof14(&self) -> CSOF14_R {
        CSOF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof15(&self) -> CSOF15_R {
        CSOF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronization Clear Overrun Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF0_W<CCFRrs> {
        CSOF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Synchronization Clear Overrun Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF1_W<CCFRrs> {
        CSOF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization Clear Overrun Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF2_W<CCFRrs> {
        CSOF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronization Clear Overrun Flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> CSOF3_W<CCFRrs> {
        CSOF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Synchronization Clear Overrun Flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn csof4(&mut self) -> CSOF4_W<CCFRrs> {
        CSOF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Synchronization Clear Overrun Flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn csof5(&mut self) -> CSOF5_W<CCFRrs> {
        CSOF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Synchronization Clear Overrun Flag 6"]
    #[inline(always)]
    #[must_use]
    pub fn csof6(&mut self) -> CSOF6_W<CCFRrs> {
        CSOF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Synchronization Clear Overrun Flag 7"]
    #[inline(always)]
    #[must_use]
    pub fn csof7(&mut self) -> CSOF7_W<CCFRrs> {
        CSOF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Synchronization Clear Overrun Flag 8"]
    #[inline(always)]
    #[must_use]
    pub fn csof8(&mut self) -> CSOF8_W<CCFRrs> {
        CSOF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Synchronization Clear Overrun Flag 9"]
    #[inline(always)]
    #[must_use]
    pub fn csof9(&mut self) -> CSOF9_W<CCFRrs> {
        CSOF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Synchronization Clear Overrun Flag 10"]
    #[inline(always)]
    #[must_use]
    pub fn csof10(&mut self) -> CSOF10_W<CCFRrs> {
        CSOF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Synchronization Clear Overrun Flag 11"]
    #[inline(always)]
    #[must_use]
    pub fn csof11(&mut self) -> CSOF11_W<CCFRrs> {
        CSOF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Synchronization Clear Overrun Flag 12"]
    #[inline(always)]
    #[must_use]
    pub fn csof12(&mut self) -> CSOF12_W<CCFRrs> {
        CSOF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    #[must_use]
    pub fn csof13(&mut self) -> CSOF13_W<CCFRrs> {
        CSOF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    #[must_use]
    pub fn csof14(&mut self) -> CSOF14_W<CCFRrs> {
        CSOF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    #[must_use]
    pub fn csof15(&mut self) -> CSOF15_W<CCFRrs> {
        CSOF15_W::new(self, 15)
    }
}
#[doc = "DMA Channel Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCFRrs;
impl crate::RegisterSpec for CCFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfr::R`](R) reader structure"]
impl crate::Readable for CCFRrs {}
#[doc = "`write(|w| ..)` method takes [`ccfr::W`](W) writer structure"]
impl crate::Writable for CCFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCFR to value 0"]
impl crate::Resettable for CCFRrs {
    const RESET_VALUE: u32 = 0;
}
