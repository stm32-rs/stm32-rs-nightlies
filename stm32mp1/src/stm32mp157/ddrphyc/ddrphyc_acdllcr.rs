///Register `DDRPHYC_ACDLLCR` reader
pub type R = crate::R<DDRPHYC_ACDLLCRrs>;
///Register `DDRPHYC_ACDLLCR` writer
pub type W = crate::W<DDRPHYC_ACDLLCRrs>;
///Field `MFBDLY` reader - MFBDLY
pub type MFBDLY_R = crate::FieldReader;
///Field `MFBDLY` writer - MFBDLY
pub type MFBDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MFWDLY` reader - MFWDLY
pub type MFWDLY_R = crate::FieldReader;
///Field `MFWDLY` writer - MFWDLY
pub type MFWDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ATESTEN` reader - ATESTEN
pub type ATESTEN_R = crate::BitReader;
///Field `ATESTEN` writer - ATESTEN
pub type ATESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLLSRST` reader - DLLSRST
pub type DLLSRST_R = crate::BitReader;
///Field `DLLSRST` writer - DLLSRST
pub type DLLSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLLDIS` reader - DLLDIS
pub type DLLDIS_R = crate::BitReader;
///Field `DLLDIS` writer - DLLDIS
pub type DLLDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
    ///Bit 18 - ATESTEN
    #[inline(always)]
    pub fn atesten(&self) -> ATESTEN_R {
        ATESTEN_R::new(((self.bits >> 18) & 1) != 0)
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
        f.debug_struct("DDRPHYC_ACDLLCR")
            .field("mfbdly", &self.mfbdly())
            .field("mfwdly", &self.mfwdly())
            .field("atesten", &self.atesten())
            .field("dllsrst", &self.dllsrst())
            .field("dlldis", &self.dlldis())
            .finish()
    }
}
impl W {
    ///Bits 6:8 - MFBDLY
    #[inline(always)]
    #[must_use]
    pub fn mfbdly(&mut self) -> MFBDLY_W<DDRPHYC_ACDLLCRrs> {
        MFBDLY_W::new(self, 6)
    }
    ///Bits 9:11 - MFWDLY
    #[inline(always)]
    #[must_use]
    pub fn mfwdly(&mut self) -> MFWDLY_W<DDRPHYC_ACDLLCRrs> {
        MFWDLY_W::new(self, 9)
    }
    ///Bit 18 - ATESTEN
    #[inline(always)]
    #[must_use]
    pub fn atesten(&mut self) -> ATESTEN_W<DDRPHYC_ACDLLCRrs> {
        ATESTEN_W::new(self, 18)
    }
    ///Bit 30 - DLLSRST
    #[inline(always)]
    #[must_use]
    pub fn dllsrst(&mut self) -> DLLSRST_W<DDRPHYC_ACDLLCRrs> {
        DLLSRST_W::new(self, 30)
    }
    ///Bit 31 - DLLDIS
    #[inline(always)]
    #[must_use]
    pub fn dlldis(&mut self) -> DLLDIS_W<DDRPHYC_ACDLLCRrs> {
        DLLDIS_W::new(self, 31)
    }
}
/**DDRPHYC AC DLL control register

You can [`read`](crate::Reg::read) this register and get [`ddrphyc_acdllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrphyc_acdllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DDRPHYC_ACDLLCR)*/
pub struct DDRPHYC_ACDLLCRrs;
impl crate::RegisterSpec for DDRPHYC_ACDLLCRrs {
    type Ux = u32;
}
///`read()` method returns [`ddrphyc_acdllcr::R`](R) reader structure
impl crate::Readable for DDRPHYC_ACDLLCRrs {}
///`write(|w| ..)` method takes [`ddrphyc_acdllcr::W`](W) writer structure
impl crate::Writable for DDRPHYC_ACDLLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRPHYC_ACDLLCR to value 0x4000_0000
impl crate::Resettable for DDRPHYC_ACDLLCRrs {
    const RESET_VALUE: u32 = 0x4000_0000;
}
