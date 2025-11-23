///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `PVDL` reader - PVD lock enable bit.
pub type PVDL_R = crate::BitReader;
///Field `PVDL` writer - PVD lock enable bit.
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHL` reader - Flash double ECC error lock bit
pub type FLASHL_R = crate::BitReader;
///Field `FLASHL` writer - Flash double ECC error lock bit
pub type FLASHL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CM7L` reader - Cortex-M7 LOCKUP (HardFault) output enable bit
pub type CM7L_R = crate::BitReader;
///Field `CM7L` writer - Cortex-M7 LOCKUP (HardFault) output enable bit
pub type CM7L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTCML` reader - D1TCM or D0TCM double ECC error signal lock
pub type DTCML_R = crate::BitReader;
///Field `DTCML` writer - D1TCM or D0TCM double ECC error signal lock
pub type DTCML_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITCML` reader - ITCM double ECC error signal lock
pub type ITCML_R = crate::BitReader;
///Field `ITCML` writer - ITCM double ECC error signal lock
pub type ITCML_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - PVD lock enable bit.
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Flash double ECC error lock bit
    #[inline(always)]
    pub fn flashl(&self) -> FLASHL_R {
        FLASHL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Cortex-M7 LOCKUP (HardFault) output enable bit
    #[inline(always)]
    pub fn cm7l(&self) -> CM7L_R {
        CM7L_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - D1TCM or D0TCM double ECC error signal lock
    #[inline(always)]
    pub fn dtcml(&self) -> DTCML_R {
        DTCML_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ITCM double ECC error signal lock
    #[inline(always)]
    pub fn itcml(&self) -> ITCML_R {
        ITCML_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("pvdl", &self.pvdl())
            .field("flashl", &self.flashl())
            .field("cm7l", &self.cm7l())
            .field("dtcml", &self.dtcml())
            .field("itcml", &self.itcml())
            .finish()
    }
}
impl W {
    ///Bit 2 - PVD lock enable bit.
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W<'_, CFGRrs> {
        PVDL_W::new(self, 2)
    }
    ///Bit 3 - Flash double ECC error lock bit
    #[inline(always)]
    pub fn flashl(&mut self) -> FLASHL_W<'_, CFGRrs> {
        FLASHL_W::new(self, 3)
    }
    ///Bit 6 - Cortex-M7 LOCKUP (HardFault) output enable bit
    #[inline(always)]
    pub fn cm7l(&mut self) -> CM7L_W<'_, CFGRrs> {
        CM7L_W::new(self, 6)
    }
    ///Bit 13 - D1TCM or D0TCM double ECC error signal lock
    #[inline(always)]
    pub fn dtcml(&mut self) -> DTCML_W<'_, CFGRrs> {
        DTCML_W::new(self, 13)
    }
    ///Bit 14 - ITCM double ECC error signal lock
    #[inline(always)]
    pub fn itcml(&mut self) -> ITCML_W<'_, CFGRrs> {
        ITCML_W::new(self, 14)
    }
}
/**SYSCFG timer break lockup register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SYSCFG:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
