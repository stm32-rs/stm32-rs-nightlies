///Register `BWTR1` reader
pub type R = crate::R<BWTR1rs>;
///Register `BWTR1` writer
pub type W = crate::W<BWTR1rs>;
///Field `ADDSET` reader - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1.
pub type ADDSET_R = crate::FieldReader;
///Field `ADDSET` writer - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1.
pub type ADDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDHLD` reader - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration.
pub type ADDHLD_R = crate::FieldReader;
///Field `ADDHLD` writer - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration.
pub type ADDHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DATAST` reader - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ...
pub type DATAST_R = crate::FieldReader;
///Field `DATAST` writer - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ...
pub type DATAST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BUSTURN` reader - Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t PC (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period greater than or equal tPC min ...
pub type BUSTURN_R = crate::FieldReader;
///Field `BUSTURN` writer - Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t PC (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period greater than or equal tPC min ...
pub type BUSTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ACCMOD` reader - Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
pub type ACCMOD_R = crate::FieldReader;
///Field `ACCMOD` writer - Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
pub type ACCMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DATAHLD` reader - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:
pub type DATAHLD_R = crate::FieldReader;
///Field `DATAHLD` writer - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:
pub type DATAHLD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1.
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration.
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15 - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ...
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t PC (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period greater than or equal tPC min ...
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 28:29 - Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:
    #[inline(always)]
    pub fn datahld(&self) -> DATAHLD_R {
        DATAHLD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BWTR1")
            .field("addset", &self.addset())
            .field("addhld", &self.addhld())
            .field("datast", &self.datast())
            .field("busturn", &self.busturn())
            .field("accmod", &self.accmod())
            .field("datahld", &self.datahld())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Address setup phase duration. These bits are written by software to define the duration of the address setup phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous accesses: ... Note: In synchronous accesses, this value is not used, the address setup phase is always 1 Flash clock period duration. In muxed mode, the minimum ADDSET value is 1.
    #[inline(always)]
    pub fn addset(&mut self) -> ADDSET_W<BWTR1rs> {
        ADDSET_W::new(self, 0)
    }
    ///Bits 4:7 - Address-hold phase duration. These bits are written by software to define the duration of the address hold phase (refer to Figure 30 to Figure 33), used in asynchronous multiplexed accesses: ... Note: In synchronous NOR Flash accesses, this value is not used, the address hold phase is always 1 Flash clock period duration.
    #[inline(always)]
    pub fn addhld(&mut self) -> ADDHLD_W<BWTR1rs> {
        ADDHLD_W::new(self, 4)
    }
    ///Bits 8:15 - Data-phase duration. These bits are written by software to define the duration of the data phase (refer to Figure 21 to Figure 33), used in asynchronous SRAM, PSRAM and NOR Flash memory accesses: ...
    #[inline(always)]
    pub fn datast(&mut self) -> DATAST_W<BWTR1rs> {
        DATAST_W::new(self, 8)
    }
    ///Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of current write transaction to next transaction on the same bank. For FRAM memories, the bus turnaround delay should be configured to match the minimum t PC (precharge time) timings. The bus turnaround delay is inserted between any consecutive transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is toggling between any consecutive accesses. (BUSTURN + 1)HCLK period greater than or equal tPC min ...
    #[inline(always)]
    pub fn busturn(&mut self) -> BUSTURN_W<BWTR1rs> {
        BUSTURN_W::new(self, 16)
    }
    ///Bits 28:29 - Access mode. Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
    #[inline(always)]
    pub fn accmod(&mut self) -> ACCMOD_W<BWTR1rs> {
        ACCMOD_W::new(self, 28)
    }
    ///Bits 30:31 - Data hold phase duration These bits are written by software to define the duration of the data hold phase in HCLK cycles (refer to Figure 21 to Figure 33), used in asynchronous write accesses:
    #[inline(always)]
    pub fn datahld(&mut self) -> DATAHLD_W<BWTR1rs> {
        DATAHLD_W::new(self, 30)
    }
}
/**SRAM/NOR-Flash write timing registers 1

You can [`read`](crate::Reg::read) this register and get [`bwtr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FMC:BWTR1)*/
pub struct BWTR1rs;
impl crate::RegisterSpec for BWTR1rs {
    type Ux = u32;
}
///`read()` method returns [`bwtr1::R`](R) reader structure
impl crate::Readable for BWTR1rs {}
///`write(|w| ..)` method takes [`bwtr1::W`](W) writer structure
impl crate::Writable for BWTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BWTR1 to value 0x0fff_ffff
impl crate::Resettable for BWTR1rs {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
