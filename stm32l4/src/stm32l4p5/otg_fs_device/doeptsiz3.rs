///Register `DOEPTSIZ3` reader
pub type R = crate::R<DOEPTSIZ3rs>;
///Register `DOEPTSIZ3` writer
pub type W = crate::W<DOEPTSIZ3rs>;
///Field `XFRSIZ` reader - Transfer size
pub type XFRSIZ_R = crate::FieldReader<u32>;
///Field `XFRSIZ` writer - Transfer size
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PKTCNT` reader - Packet count
pub type PKTCNT_R = crate::FieldReader<u16>;
///Field `PKTCNT` writer - Packet count
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RXDPID_STUPCNT` reader - Received data PID/SETUP packet count
pub type RXDPID_STUPCNT_R = crate::FieldReader;
///Field `RXDPID_STUPCNT` writer - Received data PID/SETUP packet count
pub type RXDPID_STUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    ///Bits 29:30 - Received data PID/SETUP packet count
    #[inline(always)]
    pub fn rxdpid_stupcnt(&self) -> RXDPID_STUPCNT_R {
        RXDPID_STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ3")
            .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
            .field("pktcnt", &self.pktcnt())
            .field("xfrsiz", &self.xfrsiz())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<DOEPTSIZ3rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DOEPTSIZ3rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - Received data PID/SETUP packet count
    #[inline(always)]
    pub fn rxdpid_stupcnt(&mut self) -> RXDPID_STUPCNT_W<DOEPTSIZ3rs> {
        RXDPID_STUPCNT_W::new(self, 29)
    }
}
/**device OUT endpoint-3 transfer size register

You can [`read`](crate::Reg::read) this register and get [`doeptsiz3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPTSIZ3)*/
pub struct DOEPTSIZ3rs;
impl crate::RegisterSpec for DOEPTSIZ3rs {
    type Ux = u32;
}
///`read()` method returns [`doeptsiz3::R`](R) reader structure
impl crate::Readable for DOEPTSIZ3rs {}
///`write(|w| ..)` method takes [`doeptsiz3::W`](W) writer structure
impl crate::Writable for DOEPTSIZ3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPTSIZ3 to value 0
impl crate::Resettable for DOEPTSIZ3rs {}
