///Register `WPCR3` reader
pub type R = crate::R<WPCR3rs>;
///Register `WPCR3` writer
pub type W = crate::W<WPCR3rs>;
///Field `THSZERO` reader - tHS-ZERO
pub type THSZERO_R = crate::FieldReader;
///Field `THSZERO` writer - tHS-ZERO
pub type THSZERO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TLPXD` reader - tLPX for data lanes
pub type TLPXD_R = crate::FieldReader;
///Field `TLPXD` writer - tLPX for data lanes
pub type TLPXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `THSEXIT` reader - tHSEXIT
pub type THSEXIT_R = crate::FieldReader;
///Field `THSEXIT` writer - tHSEXIT
pub type THSEXIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TLPXC` reader - tLPXC for clock lane
pub type TLPXC_R = crate::FieldReader;
///Field `TLPXC` writer - tLPXC for clock lane
pub type TLPXC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - tHS-ZERO
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - tLPX for data lanes
    #[inline(always)]
    pub fn tlpxd(&self) -> TLPXD_R {
        TLPXD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - tHSEXIT
    #[inline(always)]
    pub fn thsexit(&self) -> THSEXIT_R {
        THSEXIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - tLPXC for clock lane
    #[inline(always)]
    pub fn tlpxc(&self) -> TLPXC_R {
        TLPXC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR3")
            .field("thszero", &self.thszero())
            .field("tlpxd", &self.tlpxd())
            .field("thsexit", &self.thsexit())
            .field("tlpxc", &self.tlpxc())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - tHS-ZERO
    #[inline(always)]
    pub fn thszero(&mut self) -> THSZERO_W<'_, WPCR3rs> {
        THSZERO_W::new(self, 0)
    }
    ///Bits 8:15 - tLPX for data lanes
    #[inline(always)]
    pub fn tlpxd(&mut self) -> TLPXD_W<'_, WPCR3rs> {
        TLPXD_W::new(self, 8)
    }
    ///Bits 16:23 - tHSEXIT
    #[inline(always)]
    pub fn thsexit(&mut self) -> THSEXIT_W<'_, WPCR3rs> {
        THSEXIT_W::new(self, 16)
    }
    ///Bits 24:31 - tLPXC for clock lane
    #[inline(always)]
    pub fn tlpxc(&mut self) -> TLPXC_W<'_, WPCR3rs> {
        TLPXC_W::new(self, 24)
    }
}
/**DSI wrapper PHY configuration register 3

You can [`read`](crate::Reg::read) this register and get [`wpcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#DSIHOST:WPCR3)*/
pub struct WPCR3rs;
impl crate::RegisterSpec for WPCR3rs {
    type Ux = u32;
}
///`read()` method returns [`wpcr3::R`](R) reader structure
impl crate::Readable for WPCR3rs {}
///`write(|w| ..)` method takes [`wpcr3::W`](W) writer structure
impl crate::Writable for WPCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPCR3 to value 0
impl crate::Resettable for WPCR3rs {}
