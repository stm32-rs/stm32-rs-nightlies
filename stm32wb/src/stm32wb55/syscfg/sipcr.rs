#[doc = "Register `SIPCR` reader"]
pub type R = crate::R<SIPCRrs>;
#[doc = "Register `SIPCR` writer"]
pub type W = crate::W<SIPCRrs>;
#[doc = "Field `SAES1` reader - Enable AES1 KEY\\[7:0\\]
security."]
pub type SAES1_R = crate::BitReader;
#[doc = "Field `SAES1` writer - Enable AES1 KEY\\[7:0\\]
security."]
pub type SAES1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAES2` reader - Enable AES2 security."]
pub type SAES2_R = crate::BitReader;
#[doc = "Field `SAES2` writer - Enable AES2 security."]
pub type SAES2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPKA` reader - Enable PKA security"]
pub type SPKA_R = crate::BitReader;
#[doc = "Field `SPKA` writer - Enable PKA security"]
pub type SPKA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRNG` reader - Enable True RNG security"]
pub type SRNG_R = crate::BitReader;
#[doc = "Field `SRNG` writer - Enable True RNG security"]
pub type SRNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable AES1 KEY\\[7:0\\]
security."]
    #[inline(always)]
    pub fn saes1(&self) -> SAES1_R {
        SAES1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable AES2 security."]
    #[inline(always)]
    pub fn saes2(&self) -> SAES2_R {
        SAES2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable PKA security"]
    #[inline(always)]
    pub fn spka(&self) -> SPKA_R {
        SPKA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable True RNG security"]
    #[inline(always)]
    pub fn srng(&self) -> SRNG_R {
        SRNG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable AES1 KEY\\[7:0\\]
security."]
    #[inline(always)]
    #[must_use]
    pub fn saes1(&mut self) -> SAES1_W<SIPCRrs> {
        SAES1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable AES2 security."]
    #[inline(always)]
    #[must_use]
    pub fn saes2(&mut self) -> SAES2_W<SIPCRrs> {
        SAES2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable PKA security"]
    #[inline(always)]
    #[must_use]
    pub fn spka(&mut self) -> SPKA_W<SIPCRrs> {
        SPKA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable True RNG security"]
    #[inline(always)]
    #[must_use]
    pub fn srng(&mut self) -> SRNG_W<SIPCRrs> {
        SRNG_W::new(self, 3)
    }
}
#[doc = "secure IP control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sipcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sipcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIPCRrs;
impl crate::RegisterSpec for SIPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sipcr::R`](R) reader structure"]
impl crate::Readable for SIPCRrs {}
#[doc = "`write(|w| ..)` method takes [`sipcr::W`](W) writer structure"]
impl crate::Writable for SIPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIPCR to value 0"]
impl crate::Resettable for SIPCRrs {
    const RESET_VALUE: u32 = 0;
}
