///Register `DCFG` reader
pub type R = crate::R<DCFGrs>;
///Register `DCFG` writer
pub type W = crate::W<DCFGrs>;
///Field `DSPD` reader - Device speed
pub type DSPD_R = crate::FieldReader;
///Field `DSPD` writer - Device speed
pub type DSPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NZLSOHSK` reader - Non-zero-length status OUT handshake
pub type NZLSOHSK_R = crate::BitReader;
///Field `NZLSOHSK` writer - Non-zero-length status OUT handshake
pub type NZLSOHSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAD` reader - Device address
pub type DAD_R = crate::FieldReader;
///Field `DAD` writer - Device address
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PFIVL` reader - Periodic frame interval
pub type PFIVL_R = crate::FieldReader;
///Field `PFIVL` writer - Periodic frame interval
pub type PFIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Device speed
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Non-zero-length status OUT handshake
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:10 - Device address
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    ///Bits 11:12 - Periodic frame interval
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCFG")
            .field("dspd", &self.dspd())
            .field("nzlsohsk", &self.nzlsohsk())
            .field("dad", &self.dad())
            .field("pfivl", &self.pfivl())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Device speed
    #[inline(always)]
    pub fn dspd(&mut self) -> DSPD_W<'_, DCFGrs> {
        DSPD_W::new(self, 0)
    }
    ///Bit 2 - Non-zero-length status OUT handshake
    #[inline(always)]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W<'_, DCFGrs> {
        NZLSOHSK_W::new(self, 2)
    }
    ///Bits 4:10 - Device address
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W<'_, DCFGrs> {
        DAD_W::new(self, 4)
    }
    ///Bits 11:12 - Periodic frame interval
    #[inline(always)]
    pub fn pfivl(&mut self) -> PFIVL_W<'_, DCFGrs> {
        PFIVL_W::new(self, 11)
    }
}
/**OTG_FS device configuration register (OTG_FS_DCFG)

You can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#OTG_FS_DEVICE:DCFG)*/
pub struct DCFGrs;
impl crate::RegisterSpec for DCFGrs {
    type Ux = u32;
}
///`read()` method returns [`dcfg::R`](R) reader structure
impl crate::Readable for DCFGrs {}
///`write(|w| ..)` method takes [`dcfg::W`](W) writer structure
impl crate::Writable for DCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCFG to value 0x0220_0000
impl crate::Resettable for DCFGrs {
    const RESET_VALUE: u32 = 0x0220_0000;
}
