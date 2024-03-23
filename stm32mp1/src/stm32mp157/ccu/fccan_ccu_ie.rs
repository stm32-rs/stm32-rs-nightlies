#[doc = "Register `FCCAN_CCU_IE` reader"]
pub type R = crate::R<FCCAN_CCU_IErs>;
#[doc = "Register `FCCAN_CCU_IE` writer"]
pub type W = crate::W<FCCAN_CCU_IErs>;
#[doc = "Field `CWEE` reader - CWEE"]
pub type CWEE_R = crate::BitReader;
#[doc = "Field `CWEE` writer - CWEE"]
pub type CWEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSCE` reader - CSCE"]
pub type CSCE_R = crate::BitReader;
#[doc = "Field `CSCE` writer - CSCE"]
pub type CSCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CWEE"]
    #[inline(always)]
    pub fn cwee(&self) -> CWEE_R {
        CWEE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSCE"]
    #[inline(always)]
    pub fn csce(&self) -> CSCE_R {
        CSCE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CWEE"]
    #[inline(always)]
    #[must_use]
    pub fn cwee(&mut self) -> CWEE_W<FCCAN_CCU_IErs> {
        CWEE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CSCE"]
    #[inline(always)]
    #[must_use]
    pub fn csce(&mut self) -> CSCE_W<FCCAN_CCU_IErs> {
        CSCE_W::new(self, 1)
    }
}
#[doc = "The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fccan_ccu_ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fccan_ccu_ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCCAN_CCU_IErs;
impl crate::RegisterSpec for FCCAN_CCU_IErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fccan_ccu_ie::R`](R) reader structure"]
impl crate::Readable for FCCAN_CCU_IErs {}
#[doc = "`write(|w| ..)` method takes [`fccan_ccu_ie::W`](W) writer structure"]
impl crate::Writable for FCCAN_CCU_IErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCCAN_CCU_IE to value 0"]
impl crate::Resettable for FCCAN_CCU_IErs {
    const RESET_VALUE: u32 = 0;
}
