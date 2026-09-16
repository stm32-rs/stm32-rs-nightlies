///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SYSCFGSEC` reader - SYSCFG clock control security
pub type SYSCFGSEC_R = crate::BitReader;
///Field `SYSCFGSEC` writer - SYSCFG clock control security
pub type SYSCFGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLASSBSEC` reader - ClassB security
pub type CLASSBSEC_R = crate::BitReader;
///Field `CLASSBSEC` writer - ClassB security
pub type CLASSBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM2SEC` reader - SRAM2 security
pub type SRAM2SEC_R = crate::BitReader;
///Field `SRAM2SEC` writer - SRAM2 security
pub type SRAM2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPUSEC` reader - FPUSEC
pub type FPUSEC_R = crate::BitReader;
///Field `FPUSEC` writer - FPUSEC
pub type FPUSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SYSCFG clock control security
    #[inline(always)]
    pub fn syscfgsec(&self) -> SYSCFGSEC_R {
        SYSCFGSEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ClassB security
    #[inline(always)]
    pub fn classbsec(&self) -> CLASSBSEC_R {
        CLASSBSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM2 security
    #[inline(always)]
    pub fn sram2sec(&self) -> SRAM2SEC_R {
        SRAM2SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FPUSEC
    #[inline(always)]
    pub fn fpusec(&self) -> FPUSEC_R {
        FPUSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("sram2sec", &self.sram2sec())
            .field("classbsec", &self.classbsec())
            .field("syscfgsec", &self.syscfgsec())
            .field("fpusec", &self.fpusec())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG clock control security
    #[inline(always)]
    pub fn syscfgsec(&mut self) -> SYSCFGSEC_W<'_, SECCFGRrs> {
        SYSCFGSEC_W::new(self, 0)
    }
    ///Bit 1 - ClassB security
    #[inline(always)]
    pub fn classbsec(&mut self) -> CLASSBSEC_W<'_, SECCFGRrs> {
        CLASSBSEC_W::new(self, 1)
    }
    ///Bit 2 - SRAM2 security
    #[inline(always)]
    pub fn sram2sec(&mut self) -> SRAM2SEC_W<'_, SECCFGRrs> {
        SRAM2SEC_W::new(self, 2)
    }
    ///Bit 3 - FPUSEC
    #[inline(always)]
    pub fn fpusec(&mut self) -> FPUSEC_W<'_, SECCFGRrs> {
        FPUSEC_W::new(self, 3)
    }
}
/**SYSCFG secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SYSCFG:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {}
