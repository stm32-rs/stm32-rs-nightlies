///Register `DMAC0RXIWTR` reader
pub type R = crate::R<DMAC0RXIWTRrs>;
///Register `DMAC0RXIWTR` writer
pub type W = crate::W<DMAC0RXIWTRrs>;
///Field `RWT` reader - Receive Interrupt Watchdog Timer Count
pub type RWT_R = crate::FieldReader;
///Field `RWT` writer - Receive Interrupt Watchdog Timer Count
pub type RWT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RWTU` reader - Receive Interrupt Watchdog Timer Count Units
pub type RWTU_R = crate::FieldReader;
///Field `RWTU` writer - Receive Interrupt Watchdog Timer Count Units
pub type RWTU_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:7 - Receive Interrupt Watchdog Timer Count
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:17 - Receive Interrupt Watchdog Timer Count Units
    #[inline(always)]
    pub fn rwtu(&self) -> RWTU_R {
        RWTU_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0RXIWTR")
            .field("rwt", &self.rwt())
            .field("rwtu", &self.rwtu())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Receive Interrupt Watchdog Timer Count
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W<'_, DMAC0RXIWTRrs> {
        RWT_W::new(self, 0)
    }
    ///Bits 16:17 - Receive Interrupt Watchdog Timer Count Units
    #[inline(always)]
    pub fn rwtu(&mut self) -> RWTU_W<'_, DMAC0RXIWTRrs> {
        RWTU_W::new(self, 16)
    }
}
/**Channel 0 R0 interrupt watchdog timer register

You can [`read`](crate::Reg::read) this register and get [`dmac0rxiwtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rxiwtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0RXIWTR)*/
pub struct DMAC0RXIWTRrs;
impl crate::RegisterSpec for DMAC0RXIWTRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0rxiwtr::R`](R) reader structure
impl crate::Readable for DMAC0RXIWTRrs {}
///`write(|w| ..)` method takes [`dmac0rxiwtr::W`](W) writer structure
impl crate::Writable for DMAC0RXIWTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0RXIWTR to value 0
impl crate::Resettable for DMAC0RXIWTRrs {}
