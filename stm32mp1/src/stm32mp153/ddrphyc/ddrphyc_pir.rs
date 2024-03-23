#[doc = "Register `DDRPHYC_PIR` writer"]
pub type W = crate::W<DDRPHYC_PIRrs>;
#[doc = "Field `INIT` writer - INIT"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLSRST` writer - DLLSRST"]
pub type DLLSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLLOCK` writer - DLLLOCK"]
pub type DLLLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZCAL` writer - ZCAL"]
pub type ZCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITMSRST` writer - ITMSRST"]
pub type ITMSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRAMRST` writer - DRAMRST"]
pub type DRAMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRAMINIT` writer - DRAMINIT"]
pub type DRAMINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSTRN` writer - QSTRN"]
pub type QSTRN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RVTRN` writer - RVTRN"]
pub type RVTRN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICPC` writer - ICPC"]
pub type ICPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLBYP` writer - DLLBYP"]
pub type DLLBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTLDINIT` writer - CTLDINIT"]
pub type CTLDINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRSR` writer - CLRSR"]
pub type CLRSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKBYP` writer - LOCKBYP"]
pub type LOCKBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZCALBYP` writer - ZCALBYP"]
pub type ZCALBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITBYP` writer - INITBYP"]
pub type INITBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - INIT"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<DDRPHYC_PIRrs> {
        INIT_W::new(self, 0)
    }
    #[doc = "Bit 1 - DLLSRST"]
    #[inline(always)]
    #[must_use]
    pub fn dllsrst(&mut self) -> DLLSRST_W<DDRPHYC_PIRrs> {
        DLLSRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - DLLLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn dlllock(&mut self) -> DLLLOCK_W<DDRPHYC_PIRrs> {
        DLLLOCK_W::new(self, 2)
    }
    #[doc = "Bit 3 - ZCAL"]
    #[inline(always)]
    #[must_use]
    pub fn zcal(&mut self) -> ZCAL_W<DDRPHYC_PIRrs> {
        ZCAL_W::new(self, 3)
    }
    #[doc = "Bit 4 - ITMSRST"]
    #[inline(always)]
    #[must_use]
    pub fn itmsrst(&mut self) -> ITMSRST_W<DDRPHYC_PIRrs> {
        ITMSRST_W::new(self, 4)
    }
    #[doc = "Bit 5 - DRAMRST"]
    #[inline(always)]
    #[must_use]
    pub fn dramrst(&mut self) -> DRAMRST_W<DDRPHYC_PIRrs> {
        DRAMRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - DRAMINIT"]
    #[inline(always)]
    #[must_use]
    pub fn draminit(&mut self) -> DRAMINIT_W<DDRPHYC_PIRrs> {
        DRAMINIT_W::new(self, 6)
    }
    #[doc = "Bit 7 - QSTRN"]
    #[inline(always)]
    #[must_use]
    pub fn qstrn(&mut self) -> QSTRN_W<DDRPHYC_PIRrs> {
        QSTRN_W::new(self, 7)
    }
    #[doc = "Bit 8 - RVTRN"]
    #[inline(always)]
    #[must_use]
    pub fn rvtrn(&mut self) -> RVTRN_W<DDRPHYC_PIRrs> {
        RVTRN_W::new(self, 8)
    }
    #[doc = "Bit 16 - ICPC"]
    #[inline(always)]
    #[must_use]
    pub fn icpc(&mut self) -> ICPC_W<DDRPHYC_PIRrs> {
        ICPC_W::new(self, 16)
    }
    #[doc = "Bit 17 - DLLBYP"]
    #[inline(always)]
    #[must_use]
    pub fn dllbyp(&mut self) -> DLLBYP_W<DDRPHYC_PIRrs> {
        DLLBYP_W::new(self, 17)
    }
    #[doc = "Bit 18 - CTLDINIT"]
    #[inline(always)]
    #[must_use]
    pub fn ctldinit(&mut self) -> CTLDINIT_W<DDRPHYC_PIRrs> {
        CTLDINIT_W::new(self, 18)
    }
    #[doc = "Bit 28 - CLRSR"]
    #[inline(always)]
    #[must_use]
    pub fn clrsr(&mut self) -> CLRSR_W<DDRPHYC_PIRrs> {
        CLRSR_W::new(self, 28)
    }
    #[doc = "Bit 29 - LOCKBYP"]
    #[inline(always)]
    #[must_use]
    pub fn lockbyp(&mut self) -> LOCKBYP_W<DDRPHYC_PIRrs> {
        LOCKBYP_W::new(self, 29)
    }
    #[doc = "Bit 30 - ZCALBYP"]
    #[inline(always)]
    #[must_use]
    pub fn zcalbyp(&mut self) -> ZCALBYP_W<DDRPHYC_PIRrs> {
        ZCALBYP_W::new(self, 30)
    }
    #[doc = "Bit 31 - INITBYP"]
    #[inline(always)]
    #[must_use]
    pub fn initbyp(&mut self) -> INITBYP_W<DDRPHYC_PIRrs> {
        INITBYP_W::new(self, 31)
    }
}
#[doc = "DDRPHYC PHY initialization register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_pir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_PIRrs;
impl crate::RegisterSpec for DDRPHYC_PIRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_pir::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_PIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_PIR to value 0"]
impl crate::Resettable for DDRPHYC_PIRrs {
    const RESET_VALUE: u32 = 0;
}
