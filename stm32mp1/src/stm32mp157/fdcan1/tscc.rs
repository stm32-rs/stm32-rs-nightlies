///Register `TSCC` reader
pub type R = crate::R<TSCCrs>;
///Register `TSCC` writer
pub type W = crate::W<TSCCrs>;
///Field `TSS` reader - TSS
pub type TSS_R = crate::FieldReader;
///Field `TSS` writer - TSS
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TCP` reader - TCP
pub type TCP_R = crate::FieldReader;
///Field `TCP` writer - TCP
pub type TCP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - TSS
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    ///Bits 16:19 - TCP
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCC")
            .field("tss", &self.tss())
            .field("tcp", &self.tcp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TSS
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W<'_, TSCCrs> {
        TSS_W::new(self, 0)
    }
    ///Bits 16:19 - TCP
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W<'_, TSCCrs> {
        TCP_W::new(self, 16)
    }
}
/**FDCAN timestamp counter configuration register

You can [`read`](crate::Reg::read) this register and get [`tscc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TSCC)*/
pub struct TSCCrs;
impl crate::RegisterSpec for TSCCrs {
    type Ux = u32;
}
///`read()` method returns [`tscc::R`](R) reader structure
impl crate::Readable for TSCCrs {}
///`write(|w| ..)` method takes [`tscc::W`](W) writer structure
impl crate::Writable for TSCCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCC to value 0
impl crate::Resettable for TSCCrs {}
