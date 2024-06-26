///Register `OTG_DCFG` reader
pub type R = crate::R<OTG_DCFGrs>;
///Register `OTG_DCFG` writer
pub type W = crate::W<OTG_DCFGrs>;
///Field `DSPD` reader - DSPD
pub type DSPD_R = crate::FieldReader;
///Field `DSPD` writer - DSPD
pub type DSPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NZLSOHSK` reader - NZLSOHSK
pub type NZLSOHSK_R = crate::BitReader;
///Field `NZLSOHSK` writer - NZLSOHSK
pub type NZLSOHSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAD` reader - DAD
pub type DAD_R = crate::FieldReader;
///Field `DAD` writer - DAD
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PFIVL` reader - PFIVL
pub type PFIVL_R = crate::FieldReader;
///Field `PFIVL` writer - PFIVL
pub type PFIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `XCVRDLY` reader - XCVRDLY
pub type XCVRDLY_R = crate::BitReader;
///Field `XCVRDLY` writer - XCVRDLY
pub type XCVRDLY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRATIM` reader - ERRATIM
pub type ERRATIM_R = crate::BitReader;
///Field `ERRATIM` writer - ERRATIM
pub type ERRATIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERSCHIVL` reader - PERSCHIVL
pub type PERSCHIVL_R = crate::FieldReader;
///Field `PERSCHIVL` writer - PERSCHIVL
pub type PERSCHIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - DSPD
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - NZLSOHSK
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:10 - DAD
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    ///Bits 11:12 - PFIVL
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 14 - XCVRDLY
    #[inline(always)]
    pub fn xcvrdly(&self) -> XCVRDLY_R {
        XCVRDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ERRATIM
    #[inline(always)]
    pub fn erratim(&self) -> ERRATIM_R {
        ERRATIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 24:25 - PERSCHIVL
    #[inline(always)]
    pub fn perschivl(&self) -> PERSCHIVL_R {
        PERSCHIVL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_DCFG")
            .field("dspd", &self.dspd())
            .field("nzlsohsk", &self.nzlsohsk())
            .field("dad", &self.dad())
            .field("pfivl", &self.pfivl())
            .field("xcvrdly", &self.xcvrdly())
            .field("erratim", &self.erratim())
            .field("perschivl", &self.perschivl())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - DSPD
    #[inline(always)]
    #[must_use]
    pub fn dspd(&mut self) -> DSPD_W<OTG_DCFGrs> {
        DSPD_W::new(self, 0)
    }
    ///Bit 2 - NZLSOHSK
    #[inline(always)]
    #[must_use]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W<OTG_DCFGrs> {
        NZLSOHSK_W::new(self, 2)
    }
    ///Bits 4:10 - DAD
    #[inline(always)]
    #[must_use]
    pub fn dad(&mut self) -> DAD_W<OTG_DCFGrs> {
        DAD_W::new(self, 4)
    }
    ///Bits 11:12 - PFIVL
    #[inline(always)]
    #[must_use]
    pub fn pfivl(&mut self) -> PFIVL_W<OTG_DCFGrs> {
        PFIVL_W::new(self, 11)
    }
    ///Bit 14 - XCVRDLY
    #[inline(always)]
    #[must_use]
    pub fn xcvrdly(&mut self) -> XCVRDLY_W<OTG_DCFGrs> {
        XCVRDLY_W::new(self, 14)
    }
    ///Bit 15 - ERRATIM
    #[inline(always)]
    #[must_use]
    pub fn erratim(&mut self) -> ERRATIM_W<OTG_DCFGrs> {
        ERRATIM_W::new(self, 15)
    }
    ///Bits 24:25 - PERSCHIVL
    #[inline(always)]
    #[must_use]
    pub fn perschivl(&mut self) -> PERSCHIVL_W<OTG_DCFGrs> {
        PERSCHIVL_W::new(self, 24)
    }
}
/**This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.

You can [`read`](crate::Reg::read) this register and get [`otg_dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:OTG_DCFG)*/
pub struct OTG_DCFGrs;
impl crate::RegisterSpec for OTG_DCFGrs {
    type Ux = u32;
}
///`read()` method returns [`otg_dcfg::R`](R) reader structure
impl crate::Readable for OTG_DCFGrs {}
///`write(|w| ..)` method takes [`otg_dcfg::W`](W) writer structure
impl crate::Writable for OTG_DCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTG_DCFG to value 0x0220_0000
impl crate::Resettable for OTG_DCFGrs {
    const RESET_VALUE: u32 = 0x0220_0000;
}
