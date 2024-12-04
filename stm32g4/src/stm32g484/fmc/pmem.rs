///Register `PMEM` reader
pub type R = crate::R<PMEMrs>;
///Register `PMEM` writer
pub type W = crate::W<PMEMrs>;
///Field `MEMSETx` reader - MEMSETx
pub type MEMSETX_R = crate::FieldReader;
///Field `MEMSETx` writer - MEMSETx
pub type MEMSETX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MEMWAITx` reader - MEMWAITx
pub type MEMWAITX_R = crate::FieldReader;
///Field `MEMWAITx` writer - MEMWAITx
pub type MEMWAITX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MEMHOLDx` reader - MEMHOLDx
pub type MEMHOLDX_R = crate::FieldReader;
///Field `MEMHOLDx` writer - MEMHOLDx
pub type MEMHOLDX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MEMHIZx` reader - MEMHIZx
pub type MEMHIZX_R = crate::FieldReader;
///Field `MEMHIZx` writer - MEMHIZx
pub type MEMHIZX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - MEMSETx
    #[inline(always)]
    pub fn memsetx(&self) -> MEMSETX_R {
        MEMSETX_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - MEMWAITx
    #[inline(always)]
    pub fn memwaitx(&self) -> MEMWAITX_R {
        MEMWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - MEMHOLDx
    #[inline(always)]
    pub fn memholdx(&self) -> MEMHOLDX_R {
        MEMHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - MEMHIZx
    #[inline(always)]
    pub fn memhizx(&self) -> MEMHIZX_R {
        MEMHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMEM")
            .field("memhizx", &self.memhizx())
            .field("memholdx", &self.memholdx())
            .field("memwaitx", &self.memwaitx())
            .field("memsetx", &self.memsetx())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - MEMSETx
    #[inline(always)]
    pub fn memsetx(&mut self) -> MEMSETX_W<PMEMrs> {
        MEMSETX_W::new(self, 0)
    }
    ///Bits 8:15 - MEMWAITx
    #[inline(always)]
    pub fn memwaitx(&mut self) -> MEMWAITX_W<PMEMrs> {
        MEMWAITX_W::new(self, 8)
    }
    ///Bits 16:23 - MEMHOLDx
    #[inline(always)]
    pub fn memholdx(&mut self) -> MEMHOLDX_W<PMEMrs> {
        MEMHOLDX_W::new(self, 16)
    }
    ///Bits 24:31 - MEMHIZx
    #[inline(always)]
    pub fn memhizx(&mut self) -> MEMHIZX_W<PMEMrs> {
        MEMHIZX_W::new(self, 24)
    }
}
/**Common memory space timing register 3

You can [`read`](crate::Reg::read) this register and get [`pmem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484xx.html#FMC:PMEM)*/
pub struct PMEMrs;
impl crate::RegisterSpec for PMEMrs {
    type Ux = u32;
}
///`read()` method returns [`pmem::R`](R) reader structure
impl crate::Readable for PMEMrs {}
///`write(|w| ..)` method takes [`pmem::W`](W) writer structure
impl crate::Writable for PMEMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PMEM to value 0xfcfc_fcfc
impl crate::Resettable for PMEMrs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
