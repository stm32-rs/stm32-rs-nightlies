#[doc = "Register `DDRPHYC_PTR0` reader"]
pub type R = crate::R<DDRPHYC_PTR0rs>;
#[doc = "Register `DDRPHYC_PTR0` writer"]
pub type W = crate::W<DDRPHYC_PTR0rs>;
#[doc = "Field `TDLLSRST` reader - TDLLSRST"]
pub type TDLLSRST_R = crate::FieldReader;
#[doc = "Field `TDLLSRST` writer - TDLLSRST"]
pub type TDLLSRST_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TDLLLOCK` reader - TDLLLOCK"]
pub type TDLLLOCK_R = crate::FieldReader<u16>;
#[doc = "Field `TDLLLOCK` writer - TDLLLOCK"]
pub type TDLLLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TITMSRST` reader - TITMSRST"]
pub type TITMSRST_R = crate::FieldReader;
#[doc = "Field `TITMSRST` writer - TITMSRST"]
pub type TITMSRST_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - TDLLSRST"]
    #[inline(always)]
    pub fn tdllsrst(&self) -> TDLLSRST_R {
        TDLLSRST_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:17 - TDLLLOCK"]
    #[inline(always)]
    pub fn tdlllock(&self) -> TDLLLOCK_R {
        TDLLLOCK_R::new(((self.bits >> 6) & 0x0fff) as u16)
    }
    #[doc = "Bits 18:21 - TITMSRST"]
    #[inline(always)]
    pub fn titmsrst(&self) -> TITMSRST_R {
        TITMSRST_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - TDLLSRST"]
    #[inline(always)]
    #[must_use]
    pub fn tdllsrst(&mut self) -> TDLLSRST_W<DDRPHYC_PTR0rs> {
        TDLLSRST_W::new(self, 0)
    }
    #[doc = "Bits 6:17 - TDLLLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn tdlllock(&mut self) -> TDLLLOCK_W<DDRPHYC_PTR0rs> {
        TDLLLOCK_W::new(self, 6)
    }
    #[doc = "Bits 18:21 - TITMSRST"]
    #[inline(always)]
    #[must_use]
    pub fn titmsrst(&mut self) -> TITMSRST_W<DDRPHYC_PTR0rs> {
        TITMSRST_W::new(self, 18)
    }
}
#[doc = "DDRPHYC PT register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_ptr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_ptr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_PTR0rs;
impl crate::RegisterSpec for DDRPHYC_PTR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_ptr0::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_PTR0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_ptr0::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_PTR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_PTR0 to value 0x0022_af9b"]
impl crate::Resettable for DDRPHYC_PTR0rs {
    const RESET_VALUE: u32 = 0x0022_af9b;
}
