#[doc = "Register `SPI_CFG2` reader"]
pub type R = crate::R<SPI_CFG2rs>;
#[doc = "Register `SPI_CFG2` writer"]
pub type W = crate::W<SPI_CFG2rs>;
#[doc = "Field `MSSI` reader - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
pub type MSSI_R = crate::FieldReader;
#[doc = "Field `MSSI` writer - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
pub type MSSI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MIDI` reader - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
pub type MIDI_R = crate::FieldReader;
#[doc = "Field `MIDI` writer - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
pub type MIDI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RDIOM` reader - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
pub type RDIOM_R = crate::BitReader;
#[doc = "Field `RDIOM` writer - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
pub type RDIOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIOP` reader - RDY signal input/output polarity"]
pub type RDIOP_R = crate::BitReader;
#[doc = "Field `RDIOP` writer - RDY signal input/output polarity"]
pub type RDIOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSWP` reader - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO."]
pub type IOSWP_R = crate::BitReader;
#[doc = "Field `IOSWP` writer - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO."]
pub type IOSWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMM` reader - SPI Communication Mode"]
pub type COMM_R = crate::FieldReader;
#[doc = "Field `COMM` writer - SPI Communication Mode"]
pub type COMM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SP` reader - serial protocol others: reserved, must not be used"]
pub type SP_R = crate::FieldReader;
#[doc = "Field `SP` writer - serial protocol others: reserved, must not be used"]
pub type SP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MASTER` reader - SPI Master"]
pub type MASTER_R = crate::BitReader;
#[doc = "Field `MASTER` writer - SPI Master"]
pub type MASTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBFRST` reader - data frame format"]
pub type LSBFRST_R = crate::BitReader;
#[doc = "Field `LSBFRST` writer - data frame format"]
pub type LSBFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - clock phase"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - clock phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - clock polarity"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - clock polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
pub type SSM_R = crate::BitReader;
#[doc = "Field `SSM` writer - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSIOP` reader - SS input/output polarity"]
pub type SSIOP_R = crate::BitReader;
#[doc = "Field `SSIOP` writer - SS input/output polarity"]
pub type SSIOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOE` reader - SS output enable This bit is taken into account in Master mode only"]
pub type SSOE_R = crate::BitReader;
#[doc = "Field `SSOE` writer - SS output enable This bit is taken into account in Master mode only"]
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOM` reader - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
pub type SSOM_R = crate::BitReader;
#[doc = "Field `SSOM` writer - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
pub type SSOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFCNTR` reader - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration."]
pub type AFCNTR_R = crate::BitReader;
#[doc = "Field `AFCNTR` writer - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration."]
pub type AFCNTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
    #[inline(always)]
    pub fn mssi(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
    #[inline(always)]
    pub fn midi(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
    #[inline(always)]
    pub fn rdiom(&self) -> RDIOM_R {
        RDIOM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RDY signal input/output polarity"]
    #[inline(always)]
    pub fn rdiop(&self) -> RDIOP_R {
        RDIOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO."]
    #[inline(always)]
    pub fn ioswp(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    pub fn comm(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - serial protocol others: reserved, must not be used"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - data frame format"]
    #[inline(always)]
    pub fn lsbfrst(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    pub fn ssiop(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SS output enable This bit is taken into account in Master mode only"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
    #[inline(always)]
    pub fn ssom(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration."]
    #[inline(always)]
    pub fn afcntr(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
    #[inline(always)]
    #[must_use]
    pub fn mssi(&mut self) -> MSSI_W<SPI_CFG2rs> {
        MSSI_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn midi(&mut self) -> MIDI_W<SPI_CFG2rs> {
        MIDI_W::new(self, 4)
    }
    #[doc = "Bit 13 - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
    #[inline(always)]
    #[must_use]
    pub fn rdiom(&mut self) -> RDIOM_W<SPI_CFG2rs> {
        RDIOM_W::new(self, 13)
    }
    #[doc = "Bit 14 - RDY signal input/output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn rdiop(&mut self) -> RDIOP_W<SPI_CFG2rs> {
        RDIOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO."]
    #[inline(always)]
    #[must_use]
    pub fn ioswp(&mut self) -> IOSWP_W<SPI_CFG2rs> {
        IOSWP_W::new(self, 15)
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn comm(&mut self) -> COMM_W<SPI_CFG2rs> {
        COMM_W::new(self, 17)
    }
    #[doc = "Bits 19:21 - serial protocol others: reserved, must not be used"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<SPI_CFG2rs> {
        SP_W::new(self, 19)
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<SPI_CFG2rs> {
        MASTER_W::new(self, 22)
    }
    #[doc = "Bit 23 - data frame format"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfrst(&mut self) -> LSBFRST_W<SPI_CFG2rs> {
        LSBFRST_W::new(self, 23)
    }
    #[doc = "Bit 24 - clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<SPI_CFG2rs> {
        CPHA_W::new(self, 24)
    }
    #[doc = "Bit 25 - clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<SPI_CFG2rs> {
        CPOL_W::new(self, 25)
    }
    #[doc = "Bit 26 - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<SPI_CFG2rs> {
        SSM_W::new(self, 26)
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ssiop(&mut self) -> SSIOP_W<SPI_CFG2rs> {
        SSIOP_W::new(self, 28)
    }
    #[doc = "Bit 29 - SS output enable This bit is taken into account in Master mode only"]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<SPI_CFG2rs> {
        SSOE_W::new(self, 29)
    }
    #[doc = "Bit 30 - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
    #[inline(always)]
    #[must_use]
    pub fn ssom(&mut self) -> SSOM_W<SPI_CFG2rs> {
        SSOM_W::new(self, 30)
    }
    #[doc = "Bit 31 - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration."]
    #[inline(always)]
    #[must_use]
    pub fn afcntr(&mut self) -> AFCNTR_W<SPI_CFG2rs> {
        AFCNTR_W::new(self, 31)
    }
}
#[doc = "SPI configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CFG2rs;
impl crate::RegisterSpec for SPI_CFG2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_cfg2::R`](R) reader structure"]
impl crate::Readable for SPI_CFG2rs {}
#[doc = "`write(|w| ..)` method takes [`spi_cfg2::W`](W) writer structure"]
impl crate::Writable for SPI_CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_CFG2 to value 0"]
impl crate::Resettable for SPI_CFG2rs {
    const RESET_VALUE: u32 = 0;
}
