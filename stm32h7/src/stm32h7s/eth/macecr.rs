///Register `MACECR` reader
pub type R = crate::R<MACECRrs>;
///Register `MACECR` writer
pub type W = crate::W<MACECRrs>;
///Field `GPSL` reader - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register.
pub type GPSL_R = crate::FieldReader<u16>;
///Field `GPSL` writer - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register.
pub type GPSL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `DCRCC` reader - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets.
pub type DCRCC_R = crate::BitReader;
///Field `DCRCC` writer - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets.
pub type DCRCC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPEN` reader - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets.
pub type SPEN_R = crate::BitReader;
///Field `SPEN` writer - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets.
pub type SPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USP` reader - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5.
pub type USP_R = crate::BitReader;
///Field `USP` writer - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5.
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIPGEN` reader - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode.
pub type EIPGEN_R = crate::BitReader;
///Field `EIPGEN` writer - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode.
pub type EIPGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIPG` reader - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG = 0 and IPG = 0 give 104 bit times EIPG = 0 and IPG = 1 give 112 bit times EIPG = 0 and IPG = 2 give 120 bit times .. EIPG = 7 and IPG = 31 give 2144 bit times
pub type EIPG_R = crate::FieldReader;
///Field `EIPG` writer - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG = 0 and IPG = 0 give 104 bit times EIPG = 0 and IPG = 1 give 112 bit times EIPG = 0 and IPG = 2 give 120 bit times .. EIPG = 7 and IPG = 31 give 2144 bit times
pub type EIPG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:13 - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register.
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 16 - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets.
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets.
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5.
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode.
    #[inline(always)]
    pub fn eipgen(&self) -> EIPGEN_R {
        EIPGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:29 - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG = 0 and IPG = 0 give 104 bit times EIPG = 0 and IPG = 1 give 112 bit times EIPG = 0 and IPG = 2 give 120 bit times .. EIPG = 7 and IPG = 31 give 2144 bit times
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
    ///Bits 0:13 - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register.
    #[inline(always)]
    pub fn gpsl(&mut self) -> GPSL_W<'_, MACECRrs> {
        GPSL_W::new(self, 0)
    }
    ///Bit 16 - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets.
    #[inline(always)]
    pub fn dcrcc(&mut self) -> DCRCC_W<'_, MACECRrs> {
        DCRCC_W::new(self, 16)
    }
    ///Bit 17 - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets.
    #[inline(always)]
    pub fn spen(&mut self) -> SPEN_W<'_, MACECRrs> {
        SPEN_W::new(self, 17)
    }
    ///Bit 18 - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5.
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W<'_, MACECRrs> {
        USP_W::new(self, 18)
    }
    ///Bit 24 - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode.
    #[inline(always)]
    pub fn eipgen(&mut self) -> EIPGEN_W<'_, MACECRrs> {
        EIPGEN_W::new(self, 24)
    }
    ///Bits 25:29 - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG = 0 and IPG = 0 give 104 bit times EIPG = 0 and IPG = 1 give 112 bit times EIPG = 0 and IPG = 2 give 120 bit times .. EIPG = 7 and IPG = 31 give 2144 bit times
    #[inline(always)]
    pub fn eipg(&mut self) -> EIPG_W<'_, MACECRrs> {
        EIPG_W::new(self, 25)
    }
}
/**Extended operating mode configuration register

You can [`read`](crate::Reg::read) this register and get [`macecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACECR)*/
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
