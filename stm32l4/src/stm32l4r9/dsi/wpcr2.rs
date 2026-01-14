///Register `WPCR2` reader
pub type R = crate::R<WPCR2rs>;
///Register `WPCR2` writer
pub type W = crate::W<WPCR2rs>;
///Field `TCLKPREP` reader - tCLK-PREPARE
pub type TCLKPREP_R = crate::FieldReader;
///Field `TCLKPREP` writer - tCLK-PREPARE
pub type TCLKPREP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TCLKZEO` reader - tCLK-ZERO
pub type TCLKZEO_R = crate::FieldReader;
///Field `TCLKZEO` writer - tCLK-ZERO
pub type TCLKZEO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `THSPREP` reader - tHS-PREPARE
pub type THSPREP_R = crate::FieldReader;
///Field `THSPREP` writer - tHS-PREPARE
pub type THSPREP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `THSTRAIL` reader - tHSTRAIL
pub type THSTRAIL_R = crate::FieldReader;
///Field `THSTRAIL` writer - tHSTRAIL
pub type THSTRAIL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - tCLK-PREPARE
    #[inline(always)]
    pub fn tclkprep(&self) -> TCLKPREP_R {
        TCLKPREP_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - tCLK-ZERO
    #[inline(always)]
    pub fn tclkzeo(&self) -> TCLKZEO_R {
        TCLKZEO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - tHS-PREPARE
    #[inline(always)]
    pub fn thsprep(&self) -> THSPREP_R {
        THSPREP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - tHSTRAIL
    #[inline(always)]
    pub fn thstrail(&self) -> THSTRAIL_R {
        THSTRAIL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR2")
            .field("thstrail", &self.thstrail())
            .field("thsprep", &self.thsprep())
            .field("tclkzeo", &self.tclkzeo())
            .field("tclkprep", &self.tclkprep())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - tCLK-PREPARE
    #[inline(always)]
    pub fn tclkprep(&mut self) -> TCLKPREP_W<'_, WPCR2rs> {
        TCLKPREP_W::new(self, 0)
    }
    ///Bits 8:15 - tCLK-ZERO
    #[inline(always)]
    pub fn tclkzeo(&mut self) -> TCLKZEO_W<'_, WPCR2rs> {
        TCLKZEO_W::new(self, 8)
    }
    ///Bits 16:23 - tHS-PREPARE
    #[inline(always)]
    pub fn thsprep(&mut self) -> THSPREP_W<'_, WPCR2rs> {
        THSPREP_W::new(self, 16)
    }
    ///Bits 24:31 - tHSTRAIL
    #[inline(always)]
    pub fn thstrail(&mut self) -> THSTRAIL_W<'_, WPCR2rs> {
        THSTRAIL_W::new(self, 24)
    }
}
/**DSI Wrapper PHY Configuration Register 2

You can [`read`](crate::Reg::read) this register and get [`wpcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DSI:WPCR2)*/
pub struct WPCR2rs;
impl crate::RegisterSpec for WPCR2rs {
    type Ux = u32;
}
///`read()` method returns [`wpcr2::R`](R) reader structure
impl crate::Readable for WPCR2rs {}
///`write(|w| ..)` method takes [`wpcr2::W`](W) writer structure
impl crate::Writable for WPCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPCR2 to value 0
impl crate::Resettable for WPCR2rs {}
