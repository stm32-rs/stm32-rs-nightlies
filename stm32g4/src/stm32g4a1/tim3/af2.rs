#[doc = "Register `AF2` reader"]
pub type R = crate::R<AF2rs>;
#[doc = "Register `AF2` writer"]
pub type W = crate::W<AF2rs>;
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub type BKINE_R = crate::BitReader;
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub type BKINE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable"]
pub type BK2CMP1E_R = crate::BitReader;
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable"]
pub type BK2CMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable"]
pub type BK2CMP2E_R = crate::BitReader;
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable"]
pub type BK2CMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP3E` reader - BRK2 COMP3 enable"]
pub type BK2CMP3E_R = crate::BitReader;
#[doc = "Field `BK2CMP3E` writer - BRK2 COMP3 enable"]
pub type BK2CMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP4E` reader - BRK2 COMP4 enable"]
pub type BK2CMP4E_R = crate::BitReader;
#[doc = "Field `BK2CMP4E` writer - BRK2 COMP4 enable"]
pub type BK2CMP4E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP5E` reader - BRK2 COMP5 enable"]
pub type BK2CMP5E_R = crate::BitReader;
#[doc = "Field `BK2CMP5E` writer - BRK2 COMP5 enable"]
pub type BK2CMP5E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP6E` reader - BRK2 COMP6 enable"]
pub type BK2CMP6E_R = crate::BitReader;
#[doc = "Field `BK2CMP6E` writer - BRK2 COMP6 enable"]
pub type BK2CMP6E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP7E` reader - BRK2 COMP7 enable"]
pub type BK2CMP7E_R = crate::BitReader;
#[doc = "Field `BK2CMP7E` writer - BRK2 COMP7 enable"]
pub type BK2CMP7E_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `BK2CMP3P` reader - BRK2 COMP3 input polarity"]
pub type BK2CMP3P_R = crate::BitReader;
#[doc = "Field `BK2CMP3P` writer - BRK2 COMP3 input polarity"]
pub type BK2CMP3P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP4P` reader - BRK2 COMP4 input polarity"]
pub type BK2CMP4P_R = crate::BitReader;
#[doc = "Field `BK2CMP4P` writer - BRK2 COMP4 input polarity"]
pub type BK2CMP4P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCRSEL` reader - OCREF_CLR source selection"]
pub type OCRSEL_R = crate::FieldReader;
#[doc = "Field `OCRSEL` writer - OCREF_CLR source selection"]
pub type OCRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 3 - BRK2 COMP3 enable"]
    #[inline(always)]
    pub fn bk2cmp3e(&self) -> BK2CMP3E_R {
        BK2CMP3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BRK2 COMP4 enable"]
    #[inline(always)]
    pub fn bk2cmp4e(&self) -> BK2CMP4E_R {
        BK2CMP4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BRK2 COMP5 enable"]
    #[inline(always)]
    pub fn bk2cmp5e(&self) -> BK2CMP5E_R {
        BK2CMP5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BRK2 COMP6 enable"]
    #[inline(always)]
    pub fn bk2cmp6e(&self) -> BK2CMP6E_R {
        BK2CMP6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BRK2 COMP7 enable"]
    #[inline(always)]
    pub fn bk2cmp7e(&self) -> BK2CMP7E_R {
        BK2CMP7E_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 12 - BRK2 COMP3 input polarity"]
    #[inline(always)]
    pub fn bk2cmp3p(&self) -> BK2CMP3P_R {
        BK2CMP3P_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BRK2 COMP4 input polarity"]
    #[inline(always)]
    pub fn bk2cmp4p(&self) -> BK2CMP4P_R {
        BK2CMP4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18 - OCREF_CLR source selection"]
    #[inline(always)]
    pub fn ocrsel(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<AF2rs> {
        BKINE_W::new(self, 0)
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
    #[doc = "Bit 3 - BRK2 COMP3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp3e(&mut self) -> BK2CMP3E_W<AF2rs> {
        BK2CMP3E_W::new(self, 3)
    }
    #[doc = "Bit 4 - BRK2 COMP4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp4e(&mut self) -> BK2CMP4E_W<AF2rs> {
        BK2CMP4E_W::new(self, 4)
    }
    #[doc = "Bit 5 - BRK2 COMP5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp5e(&mut self) -> BK2CMP5E_W<AF2rs> {
        BK2CMP5E_W::new(self, 5)
    }
    #[doc = "Bit 6 - BRK2 COMP6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp6e(&mut self) -> BK2CMP6E_W<AF2rs> {
        BK2CMP6E_W::new(self, 6)
    }
    #[doc = "Bit 7 - BRK2 COMP7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp7e(&mut self) -> BK2CMP7E_W<AF2rs> {
        BK2CMP7E_W::new(self, 7)
    }
    #[doc = "Bit 9 - BRK2 BKIN input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> BK2INP_W<AF2rs> {
        BK2INP_W::new(self, 9)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity"]
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
    #[doc = "Bit 12 - BRK2 COMP3 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp3p(&mut self) -> BK2CMP3P_W<AF2rs> {
        BK2CMP3P_W::new(self, 12)
    }
    #[doc = "Bit 13 - BRK2 COMP4 input polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp4p(&mut self) -> BK2CMP4P_W<AF2rs> {
        BK2CMP4P_W::new(self, 13)
    }
    #[doc = "Bits 16:18 - OCREF_CLR source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ocrsel(&mut self) -> OCRSEL_W<AF2rs> {
        OCRSEL_W::new(self, 16)
    }
}
#[doc = "TIM alternate function option register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
