///Register `CEC_TXDR` reader
pub type R = crate::R<CEC_TXDRrs>;
///Register `CEC_TXDR` writer
pub type W = crate::W<CEC_TXDRrs>;
///Field `TXD` writer - TXD
pub type TXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CEC_TXDR").finish()
    }
}
impl W {
    ///Bits 0:7 - TXD
    #[inline(always)]
    #[must_use]
    pub fn txd(&mut self) -> TXD_W<CEC_TXDRrs> {
        TXD_W::new(self, 0)
    }
}
/**CEC Tx data register

You can [`read`](crate::Reg::read) this register and get [`cec_txdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cec_txdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDMI_CEC:CEC_TXDR)*/
pub struct CEC_TXDRrs;
impl crate::RegisterSpec for CEC_TXDRrs {
    type Ux = u32;
}
///`read()` method returns [`cec_txdr::R`](R) reader structure
impl crate::Readable for CEC_TXDRrs {}
///`write(|w| ..)` method takes [`cec_txdr::W`](W) writer structure
impl crate::Writable for CEC_TXDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CEC_TXDR to value 0
impl crate::Resettable for CEC_TXDRrs {
    const RESET_VALUE: u32 = 0;
}
