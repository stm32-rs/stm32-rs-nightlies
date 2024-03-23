#[doc = "Register `BWTR1` reader"]
pub type R = crate::R<BWTR1rs>;
#[doc = "Register `BWTR1` writer"]
pub type W = crate::W<BWTR1rs>;
#[doc = "Field `ADDSET` reader - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in KCK_FMC cycles (refer to Figure81 to Figure93), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
pub type ADDSET_R = crate::FieldReader;
#[doc = "Field `ADDSET` writer - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in KCK_FMC cycles (refer to Figure81 to Figure93), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDHLD` reader - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
pub type ADDHLD_R = crate::FieldReader;
#[doc = "Field `ADDHLD` writer - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAST` reader - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses:"]
pub type DATAST_R = crate::FieldReader;
#[doc = "Field `DATAST` writer - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses:"]
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSTURN` reader - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (tEHEL from ENx high to ENx low): (BUSTRUN + 1) KCK_FMC period &amp;#8805; tEHELmin. The programmed bus turnaround delay is inserted between a an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed mode and mode D. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank A synchronous write transfer ((in Burst or Single mode) and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write transfer (in Burst or Single mode) and a synchronous read from the same or a different bank. ..."]
pub type BUSTURN_R = crate::FieldReader;
#[doc = "Field `BUSTURN` writer - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (tEHEL from ENx high to ENx low): (BUSTRUN + 1) KCK_FMC period &amp;#8805; tEHELmin. The programmed bus turnaround delay is inserted between a an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed mode and mode D. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank A synchronous write transfer ((in Burst or Single mode) and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write transfer (in Burst or Single mode) and a synchronous read from the same or a different bank. ..."]
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACCMOD` reader - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub type ACCMOD_R = crate::FieldReader;
#[doc = "Field `ACCMOD` writer - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in KCK_FMC cycles (refer to Figure81 to Figure93), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses:"]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (tEHEL from ENx high to ENx low): (BUSTRUN + 1) KCK_FMC period &amp;#8805; tEHELmin. The programmed bus turnaround delay is inserted between a an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed mode and mode D. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank A synchronous write transfer ((in Burst or Single mode) and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write transfer (in Burst or Single mode) and a synchronous read from the same or a different bank. ..."]
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in KCK_FMC cycles (refer to Figure81 to Figure93), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1."]
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<BWTR1rs> {
        ADDSET_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration."]
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<BWTR1rs> {
        ADDHLD_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses:"]
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<BWTR1rs> {
        DATAST_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (tEHEL from ENx high to ENx low): (BUSTRUN + 1) KCK_FMC period &amp;#8805; tEHELmin. The programmed bus turnaround delay is inserted between a an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed mode and mode D. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank A synchronous write transfer ((in Burst or Single mode) and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write transfer (in Burst or Single mode) and a synchronous read from the same or a different bank. ..."]
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<BWTR1rs> {
        BUSTURN_W::new(self, 16)
    }
    #[doc = "Bits 28:29 - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<BWTR1rs> {
        ACCMOD_W::new(self, 28)
    }
}
#[doc = "This register contains the control information of each memory bank. It is used for SRAMs, PSRAMs and NOR Flash memories. When the EXTMOD bit is set in the FMC_BCRx register, then this register is active for write access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BWTR1rs;
impl crate::RegisterSpec for BWTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bwtr1::R`](R) reader structure"]
impl crate::Readable for BWTR1rs {}
#[doc = "`write(|w| ..)` method takes [`bwtr1::W`](W) writer structure"]
impl crate::Writable for BWTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BWTR1 to value 0x0fff_ffff"]
impl crate::Resettable for BWTR1rs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
