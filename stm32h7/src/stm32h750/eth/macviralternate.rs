///Register `MACVIRALTERNATE` reader
pub type R = crate::R<MACVIRALTERNATErs>;
///Register `MACVIRALTERNATE` writer
pub type W = crate::W<MACVIRALTERNATErs>;
///Field `VLT` reader - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\[15:13\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\[11:0\]: VLAN Identifier (VID) field of VLAN tag
pub type VLT_R = crate::FieldReader<u16>;
///Field `VLT` writer - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\[15:13\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\[11:0\]: VLAN Identifier (VID) field of VLAN tag
pub type VLT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CSVL` reader - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets.
pub type CSVL_R = crate::BitReader;
///Field `CSVL` writer - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets.
pub type CSVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\[15:13\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\[11:0\]: VLAN Identifier (VID) field of VLAN tag
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 19 - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets.
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVIRALTERNATE")
            .field("vlt", &self.vlt())
            .field("csvl", &self.csvl())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\[15:13\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\[11:0\]: VLAN Identifier (VID) field of VLAN tag
    #[inline(always)]
    pub fn vlt(&mut self) -> VLT_W<'_, MACVIRALTERNATErs> {
        VLT_W::new(self, 0)
    }
    ///Bit 19 - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets.
    #[inline(always)]
    pub fn csvl(&mut self) -> CSVL_W<'_, MACVIRALTERNATErs> {
        CSVL_W::new(self, 19)
    }
}
/**VLAN inclusion register

You can [`read`](crate::Reg::read) this register and get [`macviralternate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macviralternate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#ETH:MACVIRALTERNATE)*/
pub struct MACVIRALTERNATErs;
impl crate::RegisterSpec for MACVIRALTERNATErs {
    type Ux = u32;
}
///`read()` method returns [`macviralternate::R`](R) reader structure
impl crate::Readable for MACVIRALTERNATErs {}
///`write(|w| ..)` method takes [`macviralternate::W`](W) writer structure
impl crate::Writable for MACVIRALTERNATErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACVIRALTERNATE to value 0
impl crate::Resettable for MACVIRALTERNATErs {}
