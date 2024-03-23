#[doc = "Register `MACMDIOAR` reader"]
pub type R = crate::R<MACMDIOARrs>;
#[doc = "Register `MACMDIOAR` writer"]
pub type W = crate::W<MACMDIOARrs>;
#[doc = "Field `MB` reader - MII Busy The application sets this bit to instruct the SMA to initiate a Read or Write access to the MDIOS. The MAC clears this bit after the MDIO frame transfer is completed. Hence the software must not write or change any of the fields in MDIO address register (ETH_MACMDIOAR) and MDIO data register (ETH_MACMDIODR) as long as this bit is set. For write transfers, the application must first write 16-bit data in the MD field (and also RA field when C45E is set) in MDIO data register (ETH_MACMDIODR) register before setting this bit. When C45E is set, it should also write into the RA field of MDIO data register (ETH_MACMDIODR) before initiating a read transfer. When a read transfer is completed (MII busy=0), the data read from the PHY register is valid in the MD field of the MDIO data register (ETH_MACMDIODR). Note: Even if the addressed PHY is not present, there is no change in the functionality of this bit."]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - MII Busy The application sets this bit to instruct the SMA to initiate a Read or Write access to the MDIOS. The MAC clears this bit after the MDIO frame transfer is completed. Hence the software must not write or change any of the fields in MDIO address register (ETH_MACMDIOAR) and MDIO data register (ETH_MACMDIODR) as long as this bit is set. For write transfers, the application must first write 16-bit data in the MD field (and also RA field when C45E is set) in MDIO data register (ETH_MACMDIODR) register before setting this bit. When C45E is set, it should also write into the RA field of MDIO data register (ETH_MACMDIODR) before initiating a read transfer. When a read transfer is completed (MII busy=0), the data read from the PHY register is valid in the MD field of the MDIO data register (ETH_MACMDIODR). Note: Even if the addressed PHY is not present, there is no change in the functionality of this bit."]
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C45E` reader - Clause 45 PHY Enable When this bit is set, Clause 45 capable PHY is connected to MDIO. When this bit is reset, Clause 22 capable PHY is connected to MDIO."]
pub type C45E_R = crate::BitReader;
#[doc = "Field `C45E` writer - Clause 45 PHY Enable When this bit is set, Clause 45 capable PHY is connected to MDIO. When this bit is reset, Clause 22 capable PHY is connected to MDIO."]
pub type C45E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOC` reader - MII Operation Command This bit indicates the operation command to the PHY. When Clause 22 PHY is enabled, only Write and Read commands are valid."]
pub type GOC_R = crate::FieldReader;
#[doc = "Field `GOC` writer - MII Operation Command This bit indicates the operation command to the PHY. When Clause 22 PHY is enabled, only Write and Read commands are valid."]
pub type GOC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SKAP` reader - Skip Address Packet When this bit is set, the SMA does not send the address packets before read, write, or post-read increment address packets. This bit is valid only when C45E is set."]
pub type SKAP_R = crate::BitReader;
#[doc = "Field `SKAP` writer - Skip Address Packet When this bit is set, the SMA does not send the address packets before read, write, or post-read increment address packets. This bit is valid only when C45E is set."]
pub type SKAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design: 0110 to 0111: Reserved, must not be used The suggested range of CSR clock frequency applicable for each value (when Bit 11 = 0) ensures that the MDC clock is approximately between 1.0 MHz to 2.5 MHz frequency range. When Bit 11 is set, you can achieve a higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits to 1010, the resultant MDC clock is of 12.5 MHz which is above the range specified in IEEE 802.3. Program the following values only if the interfacing chips support faster MDC clocks:"]
pub type CR_R = crate::FieldReader;
#[doc = "Field `CR` writer - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design: 0110 to 0111: Reserved, must not be used The suggested range of CSR clock frequency applicable for each value (when Bit 11 = 0) ensures that the MDC clock is approximately between 1.0 MHz to 2.5 MHz frequency range. When Bit 11 is set, you can achieve a higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits to 1010, the resultant MDC clock is of 12.5 MHz which is above the range specified in IEEE 802.3. Program the following values only if the interfacing chips support faster MDC clocks:"]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NTC` reader - Number of Training Clocks This field controls the number of trailing clock cycles generated on ETH_MDC after the end of transmission of MDIO frame. The valid values can be from 0 to 7. Programming the value to 011 indicates that there are additional three clock cycles on the MDC line after the end of MDIO frame transfer."]
pub type NTC_R = crate::FieldReader;
#[doc = "Field `NTC` writer - Number of Training Clocks This field controls the number of trailing clock cycles generated on ETH_MDC after the end of transmission of MDIO frame. The valid values can be from 0 to 7. Programming the value to 011 indicates that there are additional three clock cycles on the MDC line after the end of MDIO frame transfer."]
pub type NTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDA` reader - Register/Device Address These bits select the PHY register in selected Clause 22 PHY device. These bits select the Device (MMD) in selected Clause 45 capable PHY."]
pub type RDA_R = crate::FieldReader;
#[doc = "Field `RDA` writer - Register/Device Address These bits select the PHY register in selected Clause 22 PHY device. These bits select the Device (MMD) in selected Clause 45 capable PHY."]
pub type RDA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - Physical Layer Address This field indicates which Clause 22 PHY devices (out of 32 devices) the MAC is accessing. This field indicates which Clause 45 capable PHYs (out of 32 PHYs) the MAC is accessing."]
pub type PA_R = crate::FieldReader;
#[doc = "Field `PA` writer - Physical Layer Address This field indicates which Clause 22 PHY devices (out of 32 devices) the MAC is accessing. This field indicates which Clause 45 capable PHYs (out of 32 PHYs) the MAC is accessing."]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BTB` reader - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC informs the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted). The software can thus initiate the next command which is executed immediately irrespective of the number trailing clocks generated for the previous frame. When this bit is reset, then the read/write command completion (MII busy is cleared) only after the trailing clocks are generated. In this mode, it is ensured that the NTC is always generated after each frame. This bit must not be set when NTC=0."]
pub type BTB_R = crate::BitReader;
#[doc = "Field `BTB` writer - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC informs the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted). The software can thus initiate the next command which is executed immediately irrespective of the number trailing clocks generated for the previous frame. When this bit is reset, then the read/write command completion (MII busy is cleared) only after the trailing clocks are generated. In this mode, it is ensured that the NTC is always generated after each frame. This bit must not be set when NTC=0."]
pub type BTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSE` reader - Preamble Suppression Enable When this bit is set, the SMA suppresses the 32-bit preamble and transmit MDIO frames with only 1 preamble bit. When this bit is 0, the MDIO frame always has 32 bits of preamble as defined in the IEEE specifications."]
pub type PSE_R = crate::BitReader;
#[doc = "Field `PSE` writer - Preamble Suppression Enable When this bit is set, the SMA suppresses the 32-bit preamble and transmit MDIO frames with only 1 preamble bit. When this bit is 0, the MDIO frame always has 32 bits of preamble as defined in the IEEE specifications."]
pub type PSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MII Busy The application sets this bit to instruct the SMA to initiate a Read or Write access to the MDIOS. The MAC clears this bit after the MDIO frame transfer is completed. Hence the software must not write or change any of the fields in MDIO address register (ETH_MACMDIOAR) and MDIO data register (ETH_MACMDIODR) as long as this bit is set. For write transfers, the application must first write 16-bit data in the MD field (and also RA field when C45E is set) in MDIO data register (ETH_MACMDIODR) register before setting this bit. When C45E is set, it should also write into the RA field of MDIO data register (ETH_MACMDIODR) before initiating a read transfer. When a read transfer is completed (MII busy=0), the data read from the PHY register is valid in the MD field of the MDIO data register (ETH_MACMDIODR). Note: Even if the addressed PHY is not present, there is no change in the functionality of this bit."]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clause 45 PHY Enable When this bit is set, Clause 45 capable PHY is connected to MDIO. When this bit is reset, Clause 22 capable PHY is connected to MDIO."]
    #[inline(always)]
    pub fn c45e(&self) -> C45E_R {
        C45E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - MII Operation Command This bit indicates the operation command to the PHY. When Clause 22 PHY is enabled, only Write and Read commands are valid."]
    #[inline(always)]
    pub fn goc(&self) -> GOC_R {
        GOC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Skip Address Packet When this bit is set, the SMA does not send the address packets before read, write, or post-read increment address packets. This bit is valid only when C45E is set."]
    #[inline(always)]
    pub fn skap(&self) -> SKAP_R {
        SKAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design: 0110 to 0111: Reserved, must not be used The suggested range of CSR clock frequency applicable for each value (when Bit 11 = 0) ensures that the MDC clock is approximately between 1.0 MHz to 2.5 MHz frequency range. When Bit 11 is set, you can achieve a higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits to 1010, the resultant MDC clock is of 12.5 MHz which is above the range specified in IEEE 802.3. Program the following values only if the interfacing chips support faster MDC clocks:"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks This field controls the number of trailing clock cycles generated on ETH_MDC after the end of transmission of MDIO frame. The valid values can be from 0 to 7. Programming the value to 011 indicates that there are additional three clock cycles on the MDC line after the end of MDIO frame transfer."]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Register/Device Address These bits select the PHY register in selected Clause 22 PHY device. These bits select the Device (MMD) in selected Clause 45 capable PHY."]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Physical Layer Address This field indicates which Clause 22 PHY devices (out of 32 devices) the MAC is accessing. This field indicates which Clause 45 capable PHYs (out of 32 PHYs) the MAC is accessing."]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC informs the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted). The software can thus initiate the next command which is executed immediately irrespective of the number trailing clocks generated for the previous frame. When this bit is reset, then the read/write command completion (MII busy is cleared) only after the trailing clocks are generated. In this mode, it is ensured that the NTC is always generated after each frame. This bit must not be set when NTC=0."]
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable When this bit is set, the SMA suppresses the 32-bit preamble and transmit MDIO frames with only 1 preamble bit. When this bit is 0, the MDIO frame always has 32 bits of preamble as defined in the IEEE specifications."]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MII Busy The application sets this bit to instruct the SMA to initiate a Read or Write access to the MDIOS. The MAC clears this bit after the MDIO frame transfer is completed. Hence the software must not write or change any of the fields in MDIO address register (ETH_MACMDIOAR) and MDIO data register (ETH_MACMDIODR) as long as this bit is set. For write transfers, the application must first write 16-bit data in the MD field (and also RA field when C45E is set) in MDIO data register (ETH_MACMDIODR) register before setting this bit. When C45E is set, it should also write into the RA field of MDIO data register (ETH_MACMDIODR) before initiating a read transfer. When a read transfer is completed (MII busy=0), the data read from the PHY register is valid in the MD field of the MDIO data register (ETH_MACMDIODR). Note: Even if the addressed PHY is not present, there is no change in the functionality of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<MACMDIOARrs> {
        MB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clause 45 PHY Enable When this bit is set, Clause 45 capable PHY is connected to MDIO. When this bit is reset, Clause 22 capable PHY is connected to MDIO."]
    #[inline(always)]
    #[must_use]
    pub fn c45e(&mut self) -> C45E_W<MACMDIOARrs> {
        C45E_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - MII Operation Command This bit indicates the operation command to the PHY. When Clause 22 PHY is enabled, only Write and Read commands are valid."]
    #[inline(always)]
    #[must_use]
    pub fn goc(&mut self) -> GOC_W<MACMDIOARrs> {
        GOC_W::new(self, 2)
    }
    #[doc = "Bit 4 - Skip Address Packet When this bit is set, the SMA does not send the address packets before read, write, or post-read increment address packets. This bit is valid only when C45E is set."]
    #[inline(always)]
    #[must_use]
    pub fn skap(&mut self) -> SKAP_W<MACMDIOARrs> {
        SKAP_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design: 0110 to 0111: Reserved, must not be used The suggested range of CSR clock frequency applicable for each value (when Bit 11 = 0) ensures that the MDC clock is approximately between 1.0 MHz to 2.5 MHz frequency range. When Bit 11 is set, you can achieve a higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits to 1010, the resultant MDC clock is of 12.5 MHz which is above the range specified in IEEE 802.3. Program the following values only if the interfacing chips support faster MDC clocks:"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<MACMDIOARrs> {
        CR_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks This field controls the number of trailing clock cycles generated on ETH_MDC after the end of transmission of MDIO frame. The valid values can be from 0 to 7. Programming the value to 011 indicates that there are additional three clock cycles on the MDC line after the end of MDIO frame transfer."]
    #[inline(always)]
    #[must_use]
    pub fn ntc(&mut self) -> NTC_W<MACMDIOARrs> {
        NTC_W::new(self, 12)
    }
    #[doc = "Bits 16:20 - Register/Device Address These bits select the PHY register in selected Clause 22 PHY device. These bits select the Device (MMD) in selected Clause 45 capable PHY."]
    #[inline(always)]
    #[must_use]
    pub fn rda(&mut self) -> RDA_W<MACMDIOARrs> {
        RDA_W::new(self, 16)
    }
    #[doc = "Bits 21:25 - Physical Layer Address This field indicates which Clause 22 PHY devices (out of 32 devices) the MAC is accessing. This field indicates which Clause 45 capable PHYs (out of 32 PHYs) the MAC is accessing."]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<MACMDIOARrs> {
        PA_W::new(self, 21)
    }
    #[doc = "Bit 26 - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC informs the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted). The software can thus initiate the next command which is executed immediately irrespective of the number trailing clocks generated for the previous frame. When this bit is reset, then the read/write command completion (MII busy is cleared) only after the trailing clocks are generated. In this mode, it is ensured that the NTC is always generated after each frame. This bit must not be set when NTC=0."]
    #[inline(always)]
    #[must_use]
    pub fn btb(&mut self) -> BTB_W<MACMDIOARrs> {
        BTB_W::new(self, 26)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable When this bit is set, the SMA suppresses the 32-bit preamble and transmit MDIO frames with only 1 preamble bit. When this bit is 0, the MDIO frame always has 32 bits of preamble as defined in the IEEE specifications."]
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PSE_W<MACMDIOARrs> {
        PSE_W::new(self, 27)
    }
}
#[doc = "MDIO address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmdioar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmdioar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMDIOARrs;
impl crate::RegisterSpec for MACMDIOARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmdioar::R`](R) reader structure"]
impl crate::Readable for MACMDIOARrs {}
#[doc = "`write(|w| ..)` method takes [`macmdioar::W`](W) writer structure"]
impl crate::Writable for MACMDIOARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACMDIOAR to value 0"]
impl crate::Resettable for MACMDIOARrs {
    const RESET_VALUE: u32 = 0;
}
