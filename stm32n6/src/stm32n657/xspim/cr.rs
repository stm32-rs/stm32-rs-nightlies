///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `MUXEN` reader - Multiplexed mode enable
pub type MUXEN_R = crate::BitReader;
///Field `MUXEN` writer - Multiplexed mode enable
pub type MUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - XSPI multiplexing mode
pub type MODE_R = crate::BitReader;
///Field `MODE` writer - XSPI multiplexing mode
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSEL_OVR_EN` reader - Chip select selector override enable
pub type CSSEL_OVR_EN_R = crate::BitReader;
///Field `CSSEL_OVR_EN` writer - Chip select selector override enable
pub type CSSEL_OVR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSEL_OVR_O1` reader - Chip select selector override setting for XSPI1
pub type CSSEL_OVR_O1_R = crate::BitReader;
///Field `CSSEL_OVR_O1` writer - Chip select selector override setting for XSPI1
pub type CSSEL_OVR_O1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSSEL_OVR_O2` reader - Chip select selector override setting for XSPI2
pub type CSSEL_OVR_O2_R = crate::BitReader;
///Field `CSSEL_OVR_O2` writer - Chip select selector override setting for XSPI2
pub type CSSEL_OVR_O2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REQ2ACK_TIME` reader - REQ to ACK time
pub type REQ2ACK_TIME_R = crate::FieldReader;
///Field `REQ2ACK_TIME` writer - REQ to ACK time
pub type REQ2ACK_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Multiplexed mode enable
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - XSPI multiplexing mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Chip select selector override enable
    #[inline(always)]
    pub fn cssel_ovr_en(&self) -> CSSEL_OVR_EN_R {
        CSSEL_OVR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Chip select selector override setting for XSPI1
    #[inline(always)]
    pub fn cssel_ovr_o1(&self) -> CSSEL_OVR_O1_R {
        CSSEL_OVR_O1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Chip select selector override setting for XSPI2
    #[inline(always)]
    pub fn cssel_ovr_o2(&self) -> CSSEL_OVR_O2_R {
        CSSEL_OVR_O2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 16:23 - REQ to ACK time
    #[inline(always)]
    pub fn req2ack_time(&self) -> REQ2ACK_TIME_R {
        REQ2ACK_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("muxen", &self.muxen())
            .field("mode", &self.mode())
            .field("cssel_ovr_en", &self.cssel_ovr_en())
            .field("cssel_ovr_o1", &self.cssel_ovr_o1())
            .field("cssel_ovr_o2", &self.cssel_ovr_o2())
            .field("req2ack_time", &self.req2ack_time())
            .finish()
    }
}
impl W {
    ///Bit 0 - Multiplexed mode enable
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W<'_, CRrs> {
        MUXEN_W::new(self, 0)
    }
    ///Bit 1 - XSPI multiplexing mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 1)
    }
    ///Bit 4 - Chip select selector override enable
    #[inline(always)]
    pub fn cssel_ovr_en(&mut self) -> CSSEL_OVR_EN_W<'_, CRrs> {
        CSSEL_OVR_EN_W::new(self, 4)
    }
    ///Bit 5 - Chip select selector override setting for XSPI1
    #[inline(always)]
    pub fn cssel_ovr_o1(&mut self) -> CSSEL_OVR_O1_W<'_, CRrs> {
        CSSEL_OVR_O1_W::new(self, 5)
    }
    ///Bit 6 - Chip select selector override setting for XSPI2
    #[inline(always)]
    pub fn cssel_ovr_o2(&mut self) -> CSSEL_OVR_O2_W<'_, CRrs> {
        CSSEL_OVR_O2_W::new(self, 6)
    }
    ///Bits 16:23 - REQ to ACK time
    #[inline(always)]
    pub fn req2ack_time(&mut self) -> REQ2ACK_TIME_W<'_, CRrs> {
        REQ2ACK_TIME_W::new(self, 16)
    }
}
/**XSPIM control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#XSPIM:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
