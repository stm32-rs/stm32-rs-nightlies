///Register `TSCSDIFSMPLCLRR` writer
pub type W = crate::W<TSCSDIFSMPLCLRRrs>;
///Field `SMPL_CNTER_CLEAR` writer - Sample counter clear bit
pub type SMPL_CNTER_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TSCSDIFSMPLCLRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Sample counter clear bit
    #[inline(always)]
    pub fn smpl_cnter_clear(&mut self) -> SMPL_CNTER_CLEAR_W<'_, TSCSDIFSMPLCLRRrs> {
        SMPL_CNTER_CLEAR_W::new(self, 0)
    }
}
/**DTS TSC sample clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsdifsmplclrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DTS:TSCSDIFSMPLCLRR)*/
pub struct TSCSDIFSMPLCLRRrs;
impl crate::RegisterSpec for TSCSDIFSMPLCLRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`tscsdifsmplclrr::W`](W) writer structure
impl crate::Writable for TSCSDIFSMPLCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCSDIFSMPLCLRR to value 0
impl crate::Resettable for TSCSDIFSMPLCLRRrs {}
