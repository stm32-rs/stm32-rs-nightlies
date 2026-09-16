///Register `CCCSR` reader
pub type R = crate::R<CCCSRrs>;
///Register `CCCSR` writer
pub type W = crate::W<CCCSRrs>;
///Field `EN` reader - Compensation cell enable Set this bit to enable the compensation cell.
pub type EN_R = crate::BitReader;
///Field `EN` writer - Compensation cell enable Set this bit to enable the compensation cell.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS` reader - Compensation cell code selection This bit selects the code to be applied for the I/O compensation cell.
pub type CS_R = crate::BitReader;
///Field `CS` writer - Compensation cell code selection This bit selects the code to be applied for the I/O compensation cell.
pub type CS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTO1_COMP_EN` reader - XSPIM_P1 compensation cell enable Set this bit to enable the XSPIM_P1 compensation cell.
pub type OCTO1_COMP_EN_R = crate::BitReader;
///Field `OCTO1_COMP_EN` writer - XSPIM_P1 compensation cell enable Set this bit to enable the XSPIM_P1 compensation cell.
pub type OCTO1_COMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTO1_COMP_CODESEL` reader - XSPIM_P1 compensation cell code selection This bit selects the code to be applied for the XSPIM_P1 I/O compensation cell.
pub type OCTO1_COMP_CODESEL_R = crate::BitReader;
///Field `OCTO1_COMP_CODESEL` writer - XSPIM_P1 compensation cell code selection This bit selects the code to be applied for the XSPIM_P1 I/O compensation cell.
pub type OCTO1_COMP_CODESEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTO2_COMP_EN` reader - XSPIM_P2 compensation cell enable Set this bit to enable the XSPIM_P2 compensation cell.
pub type OCTO2_COMP_EN_R = crate::BitReader;
///Field `OCTO2_COMP_CODESEL` reader - XSPIM_P2 compensation cell code selection This bit selects the code to be applied for the XSPIM_P2 I/O compensation cell.
pub type OCTO2_COMP_CODESEL_R = crate::BitReader;
///Field `READY` reader - Compensation cell ready This bit provides the status of the compensation cell.
pub type READY_R = crate::BitReader;
///Field `OCTO1_COMP_RDY` reader - XSPIM_P1 compensation cell ready This bit provides the status of the XSPIM_P1 compensation cell.
pub type OCTO1_COMP_RDY_R = crate::BitReader;
///Field `OCTO2_COMP_RDY` reader - XSPIM_P2 compensation cell ready This bit provides the status of the XSPIM_P2 compensation cell.
pub type OCTO2_COMP_RDY_R = crate::BitReader;
///Field `HSLV` reader - I/O high speed at low voltage When this bit is set, the speed of the I/Os is optimized when the device voltage is low. This bit is active only if VDDIO_HSLV user option bit is set in FLASH. It must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
pub type HSLV_R = crate::BitReader;
///Field `HSLV` writer - I/O high speed at low voltage When this bit is set, the speed of the I/Os is optimized when the device voltage is low. This bit is active only if VDDIO_HSLV user option bit is set in FLASH. It must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
pub type HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTO1_IOHSLV` reader - XSPIM_P1 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P1 I/Os is optimized when the device voltage is low. This bit is active only if OCTO1_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
pub type OCTO1_IOHSLV_R = crate::BitReader;
///Field `OCTO1_IOHSLV` writer - XSPIM_P1 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P1 I/Os is optimized when the device voltage is low. This bit is active only if OCTO1_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
pub type OCTO1_IOHSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTO2_IOHSLV` reader - XSPIM_P2 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P2 I/Os is optimized when the device voltage is low. This bit is active only if OCTO2_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
pub type OCTO2_IOHSLV_R = crate::BitReader;
///Field `OCTO2_IOHSLV` writer - XSPIM_P2 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P2 I/Os is optimized when the device voltage is low. This bit is active only if OCTO2_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
pub type OCTO2_IOHSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Compensation cell enable Set this bit to enable the compensation cell.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compensation cell code selection This bit selects the code to be applied for the I/O compensation cell.
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - XSPIM_P1 compensation cell enable Set this bit to enable the XSPIM_P1 compensation cell.
    #[inline(always)]
    pub fn octo1_comp_en(&self) -> OCTO1_COMP_EN_R {
        OCTO1_COMP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - XSPIM_P1 compensation cell code selection This bit selects the code to be applied for the XSPIM_P1 I/O compensation cell.
    #[inline(always)]
    pub fn octo1_comp_codesel(&self) -> OCTO1_COMP_CODESEL_R {
        OCTO1_COMP_CODESEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - XSPIM_P2 compensation cell enable Set this bit to enable the XSPIM_P2 compensation cell.
    #[inline(always)]
    pub fn octo2_comp_en(&self) -> OCTO2_COMP_EN_R {
        OCTO2_COMP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - XSPIM_P2 compensation cell code selection This bit selects the code to be applied for the XSPIM_P2 I/O compensation cell.
    #[inline(always)]
    pub fn octo2_comp_codesel(&self) -> OCTO2_COMP_CODESEL_R {
        OCTO2_COMP_CODESEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Compensation cell ready This bit provides the status of the compensation cell.
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - XSPIM_P1 compensation cell ready This bit provides the status of the XSPIM_P1 compensation cell.
    #[inline(always)]
    pub fn octo1_comp_rdy(&self) -> OCTO1_COMP_RDY_R {
        OCTO1_COMP_RDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - XSPIM_P2 compensation cell ready This bit provides the status of the XSPIM_P2 compensation cell.
    #[inline(always)]
    pub fn octo2_comp_rdy(&self) -> OCTO2_COMP_RDY_R {
        OCTO2_COMP_RDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - I/O high speed at low voltage When this bit is set, the speed of the I/Os is optimized when the device voltage is low. This bit is active only if VDDIO_HSLV user option bit is set in FLASH. It must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
    #[inline(always)]
    pub fn hslv(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - XSPIM_P1 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P1 I/Os is optimized when the device voltage is low. This bit is active only if OCTO1_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
    #[inline(always)]
    pub fn octo1_iohslv(&self) -> OCTO1_IOHSLV_R {
        OCTO1_IOHSLV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - XSPIM_P2 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P2 I/Os is optimized when the device voltage is low. This bit is active only if OCTO2_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
    #[inline(always)]
    pub fn octo2_iohslv(&self) -> OCTO2_IOHSLV_R {
        OCTO2_IOHSLV_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCSR")
            .field("en", &self.en())
            .field("cs", &self.cs())
            .field("octo1_comp_en", &self.octo1_comp_en())
            .field("octo1_comp_codesel", &self.octo1_comp_codesel())
            .field("octo2_comp_en", &self.octo2_comp_en())
            .field("octo2_comp_codesel", &self.octo2_comp_codesel())
            .field("ready", &self.ready())
            .field("octo1_comp_rdy", &self.octo1_comp_rdy())
            .field("octo2_comp_rdy", &self.octo2_comp_rdy())
            .field("hslv", &self.hslv())
            .field("octo1_iohslv", &self.octo1_iohslv())
            .field("octo2_iohslv", &self.octo2_iohslv())
            .finish()
    }
}
impl W {
    ///Bit 0 - Compensation cell enable Set this bit to enable the compensation cell.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CCCSRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Compensation cell code selection This bit selects the code to be applied for the I/O compensation cell.
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W<'_, CCCSRrs> {
        CS_W::new(self, 1)
    }
    ///Bit 2 - XSPIM_P1 compensation cell enable Set this bit to enable the XSPIM_P1 compensation cell.
    #[inline(always)]
    pub fn octo1_comp_en(&mut self) -> OCTO1_COMP_EN_W<'_, CCCSRrs> {
        OCTO1_COMP_EN_W::new(self, 2)
    }
    ///Bit 3 - XSPIM_P1 compensation cell code selection This bit selects the code to be applied for the XSPIM_P1 I/O compensation cell.
    #[inline(always)]
    pub fn octo1_comp_codesel(&mut self) -> OCTO1_COMP_CODESEL_W<'_, CCCSRrs> {
        OCTO1_COMP_CODESEL_W::new(self, 3)
    }
    ///Bit 16 - I/O high speed at low voltage When this bit is set, the speed of the I/Os is optimized when the device voltage is low. This bit is active only if VDDIO_HSLV user option bit is set in FLASH. It must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
    #[inline(always)]
    pub fn hslv(&mut self) -> HSLV_W<'_, CCCSRrs> {
        HSLV_W::new(self, 16)
    }
    ///Bit 17 - XSPIM_P1 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P1 I/Os is optimized when the device voltage is low. This bit is active only if OCTO1_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
    #[inline(always)]
    pub fn octo1_iohslv(&mut self) -> OCTO1_IOHSLV_W<'_, CCCSRrs> {
        OCTO1_IOHSLV_W::new(self, 17)
    }
    ///Bit 18 - XSPIM_P2 I/O high speed at low voltage When this bit is set, the speed of the XSPIM_P2 I/Os is optimized when the device voltage is low. This bit is active only if OCTO2_HSLV user option bit is set in FLASH. This bit must be used only if the device supply voltage is below 2.7 V. Setting this bit when V<sub>DD</sub> is higher than 2.7 V may be destructive.
    #[inline(always)]
    pub fn octo2_iohslv(&mut self) -> OCTO2_IOHSLV_W<'_, CCCSRrs> {
        OCTO2_IOHSLV_W::new(self, 18)
    }
}
/**SBS I/O compensation cell control and status register

You can [`read`](crate::Reg::read) this register and get [`cccsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SBS:CCCSR)*/
pub struct CCCSRrs;
impl crate::RegisterSpec for CCCSRrs {
    type Ux = u32;
}
///`read()` method returns [`cccsr::R`](R) reader structure
impl crate::Readable for CCCSRrs {}
///`write(|w| ..)` method takes [`cccsr::W`](W) writer structure
impl crate::Writable for CCCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCCSR to value 0
impl crate::Resettable for CCCSRrs {}
