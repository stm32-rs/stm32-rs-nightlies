///Register `DSI_DLTRCR` reader
pub type R = crate::R<DSI_DLTRCRrs>;
///Register `DSI_DLTRCR` writer
pub type W = crate::W<DSI_DLTRCRrs>;
///Field `MRD_TIME` reader - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress.
pub type MRD_TIME_R = crate::FieldReader<u16>;
///Field `MRD_TIME` writer - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress.
pub type MRD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress.
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_DLTRCR")
            .field("mrd_time", &self.mrd_time())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Maximum read time This field configures the maximum time required to perform a read command in lane byte clock cycles. This register can only be modified when no read command is in progress.
    #[inline(always)]
    #[must_use]
    pub fn mrd_time(&mut self) -> MRD_TIME_W<DSI_DLTRCRrs> {
        MRD_TIME_W::new(self, 0)
    }
}
/**DSI Host data lane timer read configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_dltrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_dltrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_DLTRCR)*/
pub struct DSI_DLTRCRrs;
impl crate::RegisterSpec for DSI_DLTRCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_dltrcr::R`](R) reader structure
impl crate::Readable for DSI_DLTRCRrs {}
///`write(|w| ..)` method takes [`dsi_dltrcr::W`](W) writer structure
impl crate::Writable for DSI_DLTRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSI_DLTRCR to value 0
impl crate::Resettable for DSI_DLTRCRrs {
    const RESET_VALUE: u32 = 0;
}
