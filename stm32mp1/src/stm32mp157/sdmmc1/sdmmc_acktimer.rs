#[doc = "Register `SDMMC_ACKTIMER` reader"]
pub type R = crate::R<SDMMC_ACKTIMERrs>;
#[doc = "Register `SDMMC_ACKTIMER` writer"]
pub type W = crate::W<SDMMC_ACKTIMERrs>;
#[doc = "Field `ACKTIME` reader - ACKTIME"]
pub type ACKTIME_R = crate::FieldReader<u32>;
#[doc = "Field `ACKTIME` writer - ACKTIME"]
pub type ACKTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - ACKTIME"]
    #[inline(always)]
    pub fn acktime(&self) -> ACKTIME_R {
        ACKTIME_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - ACKTIME"]
    #[inline(always)]
    #[must_use]
    pub fn acktime(&mut self) -> ACKTIME_W<SDMMC_ACKTIMERrs> {
        ACKTIME_W::new(self, 0)
    }
}
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_acktimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_acktimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_ACKTIMERrs;
impl crate::RegisterSpec for SDMMC_ACKTIMERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_acktimer::R`](R) reader structure"]
impl crate::Readable for SDMMC_ACKTIMERrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_acktimer::W`](W) writer structure"]
impl crate::Writable for SDMMC_ACKTIMERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_ACKTIMER to value 0"]
impl crate::Resettable for SDMMC_ACKTIMERrs {
    const RESET_VALUE: u32 = 0;
}
