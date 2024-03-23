#[doc = "Register `CCFR` writer"]
pub type W = crate::W<CCFRrs>;
#[doc = "CSOF0\n\nValue on reset: 0"]
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
#[doc = "Field `CSOF0` writer - CSOF0"]
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
#[doc = "Field `CSOF1` writer - CSOF1"]
pub use CSOF0_W as CSOF1_W;
#[doc = "Field `CSOF2` writer - CSOF2"]
pub use CSOF0_W as CSOF2_W;
#[doc = "Field `CSOF3` writer - CSOF3"]
pub use CSOF0_W as CSOF3_W;
#[doc = "Field `CSOF4` writer - CSOF4"]
pub use CSOF0_W as CSOF4_W;
#[doc = "Field `CSOF5` writer - CSOF5"]
pub use CSOF0_W as CSOF5_W;
#[doc = "Field `CSOF6` writer - CSOF6"]
pub use CSOF0_W as CSOF6_W;
#[doc = "Field `CSOF7` writer - CSOF7"]
pub use CSOF0_W as CSOF7_W;
#[doc = "Field `CSOF8` writer - CSOF8"]
pub use CSOF0_W as CSOF8_W;
#[doc = "Field `CSOF9` writer - CSOF9"]
pub use CSOF0_W as CSOF9_W;
#[doc = "Field `CSOF10` writer - CSOF10"]
pub use CSOF0_W as CSOF10_W;
#[doc = "Field `CSOF11` writer - CSOF11"]
pub use CSOF0_W as CSOF11_W;
#[doc = "Field `CSOF12` writer - CSOF12"]
pub use CSOF0_W as CSOF12_W;
#[doc = "Field `CSOF13` writer - CSOF13"]
pub use CSOF0_W as CSOF13_W;
impl W {
    #[doc = "Bit 0 - CSOF0"]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF0_W<CCFRrs> {
        CSOF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - CSOF1"]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF1_W<CCFRrs> {
        CSOF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - CSOF2"]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF2_W<CCFRrs> {
        CSOF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - CSOF3"]
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> CSOF3_W<CCFRrs> {
        CSOF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - CSOF4"]
    #[inline(always)]
    #[must_use]
    pub fn csof4(&mut self) -> CSOF4_W<CCFRrs> {
        CSOF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - CSOF5"]
    #[inline(always)]
    #[must_use]
    pub fn csof5(&mut self) -> CSOF5_W<CCFRrs> {
        CSOF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - CSOF6"]
    #[inline(always)]
    #[must_use]
    pub fn csof6(&mut self) -> CSOF6_W<CCFRrs> {
        CSOF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - CSOF7"]
    #[inline(always)]
    #[must_use]
    pub fn csof7(&mut self) -> CSOF7_W<CCFRrs> {
        CSOF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - CSOF8"]
    #[inline(always)]
    #[must_use]
    pub fn csof8(&mut self) -> CSOF8_W<CCFRrs> {
        CSOF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - CSOF9"]
    #[inline(always)]
    #[must_use]
    pub fn csof9(&mut self) -> CSOF9_W<CCFRrs> {
        CSOF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - CSOF10"]
    #[inline(always)]
    #[must_use]
    pub fn csof10(&mut self) -> CSOF10_W<CCFRrs> {
        CSOF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - CSOF11"]
    #[inline(always)]
    #[must_use]
    pub fn csof11(&mut self) -> CSOF11_W<CCFRrs> {
        CSOF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - CSOF12"]
    #[inline(always)]
    #[must_use]
    pub fn csof12(&mut self) -> CSOF12_W<CCFRrs> {
        CSOF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - CSOF13"]
    #[inline(always)]
    #[must_use]
    pub fn csof13(&mut self) -> CSOF13_W<CCFRrs> {
        CSOF13_W::new(self, 13)
    }
}
#[doc = "request line multiplexer interrupt channel clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCFRrs;
impl crate::RegisterSpec for CCFRrs {
    type Ux = u32;
}
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
