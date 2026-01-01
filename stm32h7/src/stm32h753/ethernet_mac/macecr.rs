///Register `MACECR` reader
pub type R = crate::R<MACECRrs>;
///Register `MACECR` writer
pub type W = crate::W<MACECRrs>;
///Field `GPSL` reader - Giant Packet Size Limit
pub type GPSL_R = crate::FieldReader<u16>;
///Field `GPSL` writer - Giant Packet Size Limit
pub type GPSL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `DCRCC` reader - Disable CRC Checking for Received Packets
pub type DCRCC_R = crate::BitReader;
///Field `DCRCC` writer - Disable CRC Checking for Received Packets
pub type DCRCC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPEN` reader - Slow Protocol Detection Enable
pub type SPEN_R = crate::BitReader;
///Field `SPEN` writer - Slow Protocol Detection Enable
pub type SPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USP` reader - Unicast Slow Protocol Packet Detect
pub type USP_R = crate::BitReader;
///Field `USP` writer - Unicast Slow Protocol Packet Detect
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIPGEN` reader - Extended Inter-Packet Gap Enable
pub type EIPGEN_R = crate::BitReader;
///Field `EIPGEN` writer - Extended Inter-Packet Gap Enable
pub type EIPGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIPG` reader - Extended Inter-Packet Gap
pub type EIPG_R = crate::FieldReader;
///Field `EIPG` writer - Extended Inter-Packet Gap
pub type EIPG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:13 - Giant Packet Size Limit
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 16 - Disable CRC Checking for Received Packets
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Slow Protocol Detection Enable
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Unicast Slow Protocol Packet Detect
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - Extended Inter-Packet Gap Enable
    #[inline(always)]
    pub fn eipgen(&self) -> EIPGEN_R {
        EIPGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:29 - Extended Inter-Packet Gap
    #[inline(always)]
    pub fn eipg(&self) -> EIPG_R {
        EIPG_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACECR")
            .field("gpsl", &self.gpsl())
            .field("dcrcc", &self.dcrcc())
            .field("spen", &self.spen())
            .field("usp", &self.usp())
            .field("eipgen", &self.eipgen())
            .field("eipg", &self.eipg())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Giant Packet Size Limit
    #[inline(always)]
    pub fn gpsl(&mut self) -> GPSL_W<'_, MACECRrs> {
        GPSL_W::new(self, 0)
    }
    ///Bit 16 - Disable CRC Checking for Received Packets
    #[inline(always)]
    pub fn dcrcc(&mut self) -> DCRCC_W<'_, MACECRrs> {
        DCRCC_W::new(self, 16)
    }
    ///Bit 17 - Slow Protocol Detection Enable
    #[inline(always)]
    pub fn spen(&mut self) -> SPEN_W<'_, MACECRrs> {
        SPEN_W::new(self, 17)
    }
    ///Bit 18 - Unicast Slow Protocol Packet Detect
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W<'_, MACECRrs> {
        USP_W::new(self, 18)
    }
    ///Bit 24 - Extended Inter-Packet Gap Enable
    #[inline(always)]
    pub fn eipgen(&mut self) -> EIPGEN_W<'_, MACECRrs> {
        EIPGEN_W::new(self, 24)
    }
    ///Bits 25:29 - Extended Inter-Packet Gap
    #[inline(always)]
    pub fn eipg(&mut self) -> EIPG_W<'_, MACECRrs> {
        EIPG_W::new(self, 25)
    }
}
/**Extended operating mode configuration register

You can [`read`](crate::Reg::read) this register and get [`macecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#Ethernet_MAC:MACECR)*/
pub struct MACECRrs;
impl crate::RegisterSpec for MACECRrs {
    type Ux = u32;
}
///`read()` method returns [`macecr::R`](R) reader structure
impl crate::Readable for MACECRrs {}
///`write(|w| ..)` method takes [`macecr::W`](W) writer structure
impl crate::Writable for MACECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACECR to value 0
impl crate::Resettable for MACECRrs {}
