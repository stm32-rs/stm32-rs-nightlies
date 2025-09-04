///Register `DX3DLLCR` reader
pub type R = crate::R<DX3DLLCRrs>;
///Register `DX3DLLCR` writer
pub type W = crate::W<DX3DLLCRrs>;
///Field `SFBDLY` reader - SFBDLY
pub type SFBDLY_R = crate::FieldReader;
///Field `SFBDLY` writer - SFBDLY
pub type SFBDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SFWDLY` reader - SFWDLY
pub type SFWDLY_R = crate::FieldReader;
///Field `SFWDLY` writer - SFWDLY
pub type SFWDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MFBDLY` reader - MFBDLY
pub type MFBDLY_R = crate::FieldReader;
///Field `MFBDLY` writer - MFBDLY
pub type MFBDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MFWDLY` reader - MFWDLY
pub type MFWDLY_R = crate::FieldReader;
///Field `MFWDLY` writer - MFWDLY
pub type MFWDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SSTART` reader - SSTART
pub type SSTART_R = crate::FieldReader;
///Field `SSTART` writer - SSTART
pub type SSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SDPHASE` reader - SDPHASE
pub type SDPHASE_R = crate::FieldReader;
///Field `SDPHASE` writer - SDPHASE
pub type SDPHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ATESTEN` reader - ATESTEN
pub type ATESTEN_R = crate::BitReader;
///Field `ATESTEN` writer - ATESTEN
pub type ATESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDLBMODE` reader - SDLBMODE
pub type SDLBMODE_R = crate::BitReader;
///Field `SDLBMODE` writer - SDLBMODE
pub type SDLBMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLLSRST` reader - DLLSRST
pub type DLLSRST_R = crate::BitReader;
///Field `DLLSRST` writer - DLLSRST
pub type DLLSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLLDIS` reader - DLLDIS
pub type DLLDIS_R = crate::BitReader;
///Field `DLLDIS` writer - DLLDIS
pub type DLLDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - SFBDLY
    #[inline(always)]
    pub fn sfbdly(&self) -> SFBDLY_R {
        SFBDLY_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - SFWDLY
    #[inline(always)]
    pub fn sfwdly(&self) -> SFWDLY_R {
        SFWDLY_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - MFBDLY
    #[inline(always)]
    pub fn mfbdly(&self) -> MFBDLY_R {
        MFBDLY_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - MFWDLY
    #[inline(always)]
    pub fn mfwdly(&self) -> MFWDLY_R {
        MFWDLY_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:13 - SSTART
    #[inline(always)]
    pub fn sstart(&self) -> SSTART_R {
        SSTART_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:17 - SDPHASE
    #[inline(always)]
    pub fn sdphase(&self) -> SDPHASE_R {
        SDPHASE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    ///Bit 18 - ATESTEN
    #[inline(always)]
    pub fn atesten(&self) -> ATESTEN_R {
        ATESTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SDLBMODE
    #[inline(always)]
    pub fn sdlbmode(&self) -> SDLBMODE_R {
        SDLBMODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 30 - DLLSRST
    #[inline(always)]
    pub fn dllsrst(&self) -> DLLSRST_R {
        DLLSRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DLLDIS
    #[inline(always)]
    pub fn dlldis(&self) -> DLLDIS_R {
        DLLDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DX3DLLCR")
            .field("sfbdly", &self.sfbdly())
            .field("sfwdly", &self.sfwdly())
            .field("mfbdly", &self.mfbdly())
            .field("mfwdly", &self.mfwdly())
            .field("sstart", &self.sstart())
            .field("sdphase", &self.sdphase())
            .field("atesten", &self.atesten())
            .field("sdlbmode", &self.sdlbmode())
            .field("dllsrst", &self.dllsrst())
            .field("dlldis", &self.dlldis())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SFBDLY
    #[inline(always)]
    pub fn sfbdly(&mut self) -> SFBDLY_W<DX3DLLCRrs> {
        SFBDLY_W::new(self, 0)
    }
    ///Bits 3:5 - SFWDLY
    #[inline(always)]
    pub fn sfwdly(&mut self) -> SFWDLY_W<DX3DLLCRrs> {
        SFWDLY_W::new(self, 3)
    }
    ///Bits 6:8 - MFBDLY
    #[inline(always)]
    pub fn mfbdly(&mut self) -> MFBDLY_W<DX3DLLCRrs> {
        MFBDLY_W::new(self, 6)
    }
    ///Bits 9:11 - MFWDLY
    #[inline(always)]
    pub fn mfwdly(&mut self) -> MFWDLY_W<DX3DLLCRrs> {
        MFWDLY_W::new(self, 9)
    }
    ///Bits 12:13 - SSTART
    #[inline(always)]
    pub fn sstart(&mut self) -> SSTART_W<DX3DLLCRrs> {
        SSTART_W::new(self, 12)
    }
    ///Bits 14:17 - SDPHASE
    #[inline(always)]
    pub fn sdphase(&mut self) -> SDPHASE_W<DX3DLLCRrs> {
        SDPHASE_W::new(self, 14)
    }
    ///Bit 18 - ATESTEN
    #[inline(always)]
    pub fn atesten(&mut self) -> ATESTEN_W<DX3DLLCRrs> {
        ATESTEN_W::new(self, 18)
    }
    ///Bit 19 - SDLBMODE
    #[inline(always)]
    pub fn sdlbmode(&mut self) -> SDLBMODE_W<DX3DLLCRrs> {
        SDLBMODE_W::new(self, 19)
    }
    ///Bit 30 - DLLSRST
    #[inline(always)]
    pub fn dllsrst(&mut self) -> DLLSRST_W<DX3DLLCRrs> {
        DLLSRST_W::new(self, 30)
    }
    ///Bit 31 - DLLDIS
    #[inline(always)]
    pub fn dlldis(&mut self) -> DLLDIS_W<DX3DLLCRrs> {
        DLLDIS_W::new(self, 31)
    }
}
/**DDRPHYC byte lane 3 DLLC register

You can [`read`](crate::Reg::read) this register and get [`dx3dllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dx3dllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DX3DLLCR)*/
pub struct DX3DLLCRrs;
impl crate::RegisterSpec for DX3DLLCRrs {
    type Ux = u32;
}
///`read()` method returns [`dx3dllcr::R`](R) reader structure
impl crate::Readable for DX3DLLCRrs {}
///`write(|w| ..)` method takes [`dx3dllcr::W`](W) writer structure
impl crate::Writable for DX3DLLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DX3DLLCR to value 0x4000_0000
impl crate::Resettable for DX3DLLCRrs {
    const RESET_VALUE: u32 = 0x4000_0000;
}
