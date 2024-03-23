#[doc = "Register `BTR1` reader"]
pub type R = crate::R<BTR1rs>;
#[doc = "Register `BTR1` writer"]
pub type W = crate::W<BTR1rs>;
#[doc = "Field `ADDSET` reader - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure 21 to Figure 33), used in SRAMs, ROMs, asynchronous NOR Flash and PSRAM: ... For each access mode address setup phase duration, refer to the respective figure (Figure 21 to Figure 33). Note: In synchronous accesses, this value is don’t care. Note: In Muxed mode or mode D, the minimum value for ADDSET is 1. Note: In mode 1 and PSRAM memory, the minimum value for ADDSET is 1."]
pub type ADDSET_R = crate::FieldReader;
#[doc = "Field `ADDSET` writer - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure 21 to Figure 33), used in SRAMs, ROMs, asynchronous NOR Flash and PSRAM: ... For each access mode address setup phase duration, refer to the respective figure (Figure 21 to Figure 33). Note: In synchronous accesses, this value is don’t care. Note: In Muxed mode or mode D, the minimum value for ADDSET is 1. Note: In mode 1 and PSRAM memory, the minimum value for ADDSET is 1."]
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDHLD` reader - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure 21 to Figure 33), used in mode D or multiplexed accesses: ... For each access mode address-hold phase duration, refer to the respective figure (Figure 21 to Figure 33). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
pub type ADDHLD_R = crate::FieldReader;
#[doc = "Field `ADDHLD` writer - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure 21 to Figure 33), used in mode D or multiplexed accesses: ... For each access mode address-hold phase duration, refer to the respective figure (Figure 21 to Figure 33). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAST` reader - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... For each memory type and access mode data-phase duration, refer to the respective figure (Figure 21 to Figure 33). Example: Mode 1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 HCLK clock cycles. Note: In synchronous accesses, this value is don’t care."]
pub type DATAST_R = crate::FieldReader;
#[doc = "Field `DATAST` writer - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... For each memory type and access mode data-phase duration, refer to the respective figure (Figure 21 to Figure 33). Example: Mode 1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 HCLK clock cycles. Note: In synchronous accesses, this value is don’t care."]
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSTURN` reader - Bus turnaround phase duration These bits are written by software to add a delay at the end of current read or write transaction to next transaction on the same bank. This delay allows to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from NEx high to NEx low) and the maximum time needed by the memory to free the data bus after a read access (t&lt;sub>EHQZ&lt;/sub>, chip enable high to output Hi-Z). This delay is recommended for mode D and muxed mode. For non-muxed memory, the bus turnaround delay can be set to minimum value. (BUSTURN + 1)HCLK period ≥ max(t&lt;sub>EHEL&lt;/sub> min, t&lt;sub>EHQZ&lt;/sub> max) For FRAM memories, the bus turnaround delay should be configured to match the minimum tPC (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read) to match the tPC memory timing. The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ t&lt;sub>PC&lt;/sub> min ..."]
pub type BUSTURN_R = crate::FieldReader;
#[doc = "Field `BUSTURN` writer - Bus turnaround phase duration These bits are written by software to add a delay at the end of current read or write transaction to next transaction on the same bank. This delay allows to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from NEx high to NEx low) and the maximum time needed by the memory to free the data bus after a read access (t&lt;sub>EHQZ&lt;/sub>, chip enable high to output Hi-Z). This delay is recommended for mode D and muxed mode. For non-muxed memory, the bus turnaround delay can be set to minimum value. (BUSTURN + 1)HCLK period ≥ max(t&lt;sub>EHEL&lt;/sub> min, t&lt;sub>EHQZ&lt;/sub> max) For FRAM memories, the bus turnaround delay should be configured to match the minimum tPC (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read) to match the tPC memory timing. The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ t&lt;sub>PC&lt;/sub> min ..."]
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKDIV` reader - Clock divide ratio (for FMC_CLK signal) Defines the period of FMC_CLK clock output signal, expressed in number of HCLK cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is don’t care. Note: Refer to Section 5.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock divide ratio (for FMC_CLK signal) Defines the period of FMC_CLK clock output signal, expressed in number of HCLK cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is don’t care. Note: Refer to Section 5.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATLAT` reader - (see note below bit descriptions): Data latency for synchronous memory For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits set), defines the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data: This timing parameter is not expressed in HCLK periods, but in FMC_CLK periods. For asynchronous access, this value is don't care."]
pub type DATLAT_R = crate::FieldReader;
#[doc = "Field `DATLAT` writer - (see note below bit descriptions): Data latency for synchronous memory For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits set), defines the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data: This timing parameter is not expressed in HCLK periods, but in FMC_CLK periods. For asynchronous access, this value is don't care."]
pub type DATLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACCMOD` reader - Access mode Specifies the asynchronous access modes as shown in the timing diagrams. These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub type ACCMOD_R = crate::FieldReader;
#[doc = "Field `ACCMOD` writer - Access mode Specifies the asynchronous access modes as shown in the timing diagrams. These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATAHLD` reader - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: For read accesses For write accesses"]
pub type DATAHLD_R = crate::FieldReader;
#[doc = "Field `DATAHLD` writer - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: For read accesses For write accesses"]
pub type DATAHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure 21 to Figure 33), used in SRAMs, ROMs, asynchronous NOR Flash and PSRAM: ... For each access mode address setup phase duration, refer to the respective figure (Figure 21 to Figure 33). Note: In synchronous accesses, this value is don’t care. Note: In Muxed mode or mode D, the minimum value for ADDSET is 1. Note: In mode 1 and PSRAM memory, the minimum value for ADDSET is 1."]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure 21 to Figure 33), used in mode D or multiplexed accesses: ... For each access mode address-hold phase duration, refer to the respective figure (Figure 21 to Figure 33). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... For each memory type and access mode data-phase duration, refer to the respective figure (Figure 21 to Figure 33). Example: Mode 1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 HCLK clock cycles. Note: In synchronous accesses, this value is don’t care."]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of current read or write transaction to next transaction on the same bank. This delay allows to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from NEx high to NEx low) and the maximum time needed by the memory to free the data bus after a read access (t&lt;sub>EHQZ&lt;/sub>, chip enable high to output Hi-Z). This delay is recommended for mode D and muxed mode. For non-muxed memory, the bus turnaround delay can be set to minimum value. (BUSTURN + 1)HCLK period ≥ max(t&lt;sub>EHEL&lt;/sub> min, t&lt;sub>EHQZ&lt;/sub> max) For FRAM memories, the bus turnaround delay should be configured to match the minimum tPC (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read) to match the tPC memory timing. The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ t&lt;sub>PC&lt;/sub> min ..."]
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Clock divide ratio (for FMC_CLK signal) Defines the period of FMC_CLK clock output signal, expressed in number of HCLK cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is don’t care. Note: Refer to Section 5.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - (see note below bit descriptions): Data latency for synchronous memory For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits set), defines the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data: This timing parameter is not expressed in HCLK periods, but in FMC_CLK periods. For asynchronous access, this value is don't care."]
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Access mode Specifies the asynchronous access modes as shown in the timing diagrams. These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: For read accesses For write accesses"]
    #[inline(always)]
    pub fn datahld(&self) -> DATAHLD_R {
        DATAHLD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure 21 to Figure 33), used in SRAMs, ROMs, asynchronous NOR Flash and PSRAM: ... For each access mode address setup phase duration, refer to the respective figure (Figure 21 to Figure 33). Note: In synchronous accesses, this value is don’t care. Note: In Muxed mode or mode D, the minimum value for ADDSET is 1. Note: In mode 1 and PSRAM memory, the minimum value for ADDSET is 1."]
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<BTR1rs> {
        ADDSET_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure 21 to Figure 33), used in mode D or multiplexed accesses: ... For each access mode address-hold phase duration, refer to the respective figure (Figure 21 to Figure 33). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<BTR1rs> {
        ADDHLD_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... For each memory type and access mode data-phase duration, refer to the respective figure (Figure 21 to Figure 33). Example: Mode 1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 HCLK clock cycles. Note: In synchronous accesses, this value is don’t care."]
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<BTR1rs> {
        DATAST_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of current read or write transaction to next transaction on the same bank. This delay allows to match the minimum time between consecutive transactions (t&lt;sub>EHEL&lt;/sub> from NEx high to NEx low) and the maximum time needed by the memory to free the data bus after a read access (t&lt;sub>EHQZ&lt;/sub>, chip enable high to output Hi-Z). This delay is recommended for mode D and muxed mode. For non-muxed memory, the bus turnaround delay can be set to minimum value. (BUSTURN + 1)HCLK period ≥ max(t&lt;sub>EHEL&lt;/sub> min, t&lt;sub>EHQZ&lt;/sub> max) For FRAM memories, the bus turnaround delay should be configured to match the minimum tPC (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read) to match the tPC memory timing. The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period ≥ t&lt;sub>PC&lt;/sub> min ..."]
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<BTR1rs> {
        BUSTURN_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Clock divide ratio (for FMC_CLK signal) Defines the period of FMC_CLK clock output signal, expressed in number of HCLK cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is don’t care. Note: Refer to Section 5.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<BTR1rs> {
        CLKDIV_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - (see note below bit descriptions): Data latency for synchronous memory For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits set), defines the number of memory clock cycles (+2) to issue to the memory before reading/writing the first data: This timing parameter is not expressed in HCLK periods, but in FMC_CLK periods. For asynchronous access, this value is don't care."]
    #[inline(always)]
    #[must_use]
    pub fn datlat(&mut self) -> DATLAT_W<BTR1rs> {
        DATLAT_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Access mode Specifies the asynchronous access modes as shown in the timing diagrams. These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<BTR1rs> {
        ACCMOD_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: For read accesses For write accesses"]
    #[inline(always)]
    #[must_use]
    pub fn datahld(&mut self) -> DATAHLD_W<BTR1rs> {
        DATAHLD_W::new(self, 30)
    }
}
#[doc = "SRAM/NOR-Flash chip-select timing register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
