///Register `MACTXTSSNR` reader
pub type R = crate::R<MACTXTSSNRrs>;
///Register `MACTXTSSNR` writer
pub type W = crate::W<MACTXTSSNRrs>;
/**Field `TXTSSLO` reader - Transmit Timestamp Status Low

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TXTSSLO_R = crate::FieldReader<u32>;
///Field `TXTSSLO` writer - Transmit Timestamp Status Low
pub type TXTSSLO_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `TXTSSMIS` reader - Transmit Timestamp Status Missed
pub type TXTSSMIS_R = crate::BitReader;
impl R {
    ///Bits 0:30 - Transmit Timestamp Status Low
    #[inline(always)]
    pub fn txtsslo(&self) -> TXTSSLO_R {
        TXTSSLO_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - Transmit Timestamp Status Missed
    #[inline(always)]
    pub fn txtssmis(&self) -> TXTSSMIS_R {
        TXTSSMIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTXTSSNR")
            .field("txtssmis", &self.txtssmis())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - Transmit Timestamp Status Low
    #[inline(always)]
    pub fn txtsslo(&mut self) -> TXTSSLO_W<MACTXTSSNRrs> {
        TXTSSLO_W::new(self, 0)
    }
}
/**Tx timestamp status nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`mactxtssnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactxtssnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACTXTSSNR)*/
pub struct MACTXTSSNRrs;
impl crate::RegisterSpec for MACTXTSSNRrs {
    type Ux = u32;
}
///`read()` method returns [`mactxtssnr::R`](R) reader structure
impl crate::Readable for MACTXTSSNRrs {}
///`write(|w| ..)` method takes [`mactxtssnr::W`](W) writer structure
impl crate::Writable for MACTXTSSNRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACTXTSSNR to value 0
impl crate::Resettable for MACTXTSSNRrs {}
