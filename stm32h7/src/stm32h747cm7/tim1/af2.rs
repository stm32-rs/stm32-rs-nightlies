#[doc = "Register `AF2` reader"]
pub type R = crate::R<AF2rs>;
#[doc = "Register `AF2` writer"]
pub type W = crate::W<AF2rs>;
#[doc = "Field `BK2INE` reader - BRK2 BKIN input enable"]
pub type BK2INE_R = crate::BitReader;
#[doc = "Field `BK2INE` writer - BRK2 BKIN input enable"]
pub type BK2INE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable"]
pub type BK2CMP1E_R = crate::BitReader;
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable"]
pub type BK2CMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable"]
pub type BK2CMP2E_R = crate::BitReader;
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable"]
pub type BK2CMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2DF1BK1E` reader - BRK2 dfsdm1_break\\[1\\]
enable"]
pub type BK2DF1BK1E_R = crate::BitReader;
#[doc = "Field `BK2DF1BK1E` writer - BRK2 dfsdm1_break\\[1\\]
enable"]
pub type BK2DF1BK1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2INP` reader - BRK2 BKIN2 input polarity"]
pub type BK2INP_R = crate::BitReader;
#[doc = "Field `BK2INP` writer - BRK2 BKIN2 input polarity"]
pub type BK2INP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarit"]
pub type BK2CMP1P_R = crate::BitReader;
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarit"]
pub type BK2CMP1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity"]
pub type BK2CMP2P_R = crate::BitReader;
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity"]
pub type BK2CMP2P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - BRK2 dfsdm1_break\\[1\\]
enable"]
    #[inline(always)]
    pub fn bk2df1bk1e(&self) -> BK2DF1BK1E_R {
        BK2DF1BK1E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarit"]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2ine(&mut self) -> BK2INE_W<AF2rs> {
        BK2INE_W::new(self, 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W<AF2rs> {
        BK2CMP1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W<AF2rs> {
        BK2CMP2E_W::new(self, 2)
    }
    #[doc = "Bit 8 - BRK2 dfsdm1_break\\[1\\]
enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2df1bk1e(&mut self) -> BK2DF1BK1E_W<AF2rs> {
        BK2DF1BK1E_W::new(self, 8)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> BK2INP_W<AF2rs> {
        BK2INP_W::new(self, 9)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarit"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W<AF2rs> {
        BK2CMP1P_W::new(self, 10)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W<AF2rs> {
        BK2CMP2P_W::new(self, 11)
    }
}
#[doc = "TIM1 Alternate function odfsdm1_breakster 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF2rs;
impl crate::RegisterSpec for AF2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af2::R`](R) reader structure"]
impl crate::Readable for AF2rs {}
#[doc = "`write(|w| ..)` method takes [`af2::W`](W) writer structure"]
impl crate::Writable for AF2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF2 to value 0"]
impl crate::Resettable for AF2rs {
    const RESET_VALUE: u32 = 0;
}
