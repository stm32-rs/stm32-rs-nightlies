///Register `SIPCR` reader
pub type R = crate::R<SIPCRrs>;
///Register `SIPCR` writer
pub type W = crate::W<SIPCRrs>;
/**Field `SAES1` reader - Enable AES1 KEY\[7:0\]
security.*/
pub type SAES1_R = crate::BitReader;
/**Field `SAES1` writer - Enable AES1 KEY\[7:0\]
security.*/
pub type SAES1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAES2` reader - Enable AES2 security.
pub type SAES2_R = crate::BitReader;
///Field `SAES2` writer - Enable AES2 security.
pub type SAES2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPKA` reader - Enable PKA security
pub type SPKA_R = crate::BitReader;
///Field `SPKA` writer - Enable PKA security
pub type SPKA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRNG` reader - Enable True RNG security
pub type SRNG_R = crate::BitReader;
///Field `SRNG` writer - Enable True RNG security
pub type SRNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bit 0 - Enable AES1 KEY\[7:0\]
    security.*/
    #[inline(always)]
    pub fn saes1(&self) -> SAES1_R {
        SAES1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable AES2 security.
    #[inline(always)]
    pub fn saes2(&self) -> SAES2_R {
        SAES2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable PKA security
    #[inline(always)]
    pub fn spka(&self) -> SPKA_R {
        SPKA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable True RNG security
    #[inline(always)]
    pub fn srng(&self) -> SRNG_R {
        SRNG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIPCR")
            .field("saes1", &self.saes1())
            .field("saes2", &self.saes2())
            .field("spka", &self.spka())
            .field("srng", &self.srng())
            .finish()
    }
}
impl W {
    /**Bit 0 - Enable AES1 KEY\[7:0\]
    security.*/
    #[inline(always)]
    #[must_use]
    pub fn saes1(&mut self) -> SAES1_W<SIPCRrs> {
        SAES1_W::new(self, 0)
    }
    ///Bit 1 - Enable AES2 security.
    #[inline(always)]
    #[must_use]
    pub fn saes2(&mut self) -> SAES2_W<SIPCRrs> {
        SAES2_W::new(self, 1)
    }
    ///Bit 2 - Enable PKA security
    #[inline(always)]
    #[must_use]
    pub fn spka(&mut self) -> SPKA_W<SIPCRrs> {
        SPKA_W::new(self, 2)
    }
    ///Bit 3 - Enable True RNG security
    #[inline(always)]
    #[must_use]
    pub fn srng(&mut self) -> SRNG_W<SIPCRrs> {
        SRNG_W::new(self, 3)
    }
}
/**secure IP control register

You can [`read`](crate::Reg::read) this register and get [`sipcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sipcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG:SIPCR)*/
pub struct SIPCRrs;
impl crate::RegisterSpec for SIPCRrs {
    type Ux = u32;
}
///`read()` method returns [`sipcr::R`](R) reader structure
impl crate::Readable for SIPCRrs {}
///`write(|w| ..)` method takes [`sipcr::W`](W) writer structure
impl crate::Writable for SIPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SIPCR to value 0
impl crate::Resettable for SIPCRrs {
    const RESET_VALUE: u32 = 0;
}
