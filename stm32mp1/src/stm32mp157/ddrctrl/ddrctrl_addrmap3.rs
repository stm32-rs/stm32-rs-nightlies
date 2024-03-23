#[doc = "Register `DDRCTRL_ADDRMAP3` reader"]
pub type R = crate::R<DDRCTRL_ADDRMAP3rs>;
#[doc = "Register `DDRCTRL_ADDRMAP3` writer"]
pub type W = crate::W<DDRCTRL_ADDRMAP3rs>;
#[doc = "Field `ADDRMAP_COL_B6` reader - ADDRMAP_COL_B6"]
pub type ADDRMAP_COL_B6_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_COL_B6` writer - ADDRMAP_COL_B6"]
pub type ADDRMAP_COL_B6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRMAP_COL_B7` reader - ADDRMAP_COL_B7"]
pub type ADDRMAP_COL_B7_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_COL_B7` writer - ADDRMAP_COL_B7"]
pub type ADDRMAP_COL_B7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADDRMAP_COL_B8` reader - ADDRMAP_COL_B8"]
pub type ADDRMAP_COL_B8_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_COL_B8` writer - ADDRMAP_COL_B8"]
pub type ADDRMAP_COL_B8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADDRMAP_COL_B9` reader - ADDRMAP_COL_B9"]
pub type ADDRMAP_COL_B9_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_COL_B9` writer - ADDRMAP_COL_B9"]
pub type ADDRMAP_COL_B9_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_COL_B6"]
    #[inline(always)]
    pub fn addrmap_col_b6(&self) -> ADDRMAP_COL_B6_R {
        ADDRMAP_COL_B6_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - ADDRMAP_COL_B7"]
    #[inline(always)]
    pub fn addrmap_col_b7(&self) -> ADDRMAP_COL_B7_R {
        ADDRMAP_COL_B7_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADDRMAP_COL_B8"]
    #[inline(always)]
    pub fn addrmap_col_b8(&self) -> ADDRMAP_COL_B8_R {
        ADDRMAP_COL_B8_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADDRMAP_COL_B9"]
    #[inline(always)]
    pub fn addrmap_col_b9(&self) -> ADDRMAP_COL_B9_R {
        ADDRMAP_COL_B9_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_COL_B6"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b6(&mut self) -> ADDRMAP_COL_B6_W<DDRCTRL_ADDRMAP3rs> {
        ADDRMAP_COL_B6_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - ADDRMAP_COL_B7"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b7(&mut self) -> ADDRMAP_COL_B7_W<DDRCTRL_ADDRMAP3rs> {
        ADDRMAP_COL_B7_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - ADDRMAP_COL_B8"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b8(&mut self) -> ADDRMAP_COL_B8_W<DDRCTRL_ADDRMAP3rs> {
        ADDRMAP_COL_B8_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - ADDRMAP_COL_B9"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b9(&mut self) -> ADDRMAP_COL_B9_W<DDRCTRL_ADDRMAP3rs> {
        ADDRMAP_COL_B9_W::new(self, 24)
    }
}
#[doc = "DDRCTRL address map register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ADDRMAP3rs;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_addrmap3::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP3rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_addrmap3::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP3 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP3rs {
    const RESET_VALUE: u32 = 0;
}
