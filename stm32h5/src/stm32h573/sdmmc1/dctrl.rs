///Register `DCTRL` reader
pub type R = crate::R<DCTRLrs>;
///Register `DCTRL` writer
pub type W = crate::W<DCTRLrs>;
///Field `DTEN` reader - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit must only be used to transfer data when no associated data transfer command is used, i.e. must not be used with SD or eMMC cards.
pub type DTEN_R = crate::BitReader;
///Field `DTEN` writer - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit must only be used to transfer data when no associated data transfer command is used, i.e. must not be used with SD or eMMC cards.
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTDIR` reader - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type DTDIR_R = crate::BitReader;
///Field `DTDIR` writer - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type DTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTMODE` reader - Data transfer mode selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type DTMODE_R = crate::FieldReader;
///Field `DTMODE` writer - Data transfer mode selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type DTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DBLOCKSIZE` reader - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (None of the remaining data are transfered.) When DDR = 1, DBLOCKSIZE = 0000 must not be used. (No data are transfered)
pub type DBLOCKSIZE_R = crate::FieldReader;
///Field `DBLOCKSIZE` writer - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (None of the remaining data are transfered.) When DDR = 1, DBLOCKSIZE = 0000 must not be used. (No data are transfered)
pub type DBLOCKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RWSTART` reader - Read Wait start If this bit is set, Read Wait operation starts.
pub type RWSTART_R = crate::BitReader;
///Field `RWSTART` writer - Read Wait start If this bit is set, Read Wait operation starts.
pub type RWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWSTOP` reader - Read Wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the R_W state to the Wait_R or Idle state.
pub type RWSTOP_R = crate::BitReader;
///Field `RWSTOP` writer - Read Wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the R_W state to the Wait_R or Idle state.
pub type RWSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWMOD` reader - Read Wait mode This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type RWMOD_R = crate::BitReader;
///Field `RWMOD` writer - Read Wait mode This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type RWMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIOEN` reader - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation.
pub type SDIOEN_R = crate::BitReader;
///Field `SDIOEN` writer - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation.
pub type SDIOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOOTACKEN` reader - Enable the reception of the boot acknowledgment This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type BOOTACKEN_R = crate::BitReader;
///Field `BOOTACKEN` writer - Enable the reception of the boot acknowledgment This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type BOOTACKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFORST` reader - FIFO reset, flushes any remaining data This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit only takes effect when a transfer error or transfer hold occurs.
pub type FIFORST_R = crate::BitReader;
///Field `FIFORST` writer - FIFO reset, flushes any remaining data This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit only takes effect when a transfer error or transfer hold occurs.
pub type FIFORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit must only be used to transfer data when no associated data transfer command is used, i.e. must not be used with SD or eMMC cards.
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Data transfer mode selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (None of the remaining data are transfered.) When DDR = 1, DBLOCKSIZE = 0000 must not be used. (No data are transfered)
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Read Wait start If this bit is set, Read Wait operation starts.
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Read Wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the R_W state to the Wait_R or Idle state.
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Read Wait mode This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation.
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable the reception of the boot acknowledgment This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn bootacken(&self) -> BOOTACKEN_R {
        BOOTACKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - FIFO reset, flushes any remaining data This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit only takes effect when a transfer error or transfer hold occurs.
    #[inline(always)]
    pub fn fiforst(&self) -> FIFORST_R {
        FIFORST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTRL")
            .field("dten", &self.dten())
            .field("dtdir", &self.dtdir())
            .field("dtmode", &self.dtmode())
            .field("dblocksize", &self.dblocksize())
            .field("rwstart", &self.rwstart())
            .field("rwstop", &self.rwstop())
            .field("rwmod", &self.rwmod())
            .field("sdioen", &self.sdioen())
            .field("bootacken", &self.bootacken())
            .field("fiforst", &self.fiforst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit must only be used to transfer data when no associated data transfer command is used, i.e. must not be used with SD or eMMC cards.
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W<DCTRLrs> {
        DTEN_W::new(self, 0)
    }
    ///Bit 1 - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn dtdir(&mut self) -> DTDIR_W<DCTRLrs> {
        DTDIR_W::new(self, 1)
    }
    ///Bits 2:3 - Data transfer mode selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn dtmode(&mut self) -> DTMODE_W<DCTRLrs> {
        DTMODE_W::new(self, 2)
    }
    ///Bits 4:7 - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (None of the remaining data are transfered.) When DDR = 1, DBLOCKSIZE = 0000 must not be used. (No data are transfered)
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<DCTRLrs> {
        DBLOCKSIZE_W::new(self, 4)
    }
    ///Bit 8 - Read Wait start If this bit is set, Read Wait operation starts.
    #[inline(always)]
    pub fn rwstart(&mut self) -> RWSTART_W<DCTRLrs> {
        RWSTART_W::new(self, 8)
    }
    ///Bit 9 - Read Wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the R_W state to the Wait_R or Idle state.
    #[inline(always)]
    pub fn rwstop(&mut self) -> RWSTOP_W<DCTRLrs> {
        RWSTOP_W::new(self, 9)
    }
    ///Bit 10 - Read Wait mode This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn rwmod(&mut self) -> RWMOD_W<DCTRLrs> {
        RWMOD_W::new(self, 10)
    }
    ///Bit 11 - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation.
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W<DCTRLrs> {
        SDIOEN_W::new(self, 11)
    }
    ///Bit 12 - Enable the reception of the boot acknowledgment This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn bootacken(&mut self) -> BOOTACKEN_W<DCTRLrs> {
        BOOTACKEN_W::new(self, 12)
    }
    ///Bit 13 - FIFO reset, flushes any remaining data This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit only takes effect when a transfer error or transfer hold occurs.
    #[inline(always)]
    pub fn fiforst(&mut self) -> FIFORST_W<DCTRLrs> {
        FIFORST_W::new(self, 13)
    }
}
/**SDMMC data control register

You can [`read`](crate::Reg::read) this register and get [`dctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SDMMC1:DCTRL)*/
pub struct DCTRLrs;
impl crate::RegisterSpec for DCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dctrl::R`](R) reader structure
impl crate::Readable for DCTRLrs {}
///`write(|w| ..)` method takes [`dctrl::W`](W) writer structure
impl crate::Writable for DCTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCTRL to value 0
impl crate::Resettable for DCTRLrs {}
