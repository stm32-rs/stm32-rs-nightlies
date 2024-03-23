#[doc = "Register `MACECR` reader"]
pub type R = crate::R<MACECRrs>;
#[doc = "Register `MACECR` writer"]
pub type W = crate::W<MACECRrs>;
#[doc = "Field `GPSL` reader - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register."]
pub type GPSL_R = crate::FieldReader<u16>;
#[doc = "Field `GPSL` writer - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register."]
pub type GPSL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DCRCC` reader - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets."]
pub type DCRCC_R = crate::BitReader;
#[doc = "Field `DCRCC` writer - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets."]
pub type DCRCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEN` reader - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets."]
pub type SPEN_R = crate::BitReader;
#[doc = "Field `SPEN` writer - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets."]
pub type SPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USP` reader - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5."]
pub type USP_R = crate::BitReader;
#[doc = "Field `USP` writer - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5."]
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIPGEN` reader - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode."]
pub type EIPGEN_R = crate::BitReader;
#[doc = "Field `EIPGEN` writer - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode."]
pub type EIPGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIPG` reader - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG�=� 0 and IPG�=�0 give 104 bit times EIPG�=� 0 and IPG�=�1 give 112 bit times EIPG�=� 0 and IPG�=�2 give 120 bit times .. EIPG�=�7 and IPG�=�31 give 2144 bit times"]
pub type EIPG_R = crate::FieldReader;
#[doc = "Field `EIPG` writer - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG�=� 0 and IPG�=�0 give 104 bit times EIPG�=� 0 and IPG�=�1 give 112 bit times EIPG�=� 0 and IPG�=�2 give 120 bit times .. EIPG�=�7 and IPG�=�31 give 2144 bit times"]
pub type EIPG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:13 - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register."]
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets."]
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets."]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5."]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode."]
    #[inline(always)]
    pub fn eipgen(&self) -> EIPGEN_R {
        EIPGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG�=� 0 and IPG�=�0 give 104 bit times EIPG�=� 0 and IPG�=�1 give 112 bit times EIPG�=� 0 and IPG�=�2 give 120 bit times .. EIPG�=�7 and IPG�=�31 give 2144 bit times"]
    #[inline(always)]
    pub fn eipg(&self) -> EIPG_R {
        EIPG_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet. The value programmed in this field must be greater than or equal to 1,518 bytes. Any other programmed value is considered as 1,518 bytes. For VLAN tagged packets, the MAC adds 4 bytes to the programmed value. For double VLAN tagged packets, the MAC adds 8 bytes to the programmed value. The value in this field is applicable when the GPSLCE bit is set in ETH_MACCR register."]
    #[inline(always)]
    #[must_use]
    pub fn gpsl(&mut self) -> GPSL_W<MACECRrs> {
        GPSL_W::new(self, 0)
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets. When this bit is reset, the MAC receiver always checks the CRC field in the received packets."]
    #[inline(always)]
    #[must_use]
    pub fn dcrcc(&mut self) -> DCRCC_W<MACECRrs> {
        DCRCC_W::new(self, 16)
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status. The MAC discards the Slow Protocol packets with invalid subtypes. When this bit is reset, the MAC forwards all error-free Slow Protocol packets to the application. The MAC considers such packets as normal Type packets."]
    #[inline(always)]
    #[must_use]
    pub fn spen(&mut self) -> SPEN_W<MACECRrs> {
        SPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address 0 high register (ETH_MACA0HR) and MAC Address 0 low register MAC Address x low register (ETH_MACAxLR). The MAC also detects the Slow Protocol packets with the Slow Protocols multicast address (01-80-C2-00-00-02). When this bit is reset, the MAC detects only Slow Protocol packets with the Slow Protocol multicast address specified in the IEEE 802.3-2008, Section 5."]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<MACECRrs> {
        USP_W::new(self, 18)
    }
    #[doc = "Bit 24 - Extended Inter-Packet Gap Enable When this bit is set, the MAC interprets EIPG field and IPG field in Operating mode configuration register (ETH_MACCR) together as minimum IPG greater than 96 bit times in steps of 8 bit times. When this bit is reset, the MAC ignores EIPG field and interprets IPG field in Operating mode configuration register (ETH_MACCR) as minimum IPG less than or equal to 96 bit times in steps of 8 bit times. Note: The extended Inter-Packet Gap feature must be enabled when operating in Full-duplex mode only. There may be undesirable effects on back-pressure function and frame transmission if it is enabled in Half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn eipgen(&mut self) -> EIPGEN_W<MACECRrs> {
        EIPGEN_W::new(self, 24)
    }
    #[doc = "Bits 25:29 - Extended Inter-Packet Gap The value in this field is applicable when the EIPGEN bit is set. This field (as Most Significant bits) along with IPG field in Operating mode configuration register (ETH_MACCR), gives the minimum IPG greater than 96 bit times in steps of 8 bit times. For example: EIPG�=� 0 and IPG�=�0 give 104 bit times EIPG�=� 0 and IPG�=�1 give 112 bit times EIPG�=� 0 and IPG�=�2 give 120 bit times .. EIPG�=�7 and IPG�=�31 give 2144 bit times"]
    #[inline(always)]
    #[must_use]
    pub fn eipg(&mut self) -> EIPG_W<MACECRrs> {
        EIPG_W::new(self, 25)
    }
}
#[doc = "Extended operating mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACECRrs;
impl crate::RegisterSpec for MACECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macecr::R`](R) reader structure"]
impl crate::Readable for MACECRrs {}
#[doc = "`write(|w| ..)` method takes [`macecr::W`](W) writer structure"]
impl crate::Writable for MACECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACECR to value 0"]
impl crate::Resettable for MACECRrs {
    const RESET_VALUE: u32 = 0;
}
