#[doc = "Register `INI6_READ_QOS` reader"]
pub type R = crate::R<INI6_READ_QOSrs>;
#[doc = "Register `INI6_READ_QOS` writer"]
pub type W = crate::W<INI6_READ_QOSrs>;
#[doc = "Field `AR_QOS` reader - Read channel QoS setting"]
pub type AR_QOS_R = crate::FieldReader;
#[doc = "Field `AR_QOS` writer - Read channel QoS setting"]
pub type AR_QOS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
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
    pub fn ar_qos(&mut self) -> AR_QOS_W<INI6_READ_QOSrs> {
        AR_QOS_W::new(self, 0)
    }
}
#[doc = "AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini6_read_qos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini6_read_qos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INI6_READ_QOSrs;
impl crate::RegisterSpec for INI6_READ_QOSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ini6_read_qos::R`](R) reader structure"]
impl crate::Readable for INI6_READ_QOSrs {}
#[doc = "`write(|w| ..)` method takes [`ini6_read_qos::W`](W) writer structure"]
impl crate::Writable for INI6_READ_QOSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INI6_READ_QOS to value 0x04"]
impl crate::Resettable for INI6_READ_QOSrs {
    const RESET_VALUE: u32 = 0x04;
}
