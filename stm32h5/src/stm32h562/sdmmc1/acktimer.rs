#[doc = "Register `ACKTIMER` reader"]
pub type R = crate::R<ACKTIMERrs>;
#[doc = "Register `ACKTIMER` writer"]
pub type W = crate::W<ACKTIMERrs>;
#[doc = "Field `ACKTIME` reader - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
pub type ACKTIME_R = crate::FieldReader<u32>;
#[doc = "Field `ACKTIME` writer - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
pub type ACKTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
    #[inline(always)]
    pub fn acktime(&self) -> ACKTIME_R {
        ACKTIME_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
    #[inline(always)]
    #[must_use]
    pub fn acktime(&mut self) -> ACKTIME_W<ACKTIMERrs> {
        ACKTIME_W::new(self, 0)
    }
}
#[doc = "SDMMC acknowledgment timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acktimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acktimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACKTIMERrs;
impl crate::RegisterSpec for ACKTIMERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acktimer::R`](R) reader structure"]
impl crate::Readable for ACKTIMERrs {}
#[doc = "`write(|w| ..)` method takes [`acktimer::W`](W) writer structure"]
impl crate::Writable for ACKTIMERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACKTIMER to value 0"]
impl crate::Resettable for ACKTIMERrs {
    const RESET_VALUE: u32 = 0;
}
