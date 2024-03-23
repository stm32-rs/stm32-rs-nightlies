#[doc = "Register `DDRCTRL_ADDRMAP4` reader"]
pub type R = crate::R<DDRCTRL_ADDRMAP4rs>;
#[doc = "Register `DDRCTRL_ADDRMAP4` writer"]
pub type W = crate::W<DDRCTRL_ADDRMAP4rs>;
#[doc = "Field `ADDRMAP_COL_B10` reader - ADDRMAP_COL_B10"]
pub type ADDRMAP_COL_B10_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_COL_B10` writer - ADDRMAP_COL_B10"]
pub type ADDRMAP_COL_B10_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADDRMAP_COL_B11` reader - ADDRMAP_COL_B11"]
pub type ADDRMAP_COL_B11_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_COL_B11` writer - ADDRMAP_COL_B11"]
pub type ADDRMAP_COL_B11_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - ADDRMAP_COL_B10"]
    #[inline(always)]
    pub fn addrmap_col_b10(&self) -> ADDRMAP_COL_B10_R {
        ADDRMAP_COL_B10_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADDRMAP_COL_B11"]
    #[inline(always)]
    pub fn addrmap_col_b11(&self) -> ADDRMAP_COL_B11_R {
        ADDRMAP_COL_B11_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADDRMAP_COL_B10"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b10(&mut self) -> ADDRMAP_COL_B10_W<DDRCTRL_ADDRMAP4rs> {
        ADDRMAP_COL_B10_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - ADDRMAP_COL_B11"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b11(&mut self) -> ADDRMAP_COL_B11_W<DDRCTRL_ADDRMAP4rs> {
        ADDRMAP_COL_B11_W::new(self, 8)
    }
}
#[doc = "DDRCTRL address map register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ADDRMAP4rs;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_addrmap4::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP4rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_addrmap4::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP4 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP4rs {
    const RESET_VALUE: u32 = 0;
}
