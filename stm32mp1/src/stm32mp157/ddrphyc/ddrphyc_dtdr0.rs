#[doc = "Register `DDRPHYC_DTDR0` reader"]
pub type R = crate::R<DDRPHYC_DTDR0rs>;
#[doc = "Register `DDRPHYC_DTDR0` writer"]
pub type W = crate::W<DDRPHYC_DTDR0rs>;
#[doc = "Field `DTBYTE0` reader - DTBYTE0"]
pub type DTBYTE0_R = crate::FieldReader;
#[doc = "Field `DTBYTE0` writer - DTBYTE0"]
pub type DTBYTE0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DTBYTE1` reader - DTBYTE1"]
pub type DTBYTE1_R = crate::FieldReader;
#[doc = "Field `DTBYTE1` writer - DTBYTE1"]
pub type DTBYTE1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DTBYTE2` reader - DTBYTE2"]
pub type DTBYTE2_R = crate::FieldReader;
#[doc = "Field `DTBYTE2` writer - DTBYTE2"]
pub type DTBYTE2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DTBYTE3` reader - DTBYTE3"]
pub type DTBYTE3_R = crate::FieldReader;
#[doc = "Field `DTBYTE3` writer - DTBYTE3"]
pub type DTBYTE3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DTBYTE0"]
    #[inline(always)]
    pub fn dtbyte0(&self) -> DTBYTE0_R {
        DTBYTE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DTBYTE1"]
    #[inline(always)]
    pub fn dtbyte1(&self) -> DTBYTE1_R {
        DTBYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DTBYTE2"]
    #[inline(always)]
    pub fn dtbyte2(&self) -> DTBYTE2_R {
        DTBYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DTBYTE3"]
    #[inline(always)]
    pub fn dtbyte3(&self) -> DTBYTE3_R {
        DTBYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTBYTE0"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte0(&mut self) -> DTBYTE0_W<DDRPHYC_DTDR0rs> {
        DTBYTE0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DTBYTE1"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte1(&mut self) -> DTBYTE1_W<DDRPHYC_DTDR0rs> {
        DTBYTE1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DTBYTE2"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte2(&mut self) -> DTBYTE2_W<DDRPHYC_DTDR0rs> {
        DTBYTE2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DTBYTE3"]
    #[inline(always)]
    #[must_use]
    pub fn dtbyte3(&mut self) -> DTBYTE3_W<DDRPHYC_DTDR0rs> {
        DTBYTE3_W::new(self, 24)
    }
}
#[doc = "DDRPHYC DTD register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dtdr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dtdr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DTDR0rs;
impl crate::RegisterSpec for DDRPHYC_DTDR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dtdr0::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DTDR0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dtdr0::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DTDR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DTDR0 to value 0xdd22_ee11"]
impl crate::Resettable for DDRPHYC_DTDR0rs {
    const RESET_VALUE: u32 = 0xdd22_ee11;
}
