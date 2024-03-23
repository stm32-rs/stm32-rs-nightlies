#[doc = "Register `DMACTXRLR` reader"]
pub type R = crate::R<DMACTXRLRrs>;
#[doc = "Register `DMACTXRLR` writer"]
pub type W = crate::W<DMACTXRLRrs>;
#[doc = "Field `TDRL` reader - Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. It is recommended to put a minimum ring descriptor length of 4. For example, you can program any value up to 0x3FF in this field. This field is 10 bits wide, if you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
pub type TDRL_R = crate::FieldReader<u16>;
#[doc = "Field `TDRL` writer - Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. It is recommended to put a minimum ring descriptor length of 4. For example, you can program any value up to 0x3FF in this field. This field is 10 bits wide, if you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
pub type TDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. It is recommended to put a minimum ring descriptor length of 4. For example, you can program any value up to 0x3FF in this field. This field is 10 bits wide, if you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
    #[inline(always)]
    pub fn tdrl(&self) -> TDRL_R {
        TDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmit Descriptor Ring Length This field sets the maximum number of Tx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. It is recommended to put a minimum ring descriptor length of 4. For example, you can program any value up to 0x3FF in this field. This field is 10 bits wide, if you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9."]
    #[inline(always)]
    #[must_use]
    pub fn tdrl(&mut self) -> TDRL_W<DMACTXRLRrs> {
        TDRL_W::new(self, 0)
    }
}
#[doc = "Channel Tx descriptor ring length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactxrlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactxrlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTXRLRrs;
impl crate::RegisterSpec for DMACTXRLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactxrlr::R`](R) reader structure"]
impl crate::Readable for DMACTXRLRrs {}
#[doc = "`write(|w| ..)` method takes [`dmactxrlr::W`](W) writer structure"]
impl crate::Writable for DMACTXRLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTXRLR to value 0"]
impl crate::Resettable for DMACTXRLRrs {
    const RESET_VALUE: u32 = 0;
}
