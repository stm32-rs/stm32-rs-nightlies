///Register `MACHT0R` reader
pub type R = crate::R<MACHT0Rrs>;
///Register `MACHT0R` writer
pub type W = crate::W<MACHT0Rrs>;
///Field `HT31T0` reader - HT31T0
pub type HT31T0_R = crate::FieldReader<u32>;
///Field `HT31T0` writer - HT31T0
pub type HT31T0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - HT31T0
    #[inline(always)]
    pub fn ht31t0(&self) -> HT31T0_R {
        HT31T0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHT0R")
            .field("ht31t0", &self.ht31t0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HT31T0
    #[inline(always)]
    pub fn ht31t0(&mut self) -> HT31T0_W<'_, MACHT0Rrs> {
        HT31T0_W::new(self, 0)
    }
}
/**The Hash Table Register 0 contains the first 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\] (in little-endian mode) or Bits\[7:0\] (in big-endian mode) of the Hash Table Register X registers are written.

You can [`read`](crate::Reg::read) this register and get [`macht0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACHT0R)*/
pub struct MACHT0Rrs;
impl crate::RegisterSpec for MACHT0Rrs {
    type Ux = u32;
}
///`read()` method returns [`macht0r::R`](R) reader structure
impl crate::Readable for MACHT0Rrs {}
///`write(|w| ..)` method takes [`macht0r::W`](W) writer structure
impl crate::Writable for MACHT0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACHT0R to value 0
impl crate::Resettable for MACHT0Rrs {}
