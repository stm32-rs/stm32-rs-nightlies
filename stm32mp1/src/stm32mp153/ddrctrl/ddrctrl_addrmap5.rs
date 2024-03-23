#[doc = "Register `DDRCTRL_ADDRMAP5` reader"]
pub type R = crate::R<DDRCTRL_ADDRMAP5rs>;
#[doc = "Register `DDRCTRL_ADDRMAP5` writer"]
pub type W = crate::W<DDRCTRL_ADDRMAP5rs>;
#[doc = "Field `ADDRMAP_ROW_B0` reader - ADDRMAP_ROW_B0"]
pub type ADDRMAP_ROW_B0_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_ROW_B0` writer - ADDRMAP_ROW_B0"]
pub type ADDRMAP_ROW_B0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRMAP_ROW_B1` reader - ADDRMAP_ROW_B1"]
pub type ADDRMAP_ROW_B1_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_ROW_B1` writer - ADDRMAP_ROW_B1"]
pub type ADDRMAP_ROW_B1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRMAP_ROW_B2_10` reader - ADDRMAP_ROW_B2_10"]
pub type ADDRMAP_ROW_B2_10_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_ROW_B2_10` writer - ADDRMAP_ROW_B2_10"]
pub type ADDRMAP_ROW_B2_10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRMAP_ROW_B11` reader - ADDRMAP_ROW_B11"]
pub type ADDRMAP_ROW_B11_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_ROW_B11` writer - ADDRMAP_ROW_B11"]
pub type ADDRMAP_ROW_B11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B0"]
    #[inline(always)]
    pub fn addrmap_row_b0(&self) -> ADDRMAP_ROW_B0_R {
        ADDRMAP_ROW_B0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B1"]
    #[inline(always)]
    pub fn addrmap_row_b1(&self) -> ADDRMAP_ROW_B1_R {
        ADDRMAP_ROW_B1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B2_10"]
    #[inline(always)]
    pub fn addrmap_row_b2_10(&self) -> ADDRMAP_ROW_B2_10_R {
        ADDRMAP_ROW_B2_10_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B11"]
    #[inline(always)]
    pub fn addrmap_row_b11(&self) -> ADDRMAP_ROW_B11_R {
        ADDRMAP_ROW_B11_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B0"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b0(&mut self) -> ADDRMAP_ROW_B0_W<DDRCTRL_ADDRMAP5rs> {
        ADDRMAP_ROW_B0_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B1"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b1(&mut self) -> ADDRMAP_ROW_B1_W<DDRCTRL_ADDRMAP5rs> {
        ADDRMAP_ROW_B1_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B2_10"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b2_10(&mut self) -> ADDRMAP_ROW_B2_10_W<DDRCTRL_ADDRMAP5rs> {
        ADDRMAP_ROW_B2_10_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B11"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b11(&mut self) -> ADDRMAP_ROW_B11_W<DDRCTRL_ADDRMAP5rs> {
        ADDRMAP_ROW_B11_W::new(self, 24)
    }
}
#[doc = "DDRCTRL address map register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ADDRMAP5rs;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_addrmap5::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP5rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_addrmap5::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP5 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP5rs {
    const RESET_VALUE: u32 = 0;
}
