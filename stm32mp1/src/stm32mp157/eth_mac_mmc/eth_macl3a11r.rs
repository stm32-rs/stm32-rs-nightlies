///Register `ETH_MACL3A11R` reader
pub type R = crate::R<ETH_MACL3A11Rrs>;
///Register `ETH_MACL3A11R` writer
pub type W = crate::W<ETH_MACL3A11Rrs>;
///Field `L3A11` reader - L3A11
pub type L3A11_R = crate::FieldReader<u32>;
///Field `L3A11` writer - L3A11
pub type L3A11_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - L3A11
    #[inline(always)]
    pub fn l3a11(&self) -> L3A11_R {
        L3A11_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACL3A11R")
            .field("l3a11", &self.l3a11())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - L3A11
    #[inline(always)]
    #[must_use]
    pub fn l3a11(&mut self) -> L3A11_W<ETH_MACL3A11Rrs> {
        L3A11_W::new(self, 0)
    }
}
/**For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\]
of the 128-bit IP Source Address or Destination Address field.

You can [`read`](crate::Reg::read) this register and get [`eth_macl3a11r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_macl3a11r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:ETH_MACL3A11R)*/
pub struct ETH_MACL3A11Rrs;
impl crate::RegisterSpec for ETH_MACL3A11Rrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macl3a11r::R`](R) reader structure
impl crate::Readable for ETH_MACL3A11Rrs {}
///`write(|w| ..)` method takes [`eth_macl3a11r::W`](W) writer structure
impl crate::Writable for ETH_MACL3A11Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACL3A11R to value 0
impl crate::Resettable for ETH_MACL3A11Rrs {
    const RESET_VALUE: u32 = 0;
}
