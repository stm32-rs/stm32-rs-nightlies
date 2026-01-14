///Register `CCVALR` reader
pub type R = crate::R<CCVALRrs>;
///Field `NSRC` reader - NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the NMOS transistors slew rate in the functional range if COMP_CODESEL = 0 in SBS_CCCSR register.
pub type NSRC_R = crate::FieldReader;
///Field `PSRC` reader - PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the PMOS transistors slew rate in the functional range if COMP_CODESEL = 0 in SBS_CCCSR register.
pub type PSRC_R = crate::FieldReader;
///Field `OCTO1_NSRC` reader - XSPIM_P1 NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the NMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 0 in SBS_CCCSR register.
pub type OCTO1_NSRC_R = crate::FieldReader;
///Field `OCTO1_PSRC` reader - XSPIM_P1 PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the PMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 0 in SBS_CCCSR register.
pub type OCTO1_PSRC_R = crate::FieldReader;
///Field `OCTO2_NSRC` reader - XSPIM_P2 NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the NMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 0 in SBS_CCCSR register.
pub type OCTO2_NSRC_R = crate::FieldReader;
///Field `OCTO2_PSRC` reader - XSPIM_P2 PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the PMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 0 in SBS_CCCSR register.
pub type OCTO2_PSRC_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the NMOS transistors slew rate in the functional range if COMP_CODESEL = 0 in SBS_CCCSR register.
    #[inline(always)]
    pub fn nsrc(&self) -> NSRC_R {
        NSRC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the PMOS transistors slew rate in the functional range if COMP_CODESEL = 0 in SBS_CCCSR register.
    #[inline(always)]
    pub fn psrc(&self) -> PSRC_R {
        PSRC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - XSPIM_P1 NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the NMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 0 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo1_nsrc(&self) -> OCTO1_NSRC_R {
        OCTO1_NSRC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - XSPIM_P1 PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the PMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 0 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo1_psrc(&self) -> OCTO1_PSRC_R {
        OCTO1_PSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - XSPIM_P2 NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the NMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 0 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo2_nsrc(&self) -> OCTO2_NSRC_R {
        OCTO2_NSRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - XSPIM_P2 PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the PMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 0 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo2_psrc(&self) -> OCTO2_PSRC_R {
        OCTO2_PSRC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCVALR")
            .field("nsrc", &self.nsrc())
            .field("psrc", &self.psrc())
            .field("octo1_nsrc", &self.octo1_nsrc())
            .field("octo1_psrc", &self.octo1_psrc())
            .field("octo2_nsrc", &self.octo2_nsrc())
            .field("octo2_psrc", &self.octo2_psrc())
            .finish()
    }
}
/**SBS compensation cell for I/Os value register

You can [`read`](crate::Reg::read) this register and get [`ccvalr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SBS:CCVALR)*/
pub struct CCVALRrs;
impl crate::RegisterSpec for CCVALRrs {
    type Ux = u32;
}
///`read()` method returns [`ccvalr::R`](R) reader structure
impl crate::Readable for CCVALRrs {}
///`reset()` method sets CCVALR to value 0x88
impl crate::Resettable for CCVALRrs {
    const RESET_VALUE: u32 = 0x88;
}
