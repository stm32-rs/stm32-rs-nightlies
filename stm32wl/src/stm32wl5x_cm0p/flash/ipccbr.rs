///Register `IPCCBR` reader
pub type R = crate::R<IPCCBRrs>;
///Register `IPCCBR` writer
pub type W = crate::W<IPCCBRrs>;
///Field `IPCCDBA` reader - IPCCDBA
pub type IPCCDBA_R = crate::FieldReader<u16>;
///Field `IPCCDBA` writer - IPCCDBA
pub type IPCCDBA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16, crate::Safe>;
impl R {
    ///Bits 0:13 - IPCCDBA
    #[inline(always)]
    pub fn ipccdba(&self) -> IPCCDBA_R {
        IPCCDBA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPCCBR")
            .field("ipccdba", &self.ipccdba())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - IPCCDBA
    #[inline(always)]
    pub fn ipccdba(&mut self) -> IPCCDBA_W<'_, IPCCBRrs> {
        IPCCDBA_W::new(self, 0)
    }
}
/**Flash IPCC data buffer address register

You can [`read`](crate::Reg::read) this register and get [`ipccbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipccbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#FLASH:IPCCBR)*/
pub struct IPCCBRrs;
impl crate::RegisterSpec for IPCCBRrs {
    type Ux = u32;
}
///`read()` method returns [`ipccbr::R`](R) reader structure
impl crate::Readable for IPCCBRrs {}
///`write(|w| ..)` method takes [`ipccbr::W`](W) writer structure
impl crate::Writable for IPCCBRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IPCCBR to value 0xffff_ffff
impl crate::Resettable for IPCCBRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
