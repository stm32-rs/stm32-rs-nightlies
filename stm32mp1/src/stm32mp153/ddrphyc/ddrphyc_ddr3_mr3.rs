#[doc = "Register `DDRPHYC_DDR3_MR3` reader"]
pub type R = crate::R<DDRPHYC_DDR3_MR3rs>;
#[doc = "Register `DDRPHYC_DDR3_MR3` writer"]
pub type W = crate::W<DDRPHYC_DDR3_MR3rs>;
#[doc = "Field `MPRLOC` reader - MPRLOC"]
pub type MPRLOC_R = crate::FieldReader;
#[doc = "Field `MPRLOC` writer - MPRLOC"]
pub type MPRLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MPR` reader - MPR"]
pub type MPR_R = crate::BitReader;
#[doc = "Field `MPR` writer - MPR"]
pub type MPR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - MPRLOC"]
    #[inline(always)]
    pub fn mprloc(&self) -> MPRLOC_R {
        MPRLOC_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - MPR"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MPRLOC"]
    #[inline(always)]
    #[must_use]
    pub fn mprloc(&mut self) -> MPRLOC_W<DDRPHYC_DDR3_MR3rs> {
        MPRLOC_W::new(self, 0)
    }
    #[doc = "Bit 2 - MPR"]
    #[inline(always)]
    #[must_use]
    pub fn mpr(&mut self) -> MPR_W<DDRPHYC_DDR3_MR3rs> {
        MPR_W::new(self, 2)
    }
}
#[doc = "DDRPHYC MR3 register for DDR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_ddr3_mr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_ddr3_mr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DDR3_MR3rs;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR3rs {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ddrphyc_ddr3_mr3::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR3rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_ddr3_mr3::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR3 to value 0"]
impl crate::Resettable for DDRPHYC_DDR3_MR3rs {
    const RESET_VALUE: u8 = 0;
}
