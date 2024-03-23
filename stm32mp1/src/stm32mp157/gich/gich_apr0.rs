#[doc = "Register `GICH_APR0` reader"]
pub type R = crate::R<GICH_APR0rs>;
#[doc = "Register `GICH_APR0` writer"]
pub type W = crate::W<GICH_APR0rs>;
#[doc = "Field `APR0` reader - APR0"]
pub type APR0_R = crate::FieldReader<u32>;
#[doc = "Field `APR0` writer - APR0"]
pub type APR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - APR0"]
    #[inline(always)]
    pub fn apr0(&self) -> APR0_R {
        APR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - APR0"]
    #[inline(always)]
    #[must_use]
    pub fn apr0(&mut self) -> APR0_W<GICH_APR0rs> {
        APR0_W::new(self, 0)
    }
}
#[doc = "GICH active priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_apr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gich_apr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICH_APR0rs;
impl crate::RegisterSpec for GICH_APR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gich_apr0::R`](R) reader structure"]
impl crate::Readable for GICH_APR0rs {}
#[doc = "`write(|w| ..)` method takes [`gich_apr0::W`](W) writer structure"]
impl crate::Writable for GICH_APR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICH_APR0 to value 0"]
impl crate::Resettable for GICH_APR0rs {
    const RESET_VALUE: u32 = 0;
}
