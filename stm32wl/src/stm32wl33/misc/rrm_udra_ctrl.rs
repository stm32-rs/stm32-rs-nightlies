///Register `RRM_UDRA_CTRL` writer
pub type W = crate::W<RRM_UDRA_CTRLrs>;
///Field `RRM_CMD_REQ` writer - Action bit: write 1 to request a RRM-UDRA command.
pub type RRM_CMD_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<RRM_UDRA_CTRLrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Action bit: write 1 to request a RRM-UDRA command.
    #[inline(always)]
    pub fn rrm_cmd_req(&mut self) -> RRM_CMD_REQ_W<'_, RRM_UDRA_CTRLrs> {
        RRM_CMD_REQ_W::new(self, 0)
    }
}
/**RRM_UDRA_CTRL register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrm_udra_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:RRM_UDRA_CTRL)*/
pub struct RRM_UDRA_CTRLrs;
impl crate::RegisterSpec for RRM_UDRA_CTRLrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rrm_udra_ctrl::W`](W) writer structure
impl crate::Writable for RRM_UDRA_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RRM_UDRA_CTRL to value 0
impl crate::Resettable for RRM_UDRA_CTRLrs {}
