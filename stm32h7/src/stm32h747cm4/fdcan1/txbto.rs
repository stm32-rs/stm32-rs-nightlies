///Register `TXBTO` reader
pub type R = crate::R<TXBTOrs>;
///Register `TXBTO` writer
pub type W = crate::W<TXBTOrs>;
///Field `TO` reader - Transmission Occurred.
pub type TO_R = crate::FieldReader<u32>;
///Field `TO` writer - Transmission Occurred.
pub type TO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Transmission Occurred.
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBTO").field("to", &self.to()).finish()
    }
}
impl W {
    ///Bits 0:31 - Transmission Occurred.
    #[inline(always)]
    pub fn to(&mut self) -> TO_W<'_, TXBTOrs> {
        TO_W::new(self, 0)
    }
}
/**FDCAN Tx Buffer Transmission Occurred Register

You can [`read`](crate::Reg::read) this register and get [`txbto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#FDCAN1:TXBTO)*/
pub struct TXBTOrs;
impl crate::RegisterSpec for TXBTOrs {
    type Ux = u32;
}
///`read()` method returns [`txbto::R`](R) reader structure
impl crate::Readable for TXBTOrs {}
///`write(|w| ..)` method takes [`txbto::W`](W) writer structure
impl crate::Writable for TXBTOrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXBTO to value 0
impl crate::Resettable for TXBTOrs {}
