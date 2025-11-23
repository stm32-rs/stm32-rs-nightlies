///Register `TTGTP` reader
pub type R = crate::R<TTGTPrs>;
///Register `TTGTP` writer
pub type W = crate::W<TTGTPrs>;
///Field `TP` reader - TP
pub type TP_R = crate::FieldReader<u16>;
///Field `TP` writer - TP
pub type TP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CTP` reader - CTP
pub type CTP_R = crate::FieldReader<u16>;
///Field `CTP` writer - CTP
pub type CTP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - TP
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - CTP
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTGTP")
            .field("tp", &self.tp())
            .field("ctp", &self.ctp())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - TP
    #[inline(always)]
    pub fn tp(&mut self) -> TP_W<'_, TTGTPrs> {
        TP_W::new(self, 0)
    }
    ///Bits 16:31 - CTP
    #[inline(always)]
    pub fn ctp(&mut self) -> CTP_W<'_, TTGTPrs> {
        CTP_W::new(self, 16)
    }
}
/**If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master.

You can [`read`](crate::Reg::read) this register and get [`ttgtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttgtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FDCAN1:TTGTP)*/
pub struct TTGTPrs;
impl crate::RegisterSpec for TTGTPrs {
    type Ux = u32;
}
///`read()` method returns [`ttgtp::R`](R) reader structure
impl crate::Readable for TTGTPrs {}
///`write(|w| ..)` method takes [`ttgtp::W`](W) writer structure
impl crate::Writable for TTGTPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTGTP to value 0
impl crate::Resettable for TTGTPrs {}
