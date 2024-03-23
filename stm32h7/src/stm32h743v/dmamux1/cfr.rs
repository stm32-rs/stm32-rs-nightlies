#[doc = "Register `CFR` writer"]
pub type W = crate::W<CFRrs>;
#[doc = "Field `CSOF(0-15)` writer - Clear synchronization overrun event flag"]
pub type CSOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Clear synchronization overrun event flag"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CSOF0` field"]
    #[inline(always)]
    #[must_use]
    pub fn csof(&mut self, n: u8) -> CSOF_W<CFRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        CSOF_W::new(self, n)
    }
    #[doc = "Bit 0 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof4(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof5(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof6(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof7(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof8(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof9(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof10(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof11(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof12(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof13(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof14(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear synchronization overrun event flag"]
    #[inline(always)]
    #[must_use]
    pub fn csof15(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 15)
    }
}
#[doc = "DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
