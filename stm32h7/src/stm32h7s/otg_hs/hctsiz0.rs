///Register `HCTSIZ0` reader
pub type R = crate::R<HCTSIZ0rs>;
///Register `HCTSIZ0` writer
pub type W = crate::W<HCTSIZ0rs>;
///Field `XFRSIZ` reader - Transfer size For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic).
pub type XFRSIZ_R = crate::FieldReader<u32>;
///Field `XFRSIZ` writer - Transfer size For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic).
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
///Field `PKTCNT` reader - Packet count This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion.
pub type PKTCNT_R = crate::FieldReader<u16>;
///Field `PKTCNT` writer - Packet count This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion.
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `DPID` reader - Data PID The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer.
pub type DPID_R = crate::FieldReader;
///Field `DPID` writer - Data PID The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer.
pub type DPID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DOPNG` reader - Do Ping This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for IN transfers, it disables the channel.
pub type DOPNG_R = crate::BitReader;
///Field `DOPNG` writer - Do Ping This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for IN transfers, it disables the channel.
pub type DOPNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:18 - Transfer size For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic).
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    ///Bits 19:28 - Packet count This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion.
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    ///Bits 29:30 - Data PID The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer.
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
    ///Bit 31 - Do Ping This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for IN transfers, it disables the channel.
    #[inline(always)]
    pub fn dopng(&self) -> DOPNG_R {
        DOPNG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ0")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("dpid", &self.dpid())
            .field("dopng", &self.dopng())
            .finish()
    }
}
impl W {
    ///Bits 0:18 - Transfer size For an OUT, this field is the number of data bytes the host sends during the transfer. For an IN, this field is the buffer size that the application has reserved for the transfer. The application is expected to program this field as an integer multiple of the maximum packet size for IN transactions (periodic and non-periodic).
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<'_, HCTSIZ0rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bits 19:28 - Packet count This field is programmed by the application with the expected number of packets to be transmitted (OUT) or received (IN). The host decrements this count on every successful transmission or reception of an OUT/IN packet. Once this count reaches zero, the application is interrupted to indicate normal completion.
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, HCTSIZ0rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - Data PID The application programs this field with the type of PID to use for the initial transaction. The host maintains this field for the rest of the transfer.
    #[inline(always)]
    pub fn dpid(&mut self) -> DPID_W<'_, HCTSIZ0rs> {
        DPID_W::new(self, 29)
    }
    ///Bit 31 - Do Ping This bit is used only for OUT transfers. Setting this field to 1 directs the host to do PING protocol. Note: Do not set this bit for IN transfers. If this bit is set for IN transfers, it disables the channel.
    #[inline(always)]
    pub fn dopng(&mut self) -> DOPNG_W<'_, HCTSIZ0rs> {
        DOPNG_W::new(self, 31)
    }
}
/**OTG host channel 0 transfer size register

You can [`read`](crate::Reg::read) this register and get [`hctsiz0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:HCTSIZ0)*/
pub struct HCTSIZ0rs;
impl crate::RegisterSpec for HCTSIZ0rs {
    type Ux = u32;
}
///`read()` method returns [`hctsiz0::R`](R) reader structure
impl crate::Readable for HCTSIZ0rs {}
///`write(|w| ..)` method takes [`hctsiz0::W`](W) writer structure
impl crate::Writable for HCTSIZ0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCTSIZ0 to value 0
impl crate::Resettable for HCTSIZ0rs {}
