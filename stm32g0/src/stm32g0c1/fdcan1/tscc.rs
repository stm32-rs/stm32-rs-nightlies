///Register `TSCC` reader
pub type R = crate::R<TSCCrs>;
///Register `TSCC` writer
pub type W = crate::W<TSCCrs>;
///Field `TSS` reader - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type TSS_R = crate::FieldReader;
///Field `TSS` writer - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TCP` reader - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times \[1 : 16\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. In CAN FD mode the internal timestamp counter TCP does not provide a constant time base due to the different CAN bit times between arbitration phase and data phase. Thus CAN FD requires an external counter for timestamp generation (TSS = 10). These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type TCP_R = crate::FieldReader;
///Field `TCP` writer - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times \[1 : 16\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. In CAN FD mode the internal timestamp counter TCP does not provide a constant time base due to the different CAN bit times between arbitration phase and data phase. Thus CAN FD requires an external counter for timestamp generation (TSS = 10). These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type TCP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    ///Bits 16:19 - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times \[1 : 16\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. In CAN FD mode the internal timestamp counter TCP does not provide a constant time base due to the different CAN bit times between arbitration phase and data phase. Thus CAN FD requires an external counter for timestamp generation (TSS = 10). These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCC")
            .field("tss", &self.tss())
            .field("tcp", &self.tcp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Timestamp select These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W<'_, TSCCrs> {
        TSS_W::new(self, 0)
    }
    ///Bits 16:19 - Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times \[1 : 16\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. In CAN FD mode the internal timestamp counter TCP does not provide a constant time base due to the different CAN bit times between arbitration phase and data phase. Thus CAN FD requires an external counter for timestamp generation (TSS = 10). These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W<'_, TSCCrs> {
        TCP_W::new(self, 16)
    }
}
/**FDCAN timestamp counter configuration register

You can [`read`](crate::Reg::read) this register and get [`tscc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#FDCAN1:TSCC)*/
pub struct TSCCrs;
impl crate::RegisterSpec for TSCCrs {
    type Ux = u32;
}
///`read()` method returns [`tscc::R`](R) reader structure
impl crate::Readable for TSCCrs {}
///`write(|w| ..)` method takes [`tscc::W`](W) writer structure
impl crate::Writable for TSCCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCC to value 0
impl crate::Resettable for TSCCrs {}
