///Register `CSR2` reader
pub type R = crate::R<CSR2rs>;
///Register `CSR2` writer
pub type W = crate::W<CSR2rs>;
///Field `BYPASS` reader - Power management unit bypass Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
pub type BYPASS_R = crate::BitReader;
///Field `BYPASS` writer - Power management unit bypass Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDOEN` reader - Low drop-out regulator enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
pub type LDOEN_R = crate::BitReader;
///Field `LDOEN` writer - Low drop-out regulator enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
pub type LDOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDEN` reader - SMPS step-down converter enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
pub type SDEN_R = crate::BitReader;
///Field `SDEN` writer - SMPS step-down converter enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPSEXTHP` reader - SMPS external power delivery selection Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
pub type SMPSEXTHP_R = crate::BitReader;
///Field `SMPSEXTHP` writer - SMPS external power delivery selection Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
pub type SMPSEXTHP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDHILEVEL` reader - SMPS step-down converter voltage output for LDO or external supply This bit is used when both the LDO and SMPS step-down converter are enabled with SDEN and LDOEN enabled or when SMPSEXTHP is enabled. In this case SDHILEVEL has to be set to 1 to confirm the regulator settings
pub type SDHILEVEL_R = crate::BitReader;
///Field `SDHILEVEL` writer - SMPS step-down converter voltage output for LDO or external supply This bit is used when both the LDO and SMPS step-down converter are enabled with SDEN and LDOEN enabled or when SMPSEXTHP is enabled. In this case SDHILEVEL has to be set to 1 to confirm the regulator settings
pub type SDHILEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBE` reader - VBAT charging enable
pub type VBE_R = crate::BitReader;
///Field `VBE` writer - VBAT charging enable
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBRS` reader - VBAT charging resistor selection
pub type VBRS_R = crate::BitReader;
///Field `VBRS` writer - VBAT charging resistor selection
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPICAP1` reader - XSPI port 1 capacitor control bits see the product datasheet for more details
pub type XSPICAP1_R = crate::FieldReader;
///Field `XSPICAP1` writer - XSPI port 1 capacitor control bits see the product datasheet for more details
pub type XSPICAP1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `XSPICAP2` reader - XSPI port 2 capacitor control bits see the product datasheet for more details
pub type XSPICAP2_R = crate::FieldReader;
///Field `XSPICAP2` writer - XSPI port 2 capacitor control bits see the product datasheet for more details
pub type XSPICAP2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EN_XSPIM1` reader - EN_XSPIM1: this bit allow the SW to enable the XSPI interface. The XSPIM_P1 supply must be stable prior to setting this bit.
pub type EN_XSPIM1_R = crate::BitReader;
///Field `EN_XSPIM1` writer - EN_XSPIM1: this bit allow the SW to enable the XSPI interface. The XSPIM_P1 supply must be stable prior to setting this bit.
pub type EN_XSPIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_XSPIM2` reader - EN_XSPIM2: this bit allows the SW to enable the XSPI interface, when available. The XSPIM_P2 supply must be stable prior to setting this bit. It should also be set when FMC is used.
pub type EN_XSPIM2_R = crate::BitReader;
///Field `EN_XSPIM2` writer - EN_XSPIM2: this bit allows the SW to enable the XSPI interface, when available. The XSPIM_P2 supply must be stable prior to setting this bit. It should also be set when FMC is used.
pub type EN_XSPIM2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDEXTRDY` reader - SMPS step-down converter external supply ready This bit is set by hardware to indicate that the external supply from the SMPS step-down converter is ready.
pub type SDEXTRDY_R = crate::BitReader;
///Field `USB33DEN` reader - VDD33_USB voltage level detector enable
pub type USB33DEN_R = crate::BitReader;
///Field `USB33DEN` writer - VDD33_USB voltage level detector enable
pub type USB33DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBREGEN` reader - USB regulator enable.
pub type USBREGEN_R = crate::BitReader;
///Field `USBREGEN` writer - USB regulator enable.
pub type USBREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB33RDY` reader - USB supply ready.
pub type USB33RDY_R = crate::BitReader;
///Field `USBHSREGEN` reader - USB HS regulator enable.
pub type USBHSREGEN_R = crate::BitReader;
///Field `USBHSREGEN` writer - USB HS regulator enable.
pub type USBHSREGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Power management unit bypass Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Low drop-out regulator enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
    #[inline(always)]
    pub fn ldoen(&self) -> LDOEN_R {
        LDOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SMPS step-down converter enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SMPS external power delivery selection Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
    #[inline(always)]
    pub fn smpsexthp(&self) -> SMPSEXTHP_R {
        SMPSEXTHP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SMPS step-down converter voltage output for LDO or external supply This bit is used when both the LDO and SMPS step-down converter are enabled with SDEN and LDOEN enabled or when SMPSEXTHP is enabled. In this case SDHILEVEL has to be set to 1 to confirm the regulator settings
    #[inline(always)]
    pub fn sdhilevel(&self) -> SDHILEVEL_R {
        SDHILEVEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - VBAT charging enable
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - VBAT charging resistor selection
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - XSPI port 1 capacitor control bits see the product datasheet for more details
    #[inline(always)]
    pub fn xspicap1(&self) -> XSPICAP1_R {
        XSPICAP1_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - XSPI port 2 capacitor control bits see the product datasheet for more details
    #[inline(always)]
    pub fn xspicap2(&self) -> XSPICAP2_R {
        XSPICAP2_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - EN_XSPIM1: this bit allow the SW to enable the XSPI interface. The XSPIM_P1 supply must be stable prior to setting this bit.
    #[inline(always)]
    pub fn en_xspim1(&self) -> EN_XSPIM1_R {
        EN_XSPIM1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - EN_XSPIM2: this bit allows the SW to enable the XSPI interface, when available. The XSPIM_P2 supply must be stable prior to setting this bit. It should also be set when FMC is used.
    #[inline(always)]
    pub fn en_xspim2(&self) -> EN_XSPIM2_R {
        EN_XSPIM2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SMPS step-down converter external supply ready This bit is set by hardware to indicate that the external supply from the SMPS step-down converter is ready.
    #[inline(always)]
    pub fn sdextrdy(&self) -> SDEXTRDY_R {
        SDEXTRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - VDD33_USB voltage level detector enable
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - USB regulator enable.
    #[inline(always)]
    pub fn usbregen(&self) -> USBREGEN_R {
        USBREGEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USB supply ready.
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - USB HS regulator enable.
    #[inline(always)]
    pub fn usbhsregen(&self) -> USBHSREGEN_R {
        USBHSREGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR2")
            .field("bypass", &self.bypass())
            .field("ldoen", &self.ldoen())
            .field("sden", &self.sden())
            .field("smpsexthp", &self.smpsexthp())
            .field("sdhilevel", &self.sdhilevel())
            .field("vbe", &self.vbe())
            .field("vbrs", &self.vbrs())
            .field("xspicap1", &self.xspicap1())
            .field("xspicap2", &self.xspicap2())
            .field("en_xspim1", &self.en_xspim1())
            .field("en_xspim2", &self.en_xspim2())
            .field("sdextrdy", &self.sdextrdy())
            .field("usb33den", &self.usb33den())
            .field("usbregen", &self.usbregen())
            .field("usb33rdy", &self.usb33rdy())
            .field("usbhsregen", &self.usbhsregen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Power management unit bypass Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<'_, CSR2rs> {
        BYPASS_W::new(self, 0)
    }
    ///Bit 1 - Low drop-out regulator enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
    #[inline(always)]
    pub fn ldoen(&mut self) -> LDOEN_W<'_, CSR2rs> {
        LDOEN_W::new(self, 1)
    }
    ///Bit 2 - SMPS step-down converter enable Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W<'_, CSR2rs> {
        SDEN_W::new(self, 2)
    }
    ///Bit 3 - SMPS external power delivery selection Note: Illegal combinations of SDHILEVEL, SMPSEXTHP, SDEN, LDOEN and BYPASS are described in Table 41.
    #[inline(always)]
    pub fn smpsexthp(&mut self) -> SMPSEXTHP_W<'_, CSR2rs> {
        SMPSEXTHP_W::new(self, 3)
    }
    ///Bit 4 - SMPS step-down converter voltage output for LDO or external supply This bit is used when both the LDO and SMPS step-down converter are enabled with SDEN and LDOEN enabled or when SMPSEXTHP is enabled. In this case SDHILEVEL has to be set to 1 to confirm the regulator settings
    #[inline(always)]
    pub fn sdhilevel(&mut self) -> SDHILEVEL_W<'_, CSR2rs> {
        SDHILEVEL_W::new(self, 4)
    }
    ///Bit 8 - VBAT charging enable
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W<'_, CSR2rs> {
        VBE_W::new(self, 8)
    }
    ///Bit 9 - VBAT charging resistor selection
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W<'_, CSR2rs> {
        VBRS_W::new(self, 9)
    }
    ///Bits 10:11 - XSPI port 1 capacitor control bits see the product datasheet for more details
    #[inline(always)]
    pub fn xspicap1(&mut self) -> XSPICAP1_W<'_, CSR2rs> {
        XSPICAP1_W::new(self, 10)
    }
    ///Bits 12:13 - XSPI port 2 capacitor control bits see the product datasheet for more details
    #[inline(always)]
    pub fn xspicap2(&mut self) -> XSPICAP2_W<'_, CSR2rs> {
        XSPICAP2_W::new(self, 12)
    }
    ///Bit 14 - EN_XSPIM1: this bit allow the SW to enable the XSPI interface. The XSPIM_P1 supply must be stable prior to setting this bit.
    #[inline(always)]
    pub fn en_xspim1(&mut self) -> EN_XSPIM1_W<'_, CSR2rs> {
        EN_XSPIM1_W::new(self, 14)
    }
    ///Bit 15 - EN_XSPIM2: this bit allows the SW to enable the XSPI interface, when available. The XSPIM_P2 supply must be stable prior to setting this bit. It should also be set when FMC is used.
    #[inline(always)]
    pub fn en_xspim2(&mut self) -> EN_XSPIM2_W<'_, CSR2rs> {
        EN_XSPIM2_W::new(self, 15)
    }
    ///Bit 24 - VDD33_USB voltage level detector enable
    #[inline(always)]
    pub fn usb33den(&mut self) -> USB33DEN_W<'_, CSR2rs> {
        USB33DEN_W::new(self, 24)
    }
    ///Bit 25 - USB regulator enable.
    #[inline(always)]
    pub fn usbregen(&mut self) -> USBREGEN_W<'_, CSR2rs> {
        USBREGEN_W::new(self, 25)
    }
    ///Bit 27 - USB HS regulator enable.
    #[inline(always)]
    pub fn usbhsregen(&mut self) -> USBHSREGEN_W<'_, CSR2rs> {
        USBHSREGEN_W::new(self, 27)
    }
}
/**PWR control register 2

You can [`read`](crate::Reg::read) this register and get [`csr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:CSR2)*/
pub struct CSR2rs;
impl crate::RegisterSpec for CSR2rs {
    type Ux = u32;
}
///`read()` method returns [`csr2::R`](R) reader structure
impl crate::Readable for CSR2rs {}
///`write(|w| ..)` method takes [`csr2::W`](W) writer structure
impl crate::Writable for CSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR2 to value 0x06
impl crate::Resettable for CSR2rs {
    const RESET_VALUE: u32 = 0x06;
}
