#[doc = "Register `RPR2` reader"]
pub type R = crate::R<RPR2rs>;
#[doc = "Register `RPR2` writer"]
pub type W = crate::W<RPR2rs>;
#[doc = "Field `RPIF35` reader - RPIF35"]
pub type RPIF35_R = crate::BitReader;
#[doc = "Field `RPIF35` writer - RPIF35"]
pub type RPIF35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF36` reader - RPIF36"]
pub type RPIF36_R = crate::BitReader;
#[doc = "Field `RPIF36` writer - RPIF36"]
pub type RPIF36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF37` reader - RPIF37"]
pub type RPIF37_R = crate::BitReader;
#[doc = "Field `RPIF37` writer - RPIF37"]
pub type RPIF37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF38` reader - RPIF38"]
pub type RPIF38_R = crate::BitReader;
#[doc = "Field `RPIF38` writer - RPIF38"]
pub type RPIF38_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - RPIF35"]
    #[inline(always)]
    pub fn rpif35(&self) -> RPIF35_R {
        RPIF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RPIF36"]
    #[inline(always)]
    pub fn rpif36(&self) -> RPIF36_R {
        RPIF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RPIF37"]
    #[inline(always)]
    pub fn rpif37(&self) -> RPIF37_R {
        RPIF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RPIF38"]
    #[inline(always)]
    pub fn rpif38(&self) -> RPIF38_R {
        RPIF38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - RPIF35"]
    #[inline(always)]
    #[must_use]
    pub fn rpif35(&mut self) -> RPIF35_W<RPR2rs> {
        RPIF35_W::new(self, 3)
    }
    #[doc = "Bit 4 - RPIF36"]
    #[inline(always)]
    #[must_use]
    pub fn rpif36(&mut self) -> RPIF36_W<RPR2rs> {
        RPIF36_W::new(self, 4)
    }
    #[doc = "Bit 5 - RPIF37"]
    #[inline(always)]
    #[must_use]
    pub fn rpif37(&mut self) -> RPIF37_W<RPR2rs> {
        RPIF37_W::new(self, 5)
    }
    #[doc = "Bit 6 - RPIF38"]
    #[inline(always)]
    #[must_use]
    pub fn rpif38(&mut self) -> RPIF38_W<RPR2rs> {
        RPIF38_W::new(self, 6)
    }
}
#[doc = "EXTI rising edge pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPR2rs;
impl crate::RegisterSpec for RPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr2::R`](R) reader structure"]
impl crate::Readable for RPR2rs {}
#[doc = "`write(|w| ..)` method takes [`rpr2::W`](W) writer structure"]
impl crate::Writable for RPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RPR2 to value 0"]
impl crate::Resettable for RPR2rs {
    const RESET_VALUE: u32 = 0;
}
