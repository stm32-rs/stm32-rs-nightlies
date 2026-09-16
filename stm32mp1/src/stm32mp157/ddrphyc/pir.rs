///Register `PIR` writer
pub type W = crate::W<PIRrs>;
///Field `INIT` writer - INIT
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLLSRST` writer - DLLSRST
pub type DLLSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLLLOCK` writer - DLLLOCK
pub type DLLLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZCAL` writer - ZCAL
pub type ZCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITMSRST` writer - ITMSRST
pub type ITMSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DRAMRST` writer - DRAMRST
pub type DRAMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DRAMINIT` writer - DRAMINIT
pub type DRAMINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `QSTRN` writer - QSTRN
pub type QSTRN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RVTRN` writer - RVTRN
pub type RVTRN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICPC` writer - ICPC
pub type ICPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLLBYP` writer - DLLBYP
pub type DLLBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTLDINIT` writer - CTLDINIT
pub type CTLDINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLRSR` writer - CLRSR
pub type CLRSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKBYP` writer - LOCKBYP
pub type LOCKBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZCALBYP` writer - ZCALBYP
pub type ZCALBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INITBYP` writer - INITBYP
pub type INITBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PIRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - INIT
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, PIRrs> {
        INIT_W::new(self, 0)
    }
    ///Bit 1 - DLLSRST
    #[inline(always)]
    pub fn dllsrst(&mut self) -> DLLSRST_W<'_, PIRrs> {
        DLLSRST_W::new(self, 1)
    }
    ///Bit 2 - DLLLOCK
    #[inline(always)]
    pub fn dlllock(&mut self) -> DLLLOCK_W<'_, PIRrs> {
        DLLLOCK_W::new(self, 2)
    }
    ///Bit 3 - ZCAL
    #[inline(always)]
    pub fn zcal(&mut self) -> ZCAL_W<'_, PIRrs> {
        ZCAL_W::new(self, 3)
    }
    ///Bit 4 - ITMSRST
    #[inline(always)]
    pub fn itmsrst(&mut self) -> ITMSRST_W<'_, PIRrs> {
        ITMSRST_W::new(self, 4)
    }
    ///Bit 5 - DRAMRST
    #[inline(always)]
    pub fn dramrst(&mut self) -> DRAMRST_W<'_, PIRrs> {
        DRAMRST_W::new(self, 5)
    }
    ///Bit 6 - DRAMINIT
    #[inline(always)]
    pub fn draminit(&mut self) -> DRAMINIT_W<'_, PIRrs> {
        DRAMINIT_W::new(self, 6)
    }
    ///Bit 7 - QSTRN
    #[inline(always)]
    pub fn qstrn(&mut self) -> QSTRN_W<'_, PIRrs> {
        QSTRN_W::new(self, 7)
    }
    ///Bit 8 - RVTRN
    #[inline(always)]
    pub fn rvtrn(&mut self) -> RVTRN_W<'_, PIRrs> {
        RVTRN_W::new(self, 8)
    }
    ///Bit 16 - ICPC
    #[inline(always)]
    pub fn icpc(&mut self) -> ICPC_W<'_, PIRrs> {
        ICPC_W::new(self, 16)
    }
    ///Bit 17 - DLLBYP
    #[inline(always)]
    pub fn dllbyp(&mut self) -> DLLBYP_W<'_, PIRrs> {
        DLLBYP_W::new(self, 17)
    }
    ///Bit 18 - CTLDINIT
    #[inline(always)]
    pub fn ctldinit(&mut self) -> CTLDINIT_W<'_, PIRrs> {
        CTLDINIT_W::new(self, 18)
    }
    ///Bit 28 - CLRSR
    #[inline(always)]
    pub fn clrsr(&mut self) -> CLRSR_W<'_, PIRrs> {
        CLRSR_W::new(self, 28)
    }
    ///Bit 29 - LOCKBYP
    #[inline(always)]
    pub fn lockbyp(&mut self) -> LOCKBYP_W<'_, PIRrs> {
        LOCKBYP_W::new(self, 29)
    }
    ///Bit 30 - ZCALBYP
    #[inline(always)]
    pub fn zcalbyp(&mut self) -> ZCALBYP_W<'_, PIRrs> {
        ZCALBYP_W::new(self, 30)
    }
    ///Bit 31 - INITBYP
    #[inline(always)]
    pub fn initbyp(&mut self) -> INITBYP_W<'_, PIRrs> {
        INITBYP_W::new(self, 31)
    }
}
/**DDRPHYC PHY initialization register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:PIR)*/
pub struct PIRrs;
impl crate::RegisterSpec for PIRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pir::W`](W) writer structure
impl crate::Writable for PIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIR to value 0
impl crate::Resettable for PIRrs {}
