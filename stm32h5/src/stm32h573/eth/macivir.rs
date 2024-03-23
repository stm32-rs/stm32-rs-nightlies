#[doc = "Register `MACIVIR` reader"]
pub type R = crate::R<MACIVIRrs>;
#[doc = "Register `MACIVIR` writer"]
pub type W = crate::W<MACIVIRrs>;
#[doc = "Field `VLT` reader - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\\[15:13\\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\\[11:0\\]: VLAN Identifier (VID) field of VLAN tag"]
pub type VLT_R = crate::FieldReader<u16>;
#[doc = "Field `VLT` writer - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\\[15:13\\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\\[11:0\\]: VLAN Identifier (VID) field of VLAN tag"]
pub type VLT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLC` reader - VLAN Tag Control in Transmit Packets The MAC removes the VLAN type (bytes 17 and 18) and VLAN tag (bytes 19 and 20) of all transmitted packets with VLAN tags. The MAC inserts VLT in bytes 19 and 20 of the packet after inserting the Type value (0x8100 or 0x88a8) in bytes 17 and 18. This operation is performed on all transmitted packets, irrespective of whether they already have a VLAN tag. The MAC replaces VLT in bytes 19 and 20 of all VLAN-type transmitted packets (Bytes 17 and 18 are 0x8100 or 0x88a8). Note: Changes to this field take effect only on the start of a packet. If you write to this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value."]
pub type VLC_R = crate::FieldReader;
#[doc = "Field `VLC` writer - VLAN Tag Control in Transmit Packets The MAC removes the VLAN type (bytes 17 and 18) and VLAN tag (bytes 19 and 20) of all transmitted packets with VLAN tags. The MAC inserts VLT in bytes 19 and 20 of the packet after inserting the Type value (0x8100 or 0x88a8) in bytes 17 and 18. This operation is performed on all transmitted packets, irrespective of whether they already have a VLAN tag. The MAC replaces VLT in bytes 19 and 20 of all VLAN-type transmitted packets (Bytes 17 and 18 are 0x8100 or 0x88a8). Note: Changes to this field take effect only on the start of a packet. If you write to this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value."]
pub type VLC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VLP` reader - VLAN Priority Control When this bit is set, the VLC field is used for VLAN deletion, insertion, or replacement. When this bit is reset, the VLC field is ignored."]
pub type VLP_R = crate::BitReader;
#[doc = "Field `VLP` writer - VLAN Priority Control When this bit is set, the VLC field is used for VLAN deletion, insertion, or replacement. When this bit is reset, the VLC field is ignored."]
pub type VLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSVL` reader - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets."]
pub type CSVL_R = crate::BitReader;
#[doc = "Field `CSVL` writer - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets."]
pub type CSVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLTI` reader - VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from the Tx descriptor"]
pub type VLTI_R = crate::BitReader;
#[doc = "Field `VLTI` writer - VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from the Tx descriptor"]
pub type VLTI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\\[15:13\\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\\[11:0\\]: VLAN Identifier (VID) field of VLAN tag"]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Packets The MAC removes the VLAN type (bytes 17 and 18) and VLAN tag (bytes 19 and 20) of all transmitted packets with VLAN tags. The MAC inserts VLT in bytes 19 and 20 of the packet after inserting the Type value (0x8100 or 0x88a8) in bytes 17 and 18. This operation is performed on all transmitted packets, irrespective of whether they already have a VLAN tag. The MAC replaces VLT in bytes 19 and 20 of all VLAN-type transmitted packets (Bytes 17 and 18 are 0x8100 or 0x88a8). Note: Changes to this field take effect only on the start of a packet. If you write to this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value."]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - VLAN Priority Control When this bit is set, the VLC field is used for VLAN deletion, insertion, or replacement. When this bit is reset, the VLC field is ignored."]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets."]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from the Tx descriptor"]
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\\[15:13\\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\\[11:0\\]: VLAN Identifier (VID) field of VLAN tag"]
    #[inline(always)]
    #[must_use]
    pub fn vlt(&mut self) -> VLT_W<MACIVIRrs> {
        VLT_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Packets The MAC removes the VLAN type (bytes 17 and 18) and VLAN tag (bytes 19 and 20) of all transmitted packets with VLAN tags. The MAC inserts VLT in bytes 19 and 20 of the packet after inserting the Type value (0x8100 or 0x88a8) in bytes 17 and 18. This operation is performed on all transmitted packets, irrespective of whether they already have a VLAN tag. The MAC replaces VLT in bytes 19 and 20 of all VLAN-type transmitted packets (Bytes 17 and 18 are 0x8100 or 0x88a8). Note: Changes to this field take effect only on the start of a packet. If you write to this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value."]
    #[inline(always)]
    #[must_use]
    pub fn vlc(&mut self) -> VLC_W<MACIVIRrs> {
        VLC_W::new(self, 16)
    }
    #[doc = "Bit 18 - VLAN Priority Control When this bit is set, the VLC field is used for VLAN deletion, insertion, or replacement. When this bit is reset, the VLC field is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn vlp(&mut self) -> VLP_W<MACIVIRrs> {
        VLP_W::new(self, 18)
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets."]
    #[inline(always)]
    #[must_use]
    pub fn csvl(&mut self) -> CSVL_W<MACIVIRrs> {
        CSVL_W::new(self, 19)
    }
    #[doc = "Bit 20 - VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from the Tx descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn vlti(&mut self) -> VLTI_W<MACIVIRrs> {
        VLTI_W::new(self, 20)
    }
}
#[doc = "Inner VLAN inclusion register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macivir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macivir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACIVIRrs;
impl crate::RegisterSpec for MACIVIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macivir::R`](R) reader structure"]
impl crate::Readable for MACIVIRrs {}
#[doc = "`write(|w| ..)` method takes [`macivir::W`](W) writer structure"]
impl crate::Writable for MACIVIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACIVIR to value 0"]
impl crate::Resettable for MACIVIRrs {
    const RESET_VALUE: u32 = 0;
}
