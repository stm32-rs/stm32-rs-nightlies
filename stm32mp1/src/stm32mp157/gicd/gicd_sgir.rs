///Register `GICD_SGIR` writer
pub type W = crate::W<GICD_SGIRrs>;
///Field `SGIINTID` writer - SGIINTID
pub type SGIINTID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `NSATT` writer - NSATT
pub type NSATT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPUTARGETLIST` writer - CPUTARGETLIST
pub type CPUTARGETLIST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TARGETLISTFILTER` writer - TARGETLISTFILTER
pub type TARGETLISTFILTER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl core::fmt::Debug for crate::generic::Reg<GICD_SGIRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:3 - SGIINTID
    #[inline(always)]
    #[must_use]
    pub fn sgiintid(&mut self) -> SGIINTID_W<GICD_SGIRrs> {
        SGIINTID_W::new(self, 0)
    }
    ///Bit 15 - NSATT
    #[inline(always)]
    #[must_use]
    pub fn nsatt(&mut self) -> NSATT_W<GICD_SGIRrs> {
        NSATT_W::new(self, 15)
    }
    ///Bits 16:17 - CPUTARGETLIST
    #[inline(always)]
    #[must_use]
    pub fn cputargetlist(&mut self) -> CPUTARGETLIST_W<GICD_SGIRrs> {
        CPUTARGETLIST_W::new(self, 16)
    }
    ///Bits 24:25 - TARGETLISTFILTER
    #[inline(always)]
    #[must_use]
    pub fn targetlistfilter(&mut self) -> TARGETLISTFILTER_W<GICD_SGIRrs> {
        TARGETLISTFILTER_W::new(self, 24)
    }
}
/**GICD software generated interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_sgir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_SGIR)*/
pub struct GICD_SGIRrs;
impl crate::RegisterSpec for GICD_SGIRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gicd_sgir::W`](W) writer structure
impl crate::Writable for GICD_SGIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_SGIR to value 0
impl crate::Resettable for GICD_SGIRrs {
    const RESET_VALUE: u32 = 0;
}
