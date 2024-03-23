#[doc = "Register `DDRCTRL_ADDRMAP1` reader"]
pub type R = crate::R<DDRCTRL_ADDRMAP1rs>;
#[doc = "Register `DDRCTRL_ADDRMAP1` writer"]
pub type W = crate::W<DDRCTRL_ADDRMAP1rs>;
#[doc = "Field `ADDRMAP_BANK_B0` reader - ADDRMAP_BANK_B0"]
pub type ADDRMAP_BANK_B0_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_BANK_B0` writer - ADDRMAP_BANK_B0"]
pub type ADDRMAP_BANK_B0_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADDRMAP_BANK_B1` reader - ADDRMAP_BANK_B1"]
pub type ADDRMAP_BANK_B1_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_BANK_B1` writer - ADDRMAP_BANK_B1"]
pub type ADDRMAP_BANK_B1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ADDRMAP_BANK_B2` reader - ADDRMAP_BANK_B2"]
pub type ADDRMAP_BANK_B2_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_BANK_B2` writer - ADDRMAP_BANK_B2"]
pub type ADDRMAP_BANK_B2_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - ADDRMAP_BANK_B0"]
    #[inline(always)]
    pub fn addrmap_bank_b0(&self) -> ADDRMAP_BANK_B0_R {
        ADDRMAP_BANK_B0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - ADDRMAP_BANK_B1"]
    #[inline(always)]
    pub fn addrmap_bank_b1(&self) -> ADDRMAP_BANK_B1_R {
        ADDRMAP_BANK_B1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - ADDRMAP_BANK_B2"]
    #[inline(always)]
    pub fn addrmap_bank_b2(&self) -> ADDRMAP_BANK_B2_R {
        ADDRMAP_BANK_B2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ADDRMAP_BANK_B0"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_bank_b0(&mut self) -> ADDRMAP_BANK_B0_W<DDRCTRL_ADDRMAP1rs> {
        ADDRMAP_BANK_B0_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - ADDRMAP_BANK_B1"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_bank_b1(&mut self) -> ADDRMAP_BANK_B1_W<DDRCTRL_ADDRMAP1rs> {
        ADDRMAP_BANK_B1_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - ADDRMAP_BANK_B2"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_bank_b2(&mut self) -> ADDRMAP_BANK_B2_W<DDRCTRL_ADDRMAP1rs> {
        ADDRMAP_BANK_B2_W::new(self, 16)
    }
}
#[doc = "DDRCTRL address map register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ADDRMAP1rs;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_addrmap1::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_addrmap1::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP1 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP1rs {
    const RESET_VALUE: u32 = 0;
}
