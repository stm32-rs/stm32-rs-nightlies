///Register `TDCR` reader
pub type R = crate::R<TDCRrs>;
///Register `TDCR` writer
pub type W = crate::W<TDCRrs>;
///Field `TDCF` reader - Transmitter delay compensation filter window length Defines the minimum value for the SSP position, dominant edges on FDCAN_RX that would result in an earlier SSP position are ignored for transmitter delay measurements. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type TDCF_R = crate::FieldReader;
///Field `TDCF` writer - Transmitter delay compensation filter window length Defines the minimum value for the SSP position, dominant edges on FDCAN_RX that would result in an earlier SSP position are ignored for transmitter delay measurements. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type TDCF_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `TDCO` reader - Transmitter delay compensation offset Offset value defining the distance between the measured delay from FDCAN_TX to FDCAN_RX and the secondary sample point. Valid values are 0 to 127 mtq. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type TDCO_R = crate::FieldReader;
///Field `TDCO` writer - Transmitter delay compensation offset Offset value defining the distance between the measured delay from FDCAN_TX to FDCAN_RX and the secondary sample point. Valid values are 0 to 127 mtq. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type TDCO_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Transmitter delay compensation filter window length Defines the minimum value for the SSP position, dominant edges on FDCAN_RX that would result in an earlier SSP position are ignored for transmitter delay measurements. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn tdcf(&self) -> TDCF_R {
        TDCF_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - Transmitter delay compensation offset Offset value defining the distance between the measured delay from FDCAN_TX to FDCAN_RX and the secondary sample point. Valid values are 0 to 127 mtq. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDCR")
            .field("tdcf", &self.tdcf())
            .field("tdco", &self.tdco())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Transmitter delay compensation filter window length Defines the minimum value for the SSP position, dominant edges on FDCAN_RX that would result in an earlier SSP position are ignored for transmitter delay measurements. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn tdcf(&mut self) -> TDCF_W<'_, TDCRrs> {
        TDCF_W::new(self, 0)
    }
    ///Bits 8:14 - Transmitter delay compensation offset Offset value defining the distance between the measured delay from FDCAN_TX to FDCAN_RX and the secondary sample point. Valid values are 0 to 127 mtq. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn tdco(&mut self) -> TDCO_W<'_, TDCRrs> {
        TDCO_W::new(self, 8)
    }
}
/**FDCAN Transmitter Delay Compensation Register

You can [`read`](crate::Reg::read) this register and get [`tdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FDCAN1:TDCR)*/
pub struct TDCRrs;
impl crate::RegisterSpec for TDCRrs {
    type Ux = u32;
}
///`read()` method returns [`tdcr::R`](R) reader structure
impl crate::Readable for TDCRrs {}
///`write(|w| ..)` method takes [`tdcr::W`](W) writer structure
impl crate::Writable for TDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDCR to value 0
impl crate::Resettable for TDCRrs {}
