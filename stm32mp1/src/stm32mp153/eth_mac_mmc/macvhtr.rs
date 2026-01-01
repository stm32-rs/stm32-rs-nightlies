///Register `MACVHTR` reader
pub type R = crate::R<MACVHTRrs>;
///Register `MACVHTR` writer
pub type W = crate::W<MACVHTRrs>;
///Field `VLHT` reader - VLHT
pub type VLHT_R = crate::FieldReader<u16>;
///Field `VLHT` writer - VLHT
pub type VLHT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - VLHT
    #[inline(always)]
    pub fn vlht(&self) -> VLHT_R {
        VLHT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVHTR")
            .field("vlht", &self.vlht())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - VLHT
    #[inline(always)]
    pub fn vlht(&mut self) -> VLHT_W<'_, MACVHTRrs> {
        VLHT_W::new(self, 0)
    }
}
/**When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[15:8\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of this register are written.

You can [`read`](crate::Reg::read) this register and get [`macvhtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvhtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACVHTR)*/
pub struct MACVHTRrs;
impl crate::RegisterSpec for MACVHTRrs {
    type Ux = u32;
}
///`read()` method returns [`macvhtr::R`](R) reader structure
impl crate::Readable for MACVHTRrs {}
///`write(|w| ..)` method takes [`macvhtr::W`](W) writer structure
impl crate::Writable for MACVHTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACVHTR to value 0
impl crate::Resettable for MACVHTRrs {}
