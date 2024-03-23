#[doc = "Register `SDMMC_CLKCR` reader"]
pub type R = crate::R<SDMMC_CLKCRrs>;
#[doc = "Register `SDMMC_CLKCR` writer"]
pub type W = crate::W<SDMMC_CLKCRrs>;
#[doc = "Field `CLKDIV` reader - Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (SDMMCCLK) and the output clock (SDMMC_CK): SDMMC_CK frequency = SDMMCCLK / \\[2 * CLKDIV\\]. 0xx: etc.. xxx: etc.."]
pub type CLKDIV_R = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (SDMMCCLK) and the output clock (SDMMC_CK): SDMMC_CK frequency = SDMMCCLK / \\[2 * CLKDIV\\]. 0xx: etc.. xxx: etc.."]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PWRSAV` reader - Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:"]
pub type PWRSAV_R = crate::BitReader;
#[doc = "Field `PWRSAV` writer - Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:"]
pub type PWRSAV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIDBUS` reader - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
pub type WIDBUS_R = crate::FieldReader;
#[doc = "Field `WIDBUS` writer - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
pub type WIDBUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NEGEDGE` reader - SDMMC_CK dephasing selection bit for data and Command. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. When clock division &amp;gt;1 (CLKDIV &amp;gt; 0) &amp;amp; DDR = 0: - SDMMC_CK edge occurs on SDMMCCLK rising edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: - Data changed on the SDMMCCLK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge. - Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge."]
pub type NEGEDGE_R = crate::BitReader;
#[doc = "Field `NEGEDGE` writer - SDMMC_CK dephasing selection bit for data and Command. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. When clock division &amp;gt;1 (CLKDIV &amp;gt; 0) &amp;amp; DDR = 0: - SDMMC_CK edge occurs on SDMMCCLK rising edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: - Data changed on the SDMMCCLK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge. - Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge."]
pub type NEGEDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWFC_EN` reader - Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in Section56.8.11."]
pub type HWFC_EN_R = crate::BitReader;
#[doc = "Field `HWFC_EN` writer - Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in Section56.8.11."]
pub type HWFC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR` reader - Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate shall only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS &amp;gt; 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate shall only be selected with clock division &amp;gt;1. (CLKDIV &amp;gt; 0)"]
pub type DDR_R = crate::BitReader;
#[doc = "Field `DDR` writer - Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate shall only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS &amp;gt; 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate shall only be selected with clock division &amp;gt;1. (CLKDIV &amp;gt; 0)"]
pub type DDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSSPEED` reader - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50, DDR50, SDR104. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
pub type BUSSPEED_R = crate::BitReader;
#[doc = "Field `BUSSPEED` writer - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50, DDR50, SDR104. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
pub type BUSSPEED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELCLKRX` reader - Receive clock selection. These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
pub type SELCLKRX_R = crate::FieldReader;
#[doc = "Field `SELCLKRX` writer - Receive clock selection. These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
pub type SELCLKRX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (SDMMCCLK) and the output clock (SDMMC_CK): SDMMC_CK frequency = SDMMCCLK / \\[2 * CLKDIV\\]. 0xx: etc.. xxx: etc.."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - SDMMC_CK dephasing selection bit for data and Command. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. When clock division &amp;gt;1 (CLKDIV &amp;gt; 0) &amp;amp; DDR = 0: - SDMMC_CK edge occurs on SDMMCCLK rising edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: - Data changed on the SDMMCCLK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge. - Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge."]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in Section56.8.11."]
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate shall only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS &amp;gt; 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate shall only be selected with clock division &amp;gt;1. (CLKDIV &amp;gt; 0)"]
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50, DDR50, SDR104. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
    #[inline(always)]
    pub fn busspeed(&self) -> BUSSPEED_R {
        BUSSPEED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Receive clock selection. These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
    #[inline(always)]
    pub fn selclkrx(&self) -> SELCLKRX_R {
        SELCLKRX_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (SDMMCCLK) and the output clock (SDMMC_CK): SDMMC_CK frequency = SDMMCCLK / \\[2 * CLKDIV\\]. 0xx: etc.. xxx: etc.."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<SDMMC_CLKCRrs> {
        CLKDIV_W::new(self, 0)
    }
    #[doc = "Bit 12 - Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsav(&mut self) -> PWRSAV_W<SDMMC_CLKCRrs> {
        PWRSAV_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
    #[inline(always)]
    #[must_use]
    pub fn widbus(&mut self) -> WIDBUS_W<SDMMC_CLKCRrs> {
        WIDBUS_W::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC_CK dephasing selection bit for data and Command. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. When clock division &amp;gt;1 (CLKDIV &amp;gt; 0) &amp;amp; DDR = 0: - SDMMC_CK edge occurs on SDMMCCLK rising edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: - Data changed on the SDMMCCLK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge. - Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. - SDMMC_CK edge occurs on SDMMCCLK rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn negedge(&mut self) -> NEGEDGE_W<SDMMC_CLKCRrs> {
        NEGEDGE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in Section56.8.11."]
    #[inline(always)]
    #[must_use]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<SDMMC_CLKCRrs> {
        HWFC_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate shall only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS &amp;gt; 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate shall only be selected with clock division &amp;gt;1. (CLKDIV &amp;gt; 0)"]
    #[inline(always)]
    #[must_use]
    pub fn ddr(&mut self) -> DDR_W<SDMMC_CLKCRrs> {
        DDR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Bus speed mode selection between DS, HS, SDR12, SDR25 and SDR50, DDR50, SDR104. This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
    #[inline(always)]
    #[must_use]
    pub fn busspeed(&mut self) -> BUSSPEED_W<SDMMC_CLKCRrs> {
        BUSSPEED_W::new(self, 19)
    }
    #[doc = "Bits 20:21 - Receive clock selection. These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
    #[inline(always)]
    #[must_use]
    pub fn selclkrx(&mut self) -> SELCLKRX_W<SDMMC_CLKCRrs> {
        SELCLKRX_W::new(self, 20)
    }
}
#[doc = "The SDMMC_CLKCR register controls the SDMMC_CK output clock, the SDMMC_RX_CLK receive clock, and the bus width.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_clkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_clkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_CLKCRrs;
impl crate::RegisterSpec for SDMMC_CLKCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_clkcr::R`](R) reader structure"]
impl crate::Readable for SDMMC_CLKCRrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_clkcr::W`](W) writer structure"]
impl crate::Writable for SDMMC_CLKCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_CLKCR to value 0"]
impl crate::Resettable for SDMMC_CLKCRrs {
    const RESET_VALUE: u32 = 0;
}
