///Register `OTG_HS_DCFG` reader
pub type R = crate::R<OTG_HS_DCFGrs>;
///Register `OTG_HS_DCFG` writer
pub type W = crate::W<OTG_HS_DCFGrs>;
///Field `DSPD` reader - Device speed
pub type DSPD_R = crate::FieldReader;
///Field `DSPD` writer - Device speed
pub type DSPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NZLSOHSK` reader - Nonzero-length status OUT handshake
pub type NZLSOHSK_R = crate::BitReader;
///Field `NZLSOHSK` writer - Nonzero-length status OUT handshake
pub type NZLSOHSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAD` reader - Device address
pub type DAD_R = crate::FieldReader;
///Field `DAD` writer - Device address
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PFIVL` reader - Periodic (micro)frame interval
pub type PFIVL_R = crate::FieldReader;
///Field `PFIVL` writer - Periodic (micro)frame interval
pub type PFIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PERSCHIVL` reader - Periodic scheduling interval
pub type PERSCHIVL_R = crate::FieldReader;
///Field `PERSCHIVL` writer - Periodic scheduling interval
pub type PERSCHIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Device speed
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Nonzero-length status OUT handshake
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:10 - Device address
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    ///Bits 11:12 - Periodic (micro)frame interval
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 24:25 - Periodic scheduling interval
    #[inline(always)]
    pub fn perschivl(&self) -> PERSCHIVL_R {
        PERSCHIVL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_DCFG")
            .field("dspd", &self.dspd())
            .field("nzlsohsk", &self.nzlsohsk())
            .field("dad", &self.dad())
            .field("pfivl", &self.pfivl())
            .field("perschivl", &self.perschivl())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Device speed
    #[inline(always)]
    pub fn dspd(&mut self) -> DSPD_W<'_, OTG_HS_DCFGrs> {
        DSPD_W::new(self, 0)
    }
    ///Bit 2 - Nonzero-length status OUT handshake
    #[inline(always)]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W<'_, OTG_HS_DCFGrs> {
        NZLSOHSK_W::new(self, 2)
    }
    ///Bits 4:10 - Device address
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W<'_, OTG_HS_DCFGrs> {
        DAD_W::new(self, 4)
    }
    ///Bits 11:12 - Periodic (micro)frame interval
    #[inline(always)]
    pub fn pfivl(&mut self) -> PFIVL_W<'_, OTG_HS_DCFGrs> {
        PFIVL_W::new(self, 11)
    }
    ///Bits 24:25 - Periodic scheduling interval
    #[inline(always)]
    pub fn perschivl(&mut self) -> PERSCHIVL_W<'_, OTG_HS_DCFGrs> {
        PERSCHIVL_W::new(self, 24)
    }
}
/**OTG_HS device configuration register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_HS_DEVICE:OTG_HS_DCFG)*/
pub struct OTG_HS_DCFGrs;
impl crate::RegisterSpec for OTG_HS_DCFGrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_dcfg::R`](R) reader structure
impl crate::Readable for OTG_HS_DCFGrs {}
///`write(|w| ..)` method takes [`otg_hs_dcfg::W`](W) writer structure
impl crate::Writable for OTG_HS_DCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_DCFG to value 0x0220_0000
impl crate::Resettable for OTG_HS_DCFGrs {
    const RESET_VALUE: u32 = 0x0220_0000;
}
