#[doc = "Register `AXIMC_M0_WRITE_QOS` reader"]
pub type R = crate::R<AXIMC_M0_WRITE_QOSrs>;
#[doc = "Register `AXIMC_M0_WRITE_QOS` writer"]
pub type W = crate::W<AXIMC_M0_WRITE_QOSrs>;
#[doc = "Field `AW_QOS` reader - AW_QOS"]
pub type AW_QOS_R = crate::FieldReader;
#[doc = "Field `AW_QOS` writer - AW_QOS"]
pub type AW_QOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AW_QOS"]
    #[inline(always)]
    pub fn aw_qos(&self) -> AW_QOS_R {
        AW_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AW_QOS"]
    #[inline(always)]
    #[must_use]
    pub fn aw_qos(&mut self) -> AW_QOS_W<AXIMC_M0_WRITE_QOSrs> {
        AW_QOS_W::new(self, 0)
    }
}
#[doc = "AXIMC master 0 write priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_m0_write_qos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aximc_m0_write_qos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_M0_WRITE_QOSrs;
impl crate::RegisterSpec for AXIMC_M0_WRITE_QOSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_m0_write_qos::R`](R) reader structure"]
impl crate::Readable for AXIMC_M0_WRITE_QOSrs {}
#[doc = "`write(|w| ..)` method takes [`aximc_m0_write_qos::W`](W) writer structure"]
impl crate::Writable for AXIMC_M0_WRITE_QOSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AXIMC_M0_WRITE_QOS to value 0x06"]
impl crate::Resettable for AXIMC_M0_WRITE_QOSrs {
    const RESET_VALUE: u32 = 0x06;
}
