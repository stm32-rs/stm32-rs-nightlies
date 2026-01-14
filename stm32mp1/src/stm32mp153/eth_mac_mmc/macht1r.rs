///Register `MACHT1R` reader
pub type R = crate::R<MACHT1Rrs>;
///Register `MACHT1R` writer
pub type W = crate::W<MACHT1Rrs>;
///Field `HT63T32` reader - HT63T32
pub type HT63T32_R = crate::FieldReader<u32>;
///Field `HT63T32` writer - HT63T32
pub type HT63T32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - HT63T32
    #[inline(always)]
    pub fn ht63t32(&self) -> HT63T32_R {
        HT63T32_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHT1R")
            .field("ht63t32", &self.ht63t32())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HT63T32
    #[inline(always)]
    pub fn ht63t32(&mut self) -> HT63T32_W<'_, MACHT1Rrs> {
        HT63T32_W::new(self, 0)
    }
}
/**The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the Hash Table Register X registers are written.

You can [`read`](crate::Reg::read) this register and get [`macht1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACHT1R)*/
pub struct MACHT1Rrs;
impl crate::RegisterSpec for MACHT1Rrs {
    type Ux = u32;
}
///`read()` method returns [`macht1r::R`](R) reader structure
impl crate::Readable for MACHT1Rrs {}
///`write(|w| ..)` method takes [`macht1r::W`](W) writer structure
impl crate::Writable for MACHT1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACHT1R to value 0
impl crate::Resettable for MACHT1Rrs {}
