///Register `DFLT3CICR` reader
pub type R = crate::R<DFLT3CICRrs>;
///Register `DFLT3CICR` writer
pub type W = crate::W<DFLT3CICRrs>;
///Field `DATSRC` reader - Source data for the digital filter
pub type DATSRC_R = crate::FieldReader;
///Field `DATSRC` writer - Source data for the digital filter
pub type DATSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CICMOD` reader - Select the CIC mode
pub type CICMOD_R = crate::FieldReader;
///Field `CICMOD` writer - Select the CIC mode
pub type CICMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCICD` reader - CIC decimation ratio selection
pub type MCICD_R = crate::FieldReader;
///Field `MCICD` writer - CIC decimation ratio selection
pub type MCICD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MCICD8` reader - CIC decimation ratio selection
pub type MCICD8_R = crate::BitReader;
///Field `MCICD8` writer - CIC decimation ratio selection
pub type MCICD8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCALE` reader - Scaling factor selection
pub type SCALE_R = crate::FieldReader;
///Field `SCALE` writer - Scaling factor selection
pub type SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:1 - Source data for the digital filter
    #[inline(always)]
    pub fn datsrc(&self) -> DATSRC_R {
        DATSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:6 - Select the CIC mode
    #[inline(always)]
    pub fn cicmod(&self) -> CICMOD_R {
        CICMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:15 - CIC decimation ratio selection
    #[inline(always)]
    pub fn mcicd(&self) -> MCICD_R {
        MCICD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - CIC decimation ratio selection
    #[inline(always)]
    pub fn mcicd8(&self) -> MCICD8_R {
        MCICD8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:25 - Scaling factor selection
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT3CICR")
            .field("datsrc", &self.datsrc())
            .field("cicmod", &self.cicmod())
            .field("mcicd", &self.mcicd())
            .field("mcicd8", &self.mcicd8())
            .field("scale", &self.scale())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Source data for the digital filter
    #[inline(always)]
    pub fn datsrc(&mut self) -> DATSRC_W<'_, DFLT3CICRrs> {
        DATSRC_W::new(self, 0)
    }
    ///Bits 4:6 - Select the CIC mode
    #[inline(always)]
    pub fn cicmod(&mut self) -> CICMOD_W<'_, DFLT3CICRrs> {
        CICMOD_W::new(self, 4)
    }
    ///Bits 8:15 - CIC decimation ratio selection
    #[inline(always)]
    pub fn mcicd(&mut self) -> MCICD_W<'_, DFLT3CICRrs> {
        MCICD_W::new(self, 8)
    }
    ///Bit 16 - CIC decimation ratio selection
    #[inline(always)]
    pub fn mcicd8(&mut self) -> MCICD8_W<'_, DFLT3CICRrs> {
        MCICD8_W::new(self, 16)
    }
    ///Bits 20:25 - Scaling factor selection
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<'_, DFLT3CICRrs> {
        SCALE_W::new(self, 20)
    }
}
/**MDF digital filter configuration register 3

You can [`read`](crate::Reg::read) this register and get [`dflt3cicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt3cicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:DFLT3CICR)*/
pub struct DFLT3CICRrs;
impl crate::RegisterSpec for DFLT3CICRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt3cicr::R`](R) reader structure
impl crate::Readable for DFLT3CICRrs {}
///`write(|w| ..)` method takes [`dflt3cicr::W`](W) writer structure
impl crate::Writable for DFLT3CICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT3CICR to value 0
impl crate::Resettable for DFLT3CICRrs {}
