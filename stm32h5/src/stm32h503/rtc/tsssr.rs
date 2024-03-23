#[doc = "Register `TSSSR` reader"]
pub type R = crate::R<TSSSRrs>;
#[doc = "Field `SS` reader - Subsecond value/synchronous binary counter values SS\\[31:0\\]
is the value of the synchronous prescaler counter when the timestamp event occurred."]
pub type SS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Subsecond value/synchronous binary counter values SS\\[31:0\\]
is the value of the synchronous prescaler counter when the timestamp event occurred."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
#[doc = "RTC timestamp subsecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSSSRrs;
impl crate::RegisterSpec for TSSSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsssr::R`](R) reader structure"]
impl crate::Readable for TSSSRrs {}
#[doc = "`reset()` method sets TSSSR to value 0"]
impl crate::Resettable for TSSSRrs {
    const RESET_VALUE: u32 = 0;
}
