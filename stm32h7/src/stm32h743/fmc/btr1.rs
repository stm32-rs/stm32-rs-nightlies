#[doc = "Register `BTR1` reader"]
pub type R = crate::R<BTR1rs>;
#[doc = "Register `BTR1` writer"]
pub type W = crate::W<BTR1rs>;
#[doc = "Field `ADDSET` reader - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure81 to Figure93), used in SRAMs, ROMs and asynchronous NOR Flash: For each access mode address setup phase duration, please refer to the respective figure (refer to Figure81 to Figure93). Note: In synchronous accesses, this value is dont care. In Muxed mode or Mode D, the minimum value for ADDSET is 1."]
pub type ADDSET_R = crate::FieldReader;
#[doc = "Field `ADDSET` writer - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure81 to Figure93), used in SRAMs, ROMs and asynchronous NOR Flash: For each access mode address setup phase duration, please refer to the respective figure (refer to Figure81 to Figure93). Note: In synchronous accesses, this value is dont care. In Muxed mode or Mode D, the minimum value for ADDSET is 1."]
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDHLD` reader - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in mode D or multiplexed accesses: For each access mode address-hold phase duration, please refer to the respective figure (Figure81 to Figure93). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
pub type ADDHLD_R = crate::FieldReader;
#[doc = "Field `ADDHLD` writer - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in mode D or multiplexed accesses: For each access mode address-hold phase duration, please refer to the respective figure (Figure81 to Figure93). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAST` reader - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous accesses: For each memory type and access mode data-phase duration, please refer to the respective figure (Figure81 to Figure93). Example: Mode1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 KCK_FMC clock cycles. Note: In synchronous accesses, this value is dont care."]
pub type DATAST_R = crate::FieldReader;
#[doc = "Field `DATAST` writer - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous accesses: For each memory type and access mode data-phase duration, please refer to the respective figure (Figure81 to Figure93). Example: Mode1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 KCK_FMC clock cycles. Note: In synchronous accesses, this value is dont care."]
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSTURN` reader - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read or read-to write transaction. The programmed bus turnaround delay is inserted between an asynchronous read (in muxed or mode D) or write transaction and any other asynchronous /synchronous read/write from/to a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except in muxed mode and mode D. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for modes muxed and D. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except in modes muxed and D mode. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or a different one in case of a read operation. Two consecutive synchronous read operations (in Burst or Single mode) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write access (in Burst or Single mode) and a synchronous read from the same or a different bank. The bus turnaround delay allows to match the minimum time between consecutive transactions (tEHEL from NEx high to NEx low) and the maximum time required by the memory to free the data bus after a read access (tEHQZ): (BUSTRUN + 1) KCK_FMC period &amp;#8805; tEHELmin and (BUSTRUN + 2)KCK_FMC period &amp;#8805; tEHQZmax if EXTMOD = 0 (BUSTRUN + 2)KCK_FMC period &amp;#8805; max (tEHELmin, tEHQZmax) if EXTMOD = 126. ..."]
pub type BUSTURN_R = crate::FieldReader;
#[doc = "Field `BUSTURN` writer - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read or read-to write transaction. The programmed bus turnaround delay is inserted between an asynchronous read (in muxed or mode D) or write transaction and any other asynchronous /synchronous read/write from/to a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except in muxed mode and mode D. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for modes muxed and D. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except in modes muxed and D mode. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or a different one in case of a read operation. Two consecutive synchronous read operations (in Burst or Single mode) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write access (in Burst or Single mode) and a synchronous read from the same or a different bank. The bus turnaround delay allows to match the minimum time between consecutive transactions (tEHEL from NEx high to NEx low) and the maximum time required by the memory to free the data bus after a read access (tEHQZ): (BUSTRUN + 1) KCK_FMC period &amp;#8805; tEHELmin and (BUSTRUN + 2)KCK_FMC period &amp;#8805; tEHQZmax if EXTMOD = 0 (BUSTRUN + 2)KCK_FMC period &amp;#8805; max (tEHELmin, tEHQZmax) if EXTMOD = 126. ..."]
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKDIV` reader - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of KCK_FMC cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section20.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of KCK_FMC cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section20.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATLAT` reader - Data latency for synchronous memory For synchronous access with read write burst mode enabled these bits define the number of memory clock cycles"]
pub type DATLAT_R = crate::FieldReader;
#[doc = "Field `DATLAT` writer - Data latency for synchronous memory For synchronous access with read write burst mode enabled these bits define the number of memory clock cycles"]
pub type DATLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACCMOD` reader - Access mode These bits specify the asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub type ACCMOD_R = crate::FieldReader;
#[doc = "Field `ACCMOD` writer - Access mode These bits specify the asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure81 to Figure93), used in SRAMs, ROMs and asynchronous NOR Flash: For each access mode address setup phase duration, please refer to the respective figure (refer to Figure81 to Figure93). Note: In synchronous accesses, this value is dont care. In Muxed mode or Mode D, the minimum value for ADDSET is 1."]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in mode D or multiplexed accesses: For each access mode address-hold phase duration, please refer to the respective figure (Figure81 to Figure93). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous accesses: For each memory type and access mode data-phase duration, please refer to the respective figure (Figure81 to Figure93). Example: Mode1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 KCK_FMC clock cycles. Note: In synchronous accesses, this value is dont care."]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read or read-to write transaction. The programmed bus turnaround delay is inserted between an asynchronous read (in muxed or mode D) or write transaction and any other asynchronous /synchronous read/write from/to a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except in muxed mode and mode D. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for modes muxed and D. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except in modes muxed and D mode. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or a different one in case of a read operation. Two consecutive synchronous read operations (in Burst or Single mode) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write access (in Burst or Single mode) and a synchronous read from the same or a different bank. The bus turnaround delay allows to match the minimum time between consecutive transactions (tEHEL from NEx high to NEx low) and the maximum time required by the memory to free the data bus after a read access (tEHQZ): (BUSTRUN + 1) KCK_FMC period &amp;#8805; tEHELmin and (BUSTRUN + 2)KCK_FMC period &amp;#8805; tEHQZmax if EXTMOD = 0 (BUSTRUN + 2)KCK_FMC period &amp;#8805; max (tEHELmin, tEHQZmax) if EXTMOD = 126. ..."]
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of KCK_FMC cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section20.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Data latency for synchronous memory For synchronous access with read write burst mode enabled these bits define the number of memory clock cycles"]
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Access mode These bits specify the asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure81 to Figure93), used in SRAMs, ROMs and asynchronous NOR Flash: For each access mode address setup phase duration, please refer to the respective figure (refer to Figure81 to Figure93). Note: In synchronous accesses, this value is dont care. In Muxed mode or Mode D, the minimum value for ADDSET is 1."]
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<BTR1rs> {
        ADDSET_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in mode D or multiplexed accesses: For each access mode address-hold phase duration, please refer to the respective figure (Figure81 to Figure93). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<BTR1rs> {
        ADDHLD_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous accesses: For each memory type and access mode data-phase duration, please refer to the respective figure (Figure81 to Figure93). Example: Mode1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 KCK_FMC clock cycles. Note: In synchronous accesses, this value is dont care."]
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<BTR1rs> {
        DATAST_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read or read-to write transaction. The programmed bus turnaround delay is inserted between an asynchronous read (in muxed or mode D) or write transaction and any other asynchronous /synchronous read/write from/to a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except in muxed mode and mode D. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for modes muxed and D. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except in modes muxed and D mode. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or a different one in case of a read operation. Two consecutive synchronous read operations (in Burst or Single mode) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write access (in Burst or Single mode) and a synchronous read from the same or a different bank. The bus turnaround delay allows to match the minimum time between consecutive transactions (tEHEL from NEx high to NEx low) and the maximum time required by the memory to free the data bus after a read access (tEHQZ): (BUSTRUN + 1) KCK_FMC period &amp;#8805; tEHELmin and (BUSTRUN + 2)KCK_FMC period &amp;#8805; tEHQZmax if EXTMOD = 0 (BUSTRUN + 2)KCK_FMC period &amp;#8805; max (tEHELmin, tEHQZmax) if EXTMOD = 126. ..."]
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<BTR1rs> {
        BUSTURN_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of KCK_FMC cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section20.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<BTR1rs> {
        CLKDIV_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Data latency for synchronous memory For synchronous access with read write burst mode enabled these bits define the number of memory clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn datlat(&mut self) -> DATLAT_W<BTR1rs> {
        DATLAT_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Access mode These bits specify the asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<BTR1rs> {
        ACCMOD_W::new(self, 28)
    }
}
#[doc = "This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, 2 registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTR1rs;
impl crate::RegisterSpec for BTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr1::R`](R) reader structure"]
impl crate::Readable for BTR1rs {}
#[doc = "`write(|w| ..)` method takes [`btr1::W`](W) writer structure"]
impl crate::Writable for BTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTR1 to value 0x0fff_ffff"]
impl crate::Resettable for BTR1rs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
