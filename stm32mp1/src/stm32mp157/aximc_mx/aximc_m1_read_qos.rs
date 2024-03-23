#[doc = "Register `AXIMC_M1_READ_QOS` reader"]
pub type R = crate::R<AXIMC_M1_READ_QOSrs>;
#[doc = "Register `AXIMC_M1_READ_QOS` writer"]
pub type W = crate::W<AXIMC_M1_READ_QOSrs>;
#[doc = "Field `AR_QOS` reader - AR_QOS"]
pub type AR_QOS_R = crate::FieldReader;
#[doc = "Field `AR_QOS` writer - AR_QOS"]
pub type AR_QOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AR_QOS"]
    #[inline(always)]
    pub fn ar_qos(&self) -> AR_QOS_R {
        AR_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AR_QOS"]
    #[inline(always)]
    #[must_use]
    pub fn ar_qos(&mut self) -> AR_QOS_W<AXIMC_M1_READ_QOSrs> {
        AR_QOS_W::new(self, 0)
    }
}
#[doc = "AXIMC master 1 read priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m1_read_qos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m1_read_qos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_M1_READ_QOSrs;
impl crate::RegisterSpec for AXIMC_M1_READ_QOSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_m1_read_qos::R`](R) reader structure"]
impl crate::Readable for AXIMC_M1_READ_QOSrs {}
#[doc = "`write(|w| ..)` method takes [`aximc_m1_read_qos::W`](W) writer structure"]
impl crate::Writable for AXIMC_M1_READ_QOSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AXIMC_M1_READ_QOS to value 0x06"]
impl crate::Resettable for AXIMC_M1_READ_QOSrs {
    const RESET_VALUE: u32 = 0x06;
}
