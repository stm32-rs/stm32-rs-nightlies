#[doc = "Register `OR2` reader"]
pub type R = crate::R<OR2rs>;
#[doc = "Register `OR2` writer"]
pub type W = crate::W<OR2rs>;
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub type BKINE_R = crate::BitReader;
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1E` reader - BRK COMP1 enable"]
pub type BKCMP1E_R = crate::BitReader;
#[doc = "Field `BKCMP1E` writer - BRK COMP1 enable"]
pub type BKCMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2E` reader - BRK COMP2 enable"]
pub type BKCMP2E_R = crate::BitReader;
#[doc = "Field `BKCMP2E` writer - BRK COMP2 enable"]
pub type BKCMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKDFBK1E` reader - BRK DFSDM_BREAK1 enable"]
pub type BKDFBK1E_R = crate::BitReader;
#[doc = "Field `BKDFBK1E` writer - BRK DFSDM_BREAK1 enable"]
pub type BKDFBK1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKINP` reader - BRK BKIN input polarity"]
pub type BKINP_R = crate::BitReader;
#[doc = "Field `BKINP` writer - BRK BKIN input polarity"]
pub type BKINP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1P` reader - BRK COMP1 input polarity"]
pub type BKCMP1P_R = crate::BitReader;
#[doc = "Field `BKCMP1P` writer - BRK COMP1 input polarity"]
pub type BKCMP1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2P` reader - BRK COMP2 input polarit"]
pub type BKCMP2P_R = crate::BitReader;
#[doc = "Field `BKCMP2P` writer - BRK COMP2 input polarit"]
pub type BKCMP2P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - BRK DFSDM_BREAK1 enable"]
    #[inline(always)]
    pub fn bkdfbk1e(&self) -> BKDFBK1E_R {
        BKDFBK1E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarit"]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<OR2rs> {
        BKINE_W::new(self, 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<OR2rs> {
        BKCMP1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<OR2rs> {
        BKCMP2E_W::new(self, 2)
    }
    #[doc = "Bit 8 - BRK DFSDM_BREAK1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkdfbk1e(&mut self) -> BKDFBK1E_W<OR2rs> {
        BKDFBK1E_W::new(self, 8)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<OR2rs> {
        BKINP_W::new(self, 9)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<OR2rs> {
        BKCMP1P_W::new(self, 10)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarit"]
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<OR2rs> {
        BKCMP2P_W::new(self, 11)
    }
}
#[doc = "TIM17 option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR2rs;
impl crate::RegisterSpec for OR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or2::R`](R) reader structure"]
impl crate::Readable for OR2rs {}
#[doc = "`write(|w| ..)` method takes [`or2::W`](W) writer structure"]
impl crate::Writable for OR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR2 to value 0"]
impl crate::Resettable for OR2rs {
    const RESET_VALUE: u32 = 0;
}
