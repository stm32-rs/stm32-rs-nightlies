#[doc = "Register `MACVTR` reader"]
pub type R = crate::R<MACVTRrs>;
#[doc = "Register `MACVTR` writer"]
pub type W = crate::W<MACVTRrs>;
#[doc = "Field `VL` reader - VLAN Tag Identifier for Receive Packets This field contains the 802.1Q VLAN tag to identify the VLAN packets. This VLAN tag identifier is compared to the 15th and 16th bytes of the packets being received for VLAN packets. The following list describes the bits of this field: Bits\\[15:13\\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\\[11:0\\]: VLAN Identifier (VID) field of VLAN tag When the ETV bit is set, only the VID is used for comparison. If this field (\\[11:0\\]
if ETV is set) is all zeros, the MAC does not check the 15th and 16th bytes for VLAN tag comparison and declares all packets with Type field value of 0x8100 or 0x88a8 as VLAN packets."]
pub type VL_R = crate::FieldReader<u16>;
#[doc = "Field `VL` writer - VLAN Tag Identifier for Receive Packets This field contains the 802.1Q VLAN tag to identify the VLAN packets. This VLAN tag identifier is compared to the 15th and 16th bytes of the packets being received for VLAN packets. The following list describes the bits of this field: Bits\\[15:13\\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\\[11:0\\]: VLAN Identifier (VID) field of VLAN tag When the ETV bit is set, only the VID is used for comparison. If this field (\\[11:0\\]
if ETV is set) is all zeros, the MAC does not check the 15th and 16th bytes for VLAN tag comparison and declares all packets with Type field value of 0x8100 or 0x88a8 as VLAN packets."]
pub type VL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier is used for comparing and filtering instead of the complete 16-bit VLAN tag. Bits\\[11:0\\]
of VLAN tag are compared with the corresponding field in the received VLAN-tagged packet. Similarly, when enabled, only 12 bits of the VLAN tag in the received packet are used for Hash-based VLAN filtering. When this bit is reset, all 16 bits of the 15th and 16th bytes of the received VLAN packet are used for comparison and VLAN Hash filtering."]
pub type ETV_R = crate::BitReader;
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier is used for comparing and filtering instead of the complete 16-bit VLAN tag. Bits\\[11:0\\]
of VLAN tag are compared with the corresponding field in the received VLAN-tagged packet. Similarly, when enabled, only 12 bits of the VLAN tag in the received packet are used for Hash-based VLAN filtering. When this bit is reset, all 16 bits of the 15th and 16th bytes of the received VLAN packet are used for comparison and VLAN Hash filtering."]
pub type ETV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable When this bit is set, this bit enables the VLAN Tag inverse matching. The packets without matching VLAN Tag are marked as matched. When reset, this bit enables the VLAN Tag perfect matching. The packets with matched VLAN Tag are marked as matched."]
pub type VTIM_R = crate::BitReader;
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable When this bit is set, this bit enables the VLAN Tag inverse matching. The packets without matching VLAN Tag are marked as matched. When reset, this bit enables the VLAN Tag perfect matching. The packets with matched VLAN Tag are marked as matched."]
pub type VTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESVL` reader - Enable S-VLAN When this bit is set, the MAC transmitter and receiver consider the S-VLAN packets (Type = 0x88A8) as valid VLAN tagged packets."]
pub type ESVL_R = crate::BitReader;
#[doc = "Field `ESVL` writer - Enable S-VLAN When this bit is set, the MAC transmitter and receiver consider the S-VLAN packets (Type = 0x88A8) as valid VLAN tagged packets."]
pub type ESVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERSVLM` reader - Enable Receive S-VLAN Match When this bit is set, the MAC receiver enables filtering or matching for S-VLAN (Type = 0x88A8) packets. When this bit is reset, the MAC receiver enables filtering or matching for C-VLAN (Type = 0x8100) packets. The ERIVLT bit determines the VLAN tag position considered for filtering or matching."]
pub type ERSVLM_R = crate::BitReader;
#[doc = "Field `ERSVLM` writer - Enable Receive S-VLAN Match When this bit is set, the MAC receiver enables filtering or matching for S-VLAN (Type = 0x88A8) packets. When this bit is reset, the MAC receiver enables filtering or matching for C-VLAN (Type = 0x8100) packets. The ERIVLT bit determines the VLAN tag position considered for filtering or matching."]
pub type ERSVLM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOVLTC` reader - Disable VLAN Type Check When this bit is set, the MAC does not check whether the VLAN Tag specified by the ERIVLT bit is of type S-VLAN or C-VLAN. When this bit is reset, the MAC filters or matches the VLAN Tag specified by the ERIVLT bit only when VLAN Tag type is similar to the one specified by the ERSVLM bit."]
pub type DOVLTC_R = crate::BitReader;
#[doc = "Field `DOVLTC` writer - Disable VLAN Type Check When this bit is set, the MAC does not check whether the VLAN Tag specified by the ERIVLT bit is of type S-VLAN or C-VLAN. When this bit is reset, the MAC filters or matches the VLAN Tag specified by the ERIVLT bit only when VLAN Tag type is similar to the one specified by the ERSVLM bit."]
pub type DOVLTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVLS` reader - Enable VLAN Tag Stripping on Receive This field indicates the stripping operation on the outer VLAN Tag in received packet:"]
pub type EVLS_R = crate::FieldReader;
#[doc = "Field `EVLS` writer - Enable VLAN Tag Stripping on Receive This field indicates the stripping operation on the outer VLAN Tag in received packet:"]
pub type EVLS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EVLRXS` reader - Enable VLAN Tag in Rx status When this bit is set, MAC provides the outer VLAN Tag in the Rx status. When this bit is reset, the MAC does not provide the outer VLAN Tag in Rx status."]
pub type EVLRXS_R = crate::BitReader;
#[doc = "Field `EVLRXS` writer - Enable VLAN Tag in Rx status When this bit is set, MAC provides the outer VLAN Tag in the Rx status. When this bit is reset, the MAC does not provide the outer VLAN Tag in Rx status."]
pub type EVLRXS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTHM` reader - VLAN Tag Hash Table Match Enable When this bit is set, the most significant four bits of CRC of VLAN Tag are used to index the content of the ETH_MACVLANHTR register. A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the packet matched the VLAN Hash table. When the ETV bit is set, the CRC of the 12-bit VLAN Identifier (VID) is used for comparison. When the ETV bit is reset, the CRC of the 16-bit VLAN tag is used for comparison. When this bit is reset, the VLAN Hash Match operation is not performed."]
pub type VTHM_R = crate::BitReader;
#[doc = "Field `VTHM` writer - VLAN Tag Hash Table Match Enable When this bit is set, the most significant four bits of CRC of VLAN Tag are used to index the content of the ETH_MACVLANHTR register. A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the packet matched the VLAN Hash table. When the ETV bit is set, the CRC of the 12-bit VLAN Identifier (VID) is used for comparison. When the ETV bit is reset, the CRC of the 16-bit VLAN tag is used for comparison. When this bit is reset, the VLAN Hash Match operation is not performed."]
pub type VTHM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDVLP` reader - Enable Double VLAN Processing When this bit is set, the MAC enables processing of up to two VLAN Tags on Tx and Rx (if present). When this bit is reset, the MAC enables processing of up to one VLAN Tag on Tx and Rx (if present)."]
pub type EDVLP_R = crate::BitReader;
#[doc = "Field `EDVLP` writer - Enable Double VLAN Processing When this bit is set, the MAC enables processing of up to two VLAN Tags on Tx and Rx (if present). When this bit is reset, the MAC enables processing of up to one VLAN Tag on Tx and Rx (if present)."]
pub type EDVLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIVLT` reader - Enable Inner VLAN Tag When this bit and the EDVLP field are set, the MAC receiver enables operation on the inner VLAN Tag (if present). When this bit is reset, the MAC receiver enables operation on the outer VLAN Tag (if present). The ERSVLM bit determines which VLAN type is enabled for filtering or matching.The ERSVLM bit and DOVLTC bit determines which VLAN type is enabled for filtering."]
pub type ERIVLT_R = crate::BitReader;
#[doc = "Field `ERIVLT` writer - Enable Inner VLAN Tag When this bit and the EDVLP field are set, the MAC receiver enables operation on the inner VLAN Tag (if present). When this bit is reset, the MAC receiver enables operation on the outer VLAN Tag (if present). The ERSVLM bit determines which VLAN type is enabled for filtering or matching.The ERSVLM bit and DOVLTC bit determines which VLAN type is enabled for filtering."]
pub type ERIVLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIVLS` reader - Enable Inner VLAN Tag Stripping on Receive This field indicates the stripping operation on inner VLAN Tag in received packet:"]
pub type EIVLS_R = crate::FieldReader;
#[doc = "Field `EIVLS` writer - Enable Inner VLAN Tag Stripping on Receive This field indicates the stripping operation on inner VLAN Tag in received packet:"]
pub type EIVLS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EIVLRXS` reader - Enable Inner VLAN Tag in Rx Status When this bit is set, the MAC provides the inner VLAN Tag in the Rx status. When this bit is reset, the MAC does not provide the inner VLAN Tag in Rx status."]
pub type EIVLRXS_R = crate::BitReader;
#[doc = "Field `EIVLRXS` writer - Enable Inner VLAN Tag in Rx Status When this bit is set, the MAC provides the inner VLAN Tag in the Rx status. When this bit is reset, the MAC does not provide the inner VLAN Tag in Rx status."]
pub type EIVLRXS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Packets This field contains the 802.1Q VLAN tag to identify the VLAN packets. This VLAN tag identifier is compared to the 15th and 16th bytes of the packets being received for VLAN packets. The following list describes the bits of this field: Bits\\[15:13\\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\\[11:0\\]: VLAN Identifier (VID) field of VLAN tag When the ETV bit is set, only the VID is used for comparison. If this field (\\[11:0\\]
if ETV is set) is all zeros, the MAC does not check the 15th and 16th bytes for VLAN tag comparison and declares all packets with Type field value of 0x8100 or 0x88a8 as VLAN packets."]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier is used for comparing and filtering instead of the complete 16-bit VLAN tag. Bits\\[11:0\\]
of VLAN tag are compared with the corresponding field in the received VLAN-tagged packet. Similarly, when enabled, only 12 bits of the VLAN tag in the received packet are used for Hash-based VLAN filtering. When this bit is reset, all 16 bits of the 15th and 16th bytes of the received VLAN packet are used for comparison and VLAN Hash filtering."]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable When this bit is set, this bit enables the VLAN Tag inverse matching. The packets without matching VLAN Tag are marked as matched. When reset, this bit enables the VLAN Tag perfect matching. The packets with matched VLAN Tag are marked as matched."]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN When this bit is set, the MAC transmitter and receiver consider the S-VLAN packets (Type = 0x88A8) as valid VLAN tagged packets."]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Receive S-VLAN Match When this bit is set, the MAC receiver enables filtering or matching for S-VLAN (Type = 0x88A8) packets. When this bit is reset, the MAC receiver enables filtering or matching for C-VLAN (Type = 0x8100) packets. The ERIVLT bit determines the VLAN tag position considered for filtering or matching."]
    #[inline(always)]
    pub fn ersvlm(&self) -> ERSVLM_R {
        ERSVLM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Disable VLAN Type Check When this bit is set, the MAC does not check whether the VLAN Tag specified by the ERIVLT bit is of type S-VLAN or C-VLAN. When this bit is reset, the MAC filters or matches the VLAN Tag specified by the ERIVLT bit only when VLAN Tag type is similar to the one specified by the ERSVLM bit."]
    #[inline(always)]
    pub fn dovltc(&self) -> DOVLTC_R {
        DOVLTC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Enable VLAN Tag Stripping on Receive This field indicates the stripping operation on the outer VLAN Tag in received packet:"]
    #[inline(always)]
    pub fn evls(&self) -> EVLS_R {
        EVLS_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - Enable VLAN Tag in Rx status When this bit is set, MAC provides the outer VLAN Tag in the Rx status. When this bit is reset, the MAC does not provide the outer VLAN Tag in Rx status."]
    #[inline(always)]
    pub fn evlrxs(&self) -> EVLRXS_R {
        EVLRXS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VLAN Tag Hash Table Match Enable When this bit is set, the most significant four bits of CRC of VLAN Tag are used to index the content of the ETH_MACVLANHTR register. A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the packet matched the VLAN Hash table. When the ETV bit is set, the CRC of the 12-bit VLAN Identifier (VID) is used for comparison. When the ETV bit is reset, the CRC of the 16-bit VLAN tag is used for comparison. When this bit is reset, the VLAN Hash Match operation is not performed."]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Double VLAN Processing When this bit is set, the MAC enables processing of up to two VLAN Tags on Tx and Rx (if present). When this bit is reset, the MAC enables processing of up to one VLAN Tag on Tx and Rx (if present)."]
    #[inline(always)]
    pub fn edvlp(&self) -> EDVLP_R {
        EDVLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Inner VLAN Tag When this bit and the EDVLP field are set, the MAC receiver enables operation on the inner VLAN Tag (if present). When this bit is reset, the MAC receiver enables operation on the outer VLAN Tag (if present). The ERSVLM bit determines which VLAN type is enabled for filtering or matching.The ERSVLM bit and DOVLTC bit determines which VLAN type is enabled for filtering."]
    #[inline(always)]
    pub fn erivlt(&self) -> ERIVLT_R {
        ERIVLT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive This field indicates the stripping operation on inner VLAN Tag in received packet:"]
    #[inline(always)]
    pub fn eivls(&self) -> EIVLS_R {
        EIVLS_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Enable Inner VLAN Tag in Rx Status When this bit is set, the MAC provides the inner VLAN Tag in the Rx status. When this bit is reset, the MAC does not provide the inner VLAN Tag in Rx status."]
    #[inline(always)]
    pub fn eivlrxs(&self) -> EIVLRXS_R {
        EIVLRXS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Packets This field contains the 802.1Q VLAN tag to identify the VLAN packets. This VLAN tag identifier is compared to the 15th and 16th bytes of the packets being received for VLAN packets. The following list describes the bits of this field: Bits\\[15:13\\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\\[11:0\\]: VLAN Identifier (VID) field of VLAN tag When the ETV bit is set, only the VID is used for comparison. If this field (\\[11:0\\]
if ETV is set) is all zeros, the MAC does not check the 15th and 16th bytes for VLAN tag comparison and declares all packets with Type field value of 0x8100 or 0x88a8 as VLAN packets."]
    #[inline(always)]
    #[must_use]
    pub fn vl(&mut self) -> VL_W<MACVTRrs> {
        VL_W::new(self, 0)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier is used for comparing and filtering instead of the complete 16-bit VLAN tag. Bits\\[11:0\\]
of VLAN tag are compared with the corresponding field in the received VLAN-tagged packet. Similarly, when enabled, only 12 bits of the VLAN tag in the received packet are used for Hash-based VLAN filtering. When this bit is reset, all 16 bits of the 15th and 16th bytes of the received VLAN packet are used for comparison and VLAN Hash filtering."]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> ETV_W<MACVTRrs> {
        ETV_W::new(self, 16)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable When this bit is set, this bit enables the VLAN Tag inverse matching. The packets without matching VLAN Tag are marked as matched. When reset, this bit enables the VLAN Tag perfect matching. The packets with matched VLAN Tag are marked as matched."]
    #[inline(always)]
    #[must_use]
    pub fn vtim(&mut self) -> VTIM_W<MACVTRrs> {
        VTIM_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable S-VLAN When this bit is set, the MAC transmitter and receiver consider the S-VLAN packets (Type = 0x88A8) as valid VLAN tagged packets."]
    #[inline(always)]
    #[must_use]
    pub fn esvl(&mut self) -> ESVL_W<MACVTRrs> {
        ESVL_W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Receive S-VLAN Match When this bit is set, the MAC receiver enables filtering or matching for S-VLAN (Type = 0x88A8) packets. When this bit is reset, the MAC receiver enables filtering or matching for C-VLAN (Type = 0x8100) packets. The ERIVLT bit determines the VLAN tag position considered for filtering or matching."]
    #[inline(always)]
    #[must_use]
    pub fn ersvlm(&mut self) -> ERSVLM_W<MACVTRrs> {
        ERSVLM_W::new(self, 19)
    }
    #[doc = "Bit 20 - Disable VLAN Type Check When this bit is set, the MAC does not check whether the VLAN Tag specified by the ERIVLT bit is of type S-VLAN or C-VLAN. When this bit is reset, the MAC filters or matches the VLAN Tag specified by the ERIVLT bit only when VLAN Tag type is similar to the one specified by the ERSVLM bit."]
    #[inline(always)]
    #[must_use]
    pub fn dovltc(&mut self) -> DOVLTC_W<MACVTRrs> {
        DOVLTC_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Enable VLAN Tag Stripping on Receive This field indicates the stripping operation on the outer VLAN Tag in received packet:"]
    #[inline(always)]
    #[must_use]
    pub fn evls(&mut self) -> EVLS_W<MACVTRrs> {
        EVLS_W::new(self, 21)
    }
    #[doc = "Bit 24 - Enable VLAN Tag in Rx status When this bit is set, MAC provides the outer VLAN Tag in the Rx status. When this bit is reset, the MAC does not provide the outer VLAN Tag in Rx status."]
    #[inline(always)]
    #[must_use]
    pub fn evlrxs(&mut self) -> EVLRXS_W<MACVTRrs> {
        EVLRXS_W::new(self, 24)
    }
    #[doc = "Bit 25 - VLAN Tag Hash Table Match Enable When this bit is set, the most significant four bits of CRC of VLAN Tag are used to index the content of the ETH_MACVLANHTR register. A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the packet matched the VLAN Hash table. When the ETV bit is set, the CRC of the 12-bit VLAN Identifier (VID) is used for comparison. When the ETV bit is reset, the CRC of the 16-bit VLAN tag is used for comparison. When this bit is reset, the VLAN Hash Match operation is not performed."]
    #[inline(always)]
    #[must_use]
    pub fn vthm(&mut self) -> VTHM_W<MACVTRrs> {
        VTHM_W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Double VLAN Processing When this bit is set, the MAC enables processing of up to two VLAN Tags on Tx and Rx (if present). When this bit is reset, the MAC enables processing of up to one VLAN Tag on Tx and Rx (if present)."]
    #[inline(always)]
    #[must_use]
    pub fn edvlp(&mut self) -> EDVLP_W<MACVTRrs> {
        EDVLP_W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Inner VLAN Tag When this bit and the EDVLP field are set, the MAC receiver enables operation on the inner VLAN Tag (if present). When this bit is reset, the MAC receiver enables operation on the outer VLAN Tag (if present). The ERSVLM bit determines which VLAN type is enabled for filtering or matching.The ERSVLM bit and DOVLTC bit determines which VLAN type is enabled for filtering."]
    #[inline(always)]
    #[must_use]
    pub fn erivlt(&mut self) -> ERIVLT_W<MACVTRrs> {
        ERIVLT_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive This field indicates the stripping operation on inner VLAN Tag in received packet:"]
    #[inline(always)]
    #[must_use]
    pub fn eivls(&mut self) -> EIVLS_W<MACVTRrs> {
        EIVLS_W::new(self, 28)
    }
    #[doc = "Bit 31 - Enable Inner VLAN Tag in Rx Status When this bit is set, the MAC provides the inner VLAN Tag in the Rx status. When this bit is reset, the MAC does not provide the inner VLAN Tag in Rx status."]
    #[inline(always)]
    #[must_use]
    pub fn eivlrxs(&mut self) -> EIVLRXS_W<MACVTRrs> {
        EIVLRXS_W::new(self, 31)
    }
}
#[doc = "VLAN tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVTRrs;
impl crate::RegisterSpec for MACVTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvtr::R`](R) reader structure"]
impl crate::Readable for MACVTRrs {}
#[doc = "`write(|w| ..)` method takes [`macvtr::W`](W) writer structure"]
impl crate::Writable for MACVTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACVTR to value 0"]
impl crate::Resettable for MACVTRrs {
    const RESET_VALUE: u32 = 0;
}
