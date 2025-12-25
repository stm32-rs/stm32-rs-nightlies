///Register `AHB2RSTR` reader
pub type R = crate::R<AHB2RSTRrs>;
///Register `AHB2RSTR` writer
pub type W = crate::W<AHB2RSTRrs>;
///Field `DCMIRST` reader - Camera interface reset
pub type DCMIRST_R = crate::BitReader;
///Field `DCMIRST` writer - Camera interface reset
pub type DCMIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPRST` reader - Cryptographic module reset
pub type CRYPRST_R = crate::BitReader;
///Field `CRYPRST` writer - Cryptographic module reset
pub type CRYPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSAHRST` reader - Hash module reset
pub type HSAHRST_R = crate::BitReader;
///Field `HSAHRST` writer - Hash module reset
pub type HSAHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGRST` reader - Random number generator module reset
pub type RNGRST_R = crate::BitReader;
///Field `RNGRST` writer - Random number generator module reset
pub type RNGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGFSRST` reader - USB OTG FS module reset
pub type OTGFSRST_R = crate::BitReader;
///Field `OTGFSRST` writer - USB OTG FS module reset
pub type OTGFSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Camera interface reset
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Cryptographic module reset
    #[inline(always)]
    pub fn cryprst(&self) -> CRYPRST_R {
        CRYPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Hash module reset
    #[inline(always)]
    pub fn hsahrst(&self) -> HSAHRST_R {
        HSAHRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Random number generator module reset
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USB OTG FS module reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR")
            .field("otgfsrst", &self.otgfsrst())
            .field("rngrst", &self.rngrst())
            .field("hsahrst", &self.hsahrst())
            .field("cryprst", &self.cryprst())
            .field("dcmirst", &self.dcmirst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Camera interface reset
    #[inline(always)]
    pub fn dcmirst(&mut self) -> DCMIRST_W<'_, AHB2RSTRrs> {
        DCMIRST_W::new(self, 0)
    }
    ///Bit 4 - Cryptographic module reset
    #[inline(always)]
    pub fn cryprst(&mut self) -> CRYPRST_W<'_, AHB2RSTRrs> {
        CRYPRST_W::new(self, 4)
    }
    ///Bit 5 - Hash module reset
    #[inline(always)]
    pub fn hsahrst(&mut self) -> HSAHRST_W<'_, AHB2RSTRrs> {
        HSAHRST_W::new(self, 5)
    }
    ///Bit 6 - Random number generator module reset
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHB2RSTRrs> {
        RNGRST_W::new(self, 6)
    }
    ///Bit 7 - USB OTG FS module reset
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<'_, AHB2RSTRrs> {
        OTGFSRST_W::new(self, 7)
    }
}
/**AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#RCC:AHB2RSTR)*/
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2rstr::R`](R) reader structure
impl crate::Readable for AHB2RSTRrs {}
///`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTRrs {}
