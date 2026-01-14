///Register `ACDLLCR` reader
pub type R = crate::R<ACDLLCRrs>;
///Register `ACDLLCR` writer
pub type W = crate::W<ACDLLCRrs>;
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
        f.debug_struct("ACDLLCR")
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
    pub fn mfbdly(&mut self) -> MFBDLY_W<'_, ACDLLCRrs> {
        MFBDLY_W::new(self, 6)
    }
    ///Bits 9:11 - MFWDLY
    #[inline(always)]
    pub fn mfwdly(&mut self) -> MFWDLY_W<'_, ACDLLCRrs> {
        MFWDLY_W::new(self, 9)
    }
    ///Bit 18 - ATESTEN
    #[inline(always)]
    pub fn atesten(&mut self) -> ATESTEN_W<'_, ACDLLCRrs> {
        ATESTEN_W::new(self, 18)
    }
    ///Bit 30 - DLLSRST
    #[inline(always)]
    pub fn dllsrst(&mut self) -> DLLSRST_W<'_, ACDLLCRrs> {
        DLLSRST_W::new(self, 30)
    }
    ///Bit 31 - DLLDIS
    #[inline(always)]
    pub fn dlldis(&mut self) -> DLLDIS_W<'_, ACDLLCRrs> {
        DLLDIS_W::new(self, 31)
    }
}
/**DDRPHYC AC DLL control register

You can [`read`](crate::Reg::read) this register and get [`acdllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acdllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:ACDLLCR)*/
pub struct ACDLLCRrs;
impl crate::RegisterSpec for ACDLLCRrs {
    type Ux = u32;
}
///`read()` method returns [`acdllcr::R`](R) reader structure
impl crate::Readable for ACDLLCRrs {}
///`write(|w| ..)` method takes [`acdllcr::W`](W) writer structure
impl crate::Writable for ACDLLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACDLLCR to value 0x4000_0000
impl crate::Resettable for ACDLLCRrs {
    const RESET_VALUE: u32 = 0x4000_0000;
}
