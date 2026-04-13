///Register `WKUPCR` reader
pub type R = crate::R<WKUPCRrs>;
///Register `WKUPCR` writer
pub type W = crate::W<WKUPCRrs>;
///Field `WKUPC` reader - Clear Wakeup pin flag for WKUP. These bits are always read as 0.
pub type WKUPC_R = crate::FieldReader;
///Field `WKUPC` writer - Clear Wakeup pin flag for WKUP. These bits are always read as 0.
pub type WKUPC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0.
    #[inline(always)]
    pub fn wkupc(&self) -> WKUPC_R {
        WKUPC_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKUPCR")
            .field("wkupc", &self.wkupc())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0.
    #[inline(always)]
    pub fn wkupc(&mut self) -> WKUPC_W<'_, WKUPCRrs> {
        WKUPC_W::new(self, 0)
    }
}
/**reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).

You can [`read`](crate::Reg::read) this register and get [`wkupcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#PWR:WKUPCR)*/
pub struct WKUPCRrs;
impl crate::RegisterSpec for WKUPCRrs {
    type Ux = u32;
}
///`read()` method returns [`wkupcr::R`](R) reader structure
impl crate::Readable for WKUPCRrs {}
///`write(|w| ..)` method takes [`wkupcr::W`](W) writer structure
impl crate::Writable for WKUPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WKUPCR to value 0
impl crate::Resettable for WKUPCRrs {}
