#[doc = "Register `ETH_MACVHTR` reader"]
pub type R = crate::R<ETH_MACVHTRrs>;
#[doc = "Register `ETH_MACVHTR` writer"]
pub type W = crate::W<ETH_MACVHTRrs>;
#[doc = "Field `VLHT` reader - VLHT"]
pub type VLHT_R = crate::FieldReader<u16>;
#[doc = "Field `VLHT` writer - VLHT"]
pub type VLHT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VLHT"]
    #[inline(always)]
    pub fn vlht(&self) -> VLHT_R {
        VLHT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLHT"]
    #[inline(always)]
    #[must_use]
    pub fn vlht(&mut self) -> VLHT_W<ETH_MACVHTRrs> {
        VLHT_W::new(self, 0)
    }
}
#[doc = "When the ERSVLM bit of ETH_MACHT1R register is set, the 16-bit VLAN Hash Table register is used for group address filtering based on the VLAN tag. For Hash filtering, the content of the 16-bit VLAN tag or 12-bit VLAN ID (based on the ETV bit of ETH_MACVTR register) in the incoming packet is passed through the CRC logic. The upper four bits of the calculated CRC are used to index the contents of the VLAN Hash table. For example, a Hash value of 1000 selects Bit 8 of the VLAN Hash table. The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the VLAN tag or ID (For steps to calculate CRC32, see Section 3.2.8 of IEEE 802.3). Perform bitwise reversal for the value obtained in step 1. Take the upper four bits from the value obtained in step 2. If the VLAN Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[15:8\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of this register are written.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macvhtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macvhtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACVHTRrs;
impl crate::RegisterSpec for ETH_MACVHTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macvhtr::R`](R) reader structure"]
impl crate::Readable for ETH_MACVHTRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macvhtr::W`](W) writer structure"]
impl crate::Writable for ETH_MACVHTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACVHTR to value 0"]
impl crate::Resettable for ETH_MACVHTRrs {
    const RESET_VALUE: u32 = 0;
}
