///Register `MTLTXQUR` reader
pub type R = crate::R<MTLTXQURrs>;
///Register `MTLTXQUR` writer
pub type W = crate::W<MTLTXQURrs>;
/**Field `UFFRMCNT` reader - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type UFFRMCNT_R = crate::FieldReader<u16>;
///Field `UFFRMCNT` writer - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read.
pub type UFFRMCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
/**Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type UFCNTOVF_R = crate::BitReader;
///Field `UFCNTOVF` writer - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened.
pub type UFCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read.
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 11 - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened.
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTXQUR").finish()
    }
}
impl W {
    ///Bits 0:10 - Underflow Packet Counter This field indicates the number of packets aborted by the controller because of Tx queue Underflow. This counter is incremented each time the MAC aborts outgoing packet because of underflow. The counter is cleared when this register is read.
    #[inline(always)]
    pub fn uffrmcnt(&mut self) -> UFFRMCNT_W<'_, MTLTXQURrs> {
        UFFRMCNT_W::new(self, 0)
    }
    ///Bit 11 - Overflow Bit for Underflow Packet Counter This bit is set every time the Tx queue Underflow Packet Counter field overflows, that is, it has crossed the maximum count. In such a scenario, the overflow packet counter is reset to all-zeros and this bit indicates that the rollover happened.
    #[inline(always)]
    pub fn ufcntovf(&mut self) -> UFCNTOVF_W<'_, MTLTXQURrs> {
        UFCNTOVF_W::new(self, 11)
    }
}
/**Tx queue underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltxqur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxqur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#ETH:MTLTXQUR)*/
pub struct MTLTXQURrs;
impl crate::RegisterSpec for MTLTXQURrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxqur::R`](R) reader structure
impl crate::Readable for MTLTXQURrs {}
///`write(|w| ..)` method takes [`mtltxqur::W`](W) writer structure
impl crate::Writable for MTLTXQURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTXQUR to value 0
impl crate::Resettable for MTLTXQURrs {}
