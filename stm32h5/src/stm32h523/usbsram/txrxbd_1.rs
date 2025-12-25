///Register `TXRXBD_1` reader
pub type R = crate::R<TXRXBD_1rs>;
///Register `TXRXBD_1` writer
pub type W = crate::W<TXRXBD_1rs>;
///Field `ADDR_TX` reader - Transmission buffer address
pub type ADDR_TX_R = crate::FieldReader<u16>;
///Field `ADDR_TX` writer - Transmission buffer address
pub type ADDR_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `COUNT_TX` reader - Transmission byte count
pub type COUNT_TX_R = crate::FieldReader<u16>;
///Field `COUNT_TX` writer - Transmission byte count
pub type COUNT_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr_tx(&self) -> ADDR_TX_R {
        ADDR_TX_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:25 - Transmission byte count
    #[inline(always)]
    pub fn count_tx(&self) -> COUNT_TX_R {
        COUNT_TX_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXRXBD_1")
            .field("addr_tx", &self.addr_tx())
            .field("count_tx", &self.count_tx())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr_tx(&mut self) -> ADDR_TX_W<'_, TXRXBD_1rs> {
        ADDR_TX_W::new(self, 0)
    }
    ///Bits 16:25 - Transmission byte count
    #[inline(always)]
    pub fn count_tx(&mut self) -> COUNT_TX_W<'_, TXRXBD_1rs> {
        COUNT_TX_W::new(self, 16)
    }
}
/**Channel/endpoint transmit buffer descriptor 1

You can [`read`](crate::Reg::read) this register and get [`txrxbd_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrxbd_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#USBSRAM:TXRXBD_1)*/
pub struct TXRXBD_1rs;
impl crate::RegisterSpec for TXRXBD_1rs {
    type Ux = u32;
}
///`read()` method returns [`txrxbd_1::R`](R) reader structure
impl crate::Readable for TXRXBD_1rs {}
///`write(|w| ..)` method takes [`txrxbd_1::W`](W) writer structure
impl crate::Writable for TXRXBD_1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXRXBD_1 to value 0
impl crate::Resettable for TXRXBD_1rs {}
