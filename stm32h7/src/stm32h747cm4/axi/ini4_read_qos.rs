#[doc = "Register `INI4_READ_QOS` reader"]
pub type R = crate::R<INI4_READ_QOSrs>;
#[doc = "Register `INI4_READ_QOS` writer"]
pub type W = crate::W<INI4_READ_QOSrs>;
#[doc = "Field `AR_QOS` reader - Read channel QoS setting"]
pub type AR_QOS_R = crate::FieldReader;
#[doc = "Field `AR_QOS` writer - Read channel QoS setting"]
pub type AR_QOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Read channel QoS setting"]
    #[inline(always)]
    pub fn ar_qos(&self) -> AR_QOS_R {
        AR_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Read channel QoS setting"]
    #[inline(always)]
    #[must_use]
    pub fn ar_qos(&mut self) -> AR_QOS_W<INI4_READ_QOSrs> {
        AR_QOS_W::new(self, 0)
    }
}
#[doc = "AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini4_read_qos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini4_read_qos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INI4_READ_QOSrs;
impl crate::RegisterSpec for INI4_READ_QOSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ini4_read_qos::R`](R) reader structure"]
impl crate::Readable for INI4_READ_QOSrs {}
#[doc = "`write(|w| ..)` method takes [`ini4_read_qos::W`](W) writer structure"]
impl crate::Writable for INI4_READ_QOSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INI4_READ_QOS to value 0x04"]
impl crate::Resettable for INI4_READ_QOSrs {
    const RESET_VALUE: u32 = 0x04;
}
