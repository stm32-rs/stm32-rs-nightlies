///Register `FDCAN_TSCV` reader
pub type R = crate::R<FDCAN_TSCVrs>;
///Register `FDCAN_TSCV` writer
pub type W = crate::W<FDCAN_TSCVrs>;
/**Field `TSC` reader - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\[TSS\]
= 01, the timestamp counter is incremented in multiples of CAN bit times depending on the configuration of TSCC\[TCP\]. A wrap around sets interrupt flag IR\[TSW\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact.*/
pub type TSC_R = crate::FieldReader<u16>;
/**Field `TSC` writer - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\[TSS\]
= 01, the timestamp counter is incremented in multiples of CAN bit times depending on the configuration of TSCC\[TCP\]. A wrap around sets interrupt flag IR\[TSW\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact.*/
pub type TSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    /**Bits 0:15 - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\[TSS\]
    = 01, the timestamp counter is incremented in multiples of CAN bit times depending on the configuration of TSCC\[TCP\]. A wrap around sets interrupt flag IR\[TSW\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact.*/
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TSCV")
            .field("tsc", &self.tsc())
            .finish()
    }
}
impl W {
    /**Bits 0:15 - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\[TSS\]
    = 01, the timestamp counter is incremented in multiples of CAN bit times depending on the configuration of TSCC\[TCP\]. A wrap around sets interrupt flag IR\[TSW\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact.*/
    #[inline(always)]
    pub fn tsc(&mut self) -> TSC_W<FDCAN_TSCVrs> {
        TSC_W::new(self, 0)
    }
}
/**FDCAN timestamp counter value register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tscv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FDCAN1:FDCAN_TSCV)*/
pub struct FDCAN_TSCVrs;
impl crate::RegisterSpec for FDCAN_TSCVrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_tscv::R`](R) reader structure
impl crate::Readable for FDCAN_TSCVrs {}
///`write(|w| ..)` method takes [`fdcan_tscv::W`](W) writer structure
impl crate::Writable for FDCAN_TSCVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TSCV to value 0
impl crate::Resettable for FDCAN_TSCVrs {
    const RESET_VALUE: u32 = 0;
}
