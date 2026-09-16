///Register `RXBC` reader
pub type R = crate::R<RXBCrs>;
///Register `RXBC` writer
pub type W = crate::W<RXBCrs>;
///Field `RBSA` reader - Rx Buffer Start Address
pub type RBSA_R = crate::FieldReader<u16>;
///Field `RBSA` writer - Rx Buffer Start Address
pub type RBSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 2:15 - Rx Buffer Start Address
    #[inline(always)]
    pub fn rbsa(&self) -> RBSA_R {
        RBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXBC").field("rbsa", &self.rbsa()).finish()
    }
}
impl W {
    ///Bits 2:15 - Rx Buffer Start Address
    #[inline(always)]
    pub fn rbsa(&mut self) -> RBSA_W<'_, RXBCrs> {
        RBSA_W::new(self, 2)
    }
}
/**FDCAN Rx Buffer Configuration Register

You can [`read`](crate::Reg::read) this register and get [`rxbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#FDCAN2:RXBC)*/
pub struct RXBCrs;
impl crate::RegisterSpec for RXBCrs {
    type Ux = u32;
}
///`read()` method returns [`rxbc::R`](R) reader structure
impl crate::Readable for RXBCrs {}
///`write(|w| ..)` method takes [`rxbc::W`](W) writer structure
impl crate::Writable for RXBCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RXBC to value 0
impl crate::Resettable for RXBCrs {}
