#[doc = "Register `MDIER4` reader"]
pub type R = crate::R<MDIER4rs>;
#[doc = "Register `MDIER4` writer"]
pub type W = crate::W<MDIER4rs>;
#[doc = "Field `MCMP1IE` reader - MCMP1IE"]
pub type MCMP1IE_R = crate::BitReader;
#[doc = "Field `MCMP1IE` writer - MCMP1IE"]
pub type MCMP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP2IE` reader - MCMP2IE"]
pub type MCMP2IE_R = crate::BitReader;
#[doc = "Field `MCMP2IE` writer - MCMP2IE"]
pub type MCMP2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP3IE` reader - MCMP3IE"]
pub type MCMP3IE_R = crate::BitReader;
#[doc = "Field `MCMP3IE` writer - MCMP3IE"]
pub type MCMP3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP4IE` reader - MCMP4IE"]
pub type MCMP4IE_R = crate::BitReader;
#[doc = "Field `MCMP4IE` writer - MCMP4IE"]
pub type MCMP4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MREPIE` reader - MREPIE"]
pub type MREPIE_R = crate::BitReader;
#[doc = "Field `MREPIE` writer - MREPIE"]
pub type MREPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCIE` reader - SYNCIE"]
pub type SYNCIE_R = crate::BitReader;
#[doc = "Field `SYNCIE` writer - SYNCIE"]
pub type SYNCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUPDIE` reader - MUPDIE"]
pub type MUPDIE_R = crate::BitReader;
#[doc = "Field `MUPDIE` writer - MUPDIE"]
pub type MUPDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP1DE` reader - MCMP1DE"]
pub type MCMP1DE_R = crate::BitReader;
#[doc = "Field `MCMP1DE` writer - MCMP1DE"]
pub type MCMP1DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP2DE` reader - MCMP2DE"]
pub type MCMP2DE_R = crate::BitReader;
#[doc = "Field `MCMP2DE` writer - MCMP2DE"]
pub type MCMP2DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP3DE` reader - MCMP3DE"]
pub type MCMP3DE_R = crate::BitReader;
#[doc = "Field `MCMP3DE` writer - MCMP3DE"]
pub type MCMP3DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP4DE` reader - MCMP4DE"]
pub type MCMP4DE_R = crate::BitReader;
#[doc = "Field `MCMP4DE` writer - MCMP4DE"]
pub type MCMP4DE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MREPDE` reader - MREPDE"]
pub type MREPDE_R = crate::BitReader;
#[doc = "Field `MREPDE` writer - MREPDE"]
pub type MREPDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDE` reader - SYNCDE"]
pub type SYNCDE_R = crate::BitReader;
#[doc = "Field `SYNCDE` writer - SYNCDE"]
pub type SYNCDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUPDDE` reader - MUPDDE"]
pub type MUPDDE_R = crate::BitReader;
#[doc = "Field `MUPDDE` writer - MUPDDE"]
pub type MUPDDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCMP1IE"]
    #[inline(always)]
    pub fn mcmp1ie(&self) -> MCMP1IE_R {
        MCMP1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCMP2IE"]
    #[inline(always)]
    pub fn mcmp2ie(&self) -> MCMP2IE_R {
        MCMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCMP3IE"]
    #[inline(always)]
    pub fn mcmp3ie(&self) -> MCMP3IE_R {
        MCMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MCMP4IE"]
    #[inline(always)]
    pub fn mcmp4ie(&self) -> MCMP4IE_R {
        MCMP4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MREPIE"]
    #[inline(always)]
    pub fn mrepie(&self) -> MREPIE_R {
        MREPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYNCIE"]
    #[inline(always)]
    pub fn syncie(&self) -> SYNCIE_R {
        SYNCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MUPDIE"]
    #[inline(always)]
    pub fn mupdie(&self) -> MUPDIE_R {
        MUPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - MCMP1DE"]
    #[inline(always)]
    pub fn mcmp1de(&self) -> MCMP1DE_R {
        MCMP1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MCMP2DE"]
    #[inline(always)]
    pub fn mcmp2de(&self) -> MCMP2DE_R {
        MCMP2DE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MCMP3DE"]
    #[inline(always)]
    pub fn mcmp3de(&self) -> MCMP3DE_R {
        MCMP3DE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MCMP4DE"]
    #[inline(always)]
    pub fn mcmp4de(&self) -> MCMP4DE_R {
        MCMP4DE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MREPDE"]
    #[inline(always)]
    pub fn mrepde(&self) -> MREPDE_R {
        MREPDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SYNCDE"]
    #[inline(always)]
    pub fn syncde(&self) -> SYNCDE_R {
        SYNCDE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MUPDDE"]
    #[inline(always)]
    pub fn mupdde(&self) -> MUPDDE_R {
        MUPDDE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCMP1IE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp1ie(&mut self) -> MCMP1IE_W<MDIER4rs> {
        MCMP1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - MCMP2IE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp2ie(&mut self) -> MCMP2IE_W<MDIER4rs> {
        MCMP2IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - MCMP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp3ie(&mut self) -> MCMP3IE_W<MDIER4rs> {
        MCMP3IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - MCMP4IE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp4ie(&mut self) -> MCMP4IE_W<MDIER4rs> {
        MCMP4IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - MREPIE"]
    #[inline(always)]
    #[must_use]
    pub fn mrepie(&mut self) -> MREPIE_W<MDIER4rs> {
        MREPIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - SYNCIE"]
    #[inline(always)]
    #[must_use]
    pub fn syncie(&mut self) -> SYNCIE_W<MDIER4rs> {
        SYNCIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - MUPDIE"]
    #[inline(always)]
    #[must_use]
    pub fn mupdie(&mut self) -> MUPDIE_W<MDIER4rs> {
        MUPDIE_W::new(self, 6)
    }
    #[doc = "Bit 16 - MCMP1DE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp1de(&mut self) -> MCMP1DE_W<MDIER4rs> {
        MCMP1DE_W::new(self, 16)
    }
    #[doc = "Bit 17 - MCMP2DE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp2de(&mut self) -> MCMP2DE_W<MDIER4rs> {
        MCMP2DE_W::new(self, 17)
    }
    #[doc = "Bit 18 - MCMP3DE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp3de(&mut self) -> MCMP3DE_W<MDIER4rs> {
        MCMP3DE_W::new(self, 18)
    }
    #[doc = "Bit 19 - MCMP4DE"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp4de(&mut self) -> MCMP4DE_W<MDIER4rs> {
        MCMP4DE_W::new(self, 19)
    }
    #[doc = "Bit 20 - MREPDE"]
    #[inline(always)]
    #[must_use]
    pub fn mrepde(&mut self) -> MREPDE_W<MDIER4rs> {
        MREPDE_W::new(self, 20)
    }
    #[doc = "Bit 21 - SYNCDE"]
    #[inline(always)]
    #[must_use]
    pub fn syncde(&mut self) -> SYNCDE_W<MDIER4rs> {
        SYNCDE_W::new(self, 21)
    }
    #[doc = "Bit 22 - MUPDDE"]
    #[inline(always)]
    #[must_use]
    pub fn mupdde(&mut self) -> MUPDDE_W<MDIER4rs> {
        MUPDDE_W::new(self, 22)
    }
}
#[doc = "MDIER4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdier4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdier4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIER4rs;
impl crate::RegisterSpec for MDIER4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdier4::R`](R) reader structure"]
impl crate::Readable for MDIER4rs {}
#[doc = "`write(|w| ..)` method takes [`mdier4::W`](W) writer structure"]
impl crate::Writable for MDIER4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIER4 to value 0"]
impl crate::Resettable for MDIER4rs {
    const RESET_VALUE: u32 = 0;
}
