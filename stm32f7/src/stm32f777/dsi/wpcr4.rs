///Register `WPCR4` reader
pub type R = crate::R<WPCR4rs>;
///Register `WPCR4` writer
pub type W = crate::W<WPCR4rs>;
///Field `THSZERO` reader - tHS-ZERO
pub type THSZERO_R = crate::FieldReader;
///Field `THSZERO` writer - tHS-ZERO
pub type THSZERO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TLPXD` reader - tLPX for Data lanes
pub type TLPXD_R = crate::FieldReader;
///Field `TLPXD` writer - tLPX for Data lanes
pub type TLPXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `THSEXIT` reader - tHSEXIT
pub type THSEXIT_R = crate::FieldReader;
///Field `THSEXIT` writer - tHSEXIT
pub type THSEXIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TLPXC` reader - tLPXC for Clock lane
pub type TLPXC_R = crate::FieldReader;
///Field `TLPXC` writer - tLPXC for Clock lane
pub type TLPXC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - tHS-ZERO
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - tLPX for Data lanes
    #[inline(always)]
    pub fn tlpxd(&self) -> TLPXD_R {
        TLPXD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - tHSEXIT
    #[inline(always)]
    pub fn thsexit(&self) -> THSEXIT_R {
        THSEXIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - tLPXC for Clock lane
    #[inline(always)]
    pub fn tlpxc(&self) -> TLPXC_R {
        TLPXC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR4")
            .field("tlpxc", &self.tlpxc())
            .field("thsexit", &self.thsexit())
            .field("tlpxd", &self.tlpxd())
            .field("thszero", &self.thszero())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - tHS-ZERO
    #[inline(always)]
    pub fn thszero(&mut self) -> THSZERO_W<WPCR4rs> {
        THSZERO_W::new(self, 0)
    }
    ///Bits 8:15 - tLPX for Data lanes
    #[inline(always)]
    pub fn tlpxd(&mut self) -> TLPXD_W<WPCR4rs> {
        TLPXD_W::new(self, 8)
    }
    ///Bits 16:23 - tHSEXIT
    #[inline(always)]
    pub fn thsexit(&mut self) -> THSEXIT_W<WPCR4rs> {
        THSEXIT_W::new(self, 16)
    }
    ///Bits 24:31 - tLPXC for Clock lane
    #[inline(always)]
    pub fn tlpxc(&mut self) -> TLPXC_W<WPCR4rs> {
        TLPXC_W::new(self, 24)
    }
}
/**DSI_WPCR4

You can [`read`](crate::Reg::read) this register and get [`wpcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#DSI:WPCR4)*/
pub struct WPCR4rs;
impl crate::RegisterSpec for WPCR4rs {
    type Ux = u32;
}
///`read()` method returns [`wpcr4::R`](R) reader structure
impl crate::Readable for WPCR4rs {}
///`write(|w| ..)` method takes [`wpcr4::W`](W) writer structure
impl crate::Writable for WPCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPCR4 to value 0x3133_302a
impl crate::Resettable for WPCR4rs {
    const RESET_VALUE: u32 = 0x3133_302a;
}
