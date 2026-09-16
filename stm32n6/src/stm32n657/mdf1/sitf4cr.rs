///Register `SITF4CR` reader
pub type R = crate::R<SITF4CRrs>;
///Register `SITF4CR` writer
pub type W = crate::W<SITF4CRrs>;
///Field `SITFEN` reader - Serial interface enable
pub type SITFEN_R = crate::BitReader;
///Field `SITFEN` writer - Serial interface enable
pub type SITFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCKSRC` reader - Serial clock source
pub type SCKSRC_R = crate::FieldReader;
///Field `SCKSRC` writer - Serial clock source
pub type SCKSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SITFMOD` reader - Serial interface type
pub type SITFMOD_R = crate::FieldReader;
///Field `SITFMOD` writer - Serial interface type
pub type SITFMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `STH` reader - Manchester symbol threshold/SPI threshold
pub type STH_R = crate::FieldReader;
///Field `STH` writer - Manchester symbol threshold/SPI threshold
pub type STH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SITFACTIVE` reader - Serial interface active flag
pub type SITFACTIVE_R = crate::BitReader;
impl R {
    ///Bit 0 - Serial interface enable
    #[inline(always)]
    pub fn sitfen(&self) -> SITFEN_R {
        SITFEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Serial clock source
    #[inline(always)]
    pub fn scksrc(&self) -> SCKSRC_R {
        SCKSRC_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:5 - Serial interface type
    #[inline(always)]
    pub fn sitfmod(&self) -> SITFMOD_R {
        SITFMOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:12 - Manchester symbol threshold/SPI threshold
    #[inline(always)]
    pub fn sth(&self) -> STH_R {
        STH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 31 - Serial interface active flag
    #[inline(always)]
    pub fn sitfactive(&self) -> SITFACTIVE_R {
        SITFACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SITF4CR")
            .field("sitfen", &self.sitfen())
            .field("scksrc", &self.scksrc())
            .field("sitfmod", &self.sitfmod())
            .field("sth", &self.sth())
            .field("sitfactive", &self.sitfactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - Serial interface enable
    #[inline(always)]
    pub fn sitfen(&mut self) -> SITFEN_W<'_, SITF4CRrs> {
        SITFEN_W::new(self, 0)
    }
    ///Bits 1:2 - Serial clock source
    #[inline(always)]
    pub fn scksrc(&mut self) -> SCKSRC_W<'_, SITF4CRrs> {
        SCKSRC_W::new(self, 1)
    }
    ///Bits 4:5 - Serial interface type
    #[inline(always)]
    pub fn sitfmod(&mut self) -> SITFMOD_W<'_, SITF4CRrs> {
        SITFMOD_W::new(self, 4)
    }
    ///Bits 8:12 - Manchester symbol threshold/SPI threshold
    #[inline(always)]
    pub fn sth(&mut self) -> STH_W<'_, SITF4CRrs> {
        STH_W::new(self, 8)
    }
}
/**MDF serial interface control register 4

You can [`read`](crate::Reg::read) this register and get [`sitf4cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitf4cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MDF1:SITF4CR)*/
pub struct SITF4CRrs;
impl crate::RegisterSpec for SITF4CRrs {
    type Ux = u32;
}
///`read()` method returns [`sitf4cr::R`](R) reader structure
impl crate::Readable for SITF4CRrs {}
///`write(|w| ..)` method takes [`sitf4cr::W`](W) writer structure
impl crate::Writable for SITF4CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SITF4CR to value 0x1f00
impl crate::Resettable for SITF4CRrs {
    const RESET_VALUE: u32 = 0x1f00;
}
