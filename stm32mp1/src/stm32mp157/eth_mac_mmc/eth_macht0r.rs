#[doc = "Register `ETH_MACHT0R` reader"]
pub type R = crate::R<ETH_MACHT0Rrs>;
#[doc = "Register `ETH_MACHT0R` writer"]
pub type W = crate::W<ETH_MACHT0Rrs>;
#[doc = "Field `HT31T0` reader - HT31T0"]
pub type HT31T0_R = crate::FieldReader<u32>;
#[doc = "Field `HT31T0` writer - HT31T0"]
pub type HT31T0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HT31T0"]
    #[inline(always)]
    pub fn ht31t0(&self) -> HT31T0_R {
        HT31T0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HT31T0"]
    #[inline(always)]
    #[must_use]
    pub fn ht31t0(&mut self) -> HT31T0_W<ETH_MACHT0Rrs> {
        HT31T0_W::new(self, 0)
    }
}
#[doc = "The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\\[31:24\\]
(in little-endian mode) or Bits\\[7:0\\]
(in big-endian mode) of the Hash Table Register X registers are written.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macht0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macht0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACHT0Rrs;
impl crate::RegisterSpec for ETH_MACHT0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macht0r::R`](R) reader structure"]
impl crate::Readable for ETH_MACHT0Rrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macht0r::W`](W) writer structure"]
impl crate::Writable for ETH_MACHT0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACHT0R to value 0"]
impl crate::Resettable for ETH_MACHT0Rrs {
    const RESET_VALUE: u32 = 0;
}
