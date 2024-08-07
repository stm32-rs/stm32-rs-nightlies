///Register `BWTR3` reader
pub type R = crate::R<BWTR3rs>;
///Register `BWTR3` writer
pub type W = crate::W<BWTR3rs>;
///Field `ADDSET` reader - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in fmc_ker_ck cycles (refer to Figure 109 to Figure 121), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 flash clock period duration. In muxed mode, the minimum ADDSET value is 1.
pub type ADDSET_R = crate::FieldReader;
///Field `ADDSET` writer - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in fmc_ker_ck cycles (refer to Figure 109 to Figure 121), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 flash clock period duration. In muxed mode, the minimum ADDSET value is 1.
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDHLD` reader - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 109 to Figure 121), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR flash accesses, this value is not used, the address hold phase is always 1 flash clock period duration.
pub type ADDHLD_R = crate::FieldReader;
///Field `ADDHLD` writer - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 109 to Figure 121), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR flash accesses, this value is not used, the address hold phase is always 1 flash clock period duration.
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATAST` reader - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 109 to Figure 121), used in asynchronous SRAM, PSRAM and NOR flash memory accesses: ...
pub type DATAST_R = crate::FieldReader;
///Field `DATAST` writer - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 109 to Figure 121), used in asynchronous SRAM, PSRAM and NOR flash memory accesses: ...
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BUSTURN` reader - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from ENx high to ENx low): (BUSTRUN + 1) fmc_ker_ck period more or equal to t&lt;sub>EHELmin&lt;/sub>. The programmed bus turnaround delay is inserted between an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. The bank can be the same or different in case of read, in case of write the bank can be different expect for muxed or mode D. In some cases, whatever the programmed BUSTURN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed and D modes. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to the same bank. A synchronous write (burst or single) transfer and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to different static bank. A synchronous write (burst or single) transfer and a synchronous read from the same or a different bank. ...
pub type BUSTURN_R = crate::FieldReader;
///Field `BUSTURN` writer - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from ENx high to ENx low): (BUSTRUN + 1) fmc_ker_ck period more or equal to t&lt;sub>EHELmin&lt;/sub>. The programmed bus turnaround delay is inserted between an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. The bank can be the same or different in case of read, in case of write the bank can be different expect for muxed or mode D. In some cases, whatever the programmed BUSTURN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed and D modes. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to the same bank. A synchronous write (burst or single) transfer and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to different static bank. A synchronous write (burst or single) transfer and a synchronous read from the same or a different bank. ...
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ACCMOD` reader - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
pub type ACCMOD_R = crate::FieldReader;
///Field `ACCMOD` writer - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in fmc_ker_ck cycles (refer to Figure 109 to Figure 121), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 flash clock period duration. In muxed mode, the minimum ADDSET value is 1.
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 109 to Figure 121), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR flash accesses, this value is not used, the address hold phase is always 1 flash clock period duration.
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15 - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 109 to Figure 121), used in asynchronous SRAM, PSRAM and NOR flash memory accesses: ...
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from ENx high to ENx low): (BUSTRUN + 1) fmc_ker_ck period more or equal to t&lt;sub>EHELmin&lt;/sub>. The programmed bus turnaround delay is inserted between an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. The bank can be the same or different in case of read, in case of write the bank can be different expect for muxed or mode D. In some cases, whatever the programmed BUSTURN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed and D modes. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to the same bank. A synchronous write (burst or single) transfer and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to different static bank. A synchronous write (burst or single) transfer and a synchronous read from the same or a different bank. ...
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 28:29 - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BWTR3")
            .field("addset", &self.addset())
            .field("addhld", &self.addhld())
            .field("datast", &self.datast())
            .field("busturn", &self.busturn())
            .field("accmod", &self.accmod())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in fmc_ker_ck cycles (refer to Figure 109 to Figure 121), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 flash clock period duration. In muxed mode, the minimum ADDSET value is 1.
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<BWTR3rs> {
        ADDSET_W::new(self, 0)
    }
    ///Bits 4:7 - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 109 to Figure 121), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR flash accesses, this value is not used, the address hold phase is always 1 flash clock period duration.
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<BWTR3rs> {
        ADDHLD_W::new(self, 4)
    }
    ///Bits 8:15 - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 109 to Figure 121), used in asynchronous SRAM, PSRAM and NOR flash memory accesses: ...
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<BWTR3rs> {
        DATAST_W::new(self, 8)
    }
    ///Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write transaction to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from ENx high to ENx low): (BUSTRUN + 1) fmc_ker_ck period more or equal to t&lt;sub>EHELmin&lt;/sub>. The programmed bus turnaround delay is inserted between an asynchronous write transfer and any other asynchronous /synchronous read or write transfer to or from a static bank. The bank can be the same or different in case of read, in case of write the bank can be different expect for muxed or mode D. In some cases, whatever the programmed BUSTURN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except for muxed and D modes. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to the same bank. A synchronous write (burst or single) transfer and an asynchronous write or read transfer to or from static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous writes (burst or single) to different static bank. A synchronous write (burst or single) transfer and a synchronous read from the same or a different bank. ...
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<BWTR3rs> {
        BUSTURN_W::new(self, 16)
    }
    ///Bits 28:29 - Access mode. These bits specify the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<BWTR3rs> {
        ACCMOD_W::new(self, 28)
    }
}
/**SRAM/NOR-flash write timing registers for bank 3

You can [`read`](crate::Reg::read) this register and get [`bwtr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FMC:BWTR3)*/
pub struct BWTR3rs;
impl crate::RegisterSpec for BWTR3rs {
    type Ux = u32;
}
///`read()` method returns [`bwtr3::R`](R) reader structure
impl crate::Readable for BWTR3rs {}
///`write(|w| ..)` method takes [`bwtr3::W`](W) writer structure
impl crate::Writable for BWTR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BWTR3 to value 0x0fff_ffff
impl crate::Resettable for BWTR3rs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
