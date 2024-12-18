///Register `CRL` reader
pub type R = crate::R<CRLrs>;
///Register `CRL` writer
pub type W = crate::W<CRLrs>;
///Field `SECF` reader - Second Flag
pub type SECF_R = crate::BitReader;
///Field `SECF` writer - Second Flag
pub type SECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRF` reader - Alarm Flag
pub type ALRF_R = crate::BitReader;
///Field `ALRF` writer - Alarm Flag
pub type ALRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OWF` reader - Overflow Flag
pub type OWF_R = crate::BitReader;
///Field `OWF` writer - Overflow Flag
pub type OWF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSF` reader - Registers Synchronized Flag
pub type RSF_R = crate::BitReader;
///Field `RSF` writer - Registers Synchronized Flag
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNF` reader - Configuration Flag
pub type CNF_R = crate::BitReader;
///Field `CNF` writer - Configuration Flag
pub type CNF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTOFF` reader - RTC operation OFF
pub type RTOFF_R = crate::BitReader;
impl R {
    ///Bit 0 - Second Flag
    #[inline(always)]
    pub fn secf(&self) -> SECF_R {
        SECF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm Flag
    #[inline(always)]
    pub fn alrf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow Flag
    #[inline(always)]
    pub fn owf(&self) -> OWF_R {
        OWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Registers Synchronized Flag
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configuration Flag
    #[inline(always)]
    pub fn cnf(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RTC operation OFF
    #[inline(always)]
    pub fn rtoff(&self) -> RTOFF_R {
        RTOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRL")
            .field("secf", &self.secf())
            .field("alrf", &self.alrf())
            .field("owf", &self.owf())
            .field("rsf", &self.rsf())
            .field("cnf", &self.cnf())
            .field("rtoff", &self.rtoff())
            .finish()
    }
}
impl W {
    ///Bit 0 - Second Flag
    #[inline(always)]
    pub fn secf(&mut self) -> SECF_W<CRLrs> {
        SECF_W::new(self, 0)
    }
    ///Bit 1 - Alarm Flag
    #[inline(always)]
    pub fn alrf(&mut self) -> ALRF_W<CRLrs> {
        ALRF_W::new(self, 1)
    }
    ///Bit 2 - Overflow Flag
    #[inline(always)]
    pub fn owf(&mut self) -> OWF_W<CRLrs> {
        OWF_W::new(self, 2)
    }
    ///Bit 3 - Registers Synchronized Flag
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<CRLrs> {
        RSF_W::new(self, 3)
    }
    ///Bit 4 - Configuration Flag
    #[inline(always)]
    pub fn cnf(&mut self) -> CNF_W<CRLrs> {
        CNF_W::new(self, 4)
    }
}
/**RTC Control Register Low

You can [`read`](crate::Reg::read) this register and get [`crl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#RTC:CRL)*/
pub struct CRLrs;
impl crate::RegisterSpec for CRLrs {
    type Ux = u32;
}
///`read()` method returns [`crl::R`](R) reader structure
impl crate::Readable for CRLrs {}
///`write(|w| ..)` method takes [`crl::W`](W) writer structure
impl crate::Writable for CRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CRL to value 0x20
impl crate::Resettable for CRLrs {
    const RESET_VALUE: u32 = 0x20;
}
