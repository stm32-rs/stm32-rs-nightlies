///Register `SDCR1` reader
pub type R = crate::R<SDCR1rs>;
///Register `SDCR1` writer
pub type W = crate::W<SDCR1rs>;
///Field `NC` reader - Number of column address bits These bits define the number of bits of a column address.
pub type NC_R = crate::FieldReader;
///Field `NC` writer - Number of column address bits These bits define the number of bits of a column address.
pub type NC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NR` reader - Number of row address bits These bits define the number of bits of a row address.
pub type NR_R = crate::FieldReader;
///Field `NR` writer - Number of row address bits These bits define the number of bits of a row address.
pub type NR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MWID` reader - Memory data bus width. These bits define the memory device width.
pub type MWID_R = crate::FieldReader;
///Field `MWID` writer - Memory data bus width. These bits define the memory device width.
pub type MWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NB` reader - Number of internal banks This bit sets the number of internal banks.
pub type NB_R = crate::BitReader;
///Field `NB` writer - Number of internal banks This bit sets the number of internal banks.
pub type NB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAS` reader - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles
pub type CAS_R = crate::FieldReader;
///Field `CAS` writer - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles
pub type CAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WP` reader - Write protection This bit enables write mode access to the SDRAM bank.
pub type WP_R = crate::BitReader;
///Field `WP` writer - Write protection This bit enables write mode access to the SDRAM bank.
pub type WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDCLK` reader - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register are don’t care.
pub type SDCLK_R = crate::FieldReader;
///Field `SDCLK` writer - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register are don’t care.
pub type SDCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RBURST` reader - Burst read This bit enables Burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is don’t care.
pub type RBURST_R = crate::BitReader;
///Field `RBURST` writer - Burst read This bit enables Burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is don’t care.
pub type RBURST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIPE` reader - Read pipe These bits define the delay, in clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only.
pub type RPIPE_R = crate::FieldReader;
///Field `RPIPE` writer - Read pipe These bits define the delay, in clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only.
pub type RPIPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Number of column address bits These bits define the number of bits of a column address.
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Number of row address bits These bits define the number of bits of a row address.
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Memory data bus width. These bits define the memory device width.
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Number of internal banks This bit sets the number of internal banks.
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Write protection This bit enables write mode access to the SDRAM bank.
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register are don’t care.
    #[inline(always)]
    pub fn sdclk(&self) -> SDCLK_R {
        SDCLK_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Burst read This bit enables Burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is don’t care.
    #[inline(always)]
    pub fn rburst(&self) -> RBURST_R {
        RBURST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - Read pipe These bits define the delay, in clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only.
    #[inline(always)]
    pub fn rpipe(&self) -> RPIPE_R {
        RPIPE_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDCR1")
            .field("nc", &self.nc())
            .field("nr", &self.nr())
            .field("mwid", &self.mwid())
            .field("nb", &self.nb())
            .field("cas", &self.cas())
            .field("wp", &self.wp())
            .field("sdclk", &self.sdclk())
            .field("rburst", &self.rburst())
            .field("rpipe", &self.rpipe())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Number of column address bits These bits define the number of bits of a column address.
    #[inline(always)]
    pub fn nc(&mut self) -> NC_W<SDCR1rs> {
        NC_W::new(self, 0)
    }
    ///Bits 2:3 - Number of row address bits These bits define the number of bits of a row address.
    #[inline(always)]
    pub fn nr(&mut self) -> NR_W<SDCR1rs> {
        NR_W::new(self, 2)
    }
    ///Bits 4:5 - Memory data bus width. These bits define the memory device width.
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<SDCR1rs> {
        MWID_W::new(self, 4)
    }
    ///Bit 6 - Number of internal banks This bit sets the number of internal banks.
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W<SDCR1rs> {
        NB_W::new(self, 6)
    }
    ///Bits 7:8 - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W<SDCR1rs> {
        CAS_W::new(self, 7)
    }
    ///Bit 9 - Write protection This bit enables write mode access to the SDRAM bank.
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W<SDCR1rs> {
        WP_W::new(self, 9)
    }
    ///Bits 10:11 - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register are don’t care.
    #[inline(always)]
    pub fn sdclk(&mut self) -> SDCLK_W<SDCR1rs> {
        SDCLK_W::new(self, 10)
    }
    ///Bit 12 - Burst read This bit enables Burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is don’t care.
    #[inline(always)]
    pub fn rburst(&mut self) -> RBURST_W<SDCR1rs> {
        RBURST_W::new(self, 12)
    }
    ///Bits 13:14 - Read pipe These bits define the delay, in clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only.
    #[inline(always)]
    pub fn rpipe(&mut self) -> RPIPE_W<SDCR1rs> {
        RPIPE_W::new(self, 13)
    }
}
/**SDRAM control registers 1

You can [`read`](crate::Reg::read) this register and get [`sdcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FMC:SDCR1)*/
pub struct SDCR1rs;
impl crate::RegisterSpec for SDCR1rs {
    type Ux = u32;
}
///`read()` method returns [`sdcr1::R`](R) reader structure
impl crate::Readable for SDCR1rs {}
///`write(|w| ..)` method takes [`sdcr1::W`](W) writer structure
impl crate::Writable for SDCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDCR1 to value 0x02d0
impl crate::Resettable for SDCR1rs {
    const RESET_VALUE: u32 = 0x02d0;
}
