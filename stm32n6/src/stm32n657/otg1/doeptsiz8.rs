///Register `DOEPTSIZ8` reader
pub type R = crate::R<DOEPTSIZ8rs>;
///Register `DOEPTSIZ8` writer
pub type W = crate::W<DOEPTSIZ8rs>;
///Field `XFRSIZ` reader - Transfer size
pub type XFRSIZ_R = crate::FieldReader<u32>;
///Field `XFRSIZ` writer - Transfer size
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PKTCNT` reader - Packet count
pub type PKTCNT_R = crate::FieldReader<u16>;
///Field `PKTCNT` writer - Packet count
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RXDPID` reader - Received data PID
pub type RXDPID_R = crate::FieldReader;
///Field `RXDPID` writer - Received data PID
pub type RXDPID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    ///Bits 29:30 - Received data PID
    #[inline(always)]
    pub fn rxdpid(&self) -> RXDPID_R {
        RXDPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ8")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpid", &self.rxdpid())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<'_, DOEPTSIZ8rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, DOEPTSIZ8rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - Received data PID
    #[inline(always)]
    pub fn rxdpid(&mut self) -> RXDPID_W<'_, DOEPTSIZ8rs> {
        RXDPID_W::new(self, 29)
    }
}
/**OTG device OUT endpoint 8 transfer size register

You can [`read`](crate::Reg::read) this register and get [`doeptsiz8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#OTG1:DOEPTSIZ8)*/
pub struct DOEPTSIZ8rs;
impl crate::RegisterSpec for DOEPTSIZ8rs {
    type Ux = u32;
}
///`read()` method returns [`doeptsiz8::R`](R) reader structure
impl crate::Readable for DOEPTSIZ8rs {}
///`write(|w| ..)` method takes [`doeptsiz8::W`](W) writer structure
impl crate::Writable for DOEPTSIZ8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPTSIZ8 to value 0
impl crate::Resettable for DOEPTSIZ8rs {}
