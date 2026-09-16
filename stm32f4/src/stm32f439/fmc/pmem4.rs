///Register `PMEM4` reader
pub type R = crate::R<PMEM4rs>;
///Register `PMEM4` writer
pub type W = crate::W<PMEM4rs>;
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
        f.debug_struct("PMEM4")
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
    pub fn memsetx(&mut self) -> MEMSETX_W<'_, PMEM4rs> {
        MEMSETX_W::new(self, 0)
    }
    ///Bits 8:15 - MEMWAITx
    #[inline(always)]
    pub fn memwaitx(&mut self) -> MEMWAITX_W<'_, PMEM4rs> {
        MEMWAITX_W::new(self, 8)
    }
    ///Bits 16:23 - MEMHOLDx
    #[inline(always)]
    pub fn memholdx(&mut self) -> MEMHOLDX_W<'_, PMEM4rs> {
        MEMHOLDX_W::new(self, 16)
    }
    ///Bits 24:31 - MEMHIZx
    #[inline(always)]
    pub fn memhizx(&mut self) -> MEMHIZX_W<'_, PMEM4rs> {
        MEMHIZX_W::new(self, 24)
    }
}
/**Common memory space timing register 4

You can [`read`](crate::Reg::read) this register and get [`pmem4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmem4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#FMC:PMEM4)*/
pub struct PMEM4rs;
impl crate::RegisterSpec for PMEM4rs {
    type Ux = u32;
}
///`read()` method returns [`pmem4::R`](R) reader structure
impl crate::Readable for PMEM4rs {}
///`write(|w| ..)` method takes [`pmem4::W`](W) writer structure
impl crate::Writable for PMEM4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMEM4 to value 0xfcfc_fcfc
impl crate::Resettable for PMEM4rs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
