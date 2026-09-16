///Register `DOEPTSIZ6` reader
pub type R = crate::R<DOEPTSIZ6rs>;
///Register `DOEPTSIZ6` writer
pub type W = crate::W<DOEPTSIZ6rs>;
///Field `XFRSIZ` reader - Transfer size This field contains the transfer size in bytes for the current endpoint. The core only interrupts the application after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the Rx FIFO and written to the external memory.
pub type XFRSIZ_R = crate::FieldReader<u32>;
///Field `XFRSIZ` writer - Transfer size This field contains the transfer size in bytes for the current endpoint. The core only interrupts the application after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the Rx FIFO and written to the external memory.
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PKTCNT` reader - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for this endpoint. This field is decremented every time a packet (maximum size or short packet) is written to the Rx FIFO.
pub type PKTCNT_R = crate::FieldReader<u16>;
///Field `PKTCNT` writer - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for this endpoint. This field is decremented every time a packet (maximum size or short packet) is written to the Rx FIFO.
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RXDPID` reader - Received data PID This is the data PID received in the last packet for this endpoint. STUPCNT\[1:0\]: SETUP packet count This field specifies the number of back-to-back SETUP data packets the endpoint can receive.
pub type RXDPID_R = crate::FieldReader;
///Field `RXDPID` writer - Received data PID This is the data PID received in the last packet for this endpoint. STUPCNT\[1:0\]: SETUP packet count This field specifies the number of back-to-back SETUP data packets the endpoint can receive.
pub type RXDPID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:18 - Transfer size This field contains the transfer size in bytes for the current endpoint. The core only interrupts the application after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the Rx FIFO and written to the external memory.
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    ///Bits 19:28 - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for this endpoint. This field is decremented every time a packet (maximum size or short packet) is written to the Rx FIFO.
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    ///Bits 29:30 - Received data PID This is the data PID received in the last packet for this endpoint. STUPCNT\[1:0\]: SETUP packet count This field specifies the number of back-to-back SETUP data packets the endpoint can receive.
    #[inline(always)]
    pub fn rxdpid(&self) -> RXDPID_R {
        RXDPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ6")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpid", &self.rxdpid())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - Transfer size This field contains the transfer size in bytes for the current endpoint. The core only interrupts the application after it has exhausted the transfer size amount of data. The transfer size can be set to the maximum packet size of the endpoint, to be interrupted at the end of each packet. The core decrements this field every time a packet is read from the Rx FIFO and written to the external memory.
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<'_, DOEPTSIZ6rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - Packet count Indicates the total number of USB packets that constitute the transfer size amount of data for this endpoint. This field is decremented every time a packet (maximum size or short packet) is written to the Rx FIFO.
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, DOEPTSIZ6rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - Received data PID This is the data PID received in the last packet for this endpoint. STUPCNT\[1:0\]: SETUP packet count This field specifies the number of back-to-back SETUP data packets the endpoint can receive.
    #[inline(always)]
    pub fn rxdpid(&mut self) -> RXDPID_W<'_, DOEPTSIZ6rs> {
        RXDPID_W::new(self, 29)
    }
}
/**OTG device OUT endpoint 6 transfer size register

You can [`read`](crate::Reg::read) this register and get [`doeptsiz6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:DOEPTSIZ6)*/
pub struct DOEPTSIZ6rs;
impl crate::RegisterSpec for DOEPTSIZ6rs {
    type Ux = u32;
}
///`read()` method returns [`doeptsiz6::R`](R) reader structure
impl crate::Readable for DOEPTSIZ6rs {}
///`write(|w| ..)` method takes [`doeptsiz6::W`](W) writer structure
impl crate::Writable for DOEPTSIZ6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPTSIZ6 to value 0
impl crate::Resettable for DOEPTSIZ6rs {}
