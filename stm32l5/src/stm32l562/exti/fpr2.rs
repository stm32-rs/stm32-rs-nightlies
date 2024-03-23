#[doc = "Register `FPR2` reader"]
pub type R = crate::R<FPR2rs>;
#[doc = "Register `FPR2` writer"]
pub type W = crate::W<FPR2rs>;
#[doc = "Field `FPIF35` reader - FPIF35"]
pub type FPIF35_R = crate::BitReader;
#[doc = "Field `FPIF35` writer - FPIF35"]
pub type FPIF35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF36` reader - FPIF36"]
pub type FPIF36_R = crate::BitReader;
#[doc = "Field `FPIF36` writer - FPIF36"]
pub type FPIF36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF37` reader - FPIF37"]
pub type FPIF37_R = crate::BitReader;
#[doc = "Field `FPIF37` writer - FPIF37"]
pub type FPIF37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF38` reader - FPIF38"]
pub type FPIF38_R = crate::BitReader;
#[doc = "Field `FPIF38` writer - FPIF38"]
pub type FPIF38_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - FPIF35"]
    #[inline(always)]
    pub fn fpif35(&self) -> FPIF35_R {
        FPIF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FPIF36"]
    #[inline(always)]
    pub fn fpif36(&self) -> FPIF36_R {
        FPIF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FPIF37"]
    #[inline(always)]
    pub fn fpif37(&self) -> FPIF37_R {
        FPIF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FPIF38"]
    #[inline(always)]
    pub fn fpif38(&self) -> FPIF38_R {
        FPIF38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FPIF35"]
    #[inline(always)]
    #[must_use]
    pub fn fpif35(&mut self) -> FPIF35_W<FPR2rs> {
        FPIF35_W::new(self, 3)
    }
    #[doc = "Bit 4 - FPIF36"]
    #[inline(always)]
    #[must_use]
    pub fn fpif36(&mut self) -> FPIF36_W<FPR2rs> {
        FPIF36_W::new(self, 4)
    }
    #[doc = "Bit 5 - FPIF37"]
    #[inline(always)]
    #[must_use]
    pub fn fpif37(&mut self) -> FPIF37_W<FPR2rs> {
        FPIF37_W::new(self, 5)
    }
    #[doc = "Bit 6 - FPIF38"]
    #[inline(always)]
    #[must_use]
    pub fn fpif38(&mut self) -> FPIF38_W<FPR2rs> {
        FPIF38_W::new(self, 6)
    }
}
#[doc = "EXTI falling edge pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPR2rs;
impl crate::RegisterSpec for FPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpr2::R`](R) reader structure"]
impl crate::Readable for FPR2rs {}
#[doc = "`write(|w| ..)` method takes [`fpr2::W`](W) writer structure"]
impl crate::Writable for FPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPR2 to value 0"]
impl crate::Resettable for FPR2rs {
    const RESET_VALUE: u32 = 0;
}
