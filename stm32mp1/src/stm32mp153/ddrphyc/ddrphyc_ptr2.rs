#[doc = "Register `DDRPHYC_PTR2` reader"]
pub type R = crate::R<DDRPHYC_PTR2rs>;
#[doc = "Register `DDRPHYC_PTR2` writer"]
pub type W = crate::W<DDRPHYC_PTR2rs>;
#[doc = "Field `TDINIT2` reader - TDINIT2"]
pub type TDINIT2_R = crate::FieldReader<u32>;
#[doc = "Field `TDINIT2` writer - TDINIT2"]
pub type TDINIT2_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `TDINIT3` reader - TDINIT3"]
pub type TDINIT3_R = crate::FieldReader<u16>;
#[doc = "Field `TDINIT3` writer - TDINIT3"]
pub type TDINIT3_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:16 - TDINIT2"]
    #[inline(always)]
    pub fn tdinit2(&self) -> TDINIT2_R {
        TDINIT2_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:26 - TDINIT3"]
    #[inline(always)]
    pub fn tdinit3(&self) -> TDINIT3_R {
        TDINIT3_R::new(((self.bits >> 17) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - TDINIT2"]
    #[inline(always)]
    #[must_use]
    pub fn tdinit2(&mut self) -> TDINIT2_W<DDRPHYC_PTR2rs> {
        TDINIT2_W::new(self, 0)
    }
    #[doc = "Bits 17:26 - TDINIT3"]
    #[inline(always)]
    #[must_use]
    pub fn tdinit3(&mut self) -> TDINIT3_W<DDRPHYC_PTR2rs> {
        TDINIT3_W::new(self, 17)
    }
}
#[doc = "DDRPHYC PT register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_ptr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_ptr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_PTR2rs;
impl crate::RegisterSpec for DDRPHYC_PTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_ptr2::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_PTR2rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_ptr2::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_PTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_PTR2 to value 0x042d_a072"]
impl crate::Resettable for DDRPHYC_PTR2rs {
    const RESET_VALUE: u32 = 0x042d_a072;
}
