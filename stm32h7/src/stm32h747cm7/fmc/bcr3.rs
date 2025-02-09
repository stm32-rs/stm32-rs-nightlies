///Register `BCR3` reader
pub type R = crate::R<BCR3rs>;
///Register `BCR3` writer
pub type W = crate::W<BCR3rs>;
///Field `MBKEN` reader - Memory bank enable bit This bit enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AXI bus.
pub type MBKEN_R = crate::BitReader;
///Field `MBKEN` writer - Memory bank enable bit This bit enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AXI bus.
pub type MBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MUXEN` reader - Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:
pub type MUXEN_R = crate::BitReader;
///Field `MUXEN` writer - Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:
pub type MUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MTYP` reader - Memory type These bits define the type of external memory attached to the corresponding memory bank:
pub type MTYP_R = crate::FieldReader;
///Field `MTYP` writer - Memory type These bits define the type of external memory attached to the corresponding memory bank:
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MWID` reader - Memory data bus width Defines the external memory device width, valid for all type of memories.
pub type MWID_R = crate::FieldReader;
///Field `MWID` writer - Memory data bus width Defines the external memory device width, valid for all type of memories.
pub type MWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FACCEN` reader - Flash access enable This bit enables NOR Flash memory access operations.
pub type FACCEN_R = crate::BitReader;
///Field `FACCEN` writer - Flash access enable This bit enables NOR Flash memory access operations.
pub type FACCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BURSTEN` reader - Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode:
pub type BURSTEN_R = crate::BitReader;
///Field `BURSTEN` writer - Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode:
pub type BURSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITPOL` reader - Wait signal polarity bit This bit defines the polarity of the wait signal from memory used for either in synchronous or asynchronous mode:
pub type WAITPOL_R = crate::BitReader;
///Field `WAITPOL` writer - Wait signal polarity bit This bit defines the polarity of the wait signal from memory used for either in synchronous or asynchronous mode:
pub type WAITPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITCFG` reader - Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:
pub type WAITCFG_R = crate::BitReader;
///Field `WAITCFG` writer - Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:
pub type WAITCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WREN` reader - Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC:
pub type WREN_R = crate::BitReader;
///Field `WREN` writer - Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC:
pub type WREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAITEN` reader - Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in synchronous mode.
pub type WAITEN_R = crate::BitReader;
///Field `WAITEN` writer - Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in synchronous mode.
pub type WAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTMOD` reader - Extended mode enable. This bit enables the FMC to program the write timings for asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the extended mode is disabled, the FMC can operate in Mode1 or Mode2 as follows: ** Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP =0x0 or 0x01) ** Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10).
pub type EXTMOD_R = crate::BitReader;
///Field `EXTMOD` writer - Extended mode enable. This bit enables the FMC to program the write timings for asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the extended mode is disabled, the FMC can operate in Mode1 or Mode2 as follows: ** Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP =0x0 or 0x01) ** Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10).
pub type EXTMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ASYNCWAIT` reader - Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol.
pub type ASYNCWAIT_R = crate::BitReader;
///Field `ASYNCWAIT` writer - Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol.
pub type ASYNCWAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPSIZE` reader - CRAM Page Size These are used for Cellular RAM 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Other configuration: reserved.
pub type CPSIZE_R = crate::FieldReader;
///Field `CPSIZE` writer - CRAM Page Size These are used for Cellular RAM 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Other configuration: reserved.
pub type CPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CBURSTRW` reader - Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register.
pub type CBURSTRW_R = crate::BitReader;
///Field `CBURSTRW` writer - Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register.
pub type CBURSTRW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCLKEN` reader - Continuous Clock Enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in synchronous mode to generate the FMC_CLK continuous clock. If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is dont care. If the synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)
pub type CCLKEN_R = crate::BitReader;
///Field `CCLKEN` writer - Continuous Clock Enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in synchronous mode to generate the FMC_CLK continuous clock. If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is dont care. If the synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)
pub type CCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WFDIS` reader - Write FIFO Disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register.
pub type WFDIS_R = crate::BitReader;
///Field `WFDIS` writer - Write FIFO Disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register.
pub type WFDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BMAP` reader - FMC bank mapping These bits allows different to remap SDRAM bank2 or swap the FMC NOR/PSRAM and SDRAM banks.Refer to Table 10 for Note: The BMAP bits of the FMC_BCR2..4 registers are dont care. It is only enabled through the FMC_BCR1 register.
pub type BMAP_R = crate::FieldReader;
///Field `BMAP` writer - FMC bank mapping These bits allows different to remap SDRAM bank2 or swap the FMC NOR/PSRAM and SDRAM banks.Refer to Table 10 for Note: The BMAP bits of the FMC_BCR2..4 registers are dont care. It is only enabled through the FMC_BCR1 register.
pub type BMAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FMCEN` reader - FMC controller Enable This bit enables/disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register.
pub type FMCEN_R = crate::BitReader;
///Field `FMCEN` writer - FMC controller Enable This bit enables/disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register.
pub type FMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Memory bank enable bit This bit enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AXI bus.
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Memory type These bits define the type of external memory attached to the corresponding memory bank:
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Memory data bus width Defines the external memory device width, valid for all type of memories.
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Flash access enable This bit enables NOR Flash memory access operations.
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode:
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wait signal polarity bit This bit defines the polarity of the wait signal from memory used for either in synchronous or asynchronous mode:
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC:
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in synchronous mode.
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Extended mode enable. This bit enables the FMC to program the write timings for asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the extended mode is disabled, the FMC can operate in Mode1 or Mode2 as follows: ** Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP =0x0 or 0x01) ** Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10).
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol.
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - CRAM Page Size These are used for Cellular RAM 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Other configuration: reserved.
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 19 - Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register.
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Continuous Clock Enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in synchronous mode to generate the FMC_CLK continuous clock. If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is dont care. If the synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)
    #[inline(always)]
    pub fn cclken(&self) -> CCLKEN_R {
        CCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Write FIFO Disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register.
    #[inline(always)]
    pub fn wfdis(&self) -> WFDIS_R {
        WFDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 24:25 - FMC bank mapping These bits allows different to remap SDRAM bank2 or swap the FMC NOR/PSRAM and SDRAM banks.Refer to Table 10 for Note: The BMAP bits of the FMC_BCR2..4 registers are dont care. It is only enabled through the FMC_BCR1 register.
    #[inline(always)]
    pub fn bmap(&self) -> BMAP_R {
        BMAP_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 31 - FMC controller Enable This bit enables/disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register.
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR3")
            .field("mbken", &self.mbken())
            .field("muxen", &self.muxen())
            .field("mtyp", &self.mtyp())
            .field("mwid", &self.mwid())
            .field("faccen", &self.faccen())
            .field("bursten", &self.bursten())
            .field("waitpol", &self.waitpol())
            .field("waitcfg", &self.waitcfg())
            .field("wren", &self.wren())
            .field("waiten", &self.waiten())
            .field("extmod", &self.extmod())
            .field("asyncwait", &self.asyncwait())
            .field("cpsize", &self.cpsize())
            .field("cburstrw", &self.cburstrw())
            .field("cclken", &self.cclken())
            .field("wfdis", &self.wfdis())
            .field("bmap", &self.bmap())
            .field("fmcen", &self.fmcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Memory bank enable bit This bit enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a disabled bank causes an ERROR on AXI bus.
    #[inline(always)]
    pub fn mbken(&mut self) -> MBKEN_W<BCR3rs> {
        MBKEN_W::new(self, 0)
    }
    ///Bit 1 - Address/data multiplexing enable bit When this bit is set, the address and data values are multiplexed on the data bus, valid only with NOR and PSRAM memories:
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W<BCR3rs> {
        MUXEN_W::new(self, 1)
    }
    ///Bits 2:3 - Memory type These bits define the type of external memory attached to the corresponding memory bank:
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W<BCR3rs> {
        MTYP_W::new(self, 2)
    }
    ///Bits 4:5 - Memory data bus width Defines the external memory device width, valid for all type of memories.
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<BCR3rs> {
        MWID_W::new(self, 4)
    }
    ///Bit 6 - Flash access enable This bit enables NOR Flash memory access operations.
    #[inline(always)]
    pub fn faccen(&mut self) -> FACCEN_W<BCR3rs> {
        FACCEN_W::new(self, 6)
    }
    ///Bit 8 - Burst enable bit This bit enables/disables synchronous accesses during read operations. It is valid only for synchronous memories operating in Burst mode:
    #[inline(always)]
    pub fn bursten(&mut self) -> BURSTEN_W<BCR3rs> {
        BURSTEN_W::new(self, 8)
    }
    ///Bit 9 - Wait signal polarity bit This bit defines the polarity of the wait signal from memory used for either in synchronous or asynchronous mode:
    #[inline(always)]
    pub fn waitpol(&mut self) -> WAITPOL_W<BCR3rs> {
        WAITPOL_W::new(self, 9)
    }
    ///Bit 11 - Wait timing configuration The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be inserted when accessing the memory in synchronous mode. This configuration bit determines if NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:
    #[inline(always)]
    pub fn waitcfg(&mut self) -> WAITCFG_W<BCR3rs> {
        WAITCFG_W::new(self, 11)
    }
    ///Bit 12 - Write enable bit This bit indicates whether write operations are enabled/disabled in the bank by the FMC:
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<BCR3rs> {
        WREN_W::new(self, 12)
    }
    ///Bit 13 - Wait enable bit This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in synchronous mode.
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W<BCR3rs> {
        WAITEN_W::new(self, 13)
    }
    ///Bit 14 - Extended mode enable. This bit enables the FMC to program the write timings for asynchronous accesses inside the FMC_BWTR register, thus resulting in different timings for read and write operations. Note: When the extended mode is disabled, the FMC can operate in Mode1 or Mode2 as follows: ** Mode 1 is the default mode when the SRAM/PSRAM memory type is selected (MTYP =0x0 or 0x01) ** Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10).
    #[inline(always)]
    pub fn extmod(&mut self) -> EXTMOD_W<BCR3rs> {
        EXTMOD_W::new(self, 14)
    }
    ///Bit 15 - Wait signal during asynchronous transfers This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol.
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<BCR3rs> {
        ASYNCWAIT_W::new(self, 15)
    }
    ///Bits 16:18 - CRAM Page Size These are used for Cellular RAM 1.5 which does not allow burst access to cross the address boundaries between pages. When these bits are configured, the FMC controller splits automatically the burst access when the memory page size is reached (refer to memory datasheet for page size). Other configuration: reserved.
    #[inline(always)]
    pub fn cpsize(&mut self) -> CPSIZE_W<BCR3rs> {
        CPSIZE_W::new(self, 16)
    }
    ///Bit 19 - Write burst enable For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx register.
    #[inline(always)]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<BCR3rs> {
        CBURSTRW_W::new(self, 19)
    }
    ///Bit 20 - Continuous Clock Enable This bit enables the FMC_CLK clock output to external memory devices. Note: The CCLKEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register. Bank 1 must be configured in synchronous mode to generate the FMC_CLK continuous clock. If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1 register. CLKDIV in FMC_BWTR1 is dont care. If the synchronous mode is used and CCLKEN bit is set, the synchronous memories connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)
    #[inline(always)]
    pub fn cclken(&mut self) -> CCLKEN_W<BCR3rs> {
        CCLKEN_W::new(self, 20)
    }
    ///Bit 21 - Write FIFO Disable This bit disables the Write FIFO used by the FMC controller. Note: The WFDIS bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register.
    #[inline(always)]
    pub fn wfdis(&mut self) -> WFDIS_W<BCR3rs> {
        WFDIS_W::new(self, 21)
    }
    ///Bits 24:25 - FMC bank mapping These bits allows different to remap SDRAM bank2 or swap the FMC NOR/PSRAM and SDRAM banks.Refer to Table 10 for Note: The BMAP bits of the FMC_BCR2..4 registers are dont care. It is only enabled through the FMC_BCR1 register.
    #[inline(always)]
    pub fn bmap(&mut self) -> BMAP_W<BCR3rs> {
        BMAP_W::new(self, 24)
    }
    ///Bit 31 - FMC controller Enable This bit enables/disables the FMC controller. Note: The FMCEN bit of the FMC_BCR2..4 registers is dont care. It is only enabled through the FMC_BCR1 register.
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<BCR3rs> {
        FMCEN_W::new(self, 31)
    }
}
/**This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.

You can [`read`](crate::Reg::read) this register and get [`bcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#FMC:BCR3)*/
pub struct BCR3rs;
impl crate::RegisterSpec for BCR3rs {
    type Ux = u32;
}
///`read()` method returns [`bcr3::R`](R) reader structure
impl crate::Readable for BCR3rs {}
///`write(|w| ..)` method takes [`bcr3::W`](W) writer structure
impl crate::Writable for BCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BCR3 to value 0x30d2
impl crate::Resettable for BCR3rs {
    const RESET_VALUE: u32 = 0x30d2;
}
