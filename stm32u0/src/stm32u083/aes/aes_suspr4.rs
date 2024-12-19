///Register `AES_SUSPR4` reader
pub type R = crate::R<AES_SUSPR4rs>;
///Register `AES_SUSPR4` writer
pub type W = crate::W<AES_SUSPR4rs>;
///Field `SUSP` reader - Suspend data AES_SUSPRx registers contain the complete internal register states of the AES when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section121.4.8: AES suspend and resume operations for more details. Read to this register returns zero when EN bit is cleared in AES_SR register. AES_SUSPRx registers are not used in other chaining modes than GCM, GMAC or CCM.
pub type SUSP_R = crate::FieldReader<u32>;
///Field `SUSP` writer - Suspend data AES_SUSPRx registers contain the complete internal register states of the AES when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section121.4.8: AES suspend and resume operations for more details. Read to this register returns zero when EN bit is cleared in AES_SR register. AES_SUSPRx registers are not used in other chaining modes than GCM, GMAC or CCM.
pub type SUSP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Suspend data AES_SUSPRx registers contain the complete internal register states of the AES when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section121.4.8: AES suspend and resume operations for more details. Read to this register returns zero when EN bit is cleared in AES_SR register. AES_SUSPRx registers are not used in other chaining modes than GCM, GMAC or CCM.
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES_SUSPR4")
            .field("susp", &self.susp())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Suspend data AES_SUSPRx registers contain the complete internal register states of the AES when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section121.4.8: AES suspend and resume operations for more details. Read to this register returns zero when EN bit is cleared in AES_SR register. AES_SUSPRx registers are not used in other chaining modes than GCM, GMAC or CCM.
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<AES_SUSPR4rs> {
        SUSP_W::new(self, 0)
    }
}
/**AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_suspr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_suspr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_SUSPR4)*/
pub struct AES_SUSPR4rs;
impl crate::RegisterSpec for AES_SUSPR4rs {
    type Ux = u32;
}
///`read()` method returns [`aes_suspr4::R`](R) reader structure
impl crate::Readable for AES_SUSPR4rs {}
///`write(|w| ..)` method takes [`aes_suspr4::W`](W) writer structure
impl crate::Writable for AES_SUSPR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AES_SUSPR4 to value 0
impl crate::Resettable for AES_SUSPR4rs {
    const RESET_VALUE: u32 = 0;
}
