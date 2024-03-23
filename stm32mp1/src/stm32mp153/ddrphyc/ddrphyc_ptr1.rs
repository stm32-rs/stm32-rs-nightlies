#[doc = "Register `DDRPHYC_PTR1` reader"]
pub type R = crate::R<DDRPHYC_PTR1rs>;
#[doc = "Register `DDRPHYC_PTR1` writer"]
pub type W = crate::W<DDRPHYC_PTR1rs>;
#[doc = "Field `TDINIT0` reader - TDINIT0"]
pub type TDINIT0_R = crate::FieldReader<u32>;
#[doc = "Field `TDINIT0` writer - TDINIT0"]
pub type TDINIT0_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `TDINIT1` reader - TDINIT1"]
pub type TDINIT1_R = crate::FieldReader;
#[doc = "Field `TDINIT1` writer - TDINIT1"]
pub type TDINIT1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:18 - TDINIT0"]
    #[inline(always)]
    pub fn tdinit0(&self) -> TDINIT0_R {
        TDINIT0_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:26 - TDINIT1"]
    #[inline(always)]
    pub fn tdinit1(&self) -> TDINIT1_R {
        TDINIT1_R::new(((self.bits >> 19) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - TDINIT0"]
    #[inline(always)]
    #[must_use]
    pub fn tdinit0(&mut self) -> TDINIT0_W<DDRPHYC_PTR1rs> {
        TDINIT0_W::new(self, 0)
    }
    #[doc = "Bits 19:26 - TDINIT1"]
    #[inline(always)]
    #[must_use]
    pub fn tdinit1(&mut self) -> TDINIT1_W<DDRPHYC_PTR1rs> {
        TDINIT1_W::new(self, 19)
    }
}
#[doc = "DDRPHYC PT register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_ptr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_ptr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_PTR1rs;
impl crate::RegisterSpec for DDRPHYC_PTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_ptr1::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_PTR1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_ptr1::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_PTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_PTR1 to value 0x0604_111d"]
impl crate::Resettable for DDRPHYC_PTR1rs {
    const RESET_VALUE: u32 = 0x0604_111d;
}
