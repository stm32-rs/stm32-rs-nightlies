#[doc = "Register `MTLTXQUR` reader"]
pub type R = crate::R<MTLTXQURrs>;
#[doc = "Register `MTLTXQUR` writer"]
pub type W = crate::W<MTLTXQURrs>;
#[doc = "Field `UFFRMCNT` reader - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type UFFRMCNT_R = crate::FieldReader<u16>;
#[doc = "Field `UFFRMCNT` writer - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read."]
pub type UFFRMCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type UFCNTOVF_R = crate::BitReader;
#[doc = "Field `UFCNTOVF` writer - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened."]
pub type UFCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read."]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read."]
    #[inline(always)]
    #[must_use]
    pub fn uffrmcnt(&mut self) -> UFFRMCNT_W<MTLTXQURrs> {
        UFFRMCNT_W::new(self, 0)
    }
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    #[must_use]
    pub fn ufcntovf(&mut self) -> UFCNTOVF_W<MTLTXQURrs> {
        UFCNTOVF_W::new(self, 11)
    }
}
#[doc = "Tx queue underflow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtltxqur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtltxqur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLTXQURrs;
impl crate::RegisterSpec for MTLTXQURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtltxqur::R`](R) reader structure"]
impl crate::Readable for MTLTXQURrs {}
#[doc = "`write(|w| ..)` method takes [`mtltxqur::W`](W) writer structure"]
impl crate::Writable for MTLTXQURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTLTXQUR to value 0"]
impl crate::Resettable for MTLTXQURrs {
    const RESET_VALUE: u32 = 0;
}
