///Register `PMEM` reader
pub type R = crate::R<PMEMrs>;
///Register `PMEM` writer
pub type W = crate::W<PMEMrs>;
///Field `MEMSET` reader - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:
pub type MEMSET_R = crate::FieldReader;
///Field `MEMSET` writer - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:
pub type MEMSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MEMWAIT` reader - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:
pub type MEMWAIT_R = crate::FieldReader;
///Field `MEMWAIT` writer - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:
pub type MEMWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MEMHOLD` reader - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:
pub type MEMHOLD_R = crate::FieldReader;
///Field `MEMHOLD` writer - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:
pub type MEMHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MEMHIZ` reader - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:
pub type MEMHIZ_R = crate::FieldReader;
///Field `MEMHIZ` writer - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:
pub type MEMHIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:
    #[inline(always)]
    pub fn memhold(&self) -> MEMHOLD_R {
        MEMHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:
    #[inline(always)]
    pub fn memhiz(&self) -> MEMHIZ_R {
        MEMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMEM")
            .field("memset", &self.memset())
            .field("memwait", &self.memwait())
            .field("memhold", &self.memhold())
            .field("memhiz", &self.memhiz())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Common memory x setup time These bits define the number of KCK_FMC (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space:
    #[inline(always)]
    pub fn memset(&mut self) -> MEMSET_W<'_, PMEMrs> {
        MEMSET_W::new(self, 0)
    }
    ///Bits 8:15 - Common memory wait time These bits define the minimum number of KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:
    #[inline(always)]
    pub fn memwait(&mut self) -> MEMWAIT_W<'_, PMEMrs> {
        MEMWAIT_W::new(self, 8)
    }
    ///Bits 16:23 - Common memory hold time These bits define the number of KCK_FMC clock cycles for write accesses and KCK_FMC+1 clock cycles for read accesses during which the address is held (and data for write accesses) after the command is de-asserted (NWE, NOE), for NAND Flash read or write access to common memory space:
    #[inline(always)]
    pub fn memhold(&mut self) -> MEMHOLD_W<'_, PMEMrs> {
        MEMHOLD_W::new(self, 16)
    }
    ///Bits 24:31 - Common memory x data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space. This is only valid for write transactions:
    #[inline(always)]
    pub fn memhiz(&mut self) -> MEMHIZ_W<'_, PMEMrs> {
        MEMHIZ_W::new(self, 24)
    }
}
/**The FMC_PMEM read/write register contains the timing information for NAND Flash memory bank. This information is used to access either the common memory space of the NAND Flash for command, address write access and data read/write access.

You can [`read`](crate::Reg::read) this register and get [`pmem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#FMC:PMEM)*/
pub struct PMEMrs;
impl crate::RegisterSpec for PMEMrs {
    type Ux = u32;
}
///`read()` method returns [`pmem::R`](R) reader structure
impl crate::Readable for PMEMrs {}
///`write(|w| ..)` method takes [`pmem::W`](W) writer structure
impl crate::Writable for PMEMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMEM to value 0xfcfc_fcfc
impl crate::Resettable for PMEMrs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
