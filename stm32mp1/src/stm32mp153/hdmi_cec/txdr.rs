///Register `TXDR` reader
pub type R = crate::R<TXDRrs>;
///Register `TXDR` writer
pub type W = crate::W<TXDRrs>;
///Field `TXD` writer - TXD
pub type TXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXDR").finish()
    }
}
impl W {
    ///Bits 0:7 - TXD
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W<'_, TXDRrs> {
        TXD_W::new(self, 0)
    }
}
/**CEC Tx data register

You can [`read`](crate::Reg::read) this register and get [`txdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDMI_CEC:TXDR)*/
pub struct TXDRrs;
impl crate::RegisterSpec for TXDRrs {
    type Ux = u32;
}
///`read()` method returns [`txdr::R`](R) reader structure
impl crate::Readable for TXDRrs {}
///`write(|w| ..)` method takes [`txdr::W`](W) writer structure
impl crate::Writable for TXDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXDR to value 0
impl crate::Resettable for TXDRrs {}
