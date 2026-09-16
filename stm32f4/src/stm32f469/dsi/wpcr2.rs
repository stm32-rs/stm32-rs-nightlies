///Register `WPCR2` reader
pub type R = crate::R<WPCR2rs>;
///Register `WPCR2` writer
pub type W = crate::W<WPCR2rs>;
///Field `TCLKPREP` reader - t<sub>CLK-PREPARE</sub> This field defines the t<sub>CLK-PREPARE</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the TCLKPREPEN bit of the DSI_WPCR0 is set. TCLKPREP = 2 x t<sub>CLK-PREPARE</sub> expressed in ns.The default value used by the D-PHY when TCLKPREPEN bit of the DSI_WPCR0 is reset is 120 (60ns + 20*UI).
pub type TCLKPREP_R = crate::FieldReader;
///Field `TCLKPREP` writer - t<sub>CLK-PREPARE</sub> This field defines the t<sub>CLK-PREPARE</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the TCLKPREPEN bit of the DSI_WPCR0 is set. TCLKPREP = 2 x t<sub>CLK-PREPARE</sub> expressed in ns.The default value used by the D-PHY when TCLKPREPEN bit of the DSI_WPCR0 is reset is 120 (60ns + 20*UI).
pub type TCLKPREP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TCLKZERO` reader - t<sub>CLK-ZERO</sub> This field defines the t<sub>CLK-ZERO</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the TCLKZEROEN bit of the DSI_WPCR0 is set. TCLKZERO = t<sub>CLK-ZERO</sub> / 2 expressed in ns.The default value used by the D-PHY when TCLKZEROEN bit of the DSI_WPCR0 is reset is 195 (390ns).
pub type TCLKZERO_R = crate::FieldReader;
///Field `TCLKZERO` writer - t<sub>CLK-ZERO</sub> This field defines the t<sub>CLK-ZERO</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the TCLKZEROEN bit of the DSI_WPCR0 is set. TCLKZERO = t<sub>CLK-ZERO</sub> / 2 expressed in ns.The default value used by the D-PHY when TCLKZEROEN bit of the DSI_WPCR0 is reset is 195 (390ns).
pub type TCLKZERO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `THSPREP` reader - t<sub>HS-PREPARE</sub> This field defines the t<sub>HS-PREPARE</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the THSPREPEN bit of the DSI_WPCR0 is set. THSPREP = 2 x t<sub>HS-PREPARE</sub> expressed in ns.The default value used by the D-PHY when THSPREPEN bit of the DSI_WPCR0 is reset is 126 (63ns + 12*UI).
pub type THSPREP_R = crate::FieldReader;
///Field `THSPREP` writer - t<sub>HS-PREPARE</sub> This field defines the t<sub>HS-PREPARE</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the THSPREPEN bit of the DSI_WPCR0 is set. THSPREP = 2 x t<sub>HS-PREPARE</sub> expressed in ns.The default value used by the D-PHY when THSPREPEN bit of the DSI_WPCR0 is reset is 126 (63ns + 12*UI).
pub type THSPREP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `THSTRAIL` reader - t<sub>HSTRAIL</sub> This field defines the t<sub>HS-TRAIL</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the THSTRAILEN bit of the DSI_WPCR0 is set. THSTRAIL = 2 x t<sub>HS-TRAIL</sub> expressed in ns.The default value used by the D-PHY when THSTRAILEN bit of the DSI_WPCR0 is reset is 140 (70ns+8*UI).
pub type THSTRAIL_R = crate::FieldReader;
///Field `THSTRAIL` writer - t<sub>HSTRAIL</sub> This field defines the t<sub>HS-TRAIL</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the THSTRAILEN bit of the DSI_WPCR0 is set. THSTRAIL = 2 x t<sub>HS-TRAIL</sub> expressed in ns.The default value used by the D-PHY when THSTRAILEN bit of the DSI_WPCR0 is reset is 140 (70ns+8*UI).
pub type THSTRAIL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - t<sub>CLK-PREPARE</sub> This field defines the t<sub>CLK-PREPARE</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the TCLKPREPEN bit of the DSI_WPCR0 is set. TCLKPREP = 2 x t<sub>CLK-PREPARE</sub> expressed in ns.The default value used by the D-PHY when TCLKPREPEN bit of the DSI_WPCR0 is reset is 120 (60ns + 20*UI).
    #[inline(always)]
    pub fn tclkprep(&self) -> TCLKPREP_R {
        TCLKPREP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - t<sub>CLK-ZERO</sub> This field defines the t<sub>CLK-ZERO</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the TCLKZEROEN bit of the DSI_WPCR0 is set. TCLKZERO = t<sub>CLK-ZERO</sub> / 2 expressed in ns.The default value used by the D-PHY when TCLKZEROEN bit of the DSI_WPCR0 is reset is 195 (390ns).
    #[inline(always)]
    pub fn tclkzero(&self) -> TCLKZERO_R {
        TCLKZERO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - t<sub>HS-PREPARE</sub> This field defines the t<sub>HS-PREPARE</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the THSPREPEN bit of the DSI_WPCR0 is set. THSPREP = 2 x t<sub>HS-PREPARE</sub> expressed in ns.The default value used by the D-PHY when THSPREPEN bit of the DSI_WPCR0 is reset is 126 (63ns + 12*UI).
    #[inline(always)]
    pub fn thsprep(&self) -> THSPREP_R {
        THSPREP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - t<sub>HSTRAIL</sub> This field defines the t<sub>HS-TRAIL</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the THSTRAILEN bit of the DSI_WPCR0 is set. THSTRAIL = 2 x t<sub>HS-TRAIL</sub> expressed in ns.The default value used by the D-PHY when THSTRAILEN bit of the DSI_WPCR0 is reset is 140 (70ns+8*UI).
    #[inline(always)]
    pub fn thstrail(&self) -> THSTRAIL_R {
        THSTRAIL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR2")
            .field("tclkprep", &self.tclkprep())
            .field("tclkzero", &self.tclkzero())
            .field("thsprep", &self.thsprep())
            .field("thstrail", &self.thstrail())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - t<sub>CLK-PREPARE</sub> This field defines the t<sub>CLK-PREPARE</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the TCLKPREPEN bit of the DSI_WPCR0 is set. TCLKPREP = 2 x t<sub>CLK-PREPARE</sub> expressed in ns.The default value used by the D-PHY when TCLKPREPEN bit of the DSI_WPCR0 is reset is 120 (60ns + 20*UI).
    #[inline(always)]
    pub fn tclkprep(&mut self) -> TCLKPREP_W<'_, WPCR2rs> {
        TCLKPREP_W::new(self, 0)
    }
    ///Bits 8:15 - t<sub>CLK-ZERO</sub> This field defines the t<sub>CLK-ZERO</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the TCLKZEROEN bit of the DSI_WPCR0 is set. TCLKZERO = t<sub>CLK-ZERO</sub> / 2 expressed in ns.The default value used by the D-PHY when TCLKZEROEN bit of the DSI_WPCR0 is reset is 195 (390ns).
    #[inline(always)]
    pub fn tclkzero(&mut self) -> TCLKZERO_W<'_, WPCR2rs> {
        TCLKZERO_W::new(self, 8)
    }
    ///Bits 16:23 - t<sub>HS-PREPARE</sub> This field defines the t<sub>HS-PREPARE</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the THSPREPEN bit of the DSI_WPCR0 is set. THSPREP = 2 x t<sub>HS-PREPARE</sub> expressed in ns.The default value used by the D-PHY when THSPREPEN bit of the DSI_WPCR0 is reset is 126 (63ns + 12*UI).
    #[inline(always)]
    pub fn thsprep(&mut self) -> THSPREP_W<'_, WPCR2rs> {
        THSPREP_W::new(self, 16)
    }
    ///Bits 24:31 - t<sub>HSTRAIL</sub> This field defines the t<sub>HS-TRAIL</sub> has specified in the MIPI<sup></sup> D-PHY specification. This value is used by the D-PHY when the THSTRAILEN bit of the DSI_WPCR0 is set. THSTRAIL = 2 x t<sub>HS-TRAIL</sub> expressed in ns.The default value used by the D-PHY when THSTRAILEN bit of the DSI_WPCR0 is reset is 140 (70ns+8*UI).
    #[inline(always)]
    pub fn thstrail(&mut self) -> THSTRAIL_W<'_, WPCR2rs> {
        THSTRAIL_W::new(self, 24)
    }
}
/**DSI Wrapper PHY configuration register 2

You can [`read`](crate::Reg::read) this register and get [`wpcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:WPCR2)*/
pub struct WPCR2rs;
impl crate::RegisterSpec for WPCR2rs {
    type Ux = u32;
}
///`read()` method returns [`wpcr2::R`](R) reader structure
impl crate::Readable for WPCR2rs {}
///`write(|w| ..)` method takes [`wpcr2::W`](W) writer structure
impl crate::Writable for WPCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPCR2 to value 0
impl crate::Resettable for WPCR2rs {}
