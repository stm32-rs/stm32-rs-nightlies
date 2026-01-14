///Register `MACL3A10R` reader
pub type R = crate::R<MACL3A10Rrs>;
///Register `MACL3A10R` writer
pub type W = crate::W<MACL3A10Rrs>;
///Field `L3A10` reader - L3A10
pub type L3A10_R = crate::FieldReader<u32>;
///Field `L3A10` writer - L3A10
pub type L3A10_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - L3A10
    #[inline(always)]
    pub fn l3a10(&self) -> L3A10_R {
        L3A10_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL3A10R")
            .field("l3a10", &self.l3a10())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - L3A10
    #[inline(always)]
    pub fn l3a10(&mut self) -> L3A10_W<'_, MACL3A10Rrs> {
        L3A10_W::new(self, 0)
    }
}
/**For IPv4 packets, the Layer 3 Address 1 Register 0 register contains the 32-bit IP Destination Address field. For IPv6 packets, it contains Bits\[63:32\] of the 128-bit IP Source Address or Destination Address field.

You can [`read`](crate::Reg::read) this register and get [`macl3a10r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a10r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACL3A10R)*/
pub struct MACL3A10Rrs;
impl crate::RegisterSpec for MACL3A10Rrs {
    type Ux = u32;
}
///`read()` method returns [`macl3a10r::R`](R) reader structure
impl crate::Readable for MACL3A10Rrs {}
///`write(|w| ..)` method takes [`macl3a10r::W`](W) writer structure
impl crate::Writable for MACL3A10Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL3A10R to value 0
impl crate::Resettable for MACL3A10Rrs {}
