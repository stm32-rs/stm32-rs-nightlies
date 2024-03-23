#[doc = "Register `DDRPHYC_ZQ0CR0` reader"]
pub type R = crate::R<DDRPHYC_ZQ0CR0rs>;
#[doc = "Register `DDRPHYC_ZQ0CR0` writer"]
pub type W = crate::W<DDRPHYC_ZQ0CR0rs>;
#[doc = "Field `ZDATA` reader - ZDATA"]
pub type ZDATA_R = crate::FieldReader<u32>;
#[doc = "Field `ZDATA` writer - ZDATA"]
pub type ZDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `ZDEN` reader - ZDEN"]
pub type ZDEN_R = crate::BitReader;
#[doc = "Field `ZDEN` writer - ZDEN"]
pub type ZDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZCALBYP` reader - ZCALBYP"]
pub type ZCALBYP_R = crate::BitReader;
#[doc = "Field `ZCALBYP` writer - ZCALBYP"]
pub type ZCALBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZCAL` reader - ZCAL"]
pub type ZCAL_R = crate::BitReader;
#[doc = "Field `ZCAL` writer - ZCAL"]
pub type ZCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZQPD` reader - ZQPD"]
pub type ZQPD_R = crate::BitReader;
#[doc = "Field `ZQPD` writer - ZQPD"]
pub type ZQPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - ZDATA"]
    #[inline(always)]
    pub fn zdata(&self) -> ZDATA_R {
        ZDATA_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28 - ZDEN"]
    #[inline(always)]
    pub fn zden(&self) -> ZDEN_R {
        ZDEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ZCALBYP"]
    #[inline(always)]
    pub fn zcalbyp(&self) -> ZCALBYP_R {
        ZCALBYP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ZCAL"]
    #[inline(always)]
    pub fn zcal(&self) -> ZCAL_R {
        ZCAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ZQPD"]
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - ZDATA"]
    #[inline(always)]
    #[must_use]
    pub fn zdata(&mut self) -> ZDATA_W<DDRPHYC_ZQ0CR0rs> {
        ZDATA_W::new(self, 0)
    }
    #[doc = "Bit 28 - ZDEN"]
    #[inline(always)]
    #[must_use]
    pub fn zden(&mut self) -> ZDEN_W<DDRPHYC_ZQ0CR0rs> {
        ZDEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - ZCALBYP"]
    #[inline(always)]
    #[must_use]
    pub fn zcalbyp(&mut self) -> ZCALBYP_W<DDRPHYC_ZQ0CR0rs> {
        ZCALBYP_W::new(self, 29)
    }
    #[doc = "Bit 30 - ZCAL"]
    #[inline(always)]
    #[must_use]
    pub fn zcal(&mut self) -> ZCAL_W<DDRPHYC_ZQ0CR0rs> {
        ZCAL_W::new(self, 30)
    }
    #[doc = "Bit 31 - ZQPD"]
    #[inline(always)]
    #[must_use]
    pub fn zqpd(&mut self) -> ZQPD_W<DDRPHYC_ZQ0CR0rs> {
        ZQPD_W::new(self, 31)
    }
}
#[doc = "DDRPHYC ZQ0C register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_zq0cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_zq0cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_ZQ0CR0rs;
impl crate::RegisterSpec for DDRPHYC_ZQ0CR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_zq0cr0::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0CR0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_zq0cr0::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_ZQ0CR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_ZQ0CR0 to value 0x014a"]
impl crate::Resettable for DDRPHYC_ZQ0CR0rs {
    const RESET_VALUE: u32 = 0x014a;
}
