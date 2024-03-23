#[doc = "Register `C2AHB3ENR` reader"]
pub type R = crate::R<C2AHB3ENRrs>;
#[doc = "Register `C2AHB3ENR` writer"]
pub type W = crate::W<C2AHB3ENRrs>;
#[doc = "Field `PKAEN` reader - CPU2 PKAEN"]
pub type PKAEN_R = crate::BitReader;
#[doc = "Field `PKAEN` writer - CPU2 PKAEN"]
pub type PKAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES2EN` reader - CPU2 AES2EN"]
pub type AES2EN_R = crate::BitReader;
#[doc = "Field `AES2EN` writer - CPU2 AES2EN"]
pub type AES2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGEN` reader - CPU2 RNGEN"]
pub type RNGEN_R = crate::BitReader;
#[doc = "Field `RNGEN` writer - CPU2 RNGEN"]
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEMEN` reader - CPU2 HSEMEN"]
pub type HSEMEN_R = crate::BitReader;
#[doc = "Field `HSEMEN` writer - CPU2 HSEMEN"]
pub type HSEMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPCCEN` reader - CPU2 IPCCEN"]
pub type IPCCEN_R = crate::BitReader;
#[doc = "Field `IPCCEN` writer - CPU2 IPCCEN"]
pub type IPCCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHEN` reader - CPU2 FLASHEN"]
pub type FLASHEN_R = crate::BitReader;
#[doc = "Field `FLASHEN` writer - CPU2 FLASHEN"]
pub type FLASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - CPU2 PKAEN"]
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU2 AES2EN"]
    #[inline(always)]
    pub fn aes2en(&self) -> AES2EN_R {
        AES2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU2 RNGEN"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU2 HSEMEN"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU2 IPCCEN"]
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU2 FLASHEN"]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - CPU2 PKAEN"]
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<C2AHB3ENRrs> {
        PKAEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU2 AES2EN"]
    #[inline(always)]
    #[must_use]
    pub fn aes2en(&mut self) -> AES2EN_W<C2AHB3ENRrs> {
        AES2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU2 RNGEN"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<C2AHB3ENRrs> {
        RNGEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - CPU2 HSEMEN"]
    #[inline(always)]
    #[must_use]
    pub fn hsemen(&mut self) -> HSEMEN_W<C2AHB3ENRrs> {
        HSEMEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - CPU2 IPCCEN"]
    #[inline(always)]
    #[must_use]
    pub fn ipccen(&mut self) -> IPCCEN_W<C2AHB3ENRrs> {
        IPCCEN_W::new(self, 20)
    }
    #[doc = "Bit 25 - CPU2 FLASHEN"]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<C2AHB3ENRrs> {
        FLASHEN_W::new(self, 25)
    }
}
#[doc = "CPU2 AHB3 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ahb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ahb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2AHB3ENRrs;
impl crate::RegisterSpec for C2AHB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2ahb3enr::R`](R) reader structure"]
impl crate::Readable for C2AHB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`c2ahb3enr::W`](W) writer structure"]
impl crate::Writable for C2AHB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2AHB3ENR to value 0x0208_0000"]
impl crate::Resettable for C2AHB3ENRrs {
    const RESET_VALUE: u32 = 0x0208_0000;
}
