#[doc = "Register `DMACRxDLAR` reader"]
pub type R = crate::R<DMACRX_DLARrs>;
#[doc = "Register `DMACRxDLAR` writer"]
pub type W = crate::W<DMACRX_DLARrs>;
#[doc = "Field `RDESLA` reader - Start of Receive List"]
pub type RDESLA_R = crate::FieldReader<u32>;
#[doc = "Field `RDESLA` writer - Start of Receive List"]
pub type RDESLA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    #[must_use]
    pub fn rdesla(&mut self) -> RDESLA_W<DMACRX_DLARrs> {
        RDESLA_W::new(self, 2)
    }
}
#[doc = "Channel Rx descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_dlar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_dlar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACRX_DLARrs;
impl crate::RegisterSpec for DMACRX_DLARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_dlar::R`](R) reader structure"]
impl crate::Readable for DMACRX_DLARrs {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_dlar::W`](W) writer structure"]
impl crate::Writable for DMACRX_DLARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACRxDLAR to value 0"]
impl crate::Resettable for DMACRX_DLARrs {
    const RESET_VALUE: u32 = 0;
}
