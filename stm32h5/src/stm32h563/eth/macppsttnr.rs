///Register `MACPPSTTNR` reader
pub type R = crate::R<MACPPSTTNRrs>;
///Register `MACPPSTTNR` writer
pub type W = crate::W<MACPPSTTNRrs>;
///Field `TTSL0` reader - Target Time Low for PPS Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the value in both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on the TRGTMODSEL0 field (Bits \[6:5\]) in PPS control register (ETH_MACPPSCR). When the TSCTRLSSR bit is set in the Timestamp control Register (ETH_MACTSCR), this value should not exceed 0x3B9A_C9FF. The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value.
pub type TTSL0_R = crate::FieldReader<u32>;
///Field `TTSL0` writer - Target Time Low for PPS Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the value in both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on the TRGTMODSEL0 field (Bits \[6:5\]) in PPS control register (ETH_MACPPSCR). When the TSCTRLSSR bit is set in the Timestamp control Register (ETH_MACTSCR), this value should not exceed 0x3B9A_C9FF. The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value.
pub type TTSL0_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `TRGTBUSY0` reader - PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS control register (ETH_MACPPSCR) is programmed to 010 or 011. Programming the PPSCMD0 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers with the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted.
pub type TRGTBUSY0_R = crate::BitReader;
///Field `TRGTBUSY0` writer - PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS control register (ETH_MACPPSCR) is programmed to 010 or 011. Programming the PPSCMD0 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers with the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted.
pub type TRGTBUSY0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - Target Time Low for PPS Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the value in both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on the TRGTMODSEL0 field (Bits \[6:5\]) in PPS control register (ETH_MACPPSCR). When the TSCTRLSSR bit is set in the Timestamp control Register (ETH_MACTSCR), this value should not exceed 0x3B9A_C9FF. The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value.
    #[inline(always)]
    pub fn ttsl0(&self) -> TTSL0_R {
        TTSL0_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS control register (ETH_MACPPSCR) is programmed to 010 or 011. Programming the PPSCMD0 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers with the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted.
    #[inline(always)]
    pub fn trgtbusy0(&self) -> TRGTBUSY0_R {
        TRGTBUSY0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPPSTTNR")
            .field("ttsl0", &self.ttsl0())
            .field("trgtbusy0", &self.trgtbusy0())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - Target Time Low for PPS Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the value in both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on the TRGTMODSEL0 field (Bits \[6:5\]) in PPS control register (ETH_MACPPSCR). When the TSCTRLSSR bit is set in the Timestamp control Register (ETH_MACTSCR), this value should not exceed 0x3B9A_C9FF. The actual start or stop time of the PPS signal output may have an error margin up to one unit of subsecond increment value.
    #[inline(always)]
    pub fn ttsl0(&mut self) -> TTSL0_W<MACPPSTTNRrs> {
        TTSL0_W::new(self, 0)
    }
    ///Bit 31 - PPS Target Time Register Busy The MAC sets this bit when the PPSCMD0 field in the PPS control register (ETH_MACPPSCR) is programmed to 010 or 011. Programming the PPSCMD0 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers with the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted.
    #[inline(always)]
    pub fn trgtbusy0(&mut self) -> TRGTBUSY0_W<MACPPSTTNRrs> {
        TRGTBUSY0_W::new(self, 31)
    }
}
/**PPS target time nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`macppsttnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:MACPPSTTNR)*/
pub struct MACPPSTTNRrs;
impl crate::RegisterSpec for MACPPSTTNRrs {
    type Ux = u32;
}
///`read()` method returns [`macppsttnr::R`](R) reader structure
impl crate::Readable for MACPPSTTNRrs {}
///`write(|w| ..)` method takes [`macppsttnr::W`](W) writer structure
impl crate::Writable for MACPPSTTNRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSTTNR to value 0
impl crate::Resettable for MACPPSTTNRrs {}
