///Register `BTR3` reader
pub type R = crate::R<BTR3rs>;
///Register `BTR3` writer
pub type W = crate::W<BTR3rs>;
///Field `ADDSET` reader - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure 109 to Figure 121), used in SRAMs, ROMs and asynchronous NOR flash: ... For each access mode address setup phase duration, please refer to the respective figure (refer to Figure 109 to Figure 121). Note: In synchronous accesses, this value is dont care. Note: In Muxed mode or mode D, the minimum value for ADDSET is 1. Note: In mode 1 and PSRAM memory, the minimum value for ADDSET is 1.
pub type ADDSET_R = crate::FieldReader;
///Field `ADDSET` writer - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure 109 to Figure 121), used in SRAMs, ROMs and asynchronous NOR flash: ... For each access mode address setup phase duration, please refer to the respective figure (refer to Figure 109 to Figure 121). Note: In synchronous accesses, this value is dont care. Note: In Muxed mode or mode D, the minimum value for ADDSET is 1. Note: In mode 1 and PSRAM memory, the minimum value for ADDSET is 1.
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDHLD` reader - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure 109 to Figure 121), used in mode D or multiplexed accesses: ... For each access mode address-hold phase duration, please refer to the respective figure (Figure 109 to Figure 121). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration.
pub type ADDHLD_R = crate::FieldReader;
///Field `ADDHLD` writer - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure 109 to Figure 121), used in mode D or multiplexed accesses: ... For each access mode address-hold phase duration, please refer to the respective figure (Figure 109 to Figure 121). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration.
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATAST` reader - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure 109 to Figure 121), used in asynchronous accesses: ... For each memory type and access mode data-phase duration, please refer to the respective figure (Figure 109 to Figure 121). Example: Mode1, write access, DATAST = 1: Data-phase duration = DATAST+1 = 1 x fmc_ker_ck clock cycles. Note: In synchronous accesses, this value is dont care.
pub type DATAST_R = crate::FieldReader;
///Field `DATAST` writer - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure 109 to Figure 121), used in asynchronous accesses: ... For each memory type and access mode data-phase duration, please refer to the respective figure (Figure 109 to Figure 121). Example: Mode1, write access, DATAST = 1: Data-phase duration = DATAST+1 = 1 x fmc_ker_ck clock cycles. Note: In synchronous accesses, this value is dont care.
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BUSTURN` reader - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read (and read-to-write) transaction. This delay allows to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from NEx high to NEx low) and the maximum time needed by the memory to free the data bus after a read access (t&lt;sub>EHQZ&lt;/sub>). The programmed bus turnaround delay is inserted between an asynchronous read (muxed or mode D) or write transaction and any other asynchronous /synchronous read or write to or from a static bank. The bank can be the same or different in case of read, in case of write the bank can be different except for muxed or mode D. In some cases, whatever the programmed BUSTURN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed and D modes. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for muxed and D modes. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except for muxed and D modes. An asynchronous (modes 1, 2, A, B or C) read and a read from another static bank. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or different for the case of read. Two consecutive synchronous reads (burst or single) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to different static bank. A synchronous write (burst or single) access and a synchronous read from the same or a different bank. ...
pub type BUSTURN_R = crate::FieldReader;
///Field `BUSTURN` writer - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read (and read-to-write) transaction. This delay allows to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from NEx high to NEx low) and the maximum time needed by the memory to free the data bus after a read access (t&lt;sub>EHQZ&lt;/sub>). The programmed bus turnaround delay is inserted between an asynchronous read (muxed or mode D) or write transaction and any other asynchronous /synchronous read or write to or from a static bank. The bank can be the same or different in case of read, in case of write the bank can be different except for muxed or mode D. In some cases, whatever the programmed BUSTURN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed and D modes. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for muxed and D modes. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except for muxed and D modes. An asynchronous (modes 1, 2, A, B or C) read and a read from another static bank. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or different for the case of read. Two consecutive synchronous reads (burst or single) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to different static bank. A synchronous write (burst or single) access and a synchronous read from the same or a different bank. ...
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CLKDIV` reader - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of fmc_ker_ck cycles: In asynchronous NOR flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section 23.7.5: Synchronous transactions for FMC_CLK divider ratio formula)
pub type CLKDIV_R = crate::FieldReader;
///Field `CLKDIV` writer - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of fmc_ker_ck cycles: In asynchronous NOR flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section 23.7.5: Synchronous transactions for FMC_CLK divider ratio formula)
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATLAT` reader - (see note below bit descriptions): Data latency for synchronous memory For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits set), these bits define the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data: This timing parameter is not expressed in fmc_ker_ck periods, but in FMC_CLK periods. For asynchronous access, this value is don't care.
pub type DATLAT_R = crate::FieldReader;
///Field `DATLAT` writer - (see note below bit descriptions): Data latency for synchronous memory For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits set), these bits define the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data: This timing parameter is not expressed in fmc_ker_ck periods, but in FMC_CLK periods. For asynchronous access, this value is don't care.
pub type DATLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ACCMOD` reader - Access mode These bits specify the Asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
pub type ACCMOD_R = crate::FieldReader;
///Field `ACCMOD` writer - Access mode These bits specify the Asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure 109 to Figure 121), used in SRAMs, ROMs and asynchronous NOR flash: ... For each access mode address setup phase duration, please refer to the respective figure (refer to Figure 109 to Figure 121). Note: In synchronous accesses, this value is dont care. Note: In Muxed mode or mode D, the minimum value for ADDSET is 1. Note: In mode 1 and PSRAM memory, the minimum value for ADDSET is 1.
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure 109 to Figure 121), used in mode D or multiplexed accesses: ... For each access mode address-hold phase duration, please refer to the respective figure (Figure 109 to Figure 121). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration.
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15 - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure 109 to Figure 121), used in asynchronous accesses: ... For each memory type and access mode data-phase duration, please refer to the respective figure (Figure 109 to Figure 121). Example: Mode1, write access, DATAST = 1: Data-phase duration = DATAST+1 = 1 x fmc_ker_ck clock cycles. Note: In synchronous accesses, this value is dont care.
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read (and read-to-write) transaction. This delay allows to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from NEx high to NEx low) and the maximum time needed by the memory to free the data bus after a read access (t&lt;sub>EHQZ&lt;/sub>). The programmed bus turnaround delay is inserted between an asynchronous read (muxed or mode D) or write transaction and any other asynchronous /synchronous read or write to or from a static bank. The bank can be the same or different in case of read, in case of write the bank can be different except for muxed or mode D. In some cases, whatever the programmed BUSTURN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed and D modes. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for muxed and D modes. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except for muxed and D modes. An asynchronous (modes 1, 2, A, B or C) read and a read from another static bank. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or different for the case of read. Two consecutive synchronous reads (burst or single) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to different static bank. A synchronous write (burst or single) access and a synchronous read from the same or a different bank. ...
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of fmc_ker_ck cycles: In asynchronous NOR flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section 23.7.5: Synchronous transactions for FMC_CLK divider ratio formula)
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - (see note below bit descriptions): Data latency for synchronous memory For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits set), these bits define the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data: This timing parameter is not expressed in fmc_ker_ck periods, but in FMC_CLK periods. For asynchronous access, this value is don't care.
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:29 - Access mode These bits specify the Asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTR3")
            .field("addset", &self.addset())
            .field("addhld", &self.addhld())
            .field("datast", &self.datast())
            .field("busturn", &self.busturn())
            .field("clkdiv", &self.clkdiv())
            .field("datlat", &self.datlat())
            .field("accmod", &self.accmod())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure 109 to Figure 121), used in SRAMs, ROMs and asynchronous NOR flash: ... For each access mode address setup phase duration, please refer to the respective figure (refer to Figure 109 to Figure 121). Note: In synchronous accesses, this value is dont care. Note: In Muxed mode or mode D, the minimum value for ADDSET is 1. Note: In mode 1 and PSRAM memory, the minimum value for ADDSET is 1.
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<BTR3rs> {
        ADDSET_W::new(self, 0)
    }
    ///Bits 4:7 - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure 109 to Figure 121), used in mode D or multiplexed accesses: ... For each access mode address-hold phase duration, please refer to the respective figure (Figure 109 to Figure 121). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration.
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<BTR3rs> {
        ADDHLD_W::new(self, 4)
    }
    ///Bits 8:15 - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure 109 to Figure 121), used in asynchronous accesses: ... For each memory type and access mode data-phase duration, please refer to the respective figure (Figure 109 to Figure 121). Example: Mode1, write access, DATAST = 1: Data-phase duration = DATAST+1 = 1 x fmc_ker_ck clock cycles. Note: In synchronous accesses, this value is dont care.
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<BTR3rs> {
        DATAST_W::new(self, 8)
    }
    ///Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read (and read-to-write) transaction. This delay allows to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from NEx high to NEx low) and the maximum time needed by the memory to free the data bus after a read access (t&lt;sub>EHQZ&lt;/sub>). The programmed bus turnaround delay is inserted between an asynchronous read (muxed or mode D) or write transaction and any other asynchronous /synchronous read or write to or from a static bank. The bank can be the same or different in case of read, in case of write the bank can be different except for muxed or mode D. In some cases, whatever the programmed BUSTURN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed and D modes. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for muxed and D modes. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except for muxed and D modes. An asynchronous (modes 1, 2, A, B or C) read and a read from another static bank. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or different for the case of read. Two consecutive synchronous reads (burst or single) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to different static bank. A synchronous write (burst or single) access and a synchronous read from the same or a different bank. ...
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<BTR3rs> {
        BUSTURN_W::new(self, 16)
    }
    ///Bits 20:23 - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of fmc_ker_ck cycles: In asynchronous NOR flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section 23.7.5: Synchronous transactions for FMC_CLK divider ratio formula)
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<BTR3rs> {
        CLKDIV_W::new(self, 20)
    }
    ///Bits 24:27 - (see note below bit descriptions): Data latency for synchronous memory For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits set), these bits define the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data: This timing parameter is not expressed in fmc_ker_ck periods, but in FMC_CLK periods. For asynchronous access, this value is don't care.
    #[inline(always)]
    #[must_use]
    pub fn datlat(&mut self) -> DATLAT_W<BTR3rs> {
        DATLAT_W::new(self, 24)
    }
    ///Bits 28:29 - Access mode These bits specify the Asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<BTR3rs> {
        ACCMOD_W::new(self, 28)
    }
}
/**SRAM/NOR-flash chip-select timing registers for bank 3

You can [`read`](crate::Reg::read) this register and get [`btr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FMC:BTR3)*/
pub struct BTR3rs;
impl crate::RegisterSpec for BTR3rs {
    type Ux = u32;
}
///`read()` method returns [`btr3::R`](R) reader structure
impl crate::Readable for BTR3rs {}
///`write(|w| ..)` method takes [`btr3::W`](W) writer structure
impl crate::Writable for BTR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BTR3 to value 0x0fff_ffff
impl crate::Resettable for BTR3rs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
