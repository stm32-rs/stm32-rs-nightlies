///Register `SECCFGR2` reader
pub type R = crate::R<SECCFGR2rs>;
///Register `SECCFGR2` writer
pub type W = crate::W<SECCFGR2rs>;
///Field `SEC32` reader - SEC32
pub type SEC32_R = crate::BitReader;
///Field `SEC32` writer - SEC32
pub type SEC32_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC33` reader - SEC33
pub type SEC33_R = crate::BitReader;
///Field `SEC33` writer - SEC33
pub type SEC33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC34` reader - SEC34
pub type SEC34_R = crate::BitReader;
///Field `SEC34` writer - SEC34
pub type SEC34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC35` reader - SEC35
pub type SEC35_R = crate::BitReader;
///Field `SEC35` writer - SEC35
pub type SEC35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC36` reader - SEC36
pub type SEC36_R = crate::BitReader;
///Field `SEC36` writer - SEC36
pub type SEC36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC37` reader - SEC37
pub type SEC37_R = crate::BitReader;
///Field `SEC37` writer - SEC37
pub type SEC37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC38` reader - SEC38
pub type SEC38_R = crate::BitReader;
///Field `SEC38` writer - SEC38
pub type SEC38_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC39` reader - SEC39
pub type SEC39_R = crate::BitReader;
///Field `SEC39` writer - SEC39
pub type SEC39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC40` reader - SEC40
pub type SEC40_R = crate::BitReader;
///Field `SEC40` writer - SEC40
pub type SEC40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC41` reader - SEC41
pub type SEC41_R = crate::BitReader;
///Field `SEC41` writer - SEC41
pub type SEC41_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC42` reader - SEC42
pub type SEC42_R = crate::BitReader;
///Field `SEC42` writer - SEC42
pub type SEC42_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SEC32
    #[inline(always)]
    pub fn sec32(&self) -> SEC32_R {
        SEC32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SEC33
    #[inline(always)]
    pub fn sec33(&self) -> SEC33_R {
        SEC33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SEC34
    #[inline(always)]
    pub fn sec34(&self) -> SEC34_R {
        SEC34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SEC35
    #[inline(always)]
    pub fn sec35(&self) -> SEC35_R {
        SEC35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SEC36
    #[inline(always)]
    pub fn sec36(&self) -> SEC36_R {
        SEC36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SEC37
    #[inline(always)]
    pub fn sec37(&self) -> SEC37_R {
        SEC37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SEC38
    #[inline(always)]
    pub fn sec38(&self) -> SEC38_R {
        SEC38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SEC39
    #[inline(always)]
    pub fn sec39(&self) -> SEC39_R {
        SEC39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SEC40
    #[inline(always)]
    pub fn sec40(&self) -> SEC40_R {
        SEC40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SEC41
    #[inline(always)]
    pub fn sec41(&self) -> SEC41_R {
        SEC41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SEC42
    #[inline(always)]
    pub fn sec42(&self) -> SEC42_R {
        SEC42_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR2")
            .field("sec32", &self.sec32())
            .field("sec33", &self.sec33())
            .field("sec34", &self.sec34())
            .field("sec35", &self.sec35())
            .field("sec36", &self.sec36())
            .field("sec37", &self.sec37())
            .field("sec38", &self.sec38())
            .field("sec39", &self.sec39())
            .field("sec40", &self.sec40())
            .field("sec41", &self.sec41())
            .field("sec42", &self.sec42())
            .finish()
    }
}
impl W {
    ///Bit 0 - SEC32
    #[inline(always)]
    pub fn sec32(&mut self) -> SEC32_W<'_, SECCFGR2rs> {
        SEC32_W::new(self, 0)
    }
    ///Bit 1 - SEC33
    #[inline(always)]
    pub fn sec33(&mut self) -> SEC33_W<'_, SECCFGR2rs> {
        SEC33_W::new(self, 1)
    }
    ///Bit 2 - SEC34
    #[inline(always)]
    pub fn sec34(&mut self) -> SEC34_W<'_, SECCFGR2rs> {
        SEC34_W::new(self, 2)
    }
    ///Bit 3 - SEC35
    #[inline(always)]
    pub fn sec35(&mut self) -> SEC35_W<'_, SECCFGR2rs> {
        SEC35_W::new(self, 3)
    }
    ///Bit 4 - SEC36
    #[inline(always)]
    pub fn sec36(&mut self) -> SEC36_W<'_, SECCFGR2rs> {
        SEC36_W::new(self, 4)
    }
    ///Bit 5 - SEC37
    #[inline(always)]
    pub fn sec37(&mut self) -> SEC37_W<'_, SECCFGR2rs> {
        SEC37_W::new(self, 5)
    }
    ///Bit 6 - SEC38
    #[inline(always)]
    pub fn sec38(&mut self) -> SEC38_W<'_, SECCFGR2rs> {
        SEC38_W::new(self, 6)
    }
    ///Bit 7 - SEC39
    #[inline(always)]
    pub fn sec39(&mut self) -> SEC39_W<'_, SECCFGR2rs> {
        SEC39_W::new(self, 7)
    }
    ///Bit 8 - SEC40
    #[inline(always)]
    pub fn sec40(&mut self) -> SEC40_W<'_, SECCFGR2rs> {
        SEC40_W::new(self, 8)
    }
    ///Bit 9 - SEC41
    #[inline(always)]
    pub fn sec41(&mut self) -> SEC41_W<'_, SECCFGR2rs> {
        SEC41_W::new(self, 9)
    }
    ///Bit 10 - SEC42
    #[inline(always)]
    pub fn sec42(&mut self) -> SEC42_W<'_, SECCFGR2rs> {
        SEC42_W::new(self, 10)
    }
}
/**EXTI security enable register

You can [`read`](crate::Reg::read) this register and get [`seccfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#EXTI:SECCFGR2)*/
pub struct SECCFGR2rs;
impl crate::RegisterSpec for SECCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr2::R`](R) reader structure
impl crate::Readable for SECCFGR2rs {}
///`write(|w| ..)` method takes [`seccfgr2::W`](W) writer structure
impl crate::Writable for SECCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR2 to value 0
impl crate::Resettable for SECCFGR2rs {}
