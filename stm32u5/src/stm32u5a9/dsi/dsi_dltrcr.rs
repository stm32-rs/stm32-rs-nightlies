#[doc = "Register `DSI_DLTRCR` reader"]
pub type R = crate::R<DSI_DLTRCRrs>;
#[doc = "Register `DSI_DLTRCR` writer"]
pub type W = crate::W<DSI_DLTRCRrs>;
#[doc = "Field `MRD_TIME` reader - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress."]
pub type MRD_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `MRD_TIME` writer - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress."]
pub type MRD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress."]
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn mrd_time(&mut self) -> MRD_TIME_W<DSI_DLTRCRrs> {
        MRD_TIME_W::new(self, 0)
    }
}
#[doc = "DSI Host data lane timer read configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dltrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dltrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_DLTRCRrs;
impl crate::RegisterSpec for DSI_DLTRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_dltrcr::R`](R) reader structure"]
impl crate::Readable for DSI_DLTRCRrs {}
#[doc = "`write(|w| ..)` method takes [`dsi_dltrcr::W`](W) writer structure"]
impl crate::Writable for DSI_DLTRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_DLTRCR to value 0"]
impl crate::Resettable for DSI_DLTRCRrs {
    const RESET_VALUE: u32 = 0;
}
