///Register `DCACHE_CMDREADDRR` reader
pub type R = crate::R<DCACHE_CMDREADDRRrs>;
///Register `DCACHE_CMDREADDRR` writer
pub type W = crate::W<DCACHE_CMDREADDRRrs>;
///Field `CMDENDADDR` reader - CMDENDADDR
pub type CMDENDADDR_R = crate::FieldReader<u32>;
///Field `CMDENDADDR` writer - CMDENDADDR
pub type CMDENDADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 4:31 - CMDENDADDR
    #[inline(always)]
    pub fn cmdendaddr(&self) -> CMDENDADDR_R {
        CMDENDADDR_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_CMDREADDRR")
            .field("cmdendaddr", &self.cmdendaddr())
            .finish()
    }
}
impl W {
    ///Bits 4:31 - CMDENDADDR
    #[inline(always)]
    #[must_use]
    pub fn cmdendaddr(&mut self) -> CMDENDADDR_W<DCACHE_CMDREADDRRrs> {
        CMDENDADDR_W::new(self, 4)
    }
}
/**command range start address register

You can [`read`](crate::Reg::read) this register and get [`dcache_cmdreaddrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_cmdreaddrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#DCACHE1:DCACHE_CMDREADDRR)*/
pub struct DCACHE_CMDREADDRRrs;
impl crate::RegisterSpec for DCACHE_CMDREADDRRrs {
    type Ux = u32;
}
///`read()` method returns [`dcache_cmdreaddrr::R`](R) reader structure
impl crate::Readable for DCACHE_CMDREADDRRrs {}
///`write(|w| ..)` method takes [`dcache_cmdreaddrr::W`](W) writer structure
impl crate::Writable for DCACHE_CMDREADDRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCACHE_CMDREADDRR to value 0
impl crate::Resettable for DCACHE_CMDREADDRRrs {
    const RESET_VALUE: u32 = 0;
}
