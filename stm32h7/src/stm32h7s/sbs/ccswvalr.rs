///Register `CCSWVALR` reader
pub type R = crate::R<CCSWVALRrs>;
///Register `CCSWVALR` writer
pub type W = crate::W<CCSWVALRrs>;
///Field `SW_NSRC` reader - Software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the NMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register.
pub type SW_NSRC_R = crate::FieldReader;
///Field `SW_NSRC` writer - Software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the NMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register.
pub type SW_NSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SW_PSRC` reader - Software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the PMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register.
pub type SW_PSRC_R = crate::FieldReader;
///Field `SW_PSRC` writer - Software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the PMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register.
pub type SW_PSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `OCTO1_SW_NSRC` reader - XSPIM_P1 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew -ate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the NMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register.
pub type OCTO1_SW_NSRC_R = crate::FieldReader;
///Field `OCTO1_SW_NSRC` writer - XSPIM_P1 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew -ate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the NMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register.
pub type OCTO1_SW_NSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `OCTO1_SW_PSRC` reader - XSPIM_P1 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the PMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register.
pub type OCTO1_SW_PSRC_R = crate::FieldReader;
///Field `OCTO1_SW_PSRC` writer - XSPIM_P1 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the PMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register.
pub type OCTO1_SW_PSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `OCTO2_SW_NSRC` reader - XSPIM_P2 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the NMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register.
pub type OCTO2_SW_NSRC_R = crate::FieldReader;
///Field `OCTO2_SW_NSRC` writer - XSPIM_P2 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the NMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register.
pub type OCTO2_SW_NSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `OCTO2_SW_PSRC` reader - XSPIM_P2 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the PMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register.
pub type OCTO2_SW_PSRC_R = crate::FieldReader;
///Field `OCTO2_SW_PSRC` writer - XSPIM_P2 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the PMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register.
pub type OCTO2_SW_PSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the NMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn sw_nsrc(&self) -> SW_NSRC_R {
        SW_NSRC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the PMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn sw_psrc(&self) -> SW_PSRC_R {
        SW_PSRC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - XSPIM_P1 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew -ate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the NMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo1_sw_nsrc(&self) -> OCTO1_SW_NSRC_R {
        OCTO1_SW_NSRC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - XSPIM_P1 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the PMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo1_sw_psrc(&self) -> OCTO1_SW_PSRC_R {
        OCTO1_SW_PSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - XSPIM_P2 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the NMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo2_sw_nsrc(&self) -> OCTO2_SW_NSRC_R {
        OCTO2_SW_NSRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - XSPIM_P2 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the PMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo2_sw_psrc(&self) -> OCTO2_SW_PSRC_R {
        OCTO2_SW_PSRC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCSWVALR")
            .field("sw_nsrc", &self.sw_nsrc())
            .field("sw_psrc", &self.sw_psrc())
            .field("octo1_sw_nsrc", &self.octo1_sw_nsrc())
            .field("octo1_sw_psrc", &self.octo1_sw_psrc())
            .field("octo2_sw_nsrc", &self.octo2_sw_nsrc())
            .field("octo2_sw_psrc", &self.octo2_sw_psrc())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the NMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn sw_nsrc(&mut self) -> SW_NSRC_W<'_, CCSWVALRrs> {
        SW_NSRC_W::new(self, 0)
    }
    ///Bits 4:7 - Software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted to compensate the PMOS transistors slew rate in the functional range if COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn sw_psrc(&mut self) -> SW_PSRC_W<'_, CCSWVALRrs> {
        SW_PSRC_W::new(self, 4)
    }
    ///Bits 8:11 - XSPIM_P1 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew -ate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the NMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo1_sw_nsrc(&mut self) -> OCTO1_SW_NSRC_W<'_, CCSWVALRrs> {
        OCTO1_SW_NSRC_W::new(self, 8)
    }
    ///Bits 12:15 - XSPIM_P1 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P1 to compensate the PMOS transistors slew rate in the functional range if OCTO1_COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo1_sw_psrc(&mut self) -> OCTO1_SW_PSRC_W<'_, CCSWVALRrs> {
        OCTO1_SW_PSRC_W::new(self, 12)
    }
    ///Bits 16:19 - XSPIM_P2 software NMOS transistors slew-rate compensation This bitfield returns the NMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the NMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo2_sw_nsrc(&mut self) -> OCTO2_SW_NSRC_W<'_, CCSWVALRrs> {
        OCTO2_SW_NSRC_W::new(self, 16)
    }
    ///Bits 20:23 - XSPIM_P2 software PMOS transistors slew-rate compensation This bitfield returns the PMOS transistors slew-rate compensation value computed by the cell. It is interpreted by XSPIM_P2 to compensate the PMOS transistors slew rate in the functional range if OCTO2_COMP_CODESEL = 1 in SBS_CCCSR register.
    #[inline(always)]
    pub fn octo2_sw_psrc(&mut self) -> OCTO2_SW_PSRC_W<'_, CCSWVALRrs> {
        OCTO2_SW_PSRC_W::new(self, 20)
    }
}
/**SBS compensation cell for I/Os software value register

You can [`read`](crate::Reg::read) this register and get [`ccswvalr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccswvalr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:CCSWVALR)*/
pub struct CCSWVALRrs;
impl crate::RegisterSpec for CCSWVALRrs {
    type Ux = u32;
}
///`read()` method returns [`ccswvalr::R`](R) reader structure
impl crate::Readable for CCSWVALRrs {}
///`write(|w| ..)` method takes [`ccswvalr::W`](W) writer structure
impl crate::Writable for CCSWVALRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCSWVALR to value 0x88
impl crate::Resettable for CCSWVALRrs {
    const RESET_VALUE: u32 = 0x88;
}
