///Register `PMEM%s` reader
pub type R = crate::R<PMEMrs>;
///Register `PMEM%s` writer
pub type W = crate::W<PMEMrs>;
///Field `MEMSET` reader - MEMSETx
pub type MEMSET_R = crate::FieldReader;
///Field `MEMSET` writer - MEMSETx
pub type MEMSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MEMWAIT` reader - MEMWAITx
pub type MEMWAIT_R = crate::FieldReader;
///Field `MEMWAIT` writer - MEMWAITx
pub type MEMWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MEMHOLD` reader - MEMHOLDx
pub type MEMHOLD_R = crate::FieldReader;
///Field `MEMHOLD` writer - MEMHOLDx
pub type MEMHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MEMHIZ` reader - MEMHIZx
pub type MEMHIZ_R = crate::FieldReader;
///Field `MEMHIZ` writer - MEMHIZx
pub type MEMHIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - MEMSETx
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - MEMWAITx
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - MEMHOLDx
    #[inline(always)]
    pub fn memhold(&self) -> MEMHOLD_R {
        MEMHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - MEMHIZx
    #[inline(always)]
    pub fn memhiz(&self) -> MEMHIZ_R {
        MEMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMEM")
            .field("memhiz", &self.memhiz())
            .field("memhold", &self.memhold())
            .field("memwait", &self.memwait())
            .field("memset", &self.memset())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - MEMSETx
    #[inline(always)]
    pub fn memset(&mut self) -> MEMSET_W<'_, PMEMrs> {
        MEMSET_W::new(self, 0)
    }
    ///Bits 8:15 - MEMWAITx
    #[inline(always)]
    pub fn memwait(&mut self) -> MEMWAIT_W<'_, PMEMrs> {
        MEMWAIT_W::new(self, 8)
    }
    ///Bits 16:23 - MEMHOLDx
    #[inline(always)]
    pub fn memhold(&mut self) -> MEMHOLD_W<'_, PMEMrs> {
        MEMHOLD_W::new(self, 16)
    }
    ///Bits 24:31 - MEMHIZx
    #[inline(always)]
    pub fn memhiz(&mut self) -> MEMHIZ_W<'_, PMEMrs> {
        MEMHIZ_W::new(self, 24)
    }
}
/**Common memory space timing register %s

You can [`read`](crate::Reg::read) this register and get [`pmem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#FMC:PMEM[2])*/
pub struct PMEMrs;
impl crate::RegisterSpec for PMEMrs {
    type Ux = u32;
}
///`read()` method returns [`pmem::R`](R) reader structure
impl crate::Readable for PMEMrs {}
///`write(|w| ..)` method takes [`pmem::W`](W) writer structure
impl crate::Writable for PMEMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMEM%s to value 0xfcfc_fcfc
impl crate::Resettable for PMEMrs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
