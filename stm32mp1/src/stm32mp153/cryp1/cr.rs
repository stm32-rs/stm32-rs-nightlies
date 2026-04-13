///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `ALGODIR` reader - ALGODIR
pub type ALGODIR_R = crate::BitReader;
///Field `ALGODIR` writer - ALGODIR
pub type ALGODIR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALGOMODE` reader - ALGOMODE
pub type ALGOMODE_R = crate::FieldReader;
///Field `ALGOMODE` writer - ALGOMODE
pub type ALGOMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DATATYPE` reader - DATATYPE
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - DATATYPE
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `KEYSIZE` reader - KEYSIZE
pub type KEYSIZE_R = crate::FieldReader;
///Field `KEYSIZE` writer - KEYSIZE
pub type KEYSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FFLUSH` writer - FFLUSH
pub type FFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPEN` reader - CRYPEN
pub type CRYPEN_R = crate::BitReader;
///Field `CRYPEN` writer - CRYPEN
pub type CRYPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCM_CCMPH` reader - GCM_CCMPH
pub type GCM_CCMPH_R = crate::FieldReader;
///Field `GCM_CCMPH` writer - GCM_CCMPH
pub type GCM_CCMPH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ALGOMODE3` reader - ALGOMODE3
pub type ALGOMODE3_R = crate::BitReader;
///Field `ALGOMODE3` writer - ALGOMODE3
pub type ALGOMODE3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPBLB` reader - NPBLB
pub type NPBLB_R = crate::FieldReader;
///Field `NPBLB` writer - NPBLB
pub type NPBLB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 2 - ALGODIR
    #[inline(always)]
    pub fn algodir(&self) -> ALGODIR_R {
        ALGODIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - ALGOMODE
    #[inline(always)]
    pub fn algomode(&self) -> ALGOMODE_R {
        ALGOMODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:7 - DATATYPE
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - KEYSIZE
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 15 - CRYPEN
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - GCM_CCMPH
    #[inline(always)]
    pub fn gcm_ccmph(&self) -> GCM_CCMPH_R {
        GCM_CCMPH_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 19 - ALGOMODE3
    #[inline(always)]
    pub fn algomode3(&self) -> ALGOMODE3_R {
        ALGOMODE3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:23 - NPBLB
    #[inline(always)]
    pub fn npblb(&self) -> NPBLB_R {
        NPBLB_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("algodir", &self.algodir())
            .field("algomode", &self.algomode())
            .field("datatype", &self.datatype())
            .field("keysize", &self.keysize())
            .field("crypen", &self.crypen())
            .field("gcm_ccmph", &self.gcm_ccmph())
            .field("algomode3", &self.algomode3())
            .field("npblb", &self.npblb())
            .finish()
    }
}
impl W {
    ///Bit 2 - ALGODIR
    #[inline(always)]
    pub fn algodir(&mut self) -> ALGODIR_W<'_, CRrs> {
        ALGODIR_W::new(self, 2)
    }
    ///Bits 3:5 - ALGOMODE
    #[inline(always)]
    pub fn algomode(&mut self) -> ALGOMODE_W<'_, CRrs> {
        ALGOMODE_W::new(self, 3)
    }
    ///Bits 6:7 - DATATYPE
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<'_, CRrs> {
        DATATYPE_W::new(self, 6)
    }
    ///Bits 8:9 - KEYSIZE
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<'_, CRrs> {
        KEYSIZE_W::new(self, 8)
    }
    ///Bit 14 - FFLUSH
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W<'_, CRrs> {
        FFLUSH_W::new(self, 14)
    }
    ///Bit 15 - CRYPEN
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W<'_, CRrs> {
        CRYPEN_W::new(self, 15)
    }
    ///Bits 16:17 - GCM_CCMPH
    #[inline(always)]
    pub fn gcm_ccmph(&mut self) -> GCM_CCMPH_W<'_, CRrs> {
        GCM_CCMPH_W::new(self, 16)
    }
    ///Bit 19 - ALGOMODE3
    #[inline(always)]
    pub fn algomode3(&mut self) -> ALGOMODE3_W<'_, CRrs> {
        ALGOMODE3_W::new(self, 19)
    }
    ///Bits 20:23 - NPBLB
    #[inline(always)]
    pub fn npblb(&mut self) -> NPBLB_W<'_, CRrs> {
        NPBLB_W::new(self, 20)
    }
}
/**CRYP control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CRYP1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
