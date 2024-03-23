#[doc = "Register `FTSR2` reader"]
pub type R = crate::R<FTSR2rs>;
#[doc = "Register `FTSR2` writer"]
pub type W = crate::W<FTSR2rs>;
#[doc = "Field `FT35` reader - FT35"]
pub type FT35_R = crate::BitReader;
#[doc = "Field `FT35` writer - FT35"]
pub type FT35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT36` reader - FT36"]
pub type FT36_R = crate::BitReader;
#[doc = "Field `FT36` writer - FT36"]
pub type FT36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT37` reader - FT37"]
pub type FT37_R = crate::BitReader;
#[doc = "Field `FT37` writer - FT37"]
pub type FT37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT38` reader - FT38"]
pub type FT38_R = crate::BitReader;
#[doc = "Field `FT38` writer - FT38"]
pub type FT38_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - FT35"]
    #[inline(always)]
    pub fn ft35(&self) -> FT35_R {
        FT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FT36"]
    #[inline(always)]
    pub fn ft36(&self) -> FT36_R {
        FT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FT37"]
    #[inline(always)]
    pub fn ft37(&self) -> FT37_R {
        FT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FT38"]
    #[inline(always)]
    pub fn ft38(&self) -> FT38_R {
        FT38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FT35"]
    #[inline(always)]
    #[must_use]
    pub fn ft35(&mut self) -> FT35_W<FTSR2rs> {
        FT35_W::new(self, 3)
    }
    #[doc = "Bit 4 - FT36"]
    #[inline(always)]
    #[must_use]
    pub fn ft36(&mut self) -> FT36_W<FTSR2rs> {
        FT36_W::new(self, 4)
    }
    #[doc = "Bit 5 - FT37"]
    #[inline(always)]
    #[must_use]
    pub fn ft37(&mut self) -> FT37_W<FTSR2rs> {
        FT37_W::new(self, 5)
    }
    #[doc = "Bit 6 - FT38"]
    #[inline(always)]
    #[must_use]
    pub fn ft38(&mut self) -> FT38_W<FTSR2rs> {
        FT38_W::new(self, 6)
    }
}
#[doc = "EXTI falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTSR2rs;
impl crate::RegisterSpec for FTSR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr2::R`](R) reader structure"]
impl crate::Readable for FTSR2rs {}
#[doc = "`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure"]
impl crate::Writable for FTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for FTSR2rs {
    const RESET_VALUE: u32 = 0;
}
