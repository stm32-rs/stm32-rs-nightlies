#[doc = "Register `MACL3A31R` reader"]
pub type R = crate::R<MACL3A31Rrs>;
#[doc = "Register `MACL3A31R` writer"]
pub type W = crate::W<MACL3A31Rrs>;
#[doc = "Field `L3A31` reader - Layer 3 Address 3 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\\[127:96\\]
of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\\[127:96\\]
of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field is not used."]
pub type L3A31_R = crate::FieldReader<u32>;
#[doc = "Field `L3A31` writer - Layer 3 Address 3 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\\[127:96\\]
of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\\[127:96\\]
of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field is not used."]
pub type L3A31_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 3 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\\[127:96\\]
of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\\[127:96\\]
of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field is not used."]
    #[inline(always)]
    pub fn l3a31(&self) -> L3A31_R {
        L3A31_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 3 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\\[127:96\\]
of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\\[127:96\\]
of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field is not used."]
    #[inline(always)]
    #[must_use]
    pub fn l3a31(&mut self) -> L3A31_W<MACL3A31Rrs> {
        L3A31_W::new(self, 0)
    }
}
#[doc = "Layer3 address 3 filter 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a31r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a31r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACL3A31Rrs;
impl crate::RegisterSpec for MACL3A31Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a31r::R`](R) reader structure"]
impl crate::Readable for MACL3A31Rrs {}
#[doc = "`write(|w| ..)` method takes [`macl3a31r::W`](W) writer structure"]
impl crate::Writable for MACL3A31Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACL3A31R to value 0"]
impl crate::Resettable for MACL3A31Rrs {
    const RESET_VALUE: u32 = 0;
}
