#[doc = "Register `OR3` reader"]
pub type R = crate::R<OR3rs>;
#[doc = "Register `OR3` writer"]
pub type W = crate::W<OR3rs>;
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
#[doc = "Field `BK2DFBK0E` reader - BRK2 DFSDM_BREAK0 enable"]
pub type BK2DFBK0E_R = crate::BitReader;
#[doc = "Field `BK2DFBK0E` writer - BRK2 DFSDM_BREAK0 enable"]
pub type BK2DFBK0E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2INP` reader - BRK2 BKIN input polarity"]
pub type BK2INP_R = crate::BitReader;
#[doc = "Field `BK2INP` writer - BRK2 BKIN input polarity"]
pub type BK2INP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarity"]
pub type BK2CMP1P_R = crate::BitReader;
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarity"]
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
    #[doc = "Bit 8 - BRK2 DFSDM_BREAK0 enable"]
    #[inline(always)]
    pub fn bk2dfbk0e(&self) -> BK2DFBK0E_R {
        BK2DFBK0E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
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
    pub fn bk2ine(&mut self) -> BK2INE_W<OR3rs> {
        BK2INE_W::new(self, 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W<OR3rs> {
        BK2CMP1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W<OR3rs> {
        BK2CMP2E_W::new(self, 2)
    }
    #[doc = "Bit 8 - BRK2 DFSDM_BREAK0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2dfbk0e(&mut self) -> BK2DFBK0E_W<OR3rs> {
        BK2DFBK0E_W::new(self, 8)
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> BK2INP_W<OR3rs> {
        BK2INP_W::new(self, 9)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W<OR3rs> {
        BK2CMP1P_W::new(self, 10)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W<OR3rs> {
        BK2CMP2P_W::new(self, 11)
    }
}
#[doc = "DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR3rs;
impl crate::RegisterSpec for OR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or3::R`](R) reader structure"]
impl crate::Readable for OR3rs {}
#[doc = "`write(|w| ..)` method takes [`or3::W`](W) writer structure"]
impl crate::Writable for OR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR3 to value 0x01"]
impl crate::Resettable for OR3rs {
    const RESET_VALUE: u32 = 0x01;
}
