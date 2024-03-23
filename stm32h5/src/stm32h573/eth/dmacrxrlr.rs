#[doc = "Register `DMACRXRLR` reader"]
pub type R = crate::R<DMACRXRLRrs>;
#[doc = "Register `DMACRXRLR` writer"]
pub type W = crate::W<DMACRXRLRrs>;
#[doc = "Field `RDRL` reader - Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. For example, you can program any value up to 0x3FF in this field. This field is 10-bit wide. If you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
pub type RDRL_R = crate::FieldReader<u16>;
#[doc = "Field `RDRL` writer - Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. For example, you can program any value up to 0x3FF in this field. This field is 10-bit wide. If you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
pub type RDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ARBS` reader - Alternate Receive Buffer Size Indicates size in bytes for Buffer 1 when ARBS\\[7:0\\]
is programmed to a non-zero value. When ARBS\\[7:0\\]�=�0, Rx Buffer1 and Rx Buffer2 sizes are based on RBSZ\\[13:0\\]
field of Channel receive control register (ETH_DMACRXCR)."]
pub type ARBS_R = crate::FieldReader;
#[doc = "Field `ARBS` writer - Alternate Receive Buffer Size Indicates size in bytes for Buffer 1 when ARBS\\[7:0\\]
is programmed to a non-zero value. When ARBS\\[7:0\\]�=�0, Rx Buffer1 and Rx Buffer2 sizes are based on RBSZ\\[13:0\\]
field of Channel receive control register (ETH_DMACRXCR)."]
pub type ARBS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. For example, you can program any value up to 0x3FF in this field. This field is 10-bit wide. If you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
    #[inline(always)]
    pub fn rdrl(&self) -> RDRL_R {
        RDRL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:23 - Alternate Receive Buffer Size Indicates size in bytes for Buffer 1 when ARBS\\[7:0\\]
is programmed to a non-zero value. When ARBS\\[7:0\\]�=�0, Rx Buffer1 and Rx Buffer2 sizes are based on RBSZ\\[13:0\\]
field of Channel receive control register (ETH_DMACRXCR)."]
    #[inline(always)]
    pub fn arbs(&self) -> ARBS_R {
        ARBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. For example, you can program any value up to 0x3FF in this field. This field is 10-bit wide. If you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
    #[inline(always)]
    #[must_use]
    pub fn rdrl(&mut self) -> RDRL_W<DMACRXRLRrs> {
        RDRL_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Alternate Receive Buffer Size Indicates size in bytes for Buffer 1 when ARBS\\[7:0\\]
is programmed to a non-zero value. When ARBS\\[7:0\\]�=�0, Rx Buffer1 and Rx Buffer2 sizes are based on RBSZ\\[13:0\\]
field of Channel receive control register (ETH_DMACRXCR)."]
    #[inline(always)]
    #[must_use]
    pub fn arbs(&mut self) -> ARBS_W<DMACRXRLRrs> {
        ARBS_W::new(self, 16)
    }
}
#[doc = "Channel Rx descriptor ring length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrxrlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrxrlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRXRLRrs;
impl crate::RegisterSpec for DMACRXRLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrxrlr::R`](R) reader structure"]
impl crate::Readable for DMACRXRLRrs {}
#[doc = "`write(|w| ..)` method takes [`dmacrxrlr::W`](W) writer structure"]
impl crate::Writable for DMACRXRLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACRXRLR to value 0"]
impl crate::Resettable for DMACRXRLRrs {
    const RESET_VALUE: u32 = 0;
}
