#[doc = "Register `CFR` writer"]
pub type W = crate::W<CFRrs>;
#[doc = "Clear synchronization overrun event flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSOF0 {
    #[doc = "1: Clear synchronization flag"]
    Clear = 1,
}
impl From<CSOF0> for bool {
    #[inline(always)]
    fn from(variant: CSOF0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSOF0` writer - Clear synchronization overrun event flag"]
pub type CSOF0_W<'a, REG> = crate::BitWriter<'a, REG, CSOF0>;
impl<'a, REG> CSOF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear synchronization flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSOF0::Clear)
    }
}
#[doc = "Field `CSOF1` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF1_W;
#[doc = "Field `CSOF2` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF2_W;
#[doc = "Field `CSOF3` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF3_W;
#[doc = "Field `CSOF4` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF4_W;
#[doc = "Field `CSOF5` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF5_W;
#[doc = "Field `CSOF6` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF6_W;
#[doc = "Field `CSOF7` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF7_W;
#[doc = "Field `CSOF8` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF8_W;
#[doc = "Field `CSOF9` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF9_W;
#[doc = "Field `CSOF10` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF10_W;
#[doc = "Field `CSOF11` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF11_W;
#[doc = "Field `CSOF12` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF12_W;
#[doc = "Field `CSOF13` writer - Clear synchronization overrun event flag"]
pub use CSOF0_W as CSOF13_W;
impl W {
    #[doc = "Bit 0 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF0_W<CFRrs> {
        CSOF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF1_W<CFRrs> {
        CSOF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF2_W<CFRrs> {
        CSOF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> CSOF3_W<CFRrs> {
        CSOF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof4(&mut self) -> CSOF4_W<CFRrs> {
        CSOF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof5(&mut self) -> CSOF5_W<CFRrs> {
        CSOF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof6(&mut self) -> CSOF6_W<CFRrs> {
        CSOF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof7(&mut self) -> CSOF7_W<CFRrs> {
        CSOF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof8(&mut self) -> CSOF8_W<CFRrs> {
        CSOF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof9(&mut self) -> CSOF9_W<CFRrs> {
        CSOF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof10(&mut self) -> CSOF10_W<CFRrs> {
        CSOF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof11(&mut self) -> CSOF11_W<CFRrs> {
        CSOF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof12(&mut self) -> CSOF12_W<CFRrs> {
        CSOF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof13(&mut self) -> CSOF13_W<CFRrs> {
        CSOF13_W::new(self, 13)
    }
}
#[doc = "clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
