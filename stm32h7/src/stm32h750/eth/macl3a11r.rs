///Register `MACL3A11R` reader
pub type R = crate::R<MACL3A11Rrs>;
///Register `MACL3A11R` writer
pub type W = crate::W<MACL3A11Rrs>;
///Field `L3A11` reader - Layer 3 Address 1 Field When the L3PEN1 and L3SAM1 bits are set in the (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\] of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset and the L3SAM1 bit is set in the (ETH_MACL3L4C1R), this field contains the value to be matched with the IP Destination Address field in the IPv4 packets.
pub type L3A11_R = crate::FieldReader<u32>;
///Field `L3A11` writer - Layer 3 Address 1 Field When the L3PEN1 and L3SAM1 bits are set in the (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\] of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset and the L3SAM1 bit is set in the (ETH_MACL3L4C1R), this field contains the value to be matched with the IP Destination Address field in the IPv4 packets.
pub type L3A11_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Layer 3 Address 1 Field When the L3PEN1 and L3SAM1 bits are set in the (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\] of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset and the L3SAM1 bit is set in the (ETH_MACL3L4C1R), this field contains the value to be matched with the IP Destination Address field in the IPv4 packets.
    #[inline(always)]
    pub fn l3a11(&self) -> L3A11_R {
        L3A11_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL3A11R")
            .field("l3a11", &self.l3a11())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Layer 3 Address 1 Field When the L3PEN1 and L3SAM1 bits are set in the (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\] of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\] of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset and the L3SAM1 bit is set in the (ETH_MACL3L4C1R), this field contains the value to be matched with the IP Destination Address field in the IPv4 packets.
    #[inline(always)]
    pub fn l3a11(&mut self) -> L3A11_W<'_, MACL3A11Rrs> {
        L3A11_W::new(self, 0)
    }
}
/**Layer3 address 1 filter 1 register

You can [`read`](crate::Reg::read) this register and get [`macl3a11r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a11r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#ETH:MACL3A11R)*/
pub struct MACL3A11Rrs;
impl crate::RegisterSpec for MACL3A11Rrs {
    type Ux = u32;
}
///`read()` method returns [`macl3a11r::R`](R) reader structure
impl crate::Readable for MACL3A11Rrs {}
///`write(|w| ..)` method takes [`macl3a11r::W`](W) writer structure
impl crate::Writable for MACL3A11Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL3A11R to value 0
impl crate::Resettable for MACL3A11Rrs {}
