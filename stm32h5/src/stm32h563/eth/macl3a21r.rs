///Register `MACL3A21R` reader
pub type R = crate::R<MACL3A21Rrs>;
///Register `MACL3A21R` writer
pub type W = crate::W<MACL3A21Rrs>;
///Field `L3A21` reader - Layer 3 Address 2 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[95:64\] of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field is not used.
pub type L3A21_R = crate::FieldReader<u32>;
///Field `L3A21` writer - Layer 3 Address 2 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[95:64\] of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field is not used.
pub type L3A21_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Layer 3 Address 2 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[95:64\] of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field is not used.
    #[inline(always)]
    pub fn l3a21(&self) -> L3A21_R {
        L3A21_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL3A21R")
            .field("l3a21", &self.l3a21())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Layer 3 Address 2 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[95:64\] of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field is not used.
    #[inline(always)]
    pub fn l3a21(&mut self) -> L3A21_W<'_, MACL3A21Rrs> {
        L3A21_W::new(self, 0)
    }
}
/**Layer3 address 2 filter 1 Register

You can [`read`](crate::Reg::read) this register and get [`macl3a21r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a21r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:MACL3A21R)*/
pub struct MACL3A21Rrs;
impl crate::RegisterSpec for MACL3A21Rrs {
    type Ux = u32;
}
///`read()` method returns [`macl3a21r::R`](R) reader structure
impl crate::Readable for MACL3A21Rrs {}
///`write(|w| ..)` method takes [`macl3a21r::W`](W) writer structure
impl crate::Writable for MACL3A21Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL3A21R to value 0
impl crate::Resettable for MACL3A21Rrs {}
