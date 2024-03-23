#[doc = "Register `TIM1_AF2` reader"]
pub type R = crate::R<TIM1_AF2rs>;
#[doc = "Register `TIM1_AF2` writer"]
pub type W = crate::W<TIM1_AF2rs>;
#[doc = "Field `BK2INE` reader - BK2INE"]
pub type BK2INE_R = crate::BitReader;
#[doc = "Field `BK2INE` writer - BK2INE"]
pub type BK2INE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2DF1BK1E` reader - BK2DF1BK1E"]
pub type BK2DF1BK1E_R = crate::BitReader;
#[doc = "Field `BK2DF1BK1E` writer - BK2DF1BK1E"]
pub type BK2DF1BK1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2INP` reader - BK2INP"]
pub type BK2INP_R = crate::BitReader;
#[doc = "Field `BK2INP` writer - BK2INP"]
pub type BK2INP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BK2INE"]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - BK2DF1BK1E"]
    #[inline(always)]
    pub fn bk2df1bk1e(&self) -> BK2DF1BK1E_R {
        BK2DF1BK1E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BK2INP"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BK2INE"]
    #[inline(always)]
    #[must_use]
    pub fn bk2ine(&mut self) -> BK2INE_W<TIM1_AF2rs> {
        BK2INE_W::new(self, 0)
    }
    #[doc = "Bit 8 - BK2DF1BK1E"]
    #[inline(always)]
    #[must_use]
    pub fn bk2df1bk1e(&mut self) -> BK2DF1BK1E_W<TIM1_AF2rs> {
        BK2DF1BK1E_W::new(self, 8)
    }
    #[doc = "Bit 9 - BK2INP"]
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> BK2INP_W<TIM1_AF2rs> {
        BK2INP_W::new(self, 9)
    }
}
#[doc = "TIM1 Alternate function register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_af2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_af2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_AF2rs;
impl crate::RegisterSpec for TIM1_AF2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_af2::R`](R) reader structure"]
impl crate::Readable for TIM1_AF2rs {}
#[doc = "`write(|w| ..)` method takes [`tim1_af2::W`](W) writer structure"]
impl crate::Writable for TIM1_AF2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_AF2 to value 0x01"]
impl crate::Resettable for TIM1_AF2rs {
    const RESET_VALUE: u32 = 0x01;
}
