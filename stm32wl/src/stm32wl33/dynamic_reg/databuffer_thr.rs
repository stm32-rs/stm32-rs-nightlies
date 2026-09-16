///Register `DATABUFFER_THR` reader
pub type R = crate::R<DATABUFFER_THRrs>;
///Register `DATABUFFER_THR` writer
pub type W = crate::W<DATABUFFER_THRrs>;
///Field `RX_ALMOST_FULL_THR` reader - Almost Full threshold for RX Data Buffers
pub type RX_ALMOST_FULL_THR_R = crate::FieldReader<u16>;
///Field `RX_ALMOST_FULL_THR` writer - Almost Full threshold for RX Data Buffers
pub type RX_ALMOST_FULL_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TX_ALMOST_EMPTY_THR` reader - Almost Empty threshold for TX Data Buffers.
pub type TX_ALMOST_EMPTY_THR_R = crate::FieldReader<u16>;
///Field `TX_ALMOST_EMPTY_THR` writer - Almost Empty threshold for TX Data Buffers.
pub type TX_ALMOST_EMPTY_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Almost Full threshold for RX Data Buffers
    #[inline(always)]
    pub fn rx_almost_full_thr(&self) -> RX_ALMOST_FULL_THR_R {
        RX_ALMOST_FULL_THR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Almost Empty threshold for TX Data Buffers.
    #[inline(always)]
    pub fn tx_almost_empty_thr(&self) -> TX_ALMOST_EMPTY_THR_R {
        TX_ALMOST_EMPTY_THR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATABUFFER_THR")
            .field("rx_almost_full_thr", &self.rx_almost_full_thr())
            .field("tx_almost_empty_thr", &self.tx_almost_empty_thr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Almost Full threshold for RX Data Buffers
    #[inline(always)]
    pub fn rx_almost_full_thr(&mut self) -> RX_ALMOST_FULL_THR_W<'_, DATABUFFER_THRrs> {
        RX_ALMOST_FULL_THR_W::new(self, 0)
    }
    ///Bits 16:31 - Almost Empty threshold for TX Data Buffers.
    #[inline(always)]
    pub fn tx_almost_empty_thr(&mut self) -> TX_ALMOST_EMPTY_THR_W<'_, DATABUFFER_THRrs> {
        TX_ALMOST_EMPTY_THR_W::new(self, 16)
    }
}
/**DATABUFFER_THR register

You can [`read`](crate::Reg::read) this register and get [`databuffer_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuffer_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:DATABUFFER_THR)*/
pub struct DATABUFFER_THRrs;
impl crate::RegisterSpec for DATABUFFER_THRrs {
    type Ux = u32;
}
///`read()` method returns [`databuffer_thr::R`](R) reader structure
impl crate::Readable for DATABUFFER_THRrs {}
///`write(|w| ..)` method takes [`databuffer_thr::W`](W) writer structure
impl crate::Writable for DATABUFFER_THRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATABUFFER_THR to value 0
impl crate::Resettable for DATABUFFER_THRrs {}
