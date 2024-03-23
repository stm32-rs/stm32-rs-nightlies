#[doc = "Register `DDRPHYC_DTPR2` reader"]
pub type R = crate::R<DDRPHYC_DTPR2rs>;
#[doc = "Register `DDRPHYC_DTPR2` writer"]
pub type W = crate::W<DDRPHYC_DTPR2rs>;
#[doc = "Field `TXS` reader - TXS"]
pub type TXS_R = crate::FieldReader<u16>;
#[doc = "Field `TXS` writer - TXS"]
pub type TXS_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TXP` reader - TXP"]
pub type TXP_R = crate::FieldReader;
#[doc = "Field `TXP` writer - TXP"]
pub type TXP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKE` reader - TCKE"]
pub type TCKE_R = crate::FieldReader;
#[doc = "Field `TCKE` writer - TCKE"]
pub type TCKE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDLLK` reader - TDLLK"]
pub type TDLLK_R = crate::FieldReader<u16>;
#[doc = "Field `TDLLK` writer - TDLLK"]
pub type TDLLK_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - TXS"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:14 - TXP"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:18 - TCKE"]
    #[inline(always)]
    pub fn tcke(&self) -> TCKE_R {
        TCKE_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:28 - TDLLK"]
    #[inline(always)]
    pub fn tdllk(&self) -> TDLLK_R {
        TDLLK_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TXS"]
    #[inline(always)]
    #[must_use]
    pub fn txs(&mut self) -> TXS_W<DDRPHYC_DTPR2rs> {
        TXS_W::new(self, 0)
    }
    #[doc = "Bits 10:14 - TXP"]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TXP_W<DDRPHYC_DTPR2rs> {
        TXP_W::new(self, 10)
    }
    #[doc = "Bits 15:18 - TCKE"]
    #[inline(always)]
    #[must_use]
    pub fn tcke(&mut self) -> TCKE_W<DDRPHYC_DTPR2rs> {
        TCKE_W::new(self, 15)
    }
    #[doc = "Bits 19:28 - TDLLK"]
    #[inline(always)]
    #[must_use]
    pub fn tdllk(&mut self) -> TDLLK_W<DDRPHYC_DTPR2rs> {
        TDLLK_W::new(self, 19)
    }
}
#[doc = "DDRPHYC DTP register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dtpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dtpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DTPR2rs;
impl crate::RegisterSpec for DDRPHYC_DTPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dtpr2::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR2rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dtpr2::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DTPR2 to value 0x2004_0d84"]
impl crate::Resettable for DDRPHYC_DTPR2rs {
    const RESET_VALUE: u32 = 0x2004_0d84;
}
