///Register `MISCRSTR` reader
pub type R = crate::R<MISCRSTRrs>;
///Register `MISCRSTR` writer
pub type W = crate::W<MISCRSTRrs>;
///Field `DBGRST` reader - DBG reset
pub type DBGRST_R = crate::BitReader;
///Field `DBGRST` writer - DBG reset
pub type DBGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHY1RST` reader - XSPIPHY1 reset
pub type XSPIPHY1RST_R = crate::BitReader;
///Field `XSPIPHY1RST` writer - XSPIPHY1 reset
pub type XSPIPHY1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XSPIPHY2RST` reader - XSPIPHY2 reset
pub type XSPIPHY2RST_R = crate::BitReader;
///Field `XSPIPHY2RST` writer - XSPIPHY2 reset
pub type XSPIPHY2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1DLLRST` reader - SDMMC1DLL reset
pub type SDMMC1DLLRST_R = crate::BitReader;
///Field `SDMMC1DLLRST` writer - SDMMC1DLL reset
pub type SDMMC1DLLRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC2DLLRST` reader - SDMMC2DLL reset
pub type SDMMC2DLLRST_R = crate::BitReader;
///Field `SDMMC2DLLRST` writer - SDMMC2DLL reset
pub type SDMMC2DLLRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DBG reset
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - XSPIPHY1 reset
    #[inline(always)]
    pub fn xspiphy1rst(&self) -> XSPIPHY1RST_R {
        XSPIPHY1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - XSPIPHY2 reset
    #[inline(always)]
    pub fn xspiphy2rst(&self) -> XSPIPHY2RST_R {
        XSPIPHY2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - SDMMC1DLL reset
    #[inline(always)]
    pub fn sdmmc1dllrst(&self) -> SDMMC1DLLRST_R {
        SDMMC1DLLRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SDMMC2DLL reset
    #[inline(always)]
    pub fn sdmmc2dllrst(&self) -> SDMMC2DLLRST_R {
        SDMMC2DLLRST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISCRSTR")
            .field("dbgrst", &self.dbgrst())
            .field("xspiphy1rst", &self.xspiphy1rst())
            .field("xspiphy2rst", &self.xspiphy2rst())
            .field("sdmmc1dllrst", &self.sdmmc1dllrst())
            .field("sdmmc2dllrst", &self.sdmmc2dllrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DBG reset
    #[inline(always)]
    pub fn dbgrst(&mut self) -> DBGRST_W<'_, MISCRSTRrs> {
        DBGRST_W::new(self, 0)
    }
    ///Bit 4 - XSPIPHY1 reset
    #[inline(always)]
    pub fn xspiphy1rst(&mut self) -> XSPIPHY1RST_W<'_, MISCRSTRrs> {
        XSPIPHY1RST_W::new(self, 4)
    }
    ///Bit 5 - XSPIPHY2 reset
    #[inline(always)]
    pub fn xspiphy2rst(&mut self) -> XSPIPHY2RST_W<'_, MISCRSTRrs> {
        XSPIPHY2RST_W::new(self, 5)
    }
    ///Bit 7 - SDMMC1DLL reset
    #[inline(always)]
    pub fn sdmmc1dllrst(&mut self) -> SDMMC1DLLRST_W<'_, MISCRSTRrs> {
        SDMMC1DLLRST_W::new(self, 7)
    }
    ///Bit 8 - SDMMC2DLL reset
    #[inline(always)]
    pub fn sdmmc2dllrst(&mut self) -> SDMMC2DLLRST_W<'_, MISCRSTRrs> {
        SDMMC2DLLRST_W::new(self, 8)
    }
}
/**RCC miscellaneous configurations reset register

You can [`read`](crate::Reg::read) this register and get [`miscrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:MISCRSTR)*/
pub struct MISCRSTRrs;
impl crate::RegisterSpec for MISCRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`miscrstr::R`](R) reader structure
impl crate::Readable for MISCRSTRrs {}
///`write(|w| ..)` method takes [`miscrstr::W`](W) writer structure
impl crate::Writable for MISCRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MISCRSTR to value 0
impl crate::Resettable for MISCRSTRrs {}
