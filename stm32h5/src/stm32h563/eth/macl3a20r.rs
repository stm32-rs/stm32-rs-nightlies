///Register `MACL3A20R` reader
pub type R = crate::R<MACL3A20Rrs>;
///Register `MACL3A20R` writer
pub type W = crate::W<MACL3A20Rrs>;
///Field `L3A20` reader - Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets. When the L3PEN0 and L3DAM0 bits are set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with Bits\[95:64\] of the IP Destination Address field in the IPv6 packets. When the L3PEN0 bit is reset in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field is not used.
pub type L3A20_R = crate::FieldReader<u32>;
///Field `L3A20` writer - Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets. When the L3PEN0 and L3DAM0 bits are set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with Bits\[95:64\] of the IP Destination Address field in the IPv6 packets. When the L3PEN0 bit is reset in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field is not used.
pub type L3A20_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets. When the L3PEN0 and L3DAM0 bits are set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with Bits\[95:64\] of the IP Destination Address field in the IPv6 packets. When the L3PEN0 bit is reset in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field is not used.
    #[inline(always)]
    pub fn l3a20(&self) -> L3A20_R {
        L3A20_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACL3A20R")
            .field("l3a20", &self.l3a20())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Layer 3 Address 2 Field When the L3PEN0 and L3SAM0 bits are set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with Bits\[95:64\] of the IP Source Address field in the IPv6 packets. When the L3PEN0 and L3DAM0 bits are set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with Bits\[95:64\] of the IP Destination Address field in the IPv6 packets. When the L3PEN0 bit is reset in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field is not used.
    #[inline(always)]
    pub fn l3a20(&mut self) -> L3A20_W<'_, MACL3A20Rrs> {
        L3A20_W::new(self, 0)
    }
}
/**Layer3 Address 2 filter 0 register

You can [`read`](crate::Reg::read) this register and get [`macl3a20r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a20r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:MACL3A20R)*/
pub struct MACL3A20Rrs;
impl crate::RegisterSpec for MACL3A20Rrs {
    type Ux = u32;
}
///`read()` method returns [`macl3a20r::R`](R) reader structure
impl crate::Readable for MACL3A20Rrs {}
///`write(|w| ..)` method takes [`macl3a20r::W`](W) writer structure
impl crate::Writable for MACL3A20Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACL3A20R to value 0
impl crate::Resettable for MACL3A20Rrs {}
