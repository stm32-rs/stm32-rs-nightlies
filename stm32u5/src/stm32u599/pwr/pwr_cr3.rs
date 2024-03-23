#[doc = "Register `PWR_CR3` reader"]
pub type R = crate::R<PWR_CR3rs>;
#[doc = "Register `PWR_CR3` writer"]
pub type W = crate::W<PWR_CR3rs>;
#[doc = "Field `REGSEL` reader - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS."]
pub type REGSEL_R = crate::BitReader;
#[doc = "Field `REGSEL` writer - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS."]
pub type REGSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTEN` reader - Fast soft start"]
pub type FSTEN_R = crate::BitReader;
#[doc = "Field `FSTEN` writer - Fast soft start"]
pub type FSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS."]
    #[inline(always)]
    pub fn regsel(&self) -> REGSEL_R {
        REGSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fast soft start"]
    #[inline(always)]
    pub fn fsten(&self) -> FSTEN_R {
        FSTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Regulator selection Note: REGSEL is reserved and must be kept at reset value in packages without SMPS."]
    #[inline(always)]
    #[must_use]
    pub fn regsel(&mut self) -> REGSEL_W<PWR_CR3rs> {
        REGSEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fast soft start"]
    #[inline(always)]
    #[must_use]
    pub fn fsten(&mut self) -> FSTEN_W<PWR_CR3rs> {
        FSTEN_W::new(self, 2)
    }
}
#[doc = "PWR control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_CR3rs;
impl crate::RegisterSpec for PWR_CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_cr3::R`](R) reader structure"]
impl crate::Readable for PWR_CR3rs {}
#[doc = "`write(|w| ..)` method takes [`pwr_cr3::W`](W) writer structure"]
impl crate::Writable for PWR_CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_CR3 to value 0"]
impl crate::Resettable for PWR_CR3rs {
    const RESET_VALUE: u32 = 0;
}
