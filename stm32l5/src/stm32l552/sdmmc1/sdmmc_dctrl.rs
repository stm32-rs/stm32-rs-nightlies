#[doc = "Register `SDMMC_DCTRL` reader"]
pub type R = crate::R<SDMMC_DCTRLrs>;
#[doc = "Register `SDMMC_DCTRL` writer"]
pub type W = crate::W<SDMMC_DCTRLrs>;
#[doc = "Field `DTEN` reader - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
pub type DTEN_R = crate::BitReader;
#[doc = "Field `DTEN` writer - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDIR` reader - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DTDIR_R = crate::BitReader;
#[doc = "Field `DTDIR` writer - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTMODE` reader - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DTMODE_R = crate::FieldReader;
#[doc = "Field `DTMODE` writer - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBLOCKSIZE` reader - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
pub type DBLOCKSIZE_R = crate::FieldReader;
#[doc = "Field `DBLOCKSIZE` writer - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
pub type DBLOCKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RWSTART` reader - Read wait start. If this bit is set, read wait operation starts."]
pub type RWSTART_R = crate::BitReader;
#[doc = "Field `RWSTART` writer - Read wait start. If this bit is set, read wait operation starts."]
pub type RWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWSTOP` reader - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
pub type RWSTOP_R = crate::BitReader;
#[doc = "Field `RWSTOP` writer - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
pub type RWSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWMOD` reader - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type RWMOD_R = crate::BitReader;
#[doc = "Field `RWMOD` writer - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type RWMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOEN` reader - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
pub type SDIOEN_R = crate::BitReader;
#[doc = "Field `SDIOEN` writer - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
pub type SDIOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTACKEN` reader - Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type BOOTACKEN_R = crate::BitReader;
#[doc = "Field `BOOTACKEN` writer - Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type BOOTACKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFORST` reader - FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
pub type FIFORST_R = crate::BitReader;
#[doc = "Field `FIFORST` writer - FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
pub type FIFORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Read wait start. If this bit is set, read wait operation starts."]
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn bootacken(&self) -> BOOTACKEN_R {
        BOOTACKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
    #[inline(always)]
    pub fn fiforst(&self) -> FIFORST_R {
        FIFORST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<SDMMC_DCTRLrs> {
        DTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn dtdir(&mut self) -> DTDIR_W<SDMMC_DCTRLrs> {
        DTDIR_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn dtmode(&mut self) -> DTMODE_W<SDMMC_DCTRLrs> {
        DTMODE_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
    #[inline(always)]
    #[must_use]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<SDMMC_DCTRLrs> {
        DBLOCKSIZE_W::new(self, 4)
    }
    #[doc = "Bit 8 - Read wait start. If this bit is set, read wait operation starts."]
    #[inline(always)]
    #[must_use]
    pub fn rwstart(&mut self) -> RWSTART_W<SDMMC_DCTRLrs> {
        RWSTART_W::new(self, 8)
    }
    #[doc = "Bit 9 - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
    #[inline(always)]
    #[must_use]
    pub fn rwstop(&mut self) -> RWSTOP_W<SDMMC_DCTRLrs> {
        RWSTOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn rwmod(&mut self) -> RWMOD_W<SDMMC_DCTRLrs> {
        RWMOD_W::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<SDMMC_DCTRLrs> {
        SDIOEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn bootacken(&mut self) -> BOOTACKEN_W<SDMMC_DCTRLrs> {
        BOOTACKEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
    #[inline(always)]
    #[must_use]
    pub fn fiforst(&mut self) -> FIFORST_W<SDMMC_DCTRLrs> {
        FIFORST_W::new(self, 13)
    }
}
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_dctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_DCTRLrs;
impl crate::RegisterSpec for SDMMC_DCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_dctrl::R`](R) reader structure"]
impl crate::Readable for SDMMC_DCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_dctrl::W`](W) writer structure"]
impl crate::Writable for SDMMC_DCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_DCTRL to value 0"]
impl crate::Resettable for SDMMC_DCTRLrs {
    const RESET_VALUE: u32 = 0;
}
