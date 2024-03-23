#[doc = "Register `ICSR` reader"]
pub type R = crate::R<ICSRrs>;
#[doc = "Register `ICSR` writer"]
pub type W = crate::W<ICSRrs>;
#[doc = "Field `ALRAWF` reader - ALRAWF"]
pub type ALRAWF_R = crate::BitReader;
#[doc = "Field `ALRBWF` reader - ALRBWF"]
pub type ALRBWF_R = crate::BitReader;
#[doc = "Field `WUTWF` reader - WUTWF"]
pub type WUTWF_R = crate::BitReader;
#[doc = "Field `SHPF` reader - SHPF"]
pub type SHPF_R = crate::BitReader;
#[doc = "Field `INITS` reader - INITS"]
pub type INITS_R = crate::BitReader;
#[doc = "Field `RSF` reader - RSF"]
pub type RSF_R = crate::BitReader;
#[doc = "Field `RSF` writer - RSF"]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITF` reader - INITF"]
pub type INITF_R = crate::BitReader;
#[doc = "Field `INIT` reader - INIT"]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - INIT"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECALPF` reader - RECALPF"]
pub type RECALPF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ALRAWF"]
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALRBWF"]
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRBWF_R {
        ALRBWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WUTWF"]
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SHPF"]
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - INITS"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INITF"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - INIT"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - RECALPF"]
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<ICSRrs> {
        RSF_W::new(self, 5)
    }
    #[doc = "Bit 7 - INIT"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<ICSRrs> {
        INIT_W::new(self, 7)
    }
}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICSRrs;
impl crate::RegisterSpec for ICSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsr::R`](R) reader structure"]
impl crate::Readable for ICSRrs {}
#[doc = "`write(|w| ..)` method takes [`icsr::W`](W) writer structure"]
impl crate::Writable for ICSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICSR to value 0x07"]
impl crate::Resettable for ICSRrs {
    const RESET_VALUE: u32 = 0x07;
}
